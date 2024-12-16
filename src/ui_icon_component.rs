 
use crate::texture_atlas_combined::TextureAtlasCombined;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::AssetFileName;
use crate::ui_icon_source::UiIconSource;
use bevy::ecs::system::EntityCommand;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;




/*
use crate::file_system_interaction::{
    ability_type::AbilityType,
    asset_loading::{AbilitySystemTypeAssets, ItemSystemTypeAssets, TextureAssets},
    item_type::ItemType,
    texture_atlas::{get_index_for_subtexture_by_name, TextureAtlasAssets},
};


use crate::ui::ui_data_sources::ui_icon_source::{UiIconSource};
*/

pub(crate) fn ui_icons_plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_icons_from_source
        .run_if(any_with_component::<UiIconComponent>)
        .in_set( DynamicIconSourcesSystemSet ) ,
    );
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub(crate) struct DynamicIconSourcesSystemSet;


// Update the UiIconComponent to use the trait object
#[derive(Component, Default)]
pub struct UiIconComponent {
    pub icon_source: Option<Box<(dyn UiIconSource + Sync + Send + 'static)>>,
}

impl UiIconComponent {
    pub fn new(icon_source: Option<Box<dyn UiIconSource + Sync + Send + 'static >>) -> Self {
        Self { icon_source }
    }
}

// Update the SetIconSource command to use the trait object
struct SetIconSource(Option<Box<dyn UiIconSource + Sync + Send + 'static>>);

impl EntityCommand for SetIconSource {
    fn apply(self, entity: Entity, world: &mut World) {
        if let Some(mut ui_icon_comp) = world.entity_mut(entity).get_mut::<UiIconComponent>() {
            ui_icon_comp.icon_source = self.0;
        }
    }
}

// Update the UiIconComponentCommands trait to use the trait object
pub trait UiIconComponentCommands<'a> {
    fn set_icon_source(&'a mut self, src: Option<Box<dyn UiIconSource  + Sync + Send + 'static>>) -> &mut EntityCommands<'a>;
}

impl<'a> UiIconComponentCommands<'a> for EntityCommands<'a> {
    fn set_icon_source(&'a mut self, src: Option<Box<dyn UiIconSource  + Sync + Send + 'static>>) -> &mut EntityCommands<'a> {
        self.queue(SetIconSource(src))
    }
}
 
fn update_icons_from_source(
     mut commands: Commands,
 
      image_node_entity_query: Query<
        (
            Entity,
            &UiIconComponent,
            &ViewVisibility
        ),
        With<UiIconComponent>,
    >,

   
    world: &World, 
) {
    
    let texture_atlases = world.resource::< Assets<TextureAtlasLayout> >();

   
   for ( entity, ui_icon_comp, view_vis) in image_node_entity_query.iter ()
    { 

        if view_vis == &ViewVisibility::HIDDEN {continue};

        

        let Some( ref ui_icon_src ) = ui_icon_comp.icon_source  else {continue} ;

        let icon_name = ui_icon_src.get_icon_name(world);

          let Some(icon_name) = icon_name else { continue };

        let icons_handles_map = ui_icon_src.get_icons_handles_map(world);
        
         let Some( texture_atlas ) = ui_icon_src.get_texture_atlas(world) else {continue} ;
       

        let Some(image_index) = get_index_for_subtexture_by_name(
            &texture_atlas ,
          //   texture_atlases,
            &icons_handles_map,
            &icon_name,
        ) else {
            continue;
        };


         commands.entity(entity)
         .atlas_texture_image (texture_atlas.image.clone())
         .atlas_texture_layout( texture_atlas.layout.clone())
         .atlas_texture_index( image_index )

         ;
            
    }
}


// helper fn 



