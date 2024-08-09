
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use playerctl::PlayerCtl;

use crate::playerctl_extra::*;
use crate::get_exe_path;
use crate::convert_to_u32;


//====================================//
//========(ELEMENTS POSITION)=========//
//====================================//
// volume bar
const VOLUME_ICON_POSITION: [i32; 2] = [525, 535];
const VOLUME_BAR_BACKGROUND_POSITION: [i32; 2] = [560, 543];
const VOLUME_LEVEL_BAR_POSITION: [i32; 2] = [560, 543];

// progress bar
const PROGRESS_BAR_BACKGROUND_POSITION: [i32; 2] = [30, 450];
const PROGRESS_BAR_POSITION: [i32; 2] = [30, 450];
const PROGRESS_BAR_TIME_REMAINING_POSITION: [i32; 2] = [700, 445];

// media buttons
const DEFAULT_BUTTON_MEDIA_POSITION: [i32; 2] = [375, 525];
const SMALL_BUTTON_MEDIA_POSITION: [i32; 2] = [283, 533];
const DEFAULT_BUTTON_PADDING: i32 = 75;
const SMALL_BUTTON_PADDING: i32 = 45;

// fonts
const VOLUME_INTEGER_FONT_POSITION: [i32; 2] = [590, 532];
const MUSIC_NAME_FONT_POSITION: [i32; 2] = [240, 310];
const MUSIC_ARTIST_NAME_FONT_POSITION: [i32; 2] = [245, 345];
const MUSIC_ALBUM_NAME_FONT_POSITION: [i32; 2] = [245, 370];

//====================================//
//=========(ELEMENTS SCALES)==========//
//====================================//
// fonts
const DEFAULT_FONT_SIZE: u16 = 20;
const SMALL_FONT_SIZE: u16 = 13;
const DEFAULT_VOLUME_FONT_SIZE: u32 = 25;

// buttons
pub const DEFAULT_BUTTON_SIZE: [i32; 2] = [50, 50];
pub const SMALL_BUTTON_SIZE: [i32; 2] = [35, 35];

// volume bar
const DEFAULT_VOLUME_BAR_SIZE: [u32; 2] = [0, 7];
const DEFAULT_VOLUME_BAR_BACKGROUND_SIZE: [u32; 2] = [35, 7];

// progress bar
const PROGRESS_BAR_SIZE: [u32; 2] = [675, 20];

//====================================//
//========(ELEMENTS ARGUMENTS)========//
//====================================//
// fonts
const DEFAULT_FONT_PATH: &str = "fonts/JetBrainsMonoNLNerdFontMono-Bold.ttf";
const DEFAULT_FONT_COLOR: Color = Color::RGB(255, 255, 255);





//=====================================================================================//
//-------------------------------------------------------------------------------------//
//-----------THE TOOLS FUNCTIONS THAT IS USED BY THE DATA CREATORS FUNCTIONS-----------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
fn font_generator<'a>(additional_text: &str, texture_creator: &'a TextureCreator<sdl2::video::WindowContext>, size: u16, text: String, x: i32, y: i32, ) -> (Texture<'a>, Rect) {
    let ttf_context = sdl2::ttf::init().unwrap();

    let exe_path = get_exe_path();
    let font_path = format!("{}{}", exe_path, DEFAULT_FONT_PATH);

    let font = ttf_context.load_font(font_path, size).unwrap();
    let surface = font.render(&format!("{}{}", additional_text, text)).blended(DEFAULT_FONT_COLOR).unwrap();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    let font_rect = Rect::new(x, y, surface.width(), surface.height());

    (texture, font_rect)
}








