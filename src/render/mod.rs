/*!
 * Handles the rendering of the display elements.
 */
mod prompt;
mod right;
mod transient;
mod right_transient;
mod continuation;
mod window_title;

use crate::configuration::core_configuration;

/**
 * Defines the implementation of a shell.
 */
trait Display
{
    /**
     * Renders the value for the display.
     * # Arguments
     * * `configuration` - The configuration with the display values.
     * * `shell` - The target shell to render to.
     */
    fn render_display(configuration: core_configuration::base_configuration::BaseConfiguration, shell: &str);
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
 * * `shell` - The name of the shell to initialise.
 */
pub fn render_display(display: &str, shell: &str)
{
    let configuration = core_configuration::load_or_create_configuration();

    match display
    {
        "prompt" => prompt::Prompt::render_display(configuration, shell),
        "right" =>  right::RightPrompt::render_display(configuration, shell),
        "transient" =>  transient::TransientPrompt::render_display(configuration, shell),
        "right-transient" =>  right_transient::RightTransientPrompt::render_display(configuration, shell),
        "continuation" => continuation::ContinuationPrompt::render_display(configuration, shell),
        "window-title" =>  window_title::WindowTitle::render_display(configuration, shell),
        _ => eprintln!("Unsupported display element: {}", display)
    }
}