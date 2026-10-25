/*!
 * Handles the rendering of display elements.
 */
use super::continuation::ContinuationPrompt;
use super::display::Display;
use super::prompt::Prompt;
use super::right::RightPrompt;
use super::right_transient::RightTransientPrompt;
use super::transient::TransientPrompt;
use super::window_title::WindowTitle;
use super::super::configuration::core_configuration::file;

/// Invokes the display of the specified display element.
/// # Arguments
/// * `display` - The name of the display element to draw.
/// * `shell` - The name of the shell to target.
/// # Returns
/// * `Ok(())` - Returns that the display was successful.
/// * `Err(String)` - Returns that the display failed.
pub fn render_display(display: &str, shell: &str) -> Result<(), String>
{
    let configuration = file::load_or_create_configuration();

    match display
    {
        "prompt" => Prompt::render_display(configuration, shell),
        "right" =>  RightPrompt::render_display(configuration, shell),
        "transient" =>  TransientPrompt::render_display(configuration, shell),
        "right-transient" =>  RightTransientPrompt::render_display(configuration, shell),
        "continuation" => ContinuationPrompt::render_display(configuration, shell),
        "window-title" =>  WindowTitle::render_display(configuration, shell),
        _ => Err(format!("Unsupported display element for rendering: {}.", display))
    }
}