/*!
 * The registry of variables that contain data.
 */
use std::collections::HashMap;
use super::super::registry::Registry;

pub struct VariableRegistry
{
    providers: HashMap<String, String>
}

impl VariableRegistry
{
    pub fn new() -> Self
    {
        Self
        {
            providers: HashMap::new()
        }
    }
}

impl Registry<String> for VariableRegistry
{
    fn register(&mut self, name: String, provider: String)
    {
        self.providers.insert(name, provider);
    }
    
    fn get(&self, name: &str) -> Option<&String>
    {
        self.providers.get(name)
    }
}
