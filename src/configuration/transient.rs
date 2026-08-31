/*!
 * Stores the transient prompt configuration.
 */
use serde::{Serialize, Deserialize};

/**
 * Stores the transient prompt configuration.
 */
#[derive(Serialize, Deserialize)]
pub struct TransientConfiguration
{
    pub content: Option<String>
}

/**
 * Implements the [Default] trait for the transient prompt.
 */
impl Default for TransientConfiguration
{
    fn default() -> Self
    {
        Self
        {
            content: Some("Transient".to_string())
        }
    }
}