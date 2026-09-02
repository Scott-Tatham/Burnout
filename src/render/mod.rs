/*!
 * Handles the rendering of the display elements.
 */

use crate::configuration::core_configuration;

mod prompt;
mod right;
mod transient;
mod right_transient;
mod continuation;
mod window_title;

/**
 * Defines the implementation of a shell.
 */
trait Display
{
    /**
     * Renders the value for the display.
     * # Arguments
     * * `configuration` - The configuration with the display values.
     */
    fn render_display(configuration: core_configuration::base_configuration::BaseConfiguration);
}

/**
 * Defines the implementation of a module.
 */
trait Module
{
    /**
     * Generates the value for the module.
     */
    fn generate_value() -> str;
}

/**
 * Invokes the display of the specified display element.
 * # Arguments
 * * `display` - The name of the display to initialise.
 */
pub fn render_display(display: &str)
{
    let configuration = core_configuration::load_or_create_configuration();

    match display
    {
        "prompt" => prompt::Prompt::render_display(configuration),
        "right" =>  right::RightPrompt::render_display(configuration),
        "transient" =>  transient::TransientPrompt::render_display(configuration),
        "right-transient" =>  right_transient::RightTransientPrompt::render_display(configuration),
        "continuation" => continuation::ContinuationPrompt::render_display(configuration),
        "window-title" =>  window_title::WindowTitle::render_display(configuration),
        _ => eprintln!("Unsupported display element: {}", display)
    }
}