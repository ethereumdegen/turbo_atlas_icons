use crate::texture_atlas_combined::TextureAtlasCombined;
use crate::texture_atlas_combined::TextureHandlesMap;
use bevy::prelude::*;

pub trait UiIconSource {
    fn get_icon_name(&self, world: &World) -> Option<String>;
    fn get_icons_handles_map<'a>(&'a self, world: &'a World) -> &'a TextureHandlesMap;
    fn get_texture_atlas<'a>(&'a self, world: &'a World) ->  &'a Option<TextureAtlasCombined>;

    //fn get_dynamic_icon_dimensions(&self, world: &World) ->  Option< [u32;2] > ;
}

/*
pub trait UiIconDynamicDimensions {
   
    fn get_dynamic_icon_dimensions(&self, world: &World) ->  Option< [u32;2] > ;
}
*/