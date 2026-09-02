/*!
 * Handles Zsh specific functionality.
 */

use crate::configuration::shell_configuration;
use super::Shell;

pub struct Zsh;

/**
 * Implements the [Shell] trait for Zsh.
 */
impl Shell for Zsh
{
    /**
     * Prints the prompt initialisation code for Zsh.
     * By printing the initialisation, it sets the prompt for that session.
     * # Arguments
     * * `configuration` - The configuration for shells.
     */
    fn print_initialisation(configuration: shell_configuration::base_configuration::BaseConfiguration)
    {
        println!("{}", &configuration.zsh.unwrap_or_default().setup.as_deref().unwrap_or_default());
    }
}