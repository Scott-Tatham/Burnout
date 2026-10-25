/*!
 * Handles the rendering of the transient prompt.
 */
use super::display::Display;
use super::super::configuration::core_configuration::base_configuration::BaseConfiguration;

pub struct TransientPrompt;

/// Implements the [Display] trait for the [TransientPrompt].
impl Display for TransientPrompt
{
    /// Renders the value for the transient prompt.
    /// # Arguments
    /// * `configuration` - The configuration with the transient prompt values.
    /// * `shell` - The shell to target with the transient prompt values.
    /// # Returns
    /// * `Ok(())` - Returns that the rendering was successful.
    /// * `Err(String)` - Returns that the rendering failed.
    fn render_display(configuration: BaseConfiguration, shell: &str) -> Result<(), String>
    {
        match shell
        {
            "bash" => println!("{}", &configuration.bash.expect("Failed to get the Bash configuration.").transient.expect("Failed to get the Bash transient.").content.expect("Failed to get the Bash content.")),
            "git_bash" => println!("{}", &configuration.bash.expect("Failed to get the Git Bash configuration.").transient.expect("Failed to get the Git Bash transient.").content.expect("Failed to get the Git Bash content.")),
            "zsh" => println!("{}", &configuration.bash.expect("Failed to get the Zsh configuration.").transient.expect("Failed to get the Zsh transient.").content.expect("Failed to get the Zsh content.")),
            "powershell" | "pwsh" => println!("{}", &configuration.bash.expect("Failed to get the PowerShell configuration.").transient.expect("Failed to get the PowerShell transient.").content.expect("Failed to get the PowerShell content.")),
            "cmd" => println!("{}", &configuration.bash.expect("Failed to get the Cmd configuration.").transient.expect("Failed to get the Cmd transient.").content.expect("Failed to get the Cmd content.")),
            _ => return Err(format!("Unsupported shell for rendering: {}.", shell))
        }

        Ok(())
    }
}