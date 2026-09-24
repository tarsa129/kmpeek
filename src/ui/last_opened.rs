use bevy::prelude::{ Resource};
use serde::{Deserialize, Serialize};
use crate::ui::util::{FileResult};

const LAST_OPENED_SIZE: usize = 10;

#[derive(Serialize, Deserialize, Resource)]
pub struct LastOpened {
    pub entries: [FileResult; LAST_OPENED_SIZE]
}

impl Default for LastOpened {
    fn default() -> Self {
        Self {
            entries: Default::default()
        }
    }
}