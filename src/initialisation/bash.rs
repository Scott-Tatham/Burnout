/*!
 * Handles the initialisation of the Bash shell.
 */

pub struct Bash;

use crate::configuration::shell_configuration;
use super::Shell;

/**
 * Implements the [Shell] trait for the Bash shell.
 */
impl Shell for Bash
{
    /**
     * Prints the prompt initialisation code for the Bash shell.
     * By printing the initialisation, it sets the prompt for that session.
     * # Arguments
     * * `configuration` - The configuration with the Bash shell initialisation values.
     */
    fn print_initialisation(configuration: shell_configuration::base_configuration::BaseConfiguration)
    {
        println!("{}", &configuration.bash.unwrap_or_default().setup.as_deref().unwrap_or_default());
    }
}