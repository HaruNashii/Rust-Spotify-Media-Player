//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------------CRATES----------------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
use sdl2::video::WindowContext;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use sdl2::rect::{Rect, Point};
use sdl2::render::TextureCreator;
use sdl2::image::LoadTexture;
use std::time::Duration;
use std::path::Path;
use std::fs;
use std::env;
use playerctl_extra::*;
use playerctl::PlayerCtl;
use renderer::{render_scene, window};

mod playerctl_extra;
mod renderer;





//=====================================================================================//
//-------------------------------------------------------------------------------------//
//-----------------------------------MAIN DATAS----------------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//

//====================================//
//===========(MISC OPTIONS)===========//
//====================================//
const AUDIO_STEP: f32 = 0.1;

//====================================//
//========(ELEMENTS POSITION)=========//
//====================================//
// the background image have the position based on the screen size (that's why it's not here)

// volume bar
const VOLUME_TEXT_POSITION: [i32;2] = [525, 535];
const VOLUME_BAR_BACKGROUND_POSITION: [i32;2] = [560, 543];
const VOLUME_LEVEL_BAR_POSITION: [i32;2] = [560, 543];
const DEFAULT_VOLUME_MUTED_BAR_ICON_INDICATOR_POSITION: [i32;2] = [527, 542];
const DEFAULT_VOLUME_LOW_BAR_ICON_INDICATOR_POSITION: [i32;2] = [543, 542];
const DEFAULT_VOLUME_MEDIUM_BAR_ICON_INDICATOR_POSITION: [i32;2] = [542, 540];
const DEFAULT_VOLUME_HIGH_BAR_ICON_INDICATOR_POSITION: [i32;2] = [542, 538];
const DEFAULT_VOLUME_BAR_PADDING: i32 = 4;
// progress bar
const PROGRESS_BAR_BACKGROUND_POSITION: [i32;2] = [30, 450];
const PROGRESS_BAR_POSITION: [i32;2] = [30, 450];
const PROGRESS_BAR_TIME_REMAINING_POSITION: [i32;2] = [700, 445];

// mini album picture
const MINI_ALBUM_PICTURE_POSITION: [i32;2] = [30, 200];

// media buttons
const DEFAULT_BUTTON_MEDIA_POSITION: [i32;2] = [375, 525];
const SMALL_BUTTON_MEDIA_POSITION: [i32;2] = [283, 533];
const DEFAULT_BUTTON_PADDING: i32 = 75;
const SMALL_BUTTON_PADDING: i32 = 45;
const SHUFFLE_INDICATOR_POSITION: [i32;2] = [289, 548];

// fonts
const VOLUME_INTEGER_FONT_POSITION: [i32;2] = [590, 532];
const MUSIC_NAME_FONT_POSITION: [i32;2] = [240, 310];
const MUSIC_ARTIST_NAME_FONT_POSITION: [i32;2] = [245, 345];
const MUSIC_ALBUM_NAME_FONT_POSITION: [i32;2] = [245, 370];

//====================================//
//=========(ELEMENTS SCALES)==========//
//====================================//
// the background image have the scale based on the screen size (that's why it's not here)

// fonts
const DEFAULT_FONT_SIZE: u16 = 20;
const SMALL_FONT_SIZE: u16 = 13;

// buttons 
const DEFAULT_BUTTON_SIZE: [i32;2] = [50, 50];
const SMALL_BUTTON_SIZE: [i32;2] = [35, 35];
const SHUFFLE_INDICATOR_SIZE: [u32;2] = [5, 5];

// volume bar
const DEFAULT_VOLUME_BAR_SIZE: [u32;2] = [0, 7];
const DEFAULT_VOLUME_BAR_BACKGROUND_SIZE: [u32;2] = [35, 7];
const DEFAULT_VOLUME_MUTED_BAR_ICON_INDICATOR_SIZE: [u32;2] = [10, 10];
const DEFAULT_VOLUME_LOW_BAR_ICON_INDICATOR_SIZE: [u32;2] = [1, 10];
const DEFAULT_VOLUME_MEDIUM_BAR_ICON_INDICATOR_SIZE: [u32;2] = [2, 13];
const DEFAULT_VOLUME_HIGH_BAR_ICON_INDICATOR_SIZE: [u32;2] = [3, 17];

// mini album picture
const MINI_ALBUM_PICTURE_SIZE: [u32;2] = [200, 200];

