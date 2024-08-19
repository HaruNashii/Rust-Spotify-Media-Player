//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------------CRATES----------------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
use std::time::Duration;
use sdl2::rect::Rect;

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





const MINI_ALBUM_PICTURE_POSITION: [i32; 2] = [30, 200];
const MINI_ALBUM_PICTURE_SIZE: [u32; 2] = [200, 200];






fn main() 
{
    let (mut canvas, texture_creator, sdl_started) = window();
    let buttons_rect_vec = buttons();
    let (playing_image_white_theme, paused_image_white_theme, next_image_white_theme, previous_image_white_theme, shuffle_on_image_white_theme, shuffle_off_image_white_theme, muted_audio_image_white_theme, low_audio_image_white_theme, medium_audio_image_white_theme, high_audio_image_white_theme, playing_image, paused_image, next_image, previous_image, shuffle_on_image, shuffle_off_image, muted_audio_image, low_audio_image, medium_audio_image, high_audio_image) = basic_ui(&texture_creator);
    let mini_album_picture_rect = Rect::new(MINI_ALBUM_PICTURE_POSITION[0], MINI_ALBUM_PICTURE_POSITION[1], MINI_ALBUM_PICTURE_SIZE[0], MINI_ALBUM_PICTURE_SIZE[1]);

    let mut event_pump = sdl_started.event_pump().unwrap();
    loop 
    {
        std::thread::sleep(Duration::from_millis(32));

        let (audio_is_muted, audio_is_low, audio_is_medium, audio_is_high, volume_icon_rect) = volume_icon();
        let (volume_level_bar, under_volume_bar) = volume_bar();
        let (music_progress_bar_background_rect, music_progress_bar_rect) = progress_bar();
        let background = song_picture(&texture_creator);
        let (text_vector_fonts, rect_vector_fonts) = fonts(&texture_creator);

        sdl_events(&buttons_rect_vec, &mut event_pump);
        render_scene(text_vector_fonts, rect_vector_fonts, &playing_image_white_theme, &paused_image_white_theme, &next_image_white_theme, &previous_image_white_theme, &shuffle_on_image_white_theme, &shuffle_off_image_white_theme, &muted_audio_image_white_theme, &low_audio_image_white_theme, &medium_audio_image_white_theme, &high_audio_image_white_theme, music_progress_bar_background_rect, music_progress_bar_rect, volume_icon_rect, &playing_image, &paused_image, &next_image, &previous_image, &shuffle_on_image, &shuffle_off_image, &muted_audio_image, &low_audio_image, &medium_audio_image, &high_audio_image, &buttons_rect_vec, volume_level_bar, under_volume_bar, background, mini_album_picture_rect, audio_is_muted, audio_is_low, audio_is_medium, audio_is_high, &mut canvas);
    }
}
