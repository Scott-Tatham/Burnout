/*!
 * Handles the rendering of the window title.
 */
pub struct WindowTitle;

use crate::configuration::core_configuration;
use super::Display;

/**
 * Implements the [Display] trait for the window title.
 */
impl Display for WindowTitle
{
    /**
    * Renders the value for the window title.
    * # Arguments
    * * `configuration` - The configuration with the window title values.
    * * `shell` - The shell to target with the window title values.
    */
    fn render_display(configuration: core_configuration::base_configuration::BaseConfiguration, shell: &str)
    {
        match shell
        {
            "bash" => println!("{}", &configuration.bash.unwrap_or_default().window_title.unwrap_or_default().content.as_deref().unwrap_or_default()),
            "zsh" => println!("{}", &configuration.zsh.unwrap_or_default().window_title.unwrap_or_default().content.as_deref().unwrap_or_default()),
            "git_bash" => println!("{}", &configuration.git_bash.unwrap_or_default().window_title.unwrap_or_default().content.as_deref().unwrap_or_default()),
            "powershell" | "pwsh" => println!("{}", &configuration.powershell.unwrap_or_default().window_title.unwrap_or_default().content.as_deref().unwrap_or_default()),
            "cmd" => println!("{}", &configuration.cmd.unwrap_or_default().window_title.unwrap_or_default().content.as_deref().unwrap_or_default()),
            _ => eprintln!("Unsupported shell for rendering: {}", shell)
        }
    }
}