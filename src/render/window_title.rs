/*!
 * Handles the rendering of the window title.
 */

pub struct WindowTitle;

use crate::configuration::core_configuration;
use super::Display;

/**
 * Implements the [Display] trait for the window title.
 */
impl Display for WindowTitle
{
    /**
    * Renders the value for the window title.
    * # Arguments
    * * `configuration` - The configuration with the window title values.
    */
    fn render_display(configuration: core_configuration::base_configuration::BaseConfiguration)
    {
        println!("{}", &configuration.window_title.unwrap_or_default().content.as_deref().unwrap_or_default());
    }
}