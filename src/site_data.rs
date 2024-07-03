use crate::tag_data;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tag_data::TagData;

/// Estructura que contiene los datos de un sitio.
/// # Atributos
/// * `questions` - Cantidad de preguntas.
/// * `words` - Cantidad de palabras.
/// * `tags` - HashMap con los datos de las etiquetas.
/// * `chatty_tags` - Vector con los 10 tags con mayor relación words/questions para ese sitio
/// # Ejemplo
/// ```rust
/// let tags: HashMap<String, TagData> = HashMap::new();
/// let site_data = SiteData {
///     questions: 0,
///     words: 0,
///     tags
///     chatty_tags: Vec::new(),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct SiteData {
    pub questions: usize,
    pub words: usize,
    pub tags: HashMap<String, TagData>,
    pub chatty_tags: Vec<String>,
}
