/*!
 * Handles the rendering of the prompt.
 */
use super::super::configuration::core_configuration::base_configuration::BaseConfiguration;
use super::display::Display;

pub struct Prompt;

/// Implements the [Display] trait for the [Prompt].
impl Display for Prompt
{
    /// Renders the value for the prompt.
    /// # Arguments
    /// * `configuration` - The configuration with the prompt values.
    /// * `shell` - The shell to target with the prompt values.
    /// # Returns
    /// * `Ok(())` - Returns that the rendering was successful.
    /// * `Err(String)` - Returns that the rendering failed.
    fn render_display(configuration: BaseConfiguration, shell: &str) -> Result<(), String>
    {
        match shell
        {
            "bash" => println!("{}", &configuration.bash.expect("Failed to get the Bash configuration.").prompt.expect("Failed to get the Bash prompt.").content.expect("Failed to get the Bash content.")),
            "git_bash" => println!("{}", &configuration.bash.expect("Failed to get the Git Bash configuration.").prompt.expect("Failed to get the Git Bash prompt.").content.expect("Failed to get the Git Bash content.")),
            "zsh" => println!("{}", &configuration.bash.expect("Failed to get the Zsh configuration.").prompt.expect("Failed to get the Zsh prompt.").content.expect("Failed to get the Zsh content.")),
            "powershell" | "pwsh" => println!("{}", &configuration.bash.expect("Failed to get the PowerShell configuration.").prompt.expect("Failed to get the PowerShell prompt.").content.expect("Failed to get the PowerShell content.")),
            "cmd" => println!("{}", &configuration.bash.expect("Failed to get the Cmd configuration.").prompt.expect("Failed to get the Cmd prompt.").content.expect("Failed to get the Cmd content.")),
            _ => return Err(format!("Unsupported shell for rendering: {}.", shell))
        }

        Ok(())
    }
}