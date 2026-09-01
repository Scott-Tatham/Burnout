/*!
 * Handles Bash specific functionality.
 */
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
     */
    fn print_initialisation()
    {
        println!(r#"BURNOUT="$(command -v burnout.exe)"; \
        PS1="$($BURNOUT)"; \
        RPROMPT="$($BURNOUT right)"; \
        PS1_TRANSIENT="$($BURNOUT transient)"; \
        RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \
        PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window-title)\007"'"#);
    }
}