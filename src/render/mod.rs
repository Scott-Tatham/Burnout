/*!
 * Handles rendering of the prompt.
 */

use crate::configuration::*;

/**
 * Defines the implementation of a module.
 */
pub trait Module
{
    /**
     * Generates the value for the module.
     */
    fn generate_value() -> str;
}

/**
 * Renders the prompt with the specified modules.
 * # Arguments
 * * `configuration` - The configuration for the prompt.
 */
pub fn render_prompt(configuration: &prompt::PromptConfiguration)
{
    println!("{}", &configuration.content.as_deref().unwrap_or_default());
}

/**
 * Renders the right side prompt with the specified modules.
 * # Arguments
 * * `configuration` - The configuration for the right side prompt.
 */
pub fn render_right_prompt(configuration: &right::RightConfiguration)
{
    println!("{}", &configuration.content.as_deref().unwrap_or_default());
}

/**
 * Renders the transient prompt with the specified modules.
 * # Arguments
 * * `configuration` - The configuration for the transient prompt.
 */
pub fn render_transient_prompt(configuration: &transient::TransientConfiguration)
{
    println!("{}", &configuration.content.as_deref().unwrap_or_default());
}

/**
 * Renders the right side transient prompt with the specified modules.
 * # Arguments
 * * `configuration` - The configuration for the right side transient prompt.
 */
pub fn render_right_transient_prompt(configuration: &right_transient::RightTransientConfiguration)
{
    println!("{}", &configuration.content.as_deref().unwrap_or_default());
}

/**
 * Renders the window title with the specified modules.
 * # Arguments
 * * `configuration` - The configuration for the window title.
 */
pub fn render_window_title(configuration: &window_title::WindowTitleConfiguration)
{
    println!("{}", &configuration.content.as_deref().unwrap_or_default());
}