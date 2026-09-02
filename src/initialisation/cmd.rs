/*!
 * Handles the initialisation of the Cmd shell.
 */

pub struct Cmd;

use crate::configuration::shell_configuration;
use super::Shell;

/**
 * Implements the [Shell] trait for the Cmd shell.
 */
impl Shell for Cmd
{
    /**
     * Prints the prompt initialisation code for the Cmd shell.
     * By printing the initialisation, it sets the prompt for that session.
     * # Arguments
     * * `configuration` - The configuration with the Cmd shell initialisation values.
     */
    fn print_initialisation(configuration: shell_configuration::base_configuration::BaseConfiguration)
    {
        println!("{}", &configuration.cmd.unwrap_or_default().setup.as_deref().unwrap_or_default());
    }
}