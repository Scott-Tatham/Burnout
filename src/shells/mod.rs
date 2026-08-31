/*!
 * Handles shell specific functionality.
 */
mod bash;
mod zsh;
mod powershell;
mod cmd;

/**
 * Defines the implementation of a shell.
 */
pub trait Shell
{
    /**
     * Prints the prompt initialisation code for the shell.
     * By printing the initialisation, it sets the prompt for that session.
     */
    fn print_initialisation();
}

/**
 * Invokes the initialisation of the specified shell.
 * # Arguments
 * * `shell` - The name of the shell to initialise.
 */
pub fn shell_initialisation(shell: &str)
{
    match shell
    {
        "bash" => bash::Bash::print_initialisation(),
        "zsh" => zsh::Zsh::print_initialisation(),
        "powershell" | "pwsh" => powershell::PowerShell::print_initialisation(),
        "cmd" => cmd::Cmd::print_initialisation(),
        _ => eprintln!("Unsupported shell: {}", shell),
    }
}