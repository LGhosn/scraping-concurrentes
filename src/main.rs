mod general_stats;
mod site_data;
mod tag_data;
mod total_stats;

use general_stats::GeneralStats;
use rayon::prelude::*;
use serde_json::Value;
use site_data::SiteData;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;
use tag_data::TagData;
use total_stats::TotalStats;

/// Directorio donde se encuentran los archivos a procesar.
const PATH_DIR: &str = "./data";

/// Dado un ``Path`` a un archivo, procesa el contenido del archivo y devuelve un tuple con la cantidad de preguntas, palabras y un HashMap con las etiquetas y sus datos.
/// # Argumentos
/// * `entry` - Un ``Path`` al archivo a procesar.
/// # Retorno
/// Un ``Result`` con un tuple que contiene la cantidad de preguntas, palabras y un HashMap con las etiquetas y sus datos.
/// En caso de error, se devuelve un ``io::Error``.
/// # Ejemplo
/// ```rust
/// let entry = Path::new("data/stackoverflow.com.json");
/// let (questions, words, tags) = match process_entry(entry) {
///     Ok((questions, words, tags)) => (questions, words, tags),
///     Err(e) => {
///         eprintln!("Error al procesar los archivos: {}", e);
///         return Ok(());
///     }
/// };
/// ```
/// # Notas
/// * Se asume que el archivo tiene el formato correcto.
fn process_json_file(entry: &Path) -> Result<(usize, usize, HashMap<String, TagData>), io::Error> {
    let content = fs::read_to_string(entry)?;
    let mut questions = 0;
    let mut words = 0;
    let mut tags_count: HashMap<String, TagData> = HashMap::new();

    for line in content.lines() {
        let json_data: Value = serde_json::from_str(line)?;
        let mut line_words: usize = 0;

        let texts = json_data["texts"]
            .as_array()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No 'texts' field found"))?;

        questions += 1; // como cada linea tiene el formato -> "text": ["title", "body"]. Indica que solo hay una pregunta por linea
        for text in texts {
            if let Some(text_str) = text.as_str() {
                words += text_str.split_whitespace().count();
                line_words += text_str.split_whitespace().count();
            }
        }
        let tags = json_data["tags"]
            .as_array()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "No 'tags' field found"))?;

        for tag in tags {
            if let Some(tag_str) = tag.as_str() {
                let tag_entry = tags_count.entry(tag_str.to_string()).or_insert(TagData {
                    questions: 0,
                    words: 0,
                });
                tag_entry.questions += 1; // Incrementa el contador de preguntas para esta etiqueta
                tag_entry.words += line_words; // Incrementa el contador de palabras para esta etiqueta
            }
        }
    }

    Ok((questions, words, tags_count))
}

