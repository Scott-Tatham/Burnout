/*!
 * Handles the rendering of the prompt.
 */

pub struct Prompt;

use crate::configuration::core_configuration;
use super::Display;

/**
 * Implements the [Display] trait for the prompt.
 */
impl Display for Prompt
{
    /**
    * Renders the value for the prompt.
    * # Arguments
    * * `configuration` - The configuration with the prompt values.
    */
    fn render_display(configuration: core_configuration::base_configuration::BaseConfiguration)
    {
        println!("{}", &configuration.prompt.unwrap_or_default().content.as_deref().unwrap_or_default());
    }
}