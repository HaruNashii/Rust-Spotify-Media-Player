use crate::shuffle_get;
use crate::status_get;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};





//====================================//
//===============(WINDOW)=============//
//====================================//
// DISCLAIMER: DON'T CHANGE IT, IT WILL BROKE ALL THE ELEMENTS POSITION
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

//====================================//
//===========(MISC OPTIONS)===========//
//====================================//
const WHITE_THEME_FOR_ICONS: bool = false;

//====================================//
//========(ELEMENTS ARGUMENTS)========//
//====================================//
// buttons
const DEFAULT_BUTTON_COLOR: Color = Color::RGB(255, 0, 0);

// progress bar
const DEFAULT_PROGRESS_BAR_COLOR: Color = Color::RGB(255, 0, 0);
const DEFAULT_BACKGROUND_PROGRESS_BAR_COLOR: Color = Color::RGB(50, 50, 50);

// volume bar
const DEFAULT_VOLUME_BAR_COLOR: Color = Color::RGB(255, 0, 0);
const DEFAULT_BACKGROUND_VOLUME_BAR_COLOR: Color = Color::RGB(50, 50, 50);

//===========================================================================================================//
//=======*red theme*=====//  others theme here
// 255, 255, 255         //
// 255, 0, 0             //
// 100, 100, 100         //
// 255, 0, 0             //
// 50, 50, 50            //
// 255, 0, 0             //
// 50, 50, 50            //
// 255, 0, 0             //
// 255, 0, 0             //
// 255, 0, 0             //
// 255, 0, 0             //
// black and white theme //
//===========================================================================================================//





