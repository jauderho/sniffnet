//! Module defining the `ConfigSettings` struct, which allows to save and reload
//! the application default configuration.

use serde::{Deserialize, Serialize};

use crate::notifications::types::notifications::Notifications;
use crate::{Language, StyleType};

#[derive(Serialize, Deserialize, Default)]
pub struct ConfigSettings {
    pub style: StyleType,
    pub language: Language,
    pub notifications: Notifications,
}
