/*!
 * Defines the implementation of a shell.
 */
use super::super::configuration::shell_configuration::file;

/// Invokes the initialisation of the specified shell.
/// # Arguments
/// * `shell` - The name of the shell to initialise.
/// # Returns
/// * `Ok(())` - Returns that the initialisation was successful.
/// * `Err(String)` - Returns that the initialisation failed.
pub fn shell_initialisation(shell: &str) -> Result<(), String>
{
    let configuration = file::load_or_create_configuration();

    match shell
    {
        "bash" => println!("{}", &configuration.bash.expect("Failed to get the Bash configuration.").setup.expect("Failed to get the Bash setup.")),
        "git_bash" => println!("{}", &configuration.git_bash.expect("Failed to get the Git Bash configuration.").setup.expect("Failed to get the Git Bash setup.")),
        "zsh" => println!("{}", &configuration.zsh.expect("Failed to get the Zsh configuration.").setup.expect("Failed to get the Zsh setup.")),
        "powershell" | "pwsh" => println!("{}", &configuration.powershell.expect("Failed to get the PowerShell configuration.").setup.expect("Failed to get the PowerShell setup.")),
        "cmd" => println!("{}", &configuration.cmd.expect("Failed to get the Cmd configuration.").setup.expect("Failed to get the Cmd setup.")),
        _ => return Err(format!("Unsupported shell for initialisation: {}.", shell))
    };

    Ok(())
}