/// Procesa en paralelo todos los archivos en el directorio ``path_dir`` y devuelve un ``GeneralStats`` con los datos de los sitios y etiquetas.
/// # Argumentos
/// * `path_dir` - Un string con el directorio a procesar.
/// # Retorno
/// Un ``Result`` con un ``GeneralStats`` con los datos de los sitios y etiquetas.
/// En caso de error, se devuelve un ``io::Error``.
/// # Ejemplo
/// ```rust
/// let processed = match process_directory("path/to/data") {
///     Ok(stats) => stats,
///     Err(e) => {
///         eprintln!("Error al procesar los archivos: {}", e);
///         return Ok(());
///     }
/// };
/// ```
/// # Notas
/// * Se asume que los archivos tienen el formato correcto.
/// * Si no se puede leer el directorio, se devuelve un ``GeneralStats`` vacío.
fn process_directory(path_dir: &str) -> Result<GeneralStats, io::Error> {
    let entries = match fs::read_dir(path_dir) {
        Ok(entries) => entries
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?,
        Err(e) => {
            eprintln!("Error al leer el directorio '{}': {}", path_dir, e);
            // Si no se puede leer el directorio, devuelvo un GeneralStats vacío
            return Ok(GeneralStats {
                padron: "106998".to_string(),
                sites: HashMap::new(),
                tags: HashMap::new(),
                totals: TotalStats {
                    chatty_sites: Vec::new(),
                    chatty_tags: Vec::new(),
                },
            });
        }
    };

    let info_sites: Vec<HashMap<String, SiteData>> = entries
        .par_iter()
        .filter_map(|entry| {
            let file_name = entry
                .file_name()
                .and_then(OsStr::to_str)
                .map(|s| s.to_string())?;
            // si el archivo es el .git o el .md lo ignoro
            if !file_name.contains(".json") {
                return None;
            }
            let result = process_json_file(entry);
            match result {
                Ok((questions, words, tags)) => {
                    let chatty_tags = get_tag_top_ten(&tags);
                    let mut info_site: HashMap<String, SiteData> = HashMap::new();
                    let site_data = SiteData {
                        questions,
                        words,
                        tags,
                        chatty_tags,
                    };
                    info_site.insert(file_name, site_data);
                    Some(info_site)
                }
                Err(e) => {
                    eprintln!("Error al procesar el archivo '{}': {}", entry.display(), e);
                    None
                }
            }
        })
        .collect();

    // Calculo los 10 tags con mayor relación palabras/preguntas entre todos los sitios
    let mut tag_data: HashMap<String, TagData> = HashMap::new();

    // Calculo los totales para cada tag
    for site in info_sites.iter().flatten() {
        for (tag, data) in site.1.tags.iter() {
            let tag_entry = tag_data.entry(tag.to_string()).or_insert(TagData {
                questions: 0,
                words: 0,
            });
            tag_entry.questions += data.questions;
            tag_entry.words += data.words;
        }
    }

    // Obtengo los 10 tags más "chatty"
    let chatty_tags = get_tag_top_ten(&tag_data);

    // Calculo los 10 sitios con mayor relación palabras/preguntas
    let chatty_sites = get_site_top_ten(&info_sites);

    let totals = TotalStats {
        chatty_sites,
        chatty_tags,
    };

    let info_sites: HashMap<String, SiteData> = info_sites.into_iter().flatten().collect();

    Ok(GeneralStats {
        padron: "106998".to_string(),
        sites: info_sites,
        tags: tag_data,
        totals,
    })
}

/// Dado un HashMap con los datos de las etiquetas, devuelve un vector con los 10 tags con mayor relación palabras/preguntas.
/// # Argumentos
/// * `data` - Un HashMap con los datos de las etiquetas.
/// # Retorno
/// Un vector con los 10 tags con mayor relación palabras/preguntas.
/// # Ejemplo
/// ```rust
/// let tags = HashMap::new();
/// let chatty_tags = get_tag_top_ten(&tags);
/// ```
fn get_tag_top_ten(data: &HashMap<String, TagData>) -> Vec<String> {
    let mut tags_ratio: Vec<(String, usize)> = data
        .iter()
        .map(|(tag, data)| (tag.clone(), data.words / data.questions))
        .collect();
    tags_ratio.sort_by_key(|&(_, ratio)| ratio);
    tags_ratio
        .iter()
        .rev()
        .take(10)
        .map(|(tag, _)| tag.clone())
        .collect()
}

/// Dado un vector con HashMaps con los datos de los sitios, devuelve un vector con los 10 sitios con mayor relación palabras/preguntas.
/// # Argumentos
/// * `data` - Un vector con HashMaps con los datos de los sitios.
/// # Retorno
/// Un vector con los 10 sitios con mayor relación palabras/preguntas.
/// # Ejemplo
/// ```rust
/// let sites = Vec::new();
/// let chatty_sites = get_site_top_ten(&sites);
/// ```
fn get_site_top_ten(data: &Vec<HashMap<String, SiteData>>) -> Vec<String> {
    let mut sites_ratio: Vec<(String, usize)> = data
        .iter()
        .flatten()
        .map(|(site, data)| (site.clone(), data.words / data.questions))
        .collect();

    sites_ratio.sort_by_key(|&(_, ratio)| ratio);
    let chatty_sites: Vec<String> = sites_ratio
        .iter()
        .rev()
        .take(10)
        .map(|(site, _)| site.clone())
        .collect();
    chatty_sites
}

