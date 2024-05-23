//use crate::file_system_interaction::asset_loading::TextureAssets;
use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::mapped::AssetFileName;

pub struct TextureAtlasCombined {
    pub layout: Handle<TextureAtlasLayout>,
    pub image: Handle<Image>,
}



pub type TextureHandlesMap = HashMap<AssetFileName,Handle<Image>>;


pub type AtlasName = String ; 


//build  this yourself  !  In your own way 
/*
#[derive(Resource, Default)]
pub(crate) struct TextureAtlasAssets {

	pub(crate) atlas_map: HashMap<AtlasName, TextureAtlasCombined>

   // pub(crate) gui_pixel_icons_atlas: Option<TextureAtlasCombined>,
   // pub(crate) ability_icons_atlas: Option<TextureAtlasCombined>,
   // pub(crate) item_icons_atlas: Option<TextureAtlasCombined>,
    // pub(crate) particles_atlas: Option<Handle<TextureAtlas>>,
}

impl TextureAtlasAssets {
    pub fn build(
        mut commands: Commands,
        texture_assets: HashMap< AtlasName, TextureHandlesMap >,
        mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
        mut images: ResMut<Assets<Image>>,
    ) {
        // Assume defaults for padding and sampling, adjust as needed
        let padding = Some(UVec2::new(2, 2));
        //let sampling = Some(ImageSampler::linear());

        let mut  atlas_map = HashMap::new();


        for (name,handles_map) in texture_assets.iter() {


        	let built_atlas_combined = build_texture_atlas(
	            handles_map,
	            Some(Vec2::new(2048., 2048.)),
	            padding,
	            &mut texture_atlases,
	            &mut images,
	        );

        	atlas_map.insert( name.clone() , built_atlas_combined ) ;
        }


      /*  let gui_pixel_icons_atlas = build_texture_atlas(
            &texture_assets.gui_pixel_icons,
            Some(Vec2::new(2048., 2048.)),
            padding,
            &mut texture_atlases,
            &mut images,
        );

        let ability_icons_atlas = build_texture_atlas(
            &texture_assets.ability_icons,
            Some(Vec2::new(2048., 2048.)),
            padding,
            &mut texture_atlases,
            &mut images,
        );

        let item_icons_atlas = build_texture_atlas(
            &texture_assets.item_icons,
            Some(Vec2::new(4096., 4096.)),
            padding,
            &mut texture_atlases,
            &mut images,
        );*/

        // Store the handles in  new resource
        commands.insert_resource(TextureAtlasAssets {
            atlas_map
        });
    }
}
*/


pub fn get_index_for_subtexture_by_name(
    texture_atlas_handle: &Handle<TextureAtlasLayout>,
    texture_atlases: & Assets<TextureAtlasLayout> ,

    image_handles_map: &HashMap<AssetFileName, Handle<Image>>,
    texture_name: &String,
) -> Option<usize> {
    if let Some(atlas) = texture_atlases.get(texture_atlas_handle) {
        if let Some(image_handle) = image_handles_map.get(texture_name.as_str()) {
            return atlas.get_texture_index(image_handle);
        }
    }
    //self.index_registry.get(texture_name) .copied() //why do we need to do plus 1 ?
    None
}

pub fn build_texture_atlas(
    handles: &HashMap<AssetFileName, Handle<Image>>,
    max_size: Option<Vec2>,
    padding: Option<UVec2>,

    //  textures: &mut ResMut<Assets<Image>>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    images: &mut ResMut<Assets<Image>>,
) -> TextureAtlasCombined {
    let mut texture_atlas_builder = TextureAtlasBuilder::default()
        .max_size(max_size.unwrap_or(Vec2::new(2048., 2048.)))
        .padding(padding.unwrap_or(UVec2::ZERO));

    // let mut texture_atlas_index_registry: TextureAtlasIndexRegistry = HashMap::new();

    for (icon_name, handle) in handles.iter() {
        if let Some(texture) = images.get(handle) {
            texture_atlas_builder.add_texture(Some(handle.clone_weak().into()), texture);

            // texture_atlas_index_registry.insert(icon_name.clone(), index) ;
            // println!("register atlas image {:?} {:?}", icon_name,index);
        } else {
            panic!(
                "Texture handle did not resolve to an `Image` asset: {:?}",
                icon_name
            );
            //  continue;
        }
    }

    let (texture_atlas, image) = texture_atlas_builder
        .finish()
        .expect("Failed to build texture atlas.");
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let image_handle = images.add(image);

    TextureAtlasCombined {
        layout: texture_atlas_handle,
        image: image_handle,
    }
}
