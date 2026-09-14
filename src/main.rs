/*!
 * The entry point for Burnout.
 */
use std::env;

pub mod configuration;
pub mod initialisation;
pub mod render;

fn main()
{
    let arguments: Vec<String> = env::args().collect();

    if arguments.len() == 1
    {
        render::render_display("prompt", "bash");

        return;
    }

    let command = arguments.get(1).map(|value| value.as_str()).unwrap_or("prompt");
    let shell = arguments.get(2).map(|value| value.as_str()).unwrap_or("bash");

    match command
    {
        "initialise" => initialisation::shell_initialisation(shell),
        _ => render::render_display(command, shell)
    }
}