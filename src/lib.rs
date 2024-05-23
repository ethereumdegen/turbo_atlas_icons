


use crate::ui_icon_component::ui_icons_plugin;
use bevy::prelude::*;

pub mod texture_atlas_combined; 
pub mod ui_icon_component;
pub mod ui_icon_source;



pub struct TurboAtlasIconsPlugin {
    
}

impl Default for TurboAtlasIconsPlugin {
    fn default() -> Self {
        Self {
            
        }
    }
}
impl Plugin for TurboAtlasIconsPlugin {
    fn build(&self, app: &mut App) {
        
           app.add_plugins( ui_icons_plugin ) ;
        
       
        
        
 
    

        
    }
}
