/*!
 * Handles Bash specific functionality.
 */

use crate::configuration::shell_configuration;
use super::Shell;

pub struct GitBash;

/**
 * Implements the [Shell] trait for Git Bash.
 */
impl Shell for GitBash
{
    /**
     * Prints the prompt initialisation code for Git Bash.
     * By printing the initialisation, it sets the prompt for that session.
     * # Arguments
     * * `configuration` - The configuration for shells.
     */
    fn print_initialisation(configuration: shell_configuration::base_configuration::BaseConfiguration)
    {
        println!("{}", &configuration.git_bash.unwrap_or_default().setup.as_deref().unwrap_or_default());
    }
}