pub fn get_index_for_subtexture_by_name(
    texture_atlas_combined: &TextureAtlasCombined,
 //   texture_atlases: & Assets<TextureAtlasLayout> ,

    image_handles_map: &HashMap<AssetFileName, Handle<Image>>,
    texture_name: &String,
) -> Option<usize> {
    //if let Some(atlas) = texture_atlases.get(texture_atlas_handle) {
        if let Some(image_handle) = image_handles_map.get(texture_name.as_str()) {
            return texture_atlas_combined.get_texture_index(image_handle);
        }
    //}
    //self.index_registry.get(texture_name) .copied() //why do we need to do plus 1 ?
    None
}





// --- sickle extensions for mut atlas texture using commands  

struct SetAtlasTextureImage {
    image_handle: Handle<Image> 
}


impl EntityCommand for SetAtlasTextureImage {
    fn apply(self, entity: Entity, world: &mut World) {
       

        let handle = self.image_handle; 

        let Some(mut image_node) = world.get_mut::<ImageNode>(entity) else {
            warn!(
                "Failed to set image on entity {:?}: No UiImage component found!",
                entity
            );
            return;
        };



      if image_node.image  != handle {
            image_node.image = handle;
        }
    }
}



pub trait SetAtlasTextureImageExt<'a> {
    fn atlas_texture_image(&'a mut self, image_handle: Handle<Image> ) -> &mut EntityCommands<'a>;
}

impl<'a> SetAtlasTextureImageExt<'a> for EntityCommands<'a> {
    fn atlas_texture_image(&'a mut self, image_handle: Handle<Image> ) -> &mut EntityCommands<'a> {
        self.queue(SetAtlasTextureImage {
            image_handle
            // check_lock: true,
        });
        self
    }
}



struct SetAtlasTextureLayout {
    layout: Handle<TextureAtlasLayout>,
}

impl EntityCommand for SetAtlasTextureLayout {
    fn apply(self, entity: Entity, world: &mut World) {
        let layout = self.layout;

      let Some(mut image_node) = world.get_mut::<ImageNode>(entity) else {
            warn!(
                "Failed to set texture atlas layout on entity {:?}: No TextureAtlas component found!",
                entity
            );
            return;
        }; 
 


        if let Some( ref mut  existing_tex_atlas)  = & mut image_node.texture_atlas {

               existing_tex_atlas.layout = layout;
        }else {

             image_node.texture_atlas = Some(TextureAtlas{ layout, index: 0})
        }

       
 
    }
}

pub trait SetAtlasTextureLayoutExt<'a> {
    fn atlas_texture_layout(&'a mut self, layout: Handle<TextureAtlasLayout>) -> &mut EntityCommands<'a>;
}

impl<'a> SetAtlasTextureLayoutExt<'a> for EntityCommands<'a> {
    fn atlas_texture_layout(&'a mut self, layout: Handle<TextureAtlasLayout>) -> &mut EntityCommands<'a> {
        self.queue(SetAtlasTextureLayout { layout });
        self
    }
}

struct SetAtlasTextureIndex {
    index: usize,
}

impl EntityCommand for SetAtlasTextureIndex {
    fn apply(self, entity: Entity, world: &mut World) {
        let index = self.index;

         let Some(mut image_node) = world.get_mut::<ImageNode>(entity) else {
            warn!(
                "Failed to set texture atlas index on entity {:?}: No TextureAtlas component found!",
                entity
            );
            return;
        }; 

         let Some(ref mut texture_atlas) = &mut image_node.texture_atlas else {
            warn!(
                "Failed to set texture atlas index on entity {:?}: No TextureAtlas component found!",
                entity
            );
            return;
        };

        if  texture_atlas.index != index {
             texture_atlas.index = index;
        }
    }
}

pub trait SetAtlasTextureIndexExt<'a> {
    fn atlas_texture_index(&'a mut self, index: usize) -> &mut EntityCommands<'a>;
}

impl<'a> SetAtlasTextureIndexExt<'a> for EntityCommands<'a> {
    fn atlas_texture_index(&'a mut self, index: usize) -> &mut EntityCommands<'a> {
        self.queue(SetAtlasTextureIndex { index });
        self
    }
}