// progress bar 
const PROGRESS_BAR_SIZE: [u32;2] = [675, 20];


//====================================//
//========(ELEMENTS ARGUMENTS)========//
//====================================//
// fonts
const DEFAULT_FONT_PATH: &str = "fonts/JetBrainsMonoNLNerdFontMono-Bold.ttf";
const DEFAULT_FONT_COLOR: Color = Color::RGB(255, 255, 255);





//=====================================================================================//
//-------------------------------------------------------------------------------------//
//-----------------------THE GENERATORS USED BY THE TOOLS FUNCTIONS--------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//

fn font_generator<'a>(additional_text: &str, texture_creator: &'a TextureCreator<sdl2::video::WindowContext>, size: u16, text: String, x: i32, y: i32) -> (Texture<'a>, Rect)
{
    let ttf_context = sdl2::ttf::init().unwrap();

    let exe_path = get_exe_path();
    let font_path = format!("{}{}", exe_path,DEFAULT_FONT_PATH);

    let font = ttf_context.load_font(font_path, size).unwrap();
    let surface = font
    .render(&format! ("{}{}", additional_text, text))
    .blended(DEFAULT_FONT_COLOR)
    .unwrap();

    let texture = texture_creator
    .create_texture_from_surface(&surface).unwrap();
    let font_rect = Rect::new(x, y, surface.width(), surface.height());

    (texture, font_rect)
}


//=====================================================================================//
//-------------------------------------------------------------------------------------//
//-----------THE TOOLS FUNCTIONS THAT IS USED BY THE DATA CREATORS FUNCTIONS-----------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//


//=====================================================================================//
//-----------------------------------OFF-TOPIC----------------------------------------//
//=====================================================================================//
fn draw_x_rect() -> (Point, Point, Point, Point)
{
    let rect = Rect::new(DEFAULT_VOLUME_MUTED_BAR_ICON_INDICATOR_POSITION[0], DEFAULT_VOLUME_MUTED_BAR_ICON_INDICATOR_POSITION[1], DEFAULT_VOLUME_MUTED_BAR_ICON_INDICATOR_SIZE[0], DEFAULT_VOLUME_MUTED_BAR_ICON_INDICATOR_SIZE[1]);

    let top_left = rect.top_left();
    let bottom_left = rect.bottom_left();
    let top_right = rect.top_right();
    let bottom_right = rect.bottom_right();

    return (top_left, bottom_left, top_right, bottom_right);
}



fn clean_string_and_spaces(received_string: String, additional_cleaning: String) -> String
{
    if additional_cleaning.len() > 1 
    {
        let clean_string_start = received_string.replace(" ", "");
        let clean_string_middle = clean_string_start.replace(&additional_cleaning, "");
        let clean_string_final = clean_string_middle.replace("\n", "");
        return clean_string_final;
    }
    else 
    {
        let clean_string_start = received_string.replace(" ", "");
        let clean_string_final = clean_string_start.replace("\n", "");
        return clean_string_final;
    }

}



//=====================================================================================//
//---------------------------------PROGRESS BAR----------------------------------------//
//=====================================================================================//
fn get_percentage(x: u32, y: u32) -> f32
{
    let result: f32 = x as f32 / y as f32 * 100.0;

    return result;
}



//-------------------------------------------------------------------------------------//
//----------------------------------BACKGROUND-----------------------------------------//
//-------------------------------------------------------------------------------------//
fn remove_unused_song_picture(current_song_picture_path: String)
{           
           if !Path::new(&current_song_picture_path).exists() {
                panic!("Temp Dir Doesn't Exist");
           };

                let dir_to_read = format!("{}/.background/", env::temp_dir().display());
                let items = fs::read_dir(dir_to_read).unwrap();

                for entry in items 
                {
                    let entry = entry.unwrap();
                    let entry_path = entry.path();
                    
                    if entry_path.display().to_string() != current_song_picture_path
                    {
                        if entry_path.is_file() {
                            fs::remove_file(&entry_path).unwrap();
                        } 
                    }
                 }
}



fn get_song_picture_data() -> (String, String, String)
{
    let song_picture_link = art_url_get();
    let temp_dir = env::temp_dir();
    let exe_path = get_exe_path();
    
    let holder_image_path = format!("{}{}", exe_path,"/ui/system/holder.png");

    let song_picture_cache_name = clean_string_and_spaces(song_picture_link, String::from("https://i.scdn.co/image/"));
    let current_song_picture_name = format!("{}{}", &song_picture_cache_name, ".png");
    let current_song_picture_path = format!("{}{}{}{}", temp_dir.display(),"/.background/", &song_picture_cache_name, ".png");


    if !Path::new(&current_song_picture_path).exists() 
    {
        return (current_song_picture_name, current_song_picture_path, holder_image_path);
    }

    return (current_song_picture_name, current_song_picture_path.clone(), current_song_picture_path);
}



fn get_exe_path() -> String {
    let mut current_path = String::new();
    match env::current_exe() {
        Ok(exe_path) => current_path.push_str(&exe_path.display().to_string()),
        Err(_) => println!("ERROR! Fail Getting Current Directory Path"),
    }
    if let Some(index) = current_path.rfind('/') {
        current_path.truncate(index + 1);
    };

    return current_path;
}





//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------THE DATA CREATOR FUNCTIONS--------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//

//=====================================================================================//
//---------------------------------------THE BACKGROUND DATA---------------------------//
//=====================================================================================//
fn download_image()
{
    //commands
    let song_picture_link = art_url_get();
    let temp_dir = env::temp_dir();

    let raw_command_argument = format!("{} {}{} {}", "wget --quiet -P",  temp_dir.display(),"/.background/", song_picture_link);
    let command_argument = raw_command_argument.replace("\n", "");
    command(&command_argument);

    //strings
    let song_picture_cache_name = clean_string_and_spaces(song_picture_link, String::from("https://i.scdn.co/image/"));
    let song_picture_cache_path = format!("{}{}{}", temp_dir.display(),"/.background/", &song_picture_cache_name);
    let current_song_picture_path = format!("{}{}{}{}", temp_dir.display(),"/.background/", &song_picture_cache_name, ".png");
    
    
    if Path::new(&current_song_picture_path).exists() {
        return;
    }

    if Path::new(&song_picture_cache_path).exists() 
    {
        fs::rename(song_picture_cache_path, current_song_picture_path).unwrap();
    }
    else 
    {
        command(&command_argument);
    }
}



//=====================================================================================//
//---------------------------------------THE UI DATA-----------------------------------//
//=====================================================================================//
fn basic_ui<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> (Rect,  Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>,  Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>,  Texture<'a>)
{
    //all this ui images localitions is using the buttons rect so to change the position of the image you will need    to change the position of the buttons
    let playing_image =                             texture_creator.load_texture("ui/Black-Theme/media-playback-start.png").unwrap();
    let paused_image =                              texture_creator.load_texture("ui/Black-Theme/media-playback-pause.png").unwrap();
    let next_image =                                texture_creator.load_texture("ui/Black-Theme/media-skip-forward.png").unwrap();
    let previous_image =                            texture_creator.load_texture("ui/Black-Theme/media-skip-backward.png").unwrap();
    let shuffle_on_image =                          texture_creator.load_texture("ui/Black-Theme/media-random-albums-amarok.png").unwrap();
    let shuffle_off_image =                         texture_creator.load_texture("ui/Black-Theme/media-playlist-shuffle.png").unwrap();

    let muted_audio_image =                         texture_creator.load_texture("ui/Black-Theme/notification-audio-volume-muted.png").unwrap();
    let low_audio_image =                           texture_creator.load_texture("ui/Black-Theme/notification-audio-volume-low.png").unwrap();
    let medium_audio_image =                        texture_creator.load_texture("ui/Black-Theme/notification-audio-volume-medium.png").unwrap();
    let high_audio_image =                          texture_creator.load_texture("ui/Black-Theme/notification-audio-volume-high.png").unwrap();
    let volume_icon_without_indicator =             texture_creator.load_texture("ui/Black-Theme/notification-audio-volume-no-indicator.png").unwrap();
    
    let playing_image_white_theme =                 texture_creator.load_texture("ui/White-Theme/media-playback-start.png").unwrap();
    let paused_image_white_theme =                  texture_creator.load_texture("ui/White-Theme/media-playback-pause.png").unwrap();
    let next_image_white_theme =                    texture_creator.load_texture("ui/White-Theme/media-skip-forward.png").unwrap();
    let previous_image_white_theme =                texture_creator.load_texture("ui/White-Theme/media-skip-backward.png").unwrap();
    let shuffle_on_image_white_theme =              texture_creator.load_texture("ui/White-Theme/media-random-albums-amarok.png").unwrap();
    let shuffle_off_image_white_theme =             texture_creator.load_texture("ui/White-Theme/media-playlist-shuffle.png").unwrap();

    let muted_audio_image_white_theme =             texture_creator.load_texture("ui/Black-Theme/notification-audio-volume-muted.png").unwrap();
    let low_audio_image_white_theme =               texture_creator.load_texture("ui/White-Theme/notification-audio-volume-low.png").unwrap();
    let medium_audio_image_white_theme =            texture_creator.load_texture("ui/White-Theme/notification-audio-volume-medium.png").unwrap();
    let high_audio_image_white_theme =              texture_creator.load_texture("ui/White-Theme/notification-audio-volume-high.png").unwrap();
    let volume_icon_without_indicator_white_theme = texture_creator.load_texture("ui/White-Theme/notification-audio-volume-no-indicator.png").unwrap();

    let volume_text_rect = Rect::new(VOLUME_TEXT_POSITION[0], VOLUME_TEXT_POSITION[1], 25, 25);
        
    return (volume_text_rect,  volume_icon_without_indicator_white_theme, playing_image_white_theme, paused_image_white_theme, next_image_white_theme, previous_image_white_theme, shuffle_on_image_white_theme, shuffle_off_image_white_theme, muted_audio_image_white_theme, low_audio_image_white_theme, medium_audio_image_white_theme, high_audio_image_white_theme, volume_icon_without_indicator, playing_image, paused_image, next_image, previous_image, shuffle_on_image, shuffle_off_image, muted_audio_image, low_audio_image, medium_audio_image, high_audio_image);
}



//=====================================================================================//
//-----------------------------------THE VOLUME BAR DATA-------------------------------//
//=====================================================================================//
fn volume_icon() -> (Rect, Rect, Rect, bool, bool, bool, bool)
{
    let volume_is_low_indicator_rect = Rect::new(DEFAULT_VOLUME_LOW_BAR_ICON_INDICATOR_POSITION[0], DEFAULT_VOLUME_LOW_BAR_ICON_INDICATOR_POSITION[1], DEFAULT_VOLUME_LOW_BAR_ICON_INDICATOR_SIZE[0], DEFAULT_VOLUME_LOW_BAR_ICON_INDICATOR_SIZE[1]);
    let volume_is_medium_indicator_rect = Rect::new(DEFAULT_VOLUME_MEDIUM_BAR_ICON_INDICATOR_POSITION[0] + DEFAULT_VOLUME_BAR_PADDING, DEFAULT_VOLUME_MEDIUM_BAR_ICON_INDICATOR_POSITION[1], DEFAULT_VOLUME_MEDIUM_BAR_ICON_INDICATOR_SIZE[0], DEFAULT_VOLUME_MEDIUM_BAR_ICON_INDICATOR_SIZE[1]);
    let volume_is_high_indicator_rect = Rect::new(DEFAULT_VOLUME_HIGH_BAR_ICON_INDICATOR_POSITION[0] + DEFAULT_VOLUME_BAR_PADDING * 2, DEFAULT_VOLUME_HIGH_BAR_ICON_INDICATOR_POSITION[1], DEFAULT_VOLUME_HIGH_BAR_ICON_INDICATOR_SIZE[0], DEFAULT_VOLUME_HIGH_BAR_ICON_INDICATOR_SIZE[1]);

    let mut audio_is_muted = false;
    let mut audio_is_low = false;
    let mut audio_is_medium = false;
    let mut audio_is_high = false;

    let volume = volume_get();

    if volume == "0.0" { audio_is_muted = true; audio_is_low = false; audio_is_medium = false; audio_is_high = false;};
    if volume == "0.1" || volume == "0.2" || volume == "0.3" || volume == "0.4" { audio_is_low = true; audio_is_medium = false; audio_is_high = false; audio_is_muted = false; };
    if volume == "0.5" || volume == "0.6" || volume == "0.7" { audio_is_medium = true; audio_is_low = false; audio_is_high = false; audio_is_muted = false; };
    if volume == "0.8" || volume == "0.9" || volume == "1.0" { audio_is_high = true; audio_is_low = false; audio_is_medium = false; audio_is_muted = false; };

    return(volume_is_low_indicator_rect, volume_is_medium_indicator_rect, volume_is_high_indicator_rect, audio_is_muted, audio_is_low, audio_is_medium, audio_is_high);
}



fn volume_bar() -> (Rect, Rect)
{
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

    return (volume_level_bar, under_volume_bar);
}


fn convert_to_u32(string: &String) -> u32 {
        //convert to u32
        let raw_string = string.replace(":", "");
        let mut string = raw_string.replace("'", "");
        string.pop();
        if string.len() < 1 { string.push_str("0")};
        let u32_to_return: u32 = string.parse().unwrap();

        u32_to_return
}

//=====================================================================================//
//-------------------------------THE PROGRESS BAR DATA---------------------------------//
//=====================================================================================//
fn progress_bar() -> (Rect, Rect)
{
    let total_time = total_time_get();
    let current_time = current_time_get();

    let music_total_time_u32 = convert_to_u32(&total_time);
    let music_current_time_u32 = convert_to_u32(&current_time);

    let music_current_time_percentage = get_percentage(music_current_time_u32, music_total_time_u32);


    let progress_bar_background_rect = Rect::new(PROGRESS_BAR_BACKGROUND_POSITION[0], PROGRESS_BAR_BACKGROUND_POSITION[1], PROGRESS_BAR_SIZE[0], PROGRESS_BAR_SIZE[1]);
    let progress_rect = Rect::new(PROGRESS_BAR_POSITION[0], PROGRESS_BAR_POSITION[1], music_current_time_percentage as u32 * PROGRESS_BAR_SIZE[0] / 100, PROGRESS_BAR_SIZE[1]);


    return (progress_bar_background_rect, progress_rect);
}



//=====================================================================================//
//----------------------------------THE BACKGROUNDS DATA-------------------------------//
//=====================================================================================//
fn song_picture<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> (Texture<'a>, Texture<'a>, Texture<'a>, Rect)
{
    let (_, current_song_picture_path, picture_to_make_texture) = get_song_picture_data();
    download_image();
    remove_unused_song_picture(current_song_picture_path);
    let current_exe = get_exe_path();

    let mini_album_picture_rect = Rect::new(MINI_ALBUM_PICTURE_POSITION[0], MINI_ALBUM_PICTURE_POSITION[1], MINI_ALBUM_PICTURE_SIZE[0], MINI_ALBUM_PICTURE_SIZE[1]);

    let background = texture_creator
    .load_texture(&picture_to_make_texture)
    .unwrap();

    let mini_album_picture = texture_creator
    .load_texture(&picture_to_make_texture)
    .unwrap();

    let effect = texture_creator
    .load_texture(&format!("{}{}", current_exe,"/ui/system/effect.png"))
    .unwrap();

    return (background, mini_album_picture, effect, mini_album_picture_rect);
}



//=====================================================================================//
//------------------------------------THE BUTTONS DATA---------------------------------//
//=====================================================================================//
fn buttons() -> (Rect, Rect, Rect, Rect, Rect)
{
    let previous_rect = Rect::new(DEFAULT_BUTTON_MEDIA_POSITION[0] - DEFAULT_BUTTON_PADDING, DEFAULT_BUTTON_MEDIA_POSITION[1], DEFAULT_BUTTON_SIZE[0].try_into().unwrap(), DEFAULT_BUTTON_SIZE[1].try_into().unwrap());
    let pause_rect = Rect::new(DEFAULT_BUTTON_MEDIA_POSITION[0], DEFAULT_BUTTON_MEDIA_POSITION[1], DEFAULT_BUTTON_SIZE[0].try_into().unwrap(), DEFAULT_BUTTON_SIZE[1].try_into().unwrap());
    let next_rect = Rect::new(DEFAULT_BUTTON_MEDIA_POSITION[0] + DEFAULT_BUTTON_PADDING, DEFAULT_BUTTON_MEDIA_POSITION[1], DEFAULT_BUTTON_SIZE[0].try_into().unwrap(), DEFAULT_BUTTON_SIZE[1].try_into().unwrap());
    let shuffle_button_rect = Rect::new(SMALL_BUTTON_MEDIA_POSITION[0] - SMALL_BUTTON_PADDING, SMALL_BUTTON_MEDIA_POSITION[1], SMALL_BUTTON_SIZE[0].try_into().unwrap(), SMALL_BUTTON_SIZE[1].try_into().unwrap());
    let shuffle_indicator_rect = Rect::new(SHUFFLE_INDICATOR_POSITION[0] - SMALL_BUTTON_PADDING, SHUFFLE_INDICATOR_POSITION[1], SHUFFLE_INDICATOR_SIZE[0], SHUFFLE_INDICATOR_SIZE[1]);

    return (previous_rect, pause_rect, next_rect, shuffle_button_rect, shuffle_indicator_rect);
}



//=====================================================================================//
//-------------------------------THE FONTS DATA----------------------------------------//
//=====================================================================================//
fn fonts<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> (Vec<Texture<'a>>, Vec<Rect>) {
    let metadata_track = PlayerCtl::metadata();
    let remaining_time = remaining_time_get();
    let volume = volume_get();
    
    // metadata
    let (music_artist_text, music_artist_rect) = font_generator(" ", &texture_creator, SMALL_FONT_SIZE, metadata_track.artist, MUSIC_ARTIST_NAME_FONT_POSITION[0], MUSIC_ARTIST_NAME_FONT_POSITION[1]);
    let (music_name_text, music_name_rect) = font_generator(" ", &texture_creator, DEFAULT_FONT_SIZE, metadata_track.title, MUSIC_NAME_FONT_POSITION[0], MUSIC_NAME_FONT_POSITION[1]);
    let (music_album_text, music_album_rect) = font_generator(" ", &texture_creator, SMALL_FONT_SIZE, metadata_track.album, MUSIC_ALBUM_NAME_FONT_POSITION[0], MUSIC_ALBUM_NAME_FONT_POSITION[1]);
    // time
    let (time_remaining_text, time_remaining_rect) = font_generator(" ", &texture_creator, DEFAULT_FONT_SIZE, remaining_time, PROGRESS_BAR_TIME_REMAINING_POSITION[0], PROGRESS_BAR_TIME_REMAINING_POSITION[1]);
    // volume
    let (volume_integer_text, volume_integer_rect) = font_generator(" ", &texture_creator, DEFAULT_FONT_SIZE, volume, VOLUME_INTEGER_FONT_POSITION[0], VOLUME_INTEGER_FONT_POSITION[1]);

    let text_vector = vec![music_artist_text, music_name_text, music_album_text, time_remaining_text, volume_integer_text];
    let rect_vector = vec![music_artist_rect, music_name_rect, music_album_rect, time_remaining_rect, volume_integer_rect]; 
    
    return (text_vector, rect_vector);
}





//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------THE SYSTEM'S THAT USES THE DATA---------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//

//=====================================================================================//
//----------------------------------THE MAIN SYSTEM------------------------------------//
//=====================================================================================//
fn system()
{
            let (mut canvas, texture_creator, sdl_started) = window();
            let (previous_rect, pause_rect, next_rect, shuffle_button_rect, shuffle_indicator_rect) = buttons();
            let (volume_rect, volume_icon_without_indicator_white_theme, playing_image_white_theme, paused_image_white_theme, next_image_white_theme, previous_image_white_theme, shuffle_on_image_white_theme, shuffle_off_image_white_theme, muted_audio_image_white_theme, low_audio_image_white_theme, medium_audio_image_white_theme, high_audio_image_white_theme, volume_icon_without_indicator, playing_image, paused_image, next_image, previous_image, shuffle_on_image, shuffle_off_image, muted_audio_image, low_audio_image, medium_audio_image, high_audio_image) = basic_ui(&texture_creator);


//===========================================THE LOOP START================================================
let mut event_pump = sdl_started.event_pump().unwrap(); 'running: loop { std::thread::sleep(Duration::from_millis(32)); 
            

            let (volume_is_low_indicator_rect, volume_is_medium_indicator_rect, volume_is_high_indicator_rect, audio_is_muted, audio_is_low, audio_is_medium, audio_is_high) = volume_icon();
            let (volume_level_bar, under_volume_bar) = volume_bar();
            //progress bar
            let (music_progress_bar_background_rect, music_progress_bar_rect) = progress_bar();
            //background and album picture
            let (background, album_picture, effect, album_picture_rect) = song_picture(&texture_creator);
            //fonts
            let (text_vector_fonts, rect_vector_fonts) = fonts(&texture_creator);
            //render
            render_scene(text_vector_fonts, rect_vector_fonts, &volume_icon_without_indicator_white_theme, &playing_image_white_theme, &paused_image_white_theme, &next_image_white_theme, &previous_image_white_theme, &shuffle_on_image_white_theme, &shuffle_off_image_white_theme, &muted_audio_image_white_theme, &low_audio_image_white_theme, &medium_audio_image_white_theme, &high_audio_image_white_theme, &volume_icon_without_indicator, volume_is_low_indicator_rect, volume_is_medium_indicator_rect, volume_is_high_indicator_rect, shuffle_indicator_rect, music_progress_bar_background_rect, music_progress_bar_rect, volume_rect, &playing_image, &paused_image, &next_image, &previous_image, &shuffle_on_image, &shuffle_off_image, &muted_audio_image, &low_audio_image, &medium_audio_image, &high_audio_image, previous_rect, pause_rect, next_rect, shuffle_button_rect, audio_is_muted, audio_is_low, audio_is_medium, audio_is_high, volume_level_bar, under_volume_bar, background, album_picture, album_picture_rect, effect, &mut canvas);


//===============================================================================================================//
//------------------------------------------------MEDIA MOUSE CHECKER--------------------------------------------//
//===============================================================================================================//
        for event in event_pump.poll_iter()
        {
            match event 
            {
                Event::MouseButtonDown {mouse_btn: MouseButton::Left, x, y, .. } => 
                {
//========================================MEDIA KEYCHECKER (MOUSE)===============================================
                    if x >= pause_rect.x && x <= pause_rect.x + DEFAULT_BUTTON_SIZE[0] && y >= pause_rect.y && y <= pause_rect.y + DEFAULT_BUTTON_SIZE[1] as i32
                    {
                        PlayerCtl::play_pause();
                    }

                    if x >= next_rect.x && x <= next_rect.x + DEFAULT_BUTTON_SIZE[0] && y >= next_rect.y && y <= next_rect.y + DEFAULT_BUTTON_SIZE[1] as i32
                    {
                        PlayerCtl::next();
                    }

                    if x >= previous_rect.x && x <= previous_rect.x + DEFAULT_BUTTON_SIZE[0] && y >= previous_rect.y && y <= previous_rect.y + DEFAULT_BUTTON_SIZE[1] as i32
                    {
                        shuffle_toggle_set();
                    }

//===========================================SHUFFLE KEYCHECKER (MOUSE)============================================
                    if x >= shuffle_button_rect.x && x <= shuffle_button_rect.x + SMALL_BUTTON_SIZE[1] && y >= shuffle_button_rect.y && y <= shuffle_button_rect.y + SMALL_BUTTON_SIZE[1] as i32
                    {
                        shuffle_toggle_set();
                    }
                }



//===============================================================================================================//
//------------------------------------------MEDIA KEYCHECKER (KEYBOARD)------------------------------------------//
//===============================================================================================================//
                Event::KeyDown { keycode: Some(Keycode::Space), .. } | Event::KeyDown { keycode: Some(Keycode::Return), .. } =>
                {
                    PlayerCtl::play_pause();
                }        
                
                Event::KeyDown { keycode: Some(Keycode::L), .. } | Event::KeyDown { keycode: Some(Keycode::Right), .. } =>
                {
                    PlayerCtl::next();
                }

                Event::KeyDown { keycode: Some(Keycode::H), .. } | Event::KeyDown { keycode: Some(Keycode::Left), .. } =>
                {
                    PlayerCtl::previous();
                }        

//========================================SHUFFLE KEYCHECKER (KEYBOARD)============================================
                Event::KeyDown { keycode: Some(Keycode::I), .. } =>
                {
                        shuffle_toggle_set();
                }

//============================================AUDIO KEYCHECKER (KEYBOARD)==========================================
                Event::KeyDown { keycode: Some(Keycode::K), .. } | Event::KeyDown { keycode: Some(Keycode::Up), .. } | Event::KeyDown { keycode: Some(Keycode::Plus), .. } => 
                {   
                    volume_up(AUDIO_STEP);
                }     

                Event::KeyDown { keycode: Some(Keycode::J), .. } | Event::KeyDown { keycode: Some(Keycode::Down), .. } | Event::KeyDown { keycode: Some(Keycode::Minus), .. } => 
                {
                    volume_down(AUDIO_STEP);
                }

//=======================================QUIT HANDLER (KEYBOARD AND SYSTEM EXIT CALL)==============================
                sdl2::event::Event::Quit { .. } |
                sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => { break 'running; }
                _=> {}
            }
        }
    }
}










//=====================================================================================//
//-------------------------------------------------------------------------------------//
//-------------------------------------STARTED FUNCTION--------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
fn main() 
{
    system();
}
