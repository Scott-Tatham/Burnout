/*!
 * Stores the window title configuration.
 */
use serde::{Serialize, Deserialize};

/**
 * Stores the window title configuration.
 */
#[derive(Serialize, Deserialize)]
pub struct WindowTitleConfiguration
{
    pub content: Option<String>
}

/**
 * Implements the [Default] trait for the window title.
 */
impl Default for WindowTitleConfiguration
{
    fn default() -> Self
    {
        Self
        {
            content: Some(" Burnout ".to_string())
        }
    }
}