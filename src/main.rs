//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------------CRATES----------------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
use std::env;
use std::time::Duration;

use background::song_picture;
use renderer::{render_scene, window};
use sdl_events::sdl_events;
use ui::{buttons, basic_ui, fonts, progress_bar, volume_bar, volume_icon};
use playerctl_extra::*;

mod playerctl_extra;
mod background;
mod renderer;
mod sdl_events;
mod ui;






pub fn clean_string_and_spaces(received_string: String, additional_cleaning: String) -> String {
    if additional_cleaning.len() > 1 {
        let clean_string_start = received_string.replace(' ', "");
        let clean_string_middle = clean_string_start.replace(&additional_cleaning, "");
        let clean_string_final = clean_string_middle.replace('\n', "");
        return clean_string_final;
    }

    let clean_string_start = received_string.replace(' ', "");
    clean_string_start.replace('\n', "")
}




pub fn convert_to_u32(string: &str) -> u32 {
    let raw_string = string.replace(':', "");
    let mut string = raw_string.replace('\'', "");
    string.pop();

    if string.is_empty() { string.push('0')};

    let u32_to_return: u32 = string.parse().unwrap();

    u32_to_return
}




pub fn get_exe_path() -> String {
    let mut current_path = String::new();
    match env::current_exe() {
        Ok(exe_path) => current_path.push_str(&exe_path.display().to_string()),
        Err(_) => println!("ERROR! Fail Getting Current Directory Path"),
    }
    if let Some(index) = current_path.rfind('/') {
        current_path.truncate(index + 1);
    };

    current_path
}




fn main() {
    let (mut canvas, texture_creator, sdl_started) = window();
    let buttons_rect_vec = buttons();
    let (playing_image_white_theme, paused_image_white_theme, next_image_white_theme, previous_image_white_theme, shuffle_on_image_white_theme, shuffle_off_image_white_theme, muted_audio_image_white_theme, low_audio_image_white_theme, medium_audio_image_white_theme, high_audio_image_white_theme, playing_image, paused_image, next_image, previous_image, shuffle_on_image, shuffle_off_image, muted_audio_image, low_audio_image, medium_audio_image, high_audio_image) = basic_ui(&texture_creator);
    
    let mut event_pump = sdl_started.event_pump().unwrap();

    loop {
        std::thread::sleep(Duration::from_millis(32));

        let (audio_is_muted, audio_is_low, audio_is_medium, audio_is_high, volume_icon_rect) = volume_icon();
        let (volume_level_bar, under_volume_bar) = volume_bar();
        let (music_progress_bar_background_rect, music_progress_bar_rect) = progress_bar();
        let (background, album_picture, effect, album_picture_rect) = song_picture(&texture_creator);
        let (text_vector_fonts, rect_vector_fonts) = fonts(&texture_creator);

        sdl_events(&buttons_rect_vec, &mut event_pump);
        render_scene(text_vector_fonts, rect_vector_fonts, &playing_image_white_theme, &paused_image_white_theme, &next_image_white_theme, &previous_image_white_theme, &shuffle_on_image_white_theme, &shuffle_off_image_white_theme, &muted_audio_image_white_theme, &low_audio_image_white_theme, &medium_audio_image_white_theme, &high_audio_image_white_theme, music_progress_bar_background_rect, music_progress_bar_rect, volume_icon_rect, &playing_image, &paused_image, &next_image, &previous_image, &shuffle_on_image, &shuffle_off_image, &muted_audio_image, &low_audio_image, &medium_audio_image, &high_audio_image, &buttons_rect_vec, volume_level_bar, under_volume_bar, background, album_picture, album_picture_rect, effect, audio_is_muted, audio_is_low, audio_is_medium, audio_is_high, &mut canvas);
    }
}
