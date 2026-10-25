/*!
 * Handles the rendering of the continuation prompt.
 */
use super::display::Display;
use super::super::configuration::core_configuration::base_configuration::BaseConfiguration;

pub struct ContinuationPrompt;

/// Implements the [Display] trait for the [ContinuationPrompt].
impl Display for ContinuationPrompt
{
    /// Renders the value for the continuation prompt.
    /// # Arguments
    /// * `configuration` - The configuration with the continuation prompt values.
    /// * `shell` - The shell to target with the continuation prompt values.
    /// # Returns
    /// * `Ok(())` - Returns that the rendering was successful.
    /// * `Err(String)` - Returns that the rendering failed.
    fn render_display(configuration: BaseConfiguration, shell: &str) -> Result<(), String>
    {
        match shell
        {
            "bash" => println!("{}", &configuration.bash.expect("Failed to get the Bash configuration.").continuation.expect("Failed to get the Bash continuation.").content.expect("Failed to get the Bash content.")),
            "git_bash" => println!("{}", &configuration.bash.expect("Failed to get the Git Bash configuration.").continuation.expect("Failed to get the Git Bash continuation.").content.expect("Failed to get the Git Bash content.")),
            "zsh" => println!("{}", &configuration.bash.expect("Failed to get the Zsh configuration.").continuation.expect("Failed to get the Zsh continuation.").content.expect("Failed to get the Zsh content.")),
            "powershell" | "pwsh" => println!("{}", &configuration.bash.expect("Failed to get the PowerShell configuration.").continuation.expect("Failed to get the PowerShell continuation.").content.expect("Failed to get the PowerShell content.")),
            "cmd" => println!("{}", &configuration.bash.expect("Failed to get the Cmd configuration.").continuation.expect("Failed to get the Cmd continuation.").content.expect("Failed to get the Cmd content.")),
            _ => return Err(format!("Unsupported shell for rendering: {}.", shell))
        }

        Ok(())
    }
}