use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Obsidian vault name (optional)
    pub vault: Option<String>,
    
    /// Base Obsidian action (new, open, search, etc.)
    pub action: String,
    
    /// Parameter name to use for the user's content (defaults to "content" for "new" action)
    pub content_param: Option<String>,
    
    /// Additional URL parameters for Obsidian
    pub parameters: HashMap<String, String>,
    
    /// Plugin-specific parameters
    pub plugin: Option<PluginConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginConfig {
    /// Plugin command or endpoint
    pub command: String,
    
    /// Additional plugin parameters
    pub params: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            vault: None,
            action: "new".to_string(),
            content_param: None, // Will default to "content" for "new" action
            parameters: HashMap::new(),
            plugin: None,
        }
    }
}

impl Config {
    /// Load configuration from the standard config directory
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            // Create default config if it doesn't exist
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }
    
    /// Save configuration to the standard config directory
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::config_path()?;
        
        // Ensure the config directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }
    
    /// Get the path to the configuration file
    pub fn config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Unable to determine config directory")?;
        Ok(config_dir.join("oqcap").join("config.toml"))
    }
    
    /// Generate the Obsidian URL based on configuration
    pub fn build_obsidian_url(&self, text: &str) -> String {
        let mut url = format!("obsidian://{}", self.action);
        let mut params = Vec::new();
        
        // Add vault parameter if specified
        if let Some(vault) = &self.vault {
            params.push(format!("vault={}", urlencoding::encode(vault)));
        }
        
        // Determine content parameter name based on action and configuration
        let content_param_name = self.get_content_param_name();
        
        // Only add content parameter if we have a parameter name
        if let Some(param_name) = content_param_name {
            params.push(format!("{}={}", param_name, urlencoding::encode(text)));
        }
        
        // Add additional parameters
        for (key, value) in &self.parameters {
            params.push(format!("{}={}", key, urlencoding::encode(value)));
        }
        
        // Add plugin parameters if specified
        if let Some(plugin) = &self.plugin {
            params.push(format!("plugin={}", urlencoding::encode(&plugin.command)));
            
            for (key, value) in &plugin.params {
                params.push(format!("{}={}", key, urlencoding::encode(value)));
            }
        }
        
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }
        
        url
    }
    
    /// Get the parameter name to use for content based on action and configuration
    fn get_content_param_name(&self) -> Option<&str> {
        // If explicitly configured, use that
        if let Some(ref param_name) = self.content_param {
            return Some(param_name);
        }
        
        // Default behavior: only "new" action gets content parameter
        match self.action.as_str() {
            "new" => Some("content"),
            _ => None, // Other actions don't get content parameter by default
        }
    }
    
}