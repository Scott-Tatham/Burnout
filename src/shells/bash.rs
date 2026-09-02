/*!
 * Handles Bash specific functionality.
 */

pub struct Bash;

use crate::configuration::shell_configuration;
use super::Shell;

/**
 * Implements the [Shell] trait for Bash.
 */
impl Shell for Bash
{
    /**
     * Prints the prompt initialisation code for Bash.
     * By printing the initialisation, it sets the prompt for that session.
     * # Arguments
     * * `configuration` - The configuration for shells.
     */
    fn print_initialisation(configuration: shell_configuration::base_configuration::BaseConfiguration)
    {
        println!("{}", &configuration.bash.unwrap_or_default().setup.as_deref().unwrap_or_default());
    }
}