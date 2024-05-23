


## Turbo Atlas Icons 


A crate for bevy to help you more easily render ui icons from an atlas.





Simply add a UiIconComponent to your Ui Node (AtlasImageBundle), set its source to a pre-registered source, and it will work! 



1. Register an icon source type   (maps the source type -> how to access the texture atlas data ) 

```





impl UiIconSource for GuiPixelIconSource {

	//describe how to get the Icon Handle Name 

    fn get_icon_name(&self, _world: &World) -> Option<String> {
        Some(self.0.clone())
    }

    //describe how to get the HashMap< IconHandleName -> ImageHandle  for this source 
   
   fn get_icons_handles_map<'a>(&'a self, world: &'a World) -> &'a TextureHandlesMap{
    	let images = world.resource::<TextureAssets>();
        &images.gui_pixel_icons
    }


    //describe how to get the texture atlas layout and overall image handle for this source 

    fn get_texture_atlas<'a>(&'a self, world: &'a World) ->  &'a Option<TextureAtlasCombined> {
        let texture_atlas_assets = world.resource::<TextureAtlasAssets>(); 

		&texture_atlas_assets.gui_pixel_icons_atlas
    }

      
}




```





2. Attach the icon source type information as a component to your AtlasImageBundle node 

```

   ui_icon_comp.icon_source = Some(Box::new( GuiPixelIconSource("soul_point_full.tga".into() ) ) );



```


3. Your icon will render ! 

(This is all assuming the TextureAtlasCombined provided to the IconSource lookup has already been loaded into memory --- see sample/texture_atlas_assets for examples of how this could be done  -- For example using bevy_asset_loader )