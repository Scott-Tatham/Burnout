/*!
 * Handles the rendering of the right transient prompt.
 */
use super::display::Display;
use super::super::configuration::core_configuration::base_configuration::BaseConfiguration;

pub struct RightTransientPrompt;

/// Implements the [Display] trait for the [RightTransientPrompt].
impl Display for RightTransientPrompt
{
    /// Renders the value for the right transient prompt.
    /// # Arguments
    /// * `configuration` - The configuration with the right transient prompt values.
    /// * `shell` - The shell to target with the right transient prompt values.
    /// # Returns
    /// * `Ok(())` - Returns that the rendering was successful.
    /// * `Err(String)` - Returns that the rendering failed.
    fn render_display(configuration: BaseConfiguration, shell: &str) -> Result<(), String>
    {
        match shell
        {
            "bash" => println!("{}", &configuration.bash.expect("Failed to get the Bash configuration.").right_transient.expect("Failed to get the Bash right transient.").content.expect("Failed to get the Bash content.")),
            "git_bash" => println!("{}", &configuration.bash.expect("Failed to get the Git Bash configuration.").right_transient.expect("Failed to get the Git Bash right transient.").content.expect("Failed to get the Git Bash content.")),
            "zsh" => println!("{}", &configuration.bash.expect("Failed to get the Zsh configuration.").right_transient.expect("Failed to get the Zsh right transient.").content.expect("Failed to get the Zsh content.")),
            "powershell" | "pwsh" => println!("{}", &configuration.bash.expect("Failed to get the PowerShell configuration.").right_transient.expect("Failed to get the PowerShell right transient.").content.expect("Failed to get the PowerShell content.")),
            "cmd" => println!("{}", &configuration.bash.expect("Failed to get the Cmd configuration.").right_transient.expect("Failed to get the Cmd right transient.").content.expect("Failed to get the Cmd content.")),
            _ => return Err(format!("Unsupported shell for rendering: {}.", shell))
        }

        Ok(())
    }
}