//use crate::file_system_interaction::asset_loading::TextureAssets;
use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::mapped::AssetFileName;

pub struct TextureAtlasCombined {
    pub layout: Handle<TextureAtlasLayout>,
    pub image: Handle<Image>,
}



pub type TextureHandlesMap = HashMap<AssetFileName,Handle<Image>>;


pub type AtlasName = String ; 

 