//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------THE DATA CREATOR FUNCTIONS--------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
pub fn basic_ui(texture_creator: &TextureCreator<WindowContext>) -> (Texture, Texture, Texture, Texture, Texture, Texture, Texture, Texture, Texture, Texture, Texture, Texture, Texture, Texture, Texture, Texture, Texture, Texture, Texture, Texture) {
    //all this ui images localitions is using the buttons rect so to change the position of the image you will need to change the position of the buttons
    let playing_image = texture_creator.load_texture("ui/Black-Theme/media-playback-start.png").unwrap();
    let paused_image = texture_creator.load_texture("ui/Black-Theme/media-playback-pause.png").unwrap();
    let next_image = texture_creator.load_texture("ui/Black-Theme/media-skip-forward.png").unwrap();
    let previous_image = texture_creator.load_texture("ui/Black-Theme/media-skip-backward.png").unwrap();
    let shuffle_on_image = texture_creator.load_texture("ui/Black-Theme/media-random-albums-amarok.png").unwrap();
    let shuffle_off_image = texture_creator.load_texture("ui/Black-Theme/media-playlist-shuffle.png").unwrap();
    let muted_audio_image = texture_creator.load_texture("ui/Black-Theme/notification-audio-volume-muted.png").unwrap();
    let low_audio_image = texture_creator.load_texture("ui/Black-Theme/notification-audio-volume-low.png").unwrap();
    let medium_audio_image = texture_creator.load_texture("ui/Black-Theme/notification-audio-volume-medium.png").unwrap();
    let high_audio_image = texture_creator.load_texture("ui/Black-Theme/notification-audio-volume-high.png").unwrap();
    let playing_image_white_theme = texture_creator.load_texture("ui/White-Theme/media-playback-start.png").unwrap();
    let paused_image_white_theme = texture_creator.load_texture("ui/White-Theme/media-playback-pause.png").unwrap();
    let next_image_white_theme = texture_creator.load_texture("ui/White-Theme/media-skip-forward.png").unwrap();
    let previous_image_white_theme = texture_creator.load_texture("ui/White-Theme/media-skip-backward.png").unwrap();
    let shuffle_on_image_white_theme = texture_creator.load_texture("ui/White-Theme/media-random-albums-amarok.png").unwrap();
    let shuffle_off_image_white_theme = texture_creator.load_texture("ui/White-Theme/media-playlist-shuffle.png").unwrap();
    let muted_audio_image_white_theme = texture_creator.load_texture("ui/Black-Theme/notification-audio-volume-muted.png").unwrap();
    let low_audio_image_white_theme = texture_creator.load_texture("ui/White-Theme/notification-audio-volume-low.png").unwrap();
    let medium_audio_image_white_theme = texture_creator.load_texture("ui/White-Theme/notification-audio-volume-medium.png").unwrap();
    let high_audio_image_white_theme = texture_creator.load_texture("ui/White-Theme/notification-audio-volume-high.png").unwrap();

    (playing_image_white_theme, paused_image_white_theme, next_image_white_theme, previous_image_white_theme, shuffle_on_image_white_theme, shuffle_off_image_white_theme, muted_audio_image_white_theme, low_audio_image_white_theme, medium_audio_image_white_theme, high_audio_image_white_theme, playing_image, paused_image, next_image, previous_image, shuffle_on_image, shuffle_off_image, muted_audio_image, low_audio_image, medium_audio_image, high_audio_image)
}



pub fn volume_icon() -> (bool, bool, bool, bool, Rect) {
    let mut audio_is_muted = false;
    let mut audio_is_low = false;
    let mut audio_is_medium = false;
    let mut audio_is_high = false;

    let volume = volume_get();

    if volume == "0.0" { audio_is_muted = true; };
    if volume == "0.1" || volume == "0.2" || volume == "0.3" || volume == "0.4" { audio_is_low = true;};
    if volume == "0.5" || volume == "0.6" || volume == "0.7" { audio_is_medium = true; };
    if volume == "0.8" || volume == "0.9" || volume == "1.0" { audio_is_high = true; };

    let volume_icon_rect = Rect::new(VOLUME_ICON_POSITION[0], VOLUME_ICON_POSITION[1], DEFAULT_VOLUME_FONT_SIZE, DEFAULT_VOLUME_FONT_SIZE);

    (audio_is_muted, audio_is_low, audio_is_medium, audio_is_high, volume_icon_rect,)
}



pub fn volume_bar() -> (Rect, Rect) {
    let mut volume_level_bar = Rect::new(VOLUME_LEVEL_BAR_POSITION[0], VOLUME_LEVEL_BAR_POSITION[1], DEFAULT_VOLUME_BAR_SIZE[0], DEFAULT_VOLUME_BAR_SIZE[1]);
    let under_volume_bar = Rect::new(VOLUME_BAR_BACKGROUND_POSITION[0], VOLUME_BAR_BACKGROUND_POSITION[1], DEFAULT_VOLUME_BAR_BACKGROUND_SIZE[0], DEFAULT_VOLUME_BAR_BACKGROUND_SIZE[1]);

    let volume = volume_get();

    if volume == "0.0" { volume_level_bar.w = 0; };
    if volume == "0.1" { volume_level_bar.w = 3; };
    if volume == "0.2" { volume_level_bar.w = 5; };
    if volume == "0.3" { volume_level_bar.w = 7; };
    if volume == "0.4" { volume_level_bar.w = 10; };
    if volume == "0.5" { volume_level_bar.w = 12; };
    if volume == "0.6" { volume_level_bar.w = 17; };
    if volume == "0.7" { volume_level_bar.w = 22; };
    if volume == "0.8" { volume_level_bar.w = 28; };
    if volume == "0.9" { volume_level_bar.w = 30; };
    if volume == "1.0" { volume_level_bar.w = 35; };

    (volume_level_bar, under_volume_bar)
}



