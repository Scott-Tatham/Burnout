/*!
 * Handles rendering of the prompt.
 */

use crate::configuration::core_configuration::{prompt, right, transient, right_transient, continuation, window_title};

/**
 * Defines the implementation of a module.
 */
pub trait Module
{
    /**
     * Generates the value for the module.
     */
    fn generate_value() -> str;
}

/**
 * Renders the prompt with the specified modules.
 * # Arguments
 * * `configuration` - The configuration for the prompt.
 */
pub fn render_prompt(configuration: &prompt::PromptConfiguration)
{
    println!("{}", &configuration.content.as_deref().unwrap_or_default());
}

/**
 * Renders the right side prompt with the specified modules.
 * # Arguments
 * * `configuration` - The configuration for the right side prompt.
 */
pub fn render_right_prompt(configuration: &right::RightConfiguration)
{
    println!("{}", &configuration.content.as_deref().unwrap_or_default());
}

/**
 * Renders the transient prompt with the specified modules.
 * # Arguments
 * * `configuration` - The configuration for the transient prompt.
 */
pub fn render_transient_prompt(configuration: &transient::TransientConfiguration)
{
    println!("{}", &configuration.content.as_deref().unwrap_or_default());
}

/**
 * Renders the right side transient prompt with the specified modules.
 * # Arguments
 * * `configuration` - The configuration for the right side transient prompt.
 */
pub fn render_right_transient_prompt(configuration: &right_transient::RightTransientConfiguration)
{
    println!("{}", &configuration.content.as_deref().unwrap_or_default());
}

/**
 * Renders the right side transient prompt with the specified modules.
 * # Arguments
 * * `configuration` - The configuration for the continuation prompt.
 */
pub fn render_continuation_prompt(configuration: &continuation::ContinuationConfiguration)
{
    println!("{}", &configuration.content.as_deref().unwrap_or_default());
}

/**
 * Renders the window title with the specified modules.
 * # Arguments
 * * `configuration` - The configuration for the window title.
 */
pub fn render_window_title(configuration: &window_title::WindowTitleConfiguration)
{
    println!("{}", &configuration.content.as_deref().unwrap_or_default());
}

#[cfg(test)]
mod tests
{
    use super::*;
    use crate::configuration::core_configuration::prompt::PromptConfiguration;
    use crate::configuration::core_configuration::right::RightConfiguration;
    use crate::configuration::core_configuration::right_transient::RightTransientConfiguration;
    use crate::configuration::core_configuration::transient::TransientConfiguration;
    use crate::configuration::core_configuration::window_title::WindowTitleConfiguration;

    /**
     * Tests the rendering of the prompt with full content.
     */
    #[test]
    fn test_render_prompt_full_content()
    {
        render_prompt(&PromptConfiguration
        {
            content: Some("Test Prompt".to_string())
        });
    }

    /**
     * Tests the rendering of the prompt with empty content.
     */
    #[test]
    fn test_render_prompt_empty_content()
    {
        render_prompt(&PromptConfiguration
        {
            content: None
        });
    }

    /**
     * Tests the rendering of the right prompt with full content.
     */
    #[test]
    fn test_render_right_prompt_full_content()
    {
        render_right_prompt(&RightConfiguration
        {
            content: Some("Test Right".to_string())
        });
    }

    /**
     * Tests the rendering of the right prompt with empty content.
     */
    #[test]
    fn test_render_right_prompt_empty_content()
    {
        render_right_prompt(&RightConfiguration
        {
            content: None
        });
    }

    /**
     * Tests the rendering of the transient prompt with full content.
     */
    #[test]
    fn test_render_transient_prompt_full_content()
    {
        render_transient_prompt(&TransientConfiguration
        {
            content: Some("Test Transient".to_string())
        });
    }

    /**
     * Tests the rendering of the transient prompt with empty content.
     */
    #[test]
    fn test_render_transient_prompt_empty_content()
    {
        render_transient_prompt(&TransientConfiguration
        {
            content: None
        });
    }

    /**
     * Tests the rendering of the right transient prompt with full content.
     */
    #[test]
    fn test_render_right_transient_prompt_full_content()
    {
        render_right_transient_prompt(&RightTransientConfiguration
        {
            content: Some("Test Right Transient".to_string())
        });
    }

    /**
     * Tests the rendering of the right transient prompt with empty content.
     */
    #[test]
    fn test_render_right_transient_prompt_empty_content()
    {
        render_right_transient_prompt(&RightTransientConfiguration
        {
            content: None
        });
    }

    /**
     * Tests the rendering of the right transient prompt with full content.
     */
    #[test]
    fn test_render_continuation_prompt_full_content()
    {
        render_right_transient_prompt(&RightTransientConfiguration
        {
            content: Some("Test Right Transient".to_string())
        });
    }

    /**
     * Tests the rendering of the right transient prompt with empty content.
     */
    #[test]
    fn test_render_continuation_prompt_empty_content()
    {
        render_right_transient_prompt(&RightTransientConfiguration
        {
            content: None
        });
    }

    /**
     * Tests the rendering of the window title with full content.
     */
    #[test]
    fn test_render_window_title_full_content()
    {
        render_window_title(&WindowTitleConfiguration
        {
            content: Some("Test Window Title".to_string())
        });
    }

    /**
     * Tests the rendering of the window title with empty content.
     */
    #[test]
    fn test_render_window_title_empty_content()
    {
        render_window_title(&WindowTitleConfiguration
        {
            content: None
        });
    }
}