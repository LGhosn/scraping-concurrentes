use serde::{Deserialize, Serialize};

/// Estructura que contiene los datos de una etiqueta.
/// # Atributos
/// * `questions` - Cantidad de preguntas.
/// * `words` - Cantidad de palabras.
/// # Ejemplo
/// ```rust
/// let tag_data = TagData {
///     questions: 0,
///     words: 0,
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct TagData {
    pub questions: usize,
    pub words: usize,
}
