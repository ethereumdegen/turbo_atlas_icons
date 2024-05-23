
impl UiIconSource for AbilityTypeIconSource {
    fn get_icon_name(&self, world: &World) -> Option<String> {
        let ability_types = world.resource::<Assets<AbilityType>>();
        let ability_system_type_assets = world.resource::<AbilitySystemTypeAssets>();

        if let Some(ability_type_handle) = ability_system_type_assets.ability_types.get(self.0.as_str()) {
            if let Some(ability_type) = ability_types.get(ability_type_handle) {
                ability_type.icon_texture.clone()
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_icons_handles_map<'a>(&'a self, world: &'a World) -> &'a TextureHandlesMap{
        
    	let images = world.resource::<TextureAssets>();
        &images.ability_icons
    }

     fn get_texture_atlas<'a>(&'a self, world: &'a World) ->  &'a Option<TextureAtlasCombined>{
    	let texture_atlas_assets = world.resource::<TextureAtlasAssets>(); 

        &texture_atlas_assets.ability_icons_atlas
    }

     
}

impl UiIconSource for ItemTypeIconSource {
    fn get_icon_name(&self, world: &World) -> Option<String> {
        let item_types = world.resource::<Assets<ItemType>>();
        let item_system_type_assets = world.resource::<ItemSystemTypeAssets>();

        if let Some(item_type_handle) = item_system_type_assets.item_types.get(self.0.as_str()) {
            if let Some(item_type) = item_types.get(item_type_handle) {
                item_type.icon_texture.clone()
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_icons_handles_map<'a>(&'a self, world: &'a World) -> &'a TextureHandlesMap {
    	let images = world.resource::<TextureAssets>();
       &images.item_icons
    }

    fn get_texture_atlas<'a>(&'a self, world: &'a World) ->  &'a Option<TextureAtlasCombined> {
       let texture_atlas_assets = world.resource::<TextureAtlasAssets>(); 

		&texture_atlas_assets.item_icons_atlas
    }

     

}

impl UiIconSource for GuiPixelIconSource {
    fn get_icon_name(&self, _world: &World) -> Option<String> {
        Some(self.0.clone())
    }

   fn get_icons_handles_map<'a>(&'a self, world: &'a World) -> &'a TextureHandlesMap{
    	let images = world.resource::<TextureAssets>();
        &images.gui_pixel_icons
    }

    fn get_texture_atlas<'a>(&'a self, world: &'a World) ->  &'a Option<TextureAtlasCombined> {
        let texture_atlas_assets = world.resource::<TextureAtlasAssets>(); 

		&texture_atlas_assets.gui_pixel_icons_atlas
    }

      
}
