use crate::site_data;
use crate::tag_data;
use crate::total_stats;
use serde::{Deserialize, Serialize};
use site_data::SiteData;
use std::collections::HashMap;
use tag_data::TagData;
use total_stats::TotalStats;

/// Estructura que contiene todos los datos de los sitios y etiquetas.
/// # Atributos
/// * `padron` - Número de padrón del alumno.
/// * `sites` - HashMap con los datos de los sitios.
/// * `tags` - HashMap con los datos de las etiquetas.
/// * `totals` - Datos totales de los sitios y etiquetas.
/// # Ejemplo
/// ```rust
/// let sites: HashMap<String, SiteData> = HashMap::new();
/// let tags: HashMap<String, TagData> = HashMap::new();
/// let totals = TotalStats {
///     chatty_sites: Vec::new(),
///     chatty_tags: Vec::new(),
/// };
/// let processed = GeneralStats {
///     padron: "106998".to_string(),
///     sites
///     tags
///     totals
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct GeneralStats {
    pub padron: String,
    pub sites: HashMap<String, SiteData>,
    pub tags: HashMap<String, TagData>,
    pub totals: TotalStats,
}
