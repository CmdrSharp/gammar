use crate::windows::display::DisplaySettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Profile {
    pub name: String,
    pub settings: DisplaySettings,
}

impl Profile {
    /// Create a new profile with the given name and display settings.
    pub fn new(name: String, settings: DisplaySettings) -> Self {
        Self { name, settings }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileManager {
    profiles: Vec<Profile>,
}

impl ProfileManager {
    /// Create a new, empty ProfileManager.
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
        }
    }

    /// Add a new profile to the manager.
    pub fn add_profile(&mut self, profile: Profile) {
        self.profiles.push(profile);
    }

    /// Remove a profile by its index. Returns the removed profile if the index was valid.
    pub fn remove_profile(&mut self, index: usize) -> Option<Profile> {
        if index < self.profiles.len() {
            return Some(self.profiles.remove(index));
        }

        None
    }

    /// Update a profile at the given index. Returns true if successful.
    pub fn update_profile(&mut self, index: usize, profile: Profile) -> bool {
        if index < self.profiles.len() {
            self.profiles[index] = profile;

            return true;
        }

        false
    }

    /// Get a reference to a profile by its index.
    pub fn get_profile(&self, index: usize) -> Option<&Profile> {
        self.profiles.get(index)
    }

    /// Get a slice of all profiles.
    pub fn get_profiles(&self) -> &[Profile] {
        &self.profiles
    }

    /// Get the number of profiles managed.
    pub fn profile_count(&self) -> usize {
        self.profiles.len()
    }

    /// Get a mutable reference to the profiles vector.
    pub fn profiles_mut(&mut self) -> &mut Vec<Profile> {
        &mut self.profiles
    }
}

impl Default for ProfileManager {
    fn default() -> Self {
        Self::new()
    }
}
