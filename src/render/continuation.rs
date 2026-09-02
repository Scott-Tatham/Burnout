/*!
 * Handles the rendering of the continuation prompt.
 */

pub struct ContinuationPrompt;

use crate::configuration::core_configuration;
use super::Display;

/**
 * Implements the [Display] trait for the continuation prompt.
 */
impl Display for ContinuationPrompt
{
    /**
    * Renders the value for the continuation prompt.
    * # Arguments
    * * `configuration` - The configuration with the continuation prompt values.
    */
    fn render_display(configuration: core_configuration::base_configuration::BaseConfiguration)
    {
        println!("{}", &configuration.continuation.unwrap_or_default().content.as_deref().unwrap_or_default());
    }
}