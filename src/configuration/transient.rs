use serde::Deserialize;

#[derive(Deserialize)]
pub struct TransientConfiguration
{
    pub enabled: bool,
    pub modules: Vec<String>,
}