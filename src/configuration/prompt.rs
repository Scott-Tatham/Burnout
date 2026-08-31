/*!
 * Stores the prompt configuration.
 */
use serde::{Serialize, Deserialize};

/**
 * Stores the prompt configuration.
 */
#[derive(Serialize, Deserialize)]
pub struct PromptConfiguration
{
    pub content: Option<String>
}

/**
 * Implements the [Default] trait for the prompt.
 */
impl Default for PromptConfiguration
{
    fn default() -> Self
    {
        Self
        {
            content: Some("Prompt".to_string())
        }
    }
}