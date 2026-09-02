/*!
 * Handles the initialisation of the shell.
 */

use crate::configuration::shell_configuration;

mod bash;
mod git_bash;
mod zsh;
mod powershell;
mod cmd;

/**
 * Defines the implementation of a shell.
 */
trait Shell
{
    /**
     * Prints the prompt initialisation code for the shell.
     * By printing the initialisation, it sets the prompt for that session.
     * # Arguments
     * * `configuration` - The configuration with the shell initialisation values.
     */
    fn print_initialisation(configuration: shell_configuration::base_configuration::BaseConfiguration);
}

/**
 * Invokes the initialisation of the specified shell.
 * # Arguments
 * * `shell` - The name of the shell to initialise.
 */
pub fn shell_initialisation(shell: &str)
{
    let configuration = shell_configuration::load_or_create_configuration();

    match shell
    {
        "bash" => bash::Bash::print_initialisation(configuration),
        "git-bash" => git_bash::GitBash::print_initialisation(configuration),
        "zsh" => zsh::Zsh::print_initialisation(configuration),
        "powershell" | "pwsh" => powershell::PowerShell::print_initialisation(configuration),
        "cmd" => cmd::Cmd::print_initialisation(configuration),
        _ => eprintln!("Unsupported shell: {}", shell)
    }
}