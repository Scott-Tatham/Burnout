/*!
 * The entry point for Burnout.
 */
pub mod configuration;
pub mod data;
pub mod initialisation;
pub mod plugin;
pub mod registration;
pub mod render;

use std::env;
use initialisation::shell;
use render::renderer;
use registration::registries::provider_registry::ProviderRegistry;
use registration::registries::variable_registry::VariableRegistry;

/// The entry point for Burnout.
fn main()
{
    let arguments: Vec<String> = env::args().collect();

    let provider_registry = ProviderRegistry::new();
    let variable_registry = VariableRegistry::new();

    if arguments.len() == 1
    {
        renderer::render("bash", "prompt").expect("The rendering of the display failed.");

        return;
    }

    let shell = arguments.get(1).map(|value| value.as_str()).unwrap_or("bash");

    if arguments.len() == 2
    {
        renderer::render(shell, "prompt").expect("The rendering of the display failed.");

        return;
    }

    let display = arguments.get(2).map(|value| value.as_str()).unwrap_or("prompt");

    match display
    {
        "initialise" => shell::shell_initialisation(shell).expect("The initialisation of the shell failed."),
        _ => renderer::render(shell, display).expect("The rendering of the display failed.")
    }
}