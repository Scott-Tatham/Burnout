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
        println!(r#"PS1='$($HOME/Bin/burnout)'; RPROMPT='$($HOME/Bin/burnout right)'; PS1_TRANSIENT='$($HOME/Bin/burnout transient)'; RPROMPT_TRANSIENT='$($HOME/Bin/burnout right-transient)'; PROMPT_COMMAND='echo -ne "\033]0;$($HOME/Bin/burnout window-title)\007"'"#);
    }
}