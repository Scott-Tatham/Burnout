/*!
 * Handles the rendering of display elements.
 */
use super::super::configuration::core_configuration::displays::display_configuration::DisplayConfiguration;
use super::super::configuration::core_configuration::file;
use super::super::configuration::core_configuration::segment::SegmentConfiguration;
use super::super::configuration::core_configuration::shells::shell_configuration::ShellConfiguration;

/// Renders the specified display element for the specified shell.
/// # Arguments
/// * `shell` - The name of the shell to target.
/// * `display` - The name of the display element to draw.
/// # Returns
/// * `Ok(())` - Returns that the rendering was successful.
/// * `Err(String)` - Returns that the rendering failed.
pub fn render(shell: &str, display: &str) -> Result<(), String>
{
    let configuration = file::load_or_create_configuration();

    match shell
    {
        "bash" => render_shell(display, configuration.bash.expect("Failed to get the Bash configuration."))?,
        "git_bash" => render_shell(display, configuration.git_bash.expect("Failed to get the Git Bash configuration."))?,
        "zsh" => render_shell(display, configuration.zsh.expect("Failed to get the Zsh configuration."))?,
        "powershell" | "pwsh" => render_shell(display, configuration.powershell.expect("Failed to get the PowerShell configuration."))?,
        "cmd" => render_shell(display, configuration.cmd.expect("Failed to get the Cmd configuration."))?,
        _ => return Err(format!("Unsupported shell for rendering: {}.", shell))
    }

    Ok(())
}

/// Renders the specified display element based on the shell.
/// # Arguments
/// * `display` - The name of the display element to draw.
/// * `configuration` - The configuration with the display values.
/// # Returns
/// * `Ok(())` - Returns that the rendering was successful.
/// * `Err(String)` - Returns that the rendering failed.
fn render_shell(display: &str, configuration: ShellConfiguration) -> Result<(), String>
{
    match display
    {
        "prompt" => render_display(configuration.prompt.expect("Failed to get the prompt configuration.")),
        "transient" => render_display(configuration.transient.expect("Failed to get the transient configuration.")),
        "right" => render_display(configuration.right.expect("Failed to get the right configuration.")),
        "right_transient" => render_display(configuration.right_transient.expect("Failed to get the right transient configuration.")),
        "window_title" => render_display(configuration.window_title.expect("Failed to get the window title configuration.")),
        _ => return Err(format!("Unsupported display element for rendering: {}.", display))
    }

    Ok(())
}

/// Renders the display element.
/// # Arguments
/// * `configuration` - The configuration with the display values.
fn render_display(configuration: DisplayConfiguration)
{
    let mut output = String::default();
    let segments = configuration.segments.expect("Failed to get the segments configuration.");

    for segment in segments
    {
        output.push_str(render_segment(segment).as_str());
    }

    println!("{}", output)
}

/// Renders the segment.
/// # Arguments
/// * `segment` - The configuration with the segment values.
/// # Returns
/// * `String` - Returns the rendered segment.
fn render_segment(segment: SegmentConfiguration) -> String
{
    let mut output = String::default();
    output.push_str(segment.prefix.unwrap_or_default().as_str());
    output.push_str(segment.content.unwrap_or_default().as_str());
    output.push_str(segment.suffix.unwrap_or_default().as_str());

    output
}