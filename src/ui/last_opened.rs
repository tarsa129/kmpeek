use bevy::app::{App, Startup, Update};
use bevy::log::{error, warn};
use bevy::prelude::{on_message, Commands, IntoScheduleConfigs, MessageReader, Res, ResMut, Resource};
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};
use crate::ui::util::{FileResult};

pub fn last_opened_plugin(app: &mut App) {
    app.add_systems(Startup, setup_last_opened)
        .add_message::<FileResult>()
        .add_systems(Update, add_last_opened_path.run_if(on_message::<FileResult>));
}

fn setup_last_opened(mut commands: Commands, pkv: Res<PkvStore>) {
    let last_opened = pkv.get::<LastOpened>("last_opened").unwrap_or_else(|error| {
        warn!("could not load recently open files; using defaults for this session: {error}");
        LastOpened::default()
    });
    commands.insert_resource(last_opened);
}

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

pub fn add_last_opened_path(mut pkv: ResMut<PkvStore>, mut last_opened: ResMut<LastOpened>, mut reader: MessageReader<FileResult>) {
    // TODO
}