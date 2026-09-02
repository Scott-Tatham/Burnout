//! Handles PowerShell specific functionality.

use crate::configuration::shell_configuration;
use super::Shell;

pub struct PowerShell;

/**
 * Implements the [Shell] trait for PowerShell.
 */
impl Shell for PowerShell
{
    /**
     * Prints the prompt initialisation code for PowerShell.
     * By printing the initialisation, it sets the prompt for that session.
     * # Arguments
     * * `configuration` - The configuration for shells.
     */
    fn print_initialisation(configuration: shell_configuration::base_configuration::BaseConfiguration)
    {
        println!("{}", &configuration.powershell.unwrap_or_default().setup.as_deref().unwrap_or_default());
    }
}