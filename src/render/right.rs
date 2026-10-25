/*!
 * Handles the rendering of the right prompt.
 */
pub struct RightPrompt;

use super::display::Display;
use super::super::configuration::core_configuration::base_configuration::BaseConfiguration;

/// Implements the [Display] trait for the [RightPrompt].
impl Display for RightPrompt
{
    /// Renders the value for the right prompt.
    /// # Arguments
    /// * `configuration` - The configuration with the right prompt values.
    /// * `shell` - The shell to target with the right prompt values.
    /// # Returns
    /// * `Ok(())` - Returns that the rendering was successful.
    /// * `Err(String)` - Returns that the rendering failed.
    fn render_display(configuration: BaseConfiguration, shell: &str) -> Result<(), String>
    {
        match shell
        {
            "bash" => println!("{}", &configuration.bash.expect("Failed to get the Bash configuration.").right.expect("Failed to get the Bash right.").content.expect("Failed to get the Bash content.")),
            "git_bash" => println!("{}", &configuration.bash.expect("Failed to get the Git Bash configuration.").right.expect("Failed to get the Git Bash right.").content.expect("Failed to get the Git Bash content.")),
            "zsh" => println!("{}", &configuration.bash.expect("Failed to get the Zsh configuration.").right.expect("Failed to get the Zsh right.").content.expect("Failed to get the Zsh content.")),
            "powershell" | "pwsh" => println!("{}", &configuration.bash.expect("Failed to get the PowerShell configuration.").right.expect("Failed to get the PowerShell right.").content.expect("Failed to get the PowerShell content.")),
            "cmd" => println!("{}", &configuration.bash.expect("Failed to get the Cmd configuration.").right.expect("Failed to get the Cmd right.").content.expect("Failed to get the Cmd content.")),
            _ => return Err(format!("Unsupported shell for rendering: {}.", shell))
        }

        Ok(())
    }
}