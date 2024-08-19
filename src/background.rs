use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use crate::art_url_get;
use crate::command;

use std::path::Path;
use std::{env, fs};
use std::env::temp_dir;

#[allow(deprecated)]
use std::env::home_dir;








fn create_temp_folder() 
{
    let temp_folder_path = format!("{}{}", temp_dir().display(), "/.background/");
    if !Path::new(&temp_folder_path).exists()
    {
        fs::create_dir_all(&temp_folder_path).unwrap();
    };
}



fn download_image() -> String
{
    let song_picture_link = art_url_get();
    let song_picture_cache_name = art_url_get().replace("https://i.scdn.co/image/", "");
    let current_song_picture_path = format!("{}{}{}{}", temp_dir().display(),"/.background/",&song_picture_cache_name,".png");

    if !Path::new(&current_song_picture_path).exists() 
    {
        command(&format!("{} {} {}", "curl --output", current_song_picture_path, song_picture_link));
    }

    current_song_picture_path
}



fn remove_unused_song_picture(current_song_picture_path: &str) 
{
    let dir_to_read = format!("{}/.background/", env::temp_dir().display());
    let items = fs::read_dir(dir_to_read).unwrap();

    for entry in items 
    {
        let entry_path = entry.unwrap().path();

        if entry_path.display().to_string() != current_song_picture_path && entry_path.is_file() 
        {
            fs::remove_file(&entry_path).unwrap();
        }
    }
}

pub struct Background<'a>
{
   pub background_texture: Texture<'a>,
   pub mini_album_texture: Texture<'a>,
   pub background_effect: Texture<'a>,
}

#[allow(deprecated)]
pub fn song_picture(texture_creator: &TextureCreator<WindowContext>) -> Background
{
    create_temp_folder();
    let background_image_path = download_image();
    remove_unused_song_picture(&background_image_path);


    let background_texture = texture_creator.load_texture(&background_image_path).unwrap();
    let mini_album_texture = texture_creator.load_texture(&background_image_path).unwrap();
    let background_effect = texture_creator.load_texture(format!("{}{}{}", home_dir().unwrap().display(), "/.config/rsmp/", "ui/system/effect.png")).unwrap();

    Background {background_texture, mini_album_texture, background_effect}
}