pub fn progress_bar() -> (Rect, Rect) {
    let total_time = total_time_get();
    let current_time = current_time_get();

    let music_total_time_u32 = convert_to_u32(&total_time);
    let music_current_time_u32 = convert_to_u32(&current_time);

    let music_current_time_percentage: f32 = music_current_time_u32 as f32 / music_total_time_u32 as f32 * 100.0;

    let progress_bar_background_rect = Rect::new(PROGRESS_BAR_BACKGROUND_POSITION[0], PROGRESS_BAR_BACKGROUND_POSITION[1], PROGRESS_BAR_SIZE[0], PROGRESS_BAR_SIZE[1]);
    let progress_rect = Rect::new(PROGRESS_BAR_POSITION[0], PROGRESS_BAR_POSITION[1], music_current_time_percentage as u32 * PROGRESS_BAR_SIZE[0] / 100, PROGRESS_BAR_SIZE[1]);

    (progress_bar_background_rect, progress_rect)
}



pub fn buttons() -> Vec<Rect> {
    let previous_rect = Rect::new(DEFAULT_BUTTON_MEDIA_POSITION[0] - DEFAULT_BUTTON_PADDING, DEFAULT_BUTTON_MEDIA_POSITION[1], DEFAULT_BUTTON_SIZE[0].try_into().unwrap(), DEFAULT_BUTTON_SIZE[1].try_into().unwrap());
    let pause_rect = Rect::new(DEFAULT_BUTTON_MEDIA_POSITION[0], DEFAULT_BUTTON_MEDIA_POSITION[1], DEFAULT_BUTTON_SIZE[0].try_into().unwrap(), DEFAULT_BUTTON_SIZE[1].try_into().unwrap());
    let next_rect = Rect::new(DEFAULT_BUTTON_MEDIA_POSITION[0] + DEFAULT_BUTTON_PADDING, DEFAULT_BUTTON_MEDIA_POSITION[1], DEFAULT_BUTTON_SIZE[0].try_into().unwrap(), DEFAULT_BUTTON_SIZE[1].try_into().unwrap());
    let shuffle_button_rect = Rect::new(SMALL_BUTTON_MEDIA_POSITION[0] - SMALL_BUTTON_PADDING, SMALL_BUTTON_MEDIA_POSITION[1], SMALL_BUTTON_SIZE[0].try_into().unwrap(), SMALL_BUTTON_SIZE[1].try_into().unwrap());

    vec![previous_rect, pause_rect, next_rect, shuffle_button_rect]
}



pub fn fonts(texture_creator: &TextureCreator<WindowContext>) -> (Vec<Texture>, Vec<Rect>) {
    let metadata_track = PlayerCtl::metadata();
    let remaining_time = remaining_time_get();
    let volume = volume_get();

    let (music_artist_text, music_artist_rect) = font_generator(" ", texture_creator, SMALL_FONT_SIZE, metadata_track.artist, MUSIC_ARTIST_NAME_FONT_POSITION[0], MUSIC_ARTIST_NAME_FONT_POSITION[1]);
    let (music_name_text, music_name_rect) = font_generator(" ", texture_creator, DEFAULT_FONT_SIZE, metadata_track.title, MUSIC_NAME_FONT_POSITION[0], MUSIC_NAME_FONT_POSITION[1]);
    let (music_album_text, music_album_rect) = font_generator(" ", texture_creator, SMALL_FONT_SIZE, metadata_track.album, MUSIC_ALBUM_NAME_FONT_POSITION[0], MUSIC_ALBUM_NAME_FONT_POSITION[1]);
    let (time_remaining_text, time_remaining_rect) = font_generator(" ", texture_creator, DEFAULT_FONT_SIZE, remaining_time, PROGRESS_BAR_TIME_REMAINING_POSITION[0], PROGRESS_BAR_TIME_REMAINING_POSITION[1]);
    let (volume_integer_text, volume_integer_rect) = font_generator(" ", texture_creator, DEFAULT_FONT_SIZE, volume, VOLUME_INTEGER_FONT_POSITION[0], VOLUME_INTEGER_FONT_POSITION[1]);

    let text_vector = vec![music_artist_text, music_name_text, music_album_text, time_remaining_text, volume_integer_text];
    let rect_vector = vec![music_artist_rect, music_name_rect, music_album_rect, time_remaining_rect, volume_integer_rect];

    (text_vector, rect_vector)
}
