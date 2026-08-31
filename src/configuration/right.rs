/*!
 * Stores the right prompt configuration.
 */
use serde::{Serialize, Deserialize};

/**
 * Stores the right prompt configuration.
 */
#[derive(Serialize, Deserialize)]
pub struct RightConfiguration
{
    pub content: Option<String>
}

/**
 * Implements the [Default] trait for the right side prompt.
 */
impl Default for RightConfiguration
{
    fn default() -> Self
    {
        Self
        {
            content: Some("Right".to_string())
        }
    }
}