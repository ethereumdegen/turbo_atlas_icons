 
use crate::texture_atlas_combined::TextureAtlasCombined;
use bevy_asset_loader::prelude::AssetFileName;
use  bevy::platform_support::collections::hash_map::HashMap;
use bevy::prelude::*;

pub type TextureHandlesMap = HashMap<AssetFileName,Handle<Image>>;


pub type AtlasName = String ; 



pub trait UiIconSource {
    fn get_icon_name(&self, world: &World) -> Option<String>;
    fn get_icons_handles_map<'a>(&'a self, world: &'a World) -> &'a TextureHandlesMap;
    fn get_texture_atlas<'a>(&'a self, world: &'a World) ->  &'a Option<TextureAtlasCombined>;
    //fn updates_dynamically(&self) -> bool;

    //fn get_dynamic_icon_dimensions(&self, world: &World) ->  Option< [u32;2] > ;
}

/*
pub trait UiIconDynamicDimensions {
   
    fn get_dynamic_icon_dimensions(&self, world: &World) ->  Option< [u32;2] > ;
}
*/