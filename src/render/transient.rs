/*!
 * Handles the rendering of the transient prompt.
 */
pub struct TransientPrompt;

use crate::configuration::core_configuration;
use super::Display;

/**
 * Implements the [Display] trait for the transient prompt.
 */
impl Display for TransientPrompt
{
    /**
    * Renders the value for the transient prompt.
    * # Arguments
    * * `configuration` - The configuration with the transient prompt values.
    * * `shell` - The shell to target with the transient prompt values.
    */
    fn render_display(configuration: core_configuration::base_configuration::BaseConfiguration, shell: &str)
    {
        match shell
        {
            "bash" => println!("{}", &configuration.bash.unwrap_or_default().transient.unwrap_or_default().content.as_deref().unwrap_or_default()),
            "zsh" => println!("{}", &configuration.zsh.unwrap_or_default().transient.unwrap_or_default().content.as_deref().unwrap_or_default()),
            "git_bash" => println!("{}", &configuration.git_bash.unwrap_or_default().transient.unwrap_or_default().content.as_deref().unwrap_or_default()),
            "powershell" | "pwsh" => println!("{}", &configuration.powershell.unwrap_or_default().transient.unwrap_or_default().content.as_deref().unwrap_or_default()),
            "cmd" => println!("{}", &configuration.cmd.unwrap_or_default().transient.unwrap_or_default().content.as_deref().unwrap_or_default()),
            _ => eprintln!("Unsupported shell for rendering: {}", shell)
        }
    }
}