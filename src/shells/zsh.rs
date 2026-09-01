/*!
 * Handles Zsh specific functionality.
 */
use super::Shell;

pub struct Zsh;

/**
 * Implements the [Shell] trait for Zsh.
 */
impl Shell for Zsh
{
    /**
     * Prints the prompt initialisation code for Zsh.
     * By printing the initialisation, it sets the prompt for that session.
     */
    fn print_initialisation()
    {
        println!(r#"BURNOUT='$(command -v burnout)'; PROMPT='$($BURNOUT)'; RPROMPT='$($BURNOUT right)'; PROMPT_TRANSIENT='$($BURNOUT transient)'; RPROMPT_TRANSIENT='$($BURNOUT right-transient)'; precmd() {{print -Pn '\e]0;$($BURNOUT window-title)\a';}}"#);
    }
}