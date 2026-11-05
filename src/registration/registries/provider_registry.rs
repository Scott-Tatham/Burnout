/*!
 * The registry of [Provider] to source data.
 */
use std::collections::HashMap;
use super::super::registry::Registry;
use super::super::super::data::provider::Provider;

pub struct ProviderRegistry
{
    providers: HashMap<String, Box<dyn Provider>>
}

impl ProviderRegistry
{
    pub fn new() -> Self
    {
        Self
        {
            providers: HashMap::new()
        }
    }
}

impl Registry<Box<dyn Provider>> for ProviderRegistry
{
    fn register(&mut self, name: String, provider: Box<dyn Provider>)
    {
        self.providers.insert(name, provider);
    }
    
    fn get(&self, name: &str) -> Option<&Box<dyn Provider>>
    {
        self.providers.get(name)
    }
}
