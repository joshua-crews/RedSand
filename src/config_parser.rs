use bevy::prelude::*;

use serde::{Deserialize, Serialize};
use serde_yaml::{self};

#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct EngineConfig {
    pub planet_lods: Vec<u32>,
    pub map_dimensions: u32,
    pub num_provinces: u32,
}

pub fn read_configs(mut commands: Commands) {
    let f = std::fs::File::open("assets/configs/engine.yml")
        .expect("Could not open engine config file.");
    let engine_config: EngineConfig =
        serde_yaml::from_reader(f).expect("Could not read engine values.");
    commands.insert_resource(engine_config);
}
