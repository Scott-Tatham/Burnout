/*!
 * Handles the rendering of the right transient prompt.
 */

pub struct RightTransientPrompt;

use crate::configuration::core_configuration;
use super::Display;

/**
 * Implements the [Display] trait for the right transient prompt.
 */
impl Display for RightTransientPrompt
{
    /**
    * Renders the value for the right transient prompt.
    * # Arguments
    * * `configuration` - The configuration with the right transient prompt values.
    */
    fn render_display(configuration: core_configuration::base_configuration::BaseConfiguration)
    {
        println!("{}", &configuration.right_transient.unwrap_or_default().content.as_deref().unwrap_or_default());
    }
}