/// Función principal que procesa los archivos y muestra los resultados.
fn main() -> io::Result<()> {
    // Obtener el número de hilos desde la línea de comandos
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Uso: {} <num_hilos>", args[0]);
        return Ok(());
    }
    let num_threads: usize = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("El argumento debe ser un número entero");
            return Ok(());
        }
    };

    let thread_pool_result = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global();

    match thread_pool_result {
        Ok(_) => {
            // ThreadPool creado correctamente
        }
        Err(err) => {
            eprintln!("Error al crear el ThreadPool: {}", err);
            return Ok(());
        }
    }

    let processed = match process_directory(PATH_DIR) {
        Ok(stats) => stats,
        Err(e) => {
            eprintln!("Error al procesar los archivos: {}", e);
            return Ok(());
        }
    };

    println!("{}", serde_json::to_string_pretty(&processed)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_process_directory() {
        let processed = match process_directory("notFoundDirectory") {
            Ok(stats) => stats,
            Err(e) => {
                eprintln!("Error al procesar los archivos: {}", e);
                return;
            }
        };

        assert_eq!(processed.padron, "106998");
        assert_eq!(processed.sites.len(), 0);
        assert_eq!(processed.tags.len(), 0);
        assert_eq!(processed.totals.chatty_sites.len(), 0);
        assert_eq!(processed.totals.chatty_tags.len(), 0);
    }

    #[test]
    fn test_process_directory() {
        //verifico que exista el directorio
        let entry = Path::new("./dataTests");
        if !entry.exists() {
            eprintln!("El directorio '{}' no existe", entry.display());
            return;
        }
        let processed = match process_directory("./dataTests") {
            Ok(stats) => stats,
            Err(e) => {
                eprintln!("Error al procesar los archivos: {}", e);
                return;
            }
        };

        assert_eq!(processed.padron, "106998");
        assert_eq!(processed.sites.len(), 2);
        assert_eq!(processed.tags.len(), 4);
        assert_eq!(processed.totals.chatty_sites.len(), 2);
        assert_eq!(processed.totals.chatty_tags.len(), 4);
    }

    #[test]
    fn test_process_json_file() {
        let entry = Path::new("dataTests/data1.jsonl");
        if !entry.exists() {
            eprintln!("El archivo '{}' no existe", entry.display());
            return;
        }
        let (questions, words, tags) = match process_json_file(entry) {
            Ok((questions, words, tags)) => (questions, words, tags),
            Err(e) => {
                eprintln!("Error al procesar los archivos: {}", e);
                return;
            }
        };

        assert_eq!(questions, 3);
        assert_eq!(words, 142);
        assert_eq!(tags.len(), 4);
    }

    #[test]
    fn test_get_tag_top_ten() {
        let mut tags: HashMap<String, TagData> = HashMap::new();
        for i in 1..=11 {
            let tag_name = format!("tag{}", i);
            let questions = if i == 11 { 1000 } else { 12 - i };
            tags.insert(
                tag_name,
                TagData {
                    questions,
                    words: 1000,
                },
            );
        }

        let chatty_tags = get_tag_top_ten(&tags);
        assert_eq!(
            chatty_tags,
            vec!["tag10", "tag9", "tag8", "tag7", "tag6", "tag5", "tag4", "tag3", "tag2", "tag1"]
        );
    }

    #[test]
    fn test_get_site_top_ten() {
        let mut sites: Vec<HashMap<String, SiteData>> = Vec::new();
        for i in 1..=11 {
            let mut site_data: HashMap<String, SiteData> = HashMap::new();
            let site_name = format!("site{}", i);
            let questions = if i == 11 { 100 } else { 12 - i };

            site_data.insert(
                site_name,
                SiteData {
                    questions,
                    words: 100,
                    tags: HashMap::new(),
                    chatty_tags: Vec::new(),
                },
            );
            sites.push(site_data);
        }

        let chatty_sites = get_site_top_ten(&sites);
        assert_eq!(
            chatty_sites,
            vec![
                "site10", "site9", "site8", "site7", "site6", "site5", "site4", "site3", "site2",
                "site1"
            ]
        );
    }
}
