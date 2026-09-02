/*!
 * Handles the rendering of the right prompt.
 */

pub struct RightPrompt;

use crate::configuration::core_configuration;
use super::Display;

/**
 * Implements the [Display] trait for the right prompt.
 */
impl Display for RightPrompt
{
    /**
    * Renders the value for the right prompt.
    * # Arguments
    * * `configuration` - The configuration with the right prompt values.
    */
    fn render_display(configuration: core_configuration::base_configuration::BaseConfiguration)
    {
        println!("{}", &configuration.right.unwrap_or_default().content.as_deref().unwrap_or_default());
    }
}