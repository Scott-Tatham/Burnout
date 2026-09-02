//! Handles Cmd specific functionality.

pub struct Cmd;

use crate::configuration::shell_configuration;
use super::Shell;

/**
 * Implements the [Shell] trait for Cmd.
 */
impl Shell for Cmd
{
    /**
     * Prints the prompt initialisation code for Cmd.
     * By printing the initialisation, it sets the prompt for that session.
     * # Arguments
     * * `configuration` - The configuration for shells.
     */
    fn print_initialisation(configuration: shell_configuration::base_configuration::BaseConfiguration)
    {
        println!("{}", &configuration.cmd.unwrap_or_default().setup.as_deref().unwrap_or_default());
    }
}