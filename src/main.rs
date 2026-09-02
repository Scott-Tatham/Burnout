/*!
 * The entry point for Burnout.
 */
use std::env;

pub mod shells;
pub mod render;
pub mod configuration;

// I set $env:PATH += ";C:\Users\Scooter\Bin", so, make this cleaner in Bootstrapping.
fn main()
{
    let arguments: Vec<String> = env::args().collect();
    let configuration = configuration::core_configuration::load_or_create_configuration();

    match arguments.get(1).map(|value| value.as_str())
    {
        Some("initialise") => shells::shell_initialisation(arguments.get(2).map(|value| value.as_str()).unwrap_or("bash")),
        Some("right") => render::render_right_prompt(&configuration.right.unwrap_or_default()),
        Some("transient") => render::render_transient_prompt(&configuration.transient.unwrap_or_default()),
        Some("right-transient") => render::render_right_transient_prompt(&configuration.right_transient.unwrap_or_default()),
        Some("window-title") => render::render_window_title(&configuration.window_title.unwrap_or_default()),
        Some("continuation") => render::render_continuation_prompt(&configuration.continuation.unwrap_or_default()),
        _ => render::render_prompt(&configuration.prompt.unwrap_or_default())
    }
}