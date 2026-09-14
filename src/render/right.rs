/*!
 * Handles the rendering of the right prompt.
 */
pub struct RightPrompt;

use crate::configuration::core_configuration;
use super::Display;

/**
 * Implements the [Display] trait for the right prompt.
 */
impl Display for RightPrompt
{
    /**
    * Renders the value for the right prompt.
    * # Arguments
    * * `configuration` - The configuration with the right prompt values.
    * * `shell` - The shell to target with the right prompt values.
    */
    fn render_display(configuration: core_configuration::base_configuration::BaseConfiguration, shell: &str)
    {
        match shell
        {
            "bash" => println!("{}", &configuration.bash.unwrap_or_default().right.unwrap_or_default().content.as_deref().unwrap_or_default()),
            "zsh" => println!("{}", &configuration.zsh.unwrap_or_default().right.unwrap_or_default().content.as_deref().unwrap_or_default()),
            "git_bash" => println!("{}", &configuration.git_bash.unwrap_or_default().right.unwrap_or_default().content.as_deref().unwrap_or_default()),
            "powershell" | "pwsh" => println!("{}", &configuration.powershell.unwrap_or_default().right.unwrap_or_default().content.as_deref().unwrap_or_default()),
            "cmd" => println!("{}", &configuration.cmd.unwrap_or_default().right.unwrap_or_default().content.as_deref().unwrap_or_default()),
            _ => eprintln!("Unsupported shell for rendering: {}", shell)
        }
    }
}