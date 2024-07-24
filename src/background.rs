use sdl2::image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use crate::art_url_get;
use crate::clean_string_and_spaces;
use crate::command;
use crate::get_exe_path;

use std::path::Path;
use std::{env, fs};



const MINI_ALBUM_PICTURE_POSITION: [i32; 2] = [30, 200];
const MINI_ALBUM_PICTURE_SIZE: [u32; 2] = [200, 200];



fn get_song_picture_data() -> String {
    let song_picture_link = art_url_get();
    let temp_dir = env::temp_dir();
    let exe_path = get_exe_path();

    let holder_image_path = format!("{}{}", exe_path, "/ui/system/holder.png");
    let song_picture_cache_name = clean_string_and_spaces(song_picture_link, String::from("https://i.scdn.co/image/"));
    let current_song_picture_path = format!("{}{}{}{}", temp_dir.display(),"/.background/",&song_picture_cache_name,".png");

    if !Path::new(&current_song_picture_path).exists() {
        return holder_image_path;
    }

    current_song_picture_path
}


fn download_image() {
    let song_picture_link = art_url_get();
    let temp_dir = env::temp_dir();

    let raw_command_argument = format!("{} {}{} {}", "wget --quiet -P", temp_dir.display(),"/.background/", song_picture_link);
    let command_argument = raw_command_argument.replace('\n', "");

    let song_picture_cache_name = clean_string_and_spaces(song_picture_link, String::from("https://i.scdn.co/image/"));
    let song_picture_cache_path = format!("{}{}{}", temp_dir.display(),"/.background/",&song_picture_cache_name);
    let current_song_picture_path = format!("{}{}{}{}", temp_dir.display(),"/.background/",&song_picture_cache_name,".png");

    if !Path::new(&current_song_picture_path).exists() {
        command(&command_argument);
    }

    if Path::new(&song_picture_cache_path).exists() {
        fs::rename(song_picture_cache_path, current_song_picture_path).unwrap();
    }
}

fn remove_unused_song_picture(current_song_picture_path: String) {
    if !Path::new(&current_song_picture_path).exists() {
        panic!("Temp Dir Doesn't Exist");
    };

    let dir_to_read = format!("{}/.background/", env::temp_dir().display());
    let items = fs::read_dir(dir_to_read).unwrap();

    for entry in items {
        let entry_path = entry.unwrap().path();

        if entry_path.display().to_string() != current_song_picture_path && entry_path.is_file() {
            fs::remove_file(&entry_path).unwrap();
        }
    }
}


pub fn song_picture(texture_creator: &TextureCreator<WindowContext>) -> (Texture, Texture, Texture, Rect) {
    download_image();
    let current_song_picture_path = get_song_picture_data();
    remove_unused_song_picture(current_song_picture_path.clone());
    let current_exe = get_exe_path();

    let mini_album_picture_rect = Rect::new(MINI_ALBUM_PICTURE_POSITION[0], MINI_ALBUM_PICTURE_POSITION[1], MINI_ALBUM_PICTURE_SIZE[0], MINI_ALBUM_PICTURE_SIZE[1]);
    let background = texture_creator.load_texture(&current_song_picture_path).unwrap();
    let mini_album_picture = texture_creator.load_texture(&current_song_picture_path).unwrap();
    let effect = texture_creator.load_texture(&format!("{}{}", current_exe, "ui/system/effect.png")).unwrap();

    (background, mini_album_picture, effect, mini_album_picture_rect,)
}
