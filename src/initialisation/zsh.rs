/*!
 * Handles the initialisation of the Zsh shell.
 */

use crate::configuration::shell_configuration;
use super::Shell;

pub struct Zsh;

/**
 * Implements the [Shell] trait for the Zsh shell.
 */
impl Shell for Zsh
{
    /**
     * Prints the prompt initialisation code for the Zsh shell.
     * By printing the initialisation, it sets the prompt for that session.
     * # Arguments
     * * `configuration` - The configuration with the Zsh shell initialisation values.
     */
    fn print_initialisation(configuration: shell_configuration::base_configuration::BaseConfiguration)
    {
        println!("{}", &configuration.zsh.unwrap_or_default().setup.as_deref().unwrap_or_default());
    }
}