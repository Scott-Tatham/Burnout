pub mod bash;
pub mod zsh;
pub mod powershell;
pub mod cmd;

pub fn dispatch_initialisation(shell: &str) {
    match shell {
        "bash" => bash::print_initialisation(),
        "zsh" => zsh::print_initialisation(),
        "powershell" | "pwsh" => powershell::print_initialisation(),
        "cmd" => cmd::print_initialisation(),
        _ => eprintln!("Unsupported shell: {}", shell),
    }
}
