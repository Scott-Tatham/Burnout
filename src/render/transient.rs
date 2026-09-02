/*!
 * Handles the rendering of the transient prompt.
 */

pub struct TransientPrompt;

use crate::configuration::core_configuration;
use super::Display;

/**
 * Implements the [Display] trait for the transient prompt.
 */
impl Display for TransientPrompt
{
    /**
    * Renders the value for the transient prompt.
    * # Arguments
    * * `configuration` - The configuration with the transient prompt values.
    */
    fn render_display(configuration: core_configuration::base_configuration::BaseConfiguration)
    {
        println!("{}", &configuration.transient.unwrap_or_default().content.as_deref().unwrap_or_default());
    }
}