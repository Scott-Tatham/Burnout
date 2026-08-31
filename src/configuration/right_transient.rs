/*!
 * Stores the right transient prompt configuration.
 */
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RightTransientConfiguration
{
    pub enabled: bool,
    pub modules: Vec<String>,
}