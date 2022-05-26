use serde::Serialize;
use serde::Deserialize;
use std::fs::OpenOptions;
use std::error::Error;
use std::io::{Read, Write};

use crate::utils::filesystem_operations::obtain_preferences_file_path;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Preferences{
    pub enable_notifications: Option<bool>,
    pub current_device_name: Option<String>
}


impl Preferences {
    pub fn default() -> Self {
        Preferences {
            enable_notifications: Some(true),
            current_device_name: None
        }
    }
}

#[derive(Clone, Debug)]
pub struct PreferencesInterface{
    preferences_file_path: Option<String>,
    pub preferences: Preferences
}

impl PreferencesInterface {

    pub fn new() -> Self {
        match PreferencesInterface::load() {
            Ok(preferences_interface) => {
                return preferences_interface
            },
            Err(e) => {
                println!("{}", e);
                return PreferencesInterface {
                    preferences_file_path: None,
                    preferences: Preferences::default()
                }
            }
        }
    }


    fn load() -> Result<PreferencesInterface, Box<dyn Error>> {
        let preferences_file_path: String = obtain_preferences_file_path()?;
        let mut file = OpenOptions::new().write(true).read(true).create(true).open(&preferences_file_path)?;
        let mut contents: String = String::new();
        file.read_to_string(&mut contents)?;
        let preferences: Preferences = toml::from_str(&contents)?;
        Ok(PreferencesInterface {
            preferences_file_path: Some(preferences_file_path),
            preferences: preferences
        })
    }

    pub fn update(self: &mut Self, preferences: Preferences) {
        self.preferences = preferences;
        match self.write() {
            Ok(_) => {},
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    fn write(self: &mut Self) -> Result<(), Box<dyn Error>> {
        if let Some(preferences_file_path) = &self.preferences_file_path {
            let mut file = OpenOptions::new().write(true).read(true).create(true).open(preferences_file_path.as_str())?;
            let contents: String = toml::to_string(&self.preferences)?;
            file.write_all(contents.as_bytes())?;
        }
        Ok(())
    }
}