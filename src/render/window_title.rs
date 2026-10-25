/*!
 * Handles the rendering of the window title.
 */
use super::display::Display;
use super::super::configuration::core_configuration::base_configuration::BaseConfiguration;

pub struct WindowTitle;

/// Implements the [Display] trait for the [WindowTitle].
impl Display for WindowTitle
{
    /// Renders the value for the window title.
    /// # Arguments
    /// * `configuration` - The configuration with the window title values.
    /// * `shell` - The shell to target with the window title values.
    /// # Returns
    /// * `Ok(())` - Returns that the rendering was successful.
    /// * `Err(String)` - Returns that the rendering failed.
    fn render_display(configuration: BaseConfiguration, shell: &str) -> Result<(), String>
    {
        match shell
        {
            "bash" => println!("{}", &configuration.bash.expect("Failed to get the Bash configuration.").window_title.expect("Failed to get the Bash window title.").content.expect("Failed to get the Bash content.")),
            "git_bash" => println!("{}", &configuration.bash.expect("Failed to get the Git Bash configuration.").window_title.expect("Failed to get the Git Bash window title.").content.expect("Failed to get the Git Bash content.")),
            "zsh" => println!("{}", &configuration.bash.expect("Failed to get the Zsh configuration.").window_title.expect("Failed to get the Zsh window title.").content.expect("Failed to get the Zsh content.")),
            "powershell" | "pwsh" => println!("{}", &configuration.bash.expect("Failed to get the PowerShell configuration.").window_title.expect("Failed to get the PowerShell window title.").content.expect("Failed to get the PowerShell content.")),
            "cmd" => println!("{}", &configuration.bash.expect("Failed to get the Cmd configuration.").window_title.expect("Failed to get the Cmd window title.").content.expect("Failed to get the Cmd content.")),
            _ => return Err(format!("Unsupported shell for rendering: {}.", shell))
        }

        Ok(())
    }
}