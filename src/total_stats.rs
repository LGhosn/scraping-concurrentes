use serde::{Deserialize, Serialize};

/// Estructura que contiene los datos totales de los sitios y etiquetas.
/// # Atributos
/// * `chatty_sites` - Vector con los 10 sitios con mayor relación words/questions.
/// * `chatty_tags` - Vector con los 10 tags con mayor relación words/questions.
/// # Ejemplo
/// ```rust
/// let totals = TotalStats {
///     chatty_sites: Vec::new(),
///     chatty_tags: Vec::new(),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct TotalStats {
    pub chatty_sites: Vec<String>,
    pub chatty_tags: Vec<String>,
}
