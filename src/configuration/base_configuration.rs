/*!
 * Stores the base configuration.
 */
use serde::{Serialize, Deserialize};
use crate::
{
    configuration::
    {
        prompt::PromptConfiguration,
        right::RightConfiguration,
        transient::TransientConfiguration,
        right_transient::RightTransientConfiguration,
        window_title::WindowTitleConfiguration
    }
};

/**
 * Stores the base configuration.
 */
#[derive(Serialize, Deserialize)]
pub struct BaseConfiguration
{
    pub prompt: Option<PromptConfiguration>,
    pub right: Option<RightConfiguration>,
    pub transient: Option<TransientConfiguration>,
    pub right_transient: Option<RightTransientConfiguration>,
    pub window_title: Option<WindowTitleConfiguration>
}

/**
 * Implements the [Default] trait for the prompt.
 */
impl Default for BaseConfiguration
{
    fn default() -> Self
    {
        Self
        {
            prompt: Some(PromptConfiguration::default()),
            right: Some(RightConfiguration::default()),
            transient: Some(TransientConfiguration::default()),
            right_transient: Some(RightTransientConfiguration::default()),
            window_title: Some(WindowTitleConfiguration::default())
        }
    }
}