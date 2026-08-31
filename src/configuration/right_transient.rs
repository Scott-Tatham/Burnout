/*!
 * Stores the right transient prompt configuration.
 */
use serde::{Serialize, Deserialize};

/**
 * Stores the right transient prompt configuration.
 */
#[derive(Serialize, Deserialize)]
pub struct RightTransientConfiguration
{
    pub content: Option<String>
}

/**
 * Implements the [Default] trait for the right side transient prompt.
 */
impl Default for RightTransientConfiguration
{
    fn default() -> Self
    {
        Self
        {
            content: Some("Right Transient".to_string())
        }
    }
}