pub fn window() -> (Canvas<Window>, TextureCreator<WindowContext>, sdl2::Sdl) {
    let sdl_started = sdl2::init().unwrap();
    let video_system = sdl_started.video().unwrap();
    let window = video_system.window("Media", WINDOW_WIDTH, WINDOW_HEIGHT).position_centered().build().map_err(|e| e.to_string()).unwrap();

    let mut canvas = window.into_canvas().accelerated().build().map_err(|e| e.to_string()).unwrap();
    canvas.set_logical_size(WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();

    let texture_creator = canvas.texture_creator();

    (canvas, texture_creator, sdl_started)
}


pub fn render_scene(text_vector_fonts: Vec<Texture>, rect_vector_fonts: Vec<Rect>, playing_image_white_theme: &Texture, paused_image_white_theme: &Texture, next_image_white_theme: &Texture, previous_image_white_theme: &Texture, shuffle_on_image_white_theme: &Texture, shuffle_off_image_white_theme: &Texture, muted_audio_image_white_theme: &Texture, low_audio_image_white_theme: &Texture, medium_audio_image_white_theme: &Texture, high_audio_image_white_theme: &Texture, music_progress_bar_background: Rect, music_progress_bar_rect: Rect, volume_rect: Rect, playing_image: &Texture, paused_image: &Texture, next_image: &Texture, previous_image: &Texture, shuffle_on_image: &Texture, shuffle_off_image: &Texture, muted_audio_image: &Texture, low_audio_image: &Texture, medium_audio_image: &Texture, high_audio_image: &Texture, previous_rect: Rect, pause_rect: Rect, next_rect: Rect, shuffle_button_rect: Rect, volume_level_bar: Rect, under_volume_bar: Rect, background: Texture, album_picture: Texture, album_picture_rect: Rect, effect: Texture, audio_is_muted: bool, audio_is_low: bool, audio_is_medium: bool, audio_is_high: bool, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // background
    canvas.copy(&background, None, None).unwrap();
    canvas.copy(&effect, None, None).unwrap();

    // mini album picture
    canvas.copy(&album_picture, None, album_picture_rect).unwrap();

    // media buttons
    canvas.set_draw_color(DEFAULT_BUTTON_COLOR);
    canvas.fill_rect(shuffle_button_rect).unwrap();
    canvas.fill_rect(previous_rect).unwrap();
    canvas.fill_rect(pause_rect).unwrap();
    canvas.fill_rect(next_rect).unwrap();

    // volume bar
    canvas.set_draw_color(DEFAULT_BACKGROUND_VOLUME_BAR_COLOR);
    canvas.fill_rect(under_volume_bar).unwrap();
    canvas.set_draw_color(DEFAULT_VOLUME_BAR_COLOR);
    canvas.fill_rect(volume_level_bar).unwrap();

    // progress bar
    canvas.set_draw_color(DEFAULT_BACKGROUND_PROGRESS_BAR_COLOR);
    canvas.fill_rect(music_progress_bar_background).unwrap();
    canvas.set_draw_color(DEFAULT_PROGRESS_BAR_COLOR);
    canvas.fill_rect(music_progress_bar_rect).unwrap();

    // media buttons
    if !WHITE_THEME_FOR_ICONS { canvas.copy(next_image, None, next_rect).unwrap(); }
    if !WHITE_THEME_FOR_ICONS { canvas.copy(previous_image, None, previous_rect).unwrap(); }
    if WHITE_THEME_FOR_ICONS { canvas.copy(next_image_white_theme, None, next_rect).unwrap();}
    if WHITE_THEME_FOR_ICONS { canvas.copy(previous_image_white_theme, None, previous_rect).unwrap(); }

    // audio icon
    if audio_is_muted && !WHITE_THEME_FOR_ICONS {canvas.copy(muted_audio_image, None, volume_rect).unwrap(); }
    if audio_is_low && !WHITE_THEME_FOR_ICONS {canvas.copy(low_audio_image, None, volume_rect).unwrap(); }
    if audio_is_medium && !WHITE_THEME_FOR_ICONS {canvas.copy(medium_audio_image, None, volume_rect).unwrap(); }
    if audio_is_high && !WHITE_THEME_FOR_ICONS { canvas.copy(high_audio_image, None, volume_rect).unwrap(); }

    if audio_is_muted && WHITE_THEME_FOR_ICONS { canvas.copy(muted_audio_image_white_theme, None, volume_rect).unwrap(); }
    if audio_is_low && WHITE_THEME_FOR_ICONS { canvas.copy(low_audio_image_white_theme, None, volume_rect).unwrap(); }
    if audio_is_medium && WHITE_THEME_FOR_ICONS { canvas.copy(medium_audio_image_white_theme, None, volume_rect).unwrap(); }
    if audio_is_high && WHITE_THEME_FOR_ICONS { canvas.copy(high_audio_image_white_theme, None, volume_rect).unwrap(); }

    // status icon
    let mut status_info = status_get();
    status_info.pop();
    if status_info == "Paused" && !WHITE_THEME_FOR_ICONS { canvas.copy(paused_image, None, pause_rect).unwrap(); }
    if status_info == "Paused" && WHITE_THEME_FOR_ICONS { canvas.copy(paused_image_white_theme, None, pause_rect).unwrap(); }
    if status_info == "Playing" && !WHITE_THEME_FOR_ICONS { canvas.copy(playing_image, None, pause_rect).unwrap(); }
    if status_info == "Playing" && WHITE_THEME_FOR_ICONS { canvas.copy(playing_image_white_theme, None, pause_rect).unwrap(); }

    // shuffle icon
    let mut shuffle_info = shuffle_get();
    shuffle_info.pop();
    if shuffle_info == "On" && !WHITE_THEME_FOR_ICONS { canvas.copy(shuffle_on_image, None, shuffle_button_rect).unwrap(); }
    if shuffle_info == "On" && WHITE_THEME_FOR_ICONS { canvas.copy(shuffle_on_image_white_theme, None, shuffle_button_rect).unwrap(); }
    if shuffle_info == "Off" && !WHITE_THEME_FOR_ICONS { canvas.copy(shuffle_off_image, None, shuffle_button_rect).unwrap(); }
    if shuffle_info == "Off" && WHITE_THEME_FOR_ICONS { canvas.copy(shuffle_off_image_white_theme, None, shuffle_button_rect).unwrap(); }

    // fonts
    for index in 0..text_vector_fonts.len() { canvas.copy(&text_vector_fonts[index], None, rect_vector_fonts[index]).unwrap(); }

    canvas.present();
}
