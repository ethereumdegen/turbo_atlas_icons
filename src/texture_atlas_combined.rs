//use crate::file_system_interaction::asset_loading::TextureAssets;
use bevy::{prelude::*};
use bevy_asset_loader::mapped::AssetFileName;
use  bevy::platform_support::collections::hash_map::HashMap;

pub struct TextureAtlasCombined {
    pub layout: Handle<TextureAtlasLayout>,
    pub sources:  TextureAtlasSources , //new in 0.15 
    pub image: Handle<Image>,
}

impl TextureAtlasCombined {

    pub fn get_texture_index(&self, image_handle: &Handle<Image>) -> Option<usize>{


        return self.sources.texture_index(image_handle)
    }
}


pub type TextureHandlesMap = HashMap<AssetFileName,Handle<Image>>;


pub type AtlasName = String ; 

 