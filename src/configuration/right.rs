use serde::Deserialize;

#[derive(Deserialize)]
pub struct RightConfiguration
{
    pub enabled: bool,
    pub modules: Vec<String>,
}