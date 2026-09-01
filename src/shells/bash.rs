/*!
 * Handles Bash specific functionality.
 */
use super::Shell;

pub struct Bash;

/**
 * Implements the [Shell] trait for Bash.
 */
impl Shell for Bash
{
    /**
     * Prints the prompt initialisation code for Bash.
     * By printing the initialisation, it sets the prompt for that session.
     */
    fn print_initialisation()
    {
        println!(r#"BURNOUT="$(command -v burnout)"; \
        PS1="$($BURNOUT)"; \
        RPROMPT="$($BURNOUT right)"; \
        PS1_TRANSIENT="$($BURNOUT transient)"; \
        RPROMPT_TRANSIENT="$($BURNOUT right-transient)"; \
        PS2="$($BURNOUT continuation)" \
        PROMPT_COMMAND='echo -ne "\033]0;$($BURNOUT window-title)\007"'"#);
    }
}