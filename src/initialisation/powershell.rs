/*!
 * Handles the initialisation of the PowerShell shell.
 */

use crate::configuration::shell_configuration;
use super::Shell;

pub struct PowerShell;

/**
 * Implements the [Shell] trait for the PowerShell shell.
 */
impl Shell for PowerShell
{
    /**
     * Prints the prompt initialisation code for the PowerShell shell.
     * By printing the initialisation, it sets the prompt for that session.
     * # Arguments
     * * `configuration` - The configuration with the PowerShell shell initialisation values.
     */
    fn print_initialisation(configuration: shell_configuration::base_configuration::BaseConfiguration)
    {
        println!("{}", &configuration.powershell.unwrap_or_default().setup.as_deref().unwrap_or_default());
    }
}