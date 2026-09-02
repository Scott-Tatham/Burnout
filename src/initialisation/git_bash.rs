/*!
 * Handles the initialisation of the Git Bash shell.
 */

use crate::configuration::shell_configuration;
use super::Shell;

pub struct GitBash;

/**
 * Implements the [Shell] trait for the Git Bash shell.
 */
impl Shell for GitBash
{
    /**
     * Prints the prompt initialisation code for the Git Bash shell.
     * By printing the initialisation, it sets the prompt for that session.
     * # Arguments
     * * `configuration` - The configuration with the Git Bash shell initialisation values.
     */
    fn print_initialisation(configuration: shell_configuration::base_configuration::BaseConfiguration)
    {
        println!("{}", &configuration.git_bash.unwrap_or_default().setup.as_deref().unwrap_or_default());
    }
}