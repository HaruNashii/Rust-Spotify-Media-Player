//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------------CRATES----------------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
use std::io::Read;
use sdl2::video::WindowContext;
use std::time::{Duration, Instant};
use std::process::{Command, Stdio};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::image::LoadTexture;
use std::path::Path;
use std::fs;










//=====================================================================================//
//-------------------------------------------------------------------------------------//
//-----------------------------------MAIN DATAS----------------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//

///!====================================//
///!===============(WINDOW)=============//
///!====================================//
///! DISCLAIMER: DON'T CHANGE IT, IT WILL BROKE ALL THE ELEMENTS POSITION
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;



//====================================//
//========(ELEMENTS POSITION)=========//
//====================================//
// the background album image have the position based on the screen size (that's why it's not here)

// volume bar
const VOLUME_TEXT_POSITION: [i32;2] = [525, 535];
const VOLUME_BAR_BACKGROUND_POSITION: [i32;2] = [560, 543];
const VOLUME_LEVEL_BAR_POSITION: [i32;2] = [560, 543];

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

// fonts
const VOLUME_INTEGER_FONT_POSITION: [i32;2] = [600, 532];
const MUSIC_NAME_FONT_POSITION: [i32;2] = [250, 240];
const MUSIC_ARTIST_NAME_FONT_POSITION: [i32;2] = [255, 275];
const MUSIC_ALBUM_NAME_FONT_POSITION: [i32;2] = [255, 300];



//====================================//
//=========(ELEMENTS SCALES)==========//
//====================================//
// the background album image have the scale based on the screen size (that's why it's not here)

// fonts
const DEFAULT_FONT_SIZE: u16 = 20;
const SMALL_FONT_SIZE: u16 = 13;

// buttons 
const DEFAULT_BUTTON_SIZE: [i32;2] = [50, 50];
const SMALL_BUTTON_SIZE: [i32;2] = [35, 35];

// volume bar
const DEFAULT_VOLUME_BAR_SIZE: [u32;2] = [0, 7];
const DEFAULT_VOLUME_BAR_BACKGROUND_SIZE: [u32;2] = [35, 7];

// mini album picture
const MINI_ALBUM_PICTURE_SIZE: [u32;2] = [200, 200];

// progress bar 
const PROGRESS_BAR_SIZE: [u32;2] = [675, 20];


//====================================//
//========(ELEMENTS ARGUMENTS)========//
//====================================//
// fonts
const DEFAULT_FONT_PATH: &str = "fonts/JetBrainsMonoNLNerdFontMono-Bold.ttf";
const DEFAULT_FONT_COLOR: sdl2::pixels::Color = Color::RGB(255, 255, 255);

// buttons
const DEFAULT_BUTTON_COLOR: sdl2::pixels::Color = Color::RGB(0, 153, 107);

// progress bar 
const DEFAULT_PROGRESS_BAR_COLOR: sdl2::pixels::Color = Color::RGB(0, 153, 107);
const DEFAULT_BACKGROUND_PROGRESS_BAR_COLOR: sdl2::pixels::Color = Color::RGB(150, 150, 150);

// volume bar 
const DEFAULT_VOLUME_BAR_COLOR: sdl2::pixels::Color = Color::RGB(0, 153, 107);
const DEFAULT_BACKGROUND_VOLUME_BAR_COLOR: sdl2::pixels::Color = Color::RGB(150, 150, 150);










//=====================================================================================//
//-------------------------------------------------------------------------------------//
//-----------------------THE GENERATORS USED BY THE TOOLS FUNCTIONS--------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//

//=====================================================================================//
//-----------------------------------USED BY FONTS()-----------------------------------//
//=====================================================================================//
fn font_generator<'a>(additional_text: &str, texture_creator: &'a TextureCreator<sdl2::video::WindowContext>, size: u16, text: String, x: i32, y: i32) -> (Texture<'a>, Rect)
{
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font(DEFAULT_FONT_PATH, size).unwrap();
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
//-----------------------------------USED BY COMMANDS()--------------------------------//
//=====================================================================================//
fn gen_command(shell: &str, arg_one: &str, arg_two: &str) -> Command
{
        let mut command = Command::new(shell);
        command.arg(arg_one);
        command.arg(arg_two);

        return command;
}

fn gen_command_with_output(shell: &str, arg_one: &str, arg_two: &str) -> Command 
{
        let mut command_with_output = Command::new(shell);
        command_with_output.arg(arg_one);
        command_with_output.arg(arg_two)
        .stdout(Stdio::piped());

        return command_with_output;
}



//=====================================================================================//
//-----------------------------------USED BY BACKGROUND()------------------------------//
//=====================================================================================//
fn gen_image<'a>(path: &'a str, texture_creator: &'a TextureCreator<WindowContext>) -> Texture<'a>
{
        let image = texture_creator
        .load_texture(path)
        .unwrap();
    
        return image;
}










//=====================================================================================//
//-------------------------------------------------------------------------------------//
//-----------THE TOOLS FUNCTIONS THAT IS USED BY THE DATA CREATORS FUNCTIONS-----------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//

//=====================================================================================//
//-----------------------------------VOLUME BAR----------------------------------------//
//=====================================================================================//
fn get_volume_value_in_string(volume_info: &mut Command) -> String
{
    let mut volume_command_string = String::new();
    volume_command_string.clear();
    volume_info.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut volume_command_string).unwrap();
    volume_command_string.pop();
    while volume_command_string.len() >= 4 { volume_command_string.pop(); }

    return volume_command_string;
}



//=====================================================================================//
//---------------------------------PROGRESS BAR----------------------------------------//
//=====================================================================================//
fn extract_or_update_string_of_command(command: &mut Command) -> String
{
        let mut string = String::new();
        string.clear(); 
        command.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut string).unwrap();
        string.pop();
    
        return string;
}

fn extract_string_of_command_and_convert_to_u32(command: &mut Command) -> u32
{
        //extract from command
        let mut string = String::new();    
        string.clear();
        command.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut string).unwrap();
        
        //convert to u32
        let mut string = string.replace(":", "");
        string.pop();
        if string.len() < 1 { string.push_str("0")};
        let u32_to_return: u32 = string.parse().unwrap();

        return u32_to_return;
}

fn get_percentage(x: u32, y: u32) -> f32
{
    let result: f32 = x as f32 / y as f32 * 100.0;

    return result;
}



//-------------------------------------------------------------------------------------//
//----------------------------------BACKGROUND-----------------------------------------//
//-------------------------------------------------------------------------------------//
fn download_image(get_background_link: &mut Command)
{
    let folder_path = String::from(".background/");

    //command
    let mut background_link = String::new();
    get_background_link.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut background_link).unwrap();
    let raw_string_command_to_download_the_file = format!("{} {} {} {}", "wget --quiet -P", "$PWD/.background/", background_link, "&& exit");
    let string_command_to_download_the_file = &raw_string_command_to_download_the_file.replace("\n", "");
    let mut download_image_command = gen_command("bash", "-c", &string_command_to_download_the_file);


    let raw_background_link_without_https_and_format = background_link.replace("https://i.scdn.co/image/", "");
    let background_link_without_https_and_format = raw_background_link_without_https_and_format.replace("\n", "");

    let raw_background_link_without_https_with_format = format!("{}{}", background_link_without_https_and_format,".png");
    let background_link_without_https_with_format = raw_background_link_without_https_with_format.replace("\n", "");


    let raw_image_path_without_format = format!("{}{}", folder_path, background_link_without_https_and_format);
    let image_path_without_format = raw_image_path_without_format.replace("\n", "");
    
    let raw_image_path_with_format = format!("{}{}", folder_path, background_link_without_https_with_format);
    let image_path_with_format = raw_image_path_with_format.replace("\n", "");


    let folder_to_check_with_the_file = Path::new(&folder_path).join(background_link_without_https_and_format.clone());
    let folder_to_check_with_the_file_2 = Path::new(&folder_path).join(background_link_without_https_with_format.clone());
    if folder_to_check_with_the_file.exists() 
    {
           fs::rename(image_path_without_format, image_path_with_format.clone()).unwrap();
           
           let items = fs::read_dir(&folder_path).unwrap();
           for entry in items 
           {
               let entry = entry.unwrap();
               let entry_path = entry.path();
               
               if entry_path.file_name() == Some(std::ffi::OsStr::new(&background_link_without_https_with_format)) 
               {
                   continue; 
               }

               if entry_path.is_file() 
               {
                   fs::remove_file(&entry_path).unwrap();
               } 
           }

    }
    else 
    {
            if !folder_to_check_with_the_file_2.exists() 
            {
                download_image_command.spawn().unwrap();
            }
    }
}



fn get_background_info(get_background_link: &mut Command) -> String
{
    let mut string = String::new();
    if string.len() > 1 { string.clear(); };
    get_background_link.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut string).unwrap();   
    
    if string.len() >= 24 {  string = (&string[24..]).to_string(); };

    let str: &str = &string;
    let mut image_name: String = str.replace("\n", "");
    let image_name_str: &str = &image_name;

    let path_to_check = format!("{}{}{}", ".background/", image_name_str, ".png");
    if Path::new(&path_to_check).exists() 
    {
        image_name = path_to_check.clone();
    }
    else 
    {
        image_name.clear();
        image_name.push_str(".background/system/holder.png");
    };
     
    return image_name;
}










//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------THE DATA CREATOR FUNCTIONS--------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//

//=====================================================================================//
//--------------------------------THE SONG DATA COLLECTOR------------------------------//
//=====================================================================================//
fn check_music_name(music_name_info: &mut Command) -> String 
{
    let mut music_name_string = String::new();
    music_name_string.clear();
    music_name_info.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut music_name_string).unwrap();
    music_name_string.pop();
    
    return music_name_string;
}

fn check_music_artist(music_artist_info: &mut Command) -> String 
{
    let mut music_artist_string = String::new();
    music_artist_string.clear();
    music_artist_info.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut music_artist_string).unwrap();
    music_artist_string.pop();

    return music_artist_string;
}

fn check_music_album(music_album_info: &mut Command) -> String 
{
    let mut music_album_string = String::new();
    music_album_string.clear();
    music_album_info.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut music_album_string).unwrap();
    music_album_string.pop();

    return music_album_string;
}

fn check_status(status_info: &mut Command) -> bool
{
    let mut status_info_string = String::new();
    status_info_string.clear();
    status_info.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut status_info_string).unwrap();
    status_info_string.pop();

    let mut status_bool = false;
    if status_info_string == String::from("Playing") { status_bool = true; };
    if status_info_string == String::from("Paused") { status_bool = false; };

    return status_bool;
}

fn check_shuffle(shuffle_info: &mut Command) -> bool
{
    let mut shuffle_string = String::new();
    shuffle_string.clear();
    shuffle_info.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut shuffle_string).unwrap();
    shuffle_string.pop();
    
    let mut shuffle_bool = false;
    if shuffle_string == String::from("On") { shuffle_bool = true; };
    if shuffle_string == String::from("Off") { shuffle_bool = false; };
    
    return shuffle_bool;
}



//=====================================================================================//
//---------------------------------------THE UI DATA-----------------------------------//
//=====================================================================================//
fn basic_ui<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> (Rect, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>,  Texture<'a>)
{
    //all this ui images localitions is using the buttons rect so to change the position of the image you will need    to change the position of the buttons
    let playing_image = gen_image("ui/media-playback-start.png", texture_creator);
    let paused_image = gen_image("ui/media-playback-pause.png", texture_creator);
    let next_image = gen_image("ui/media-skip-forward.png", texture_creator);
    let previous_image = gen_image("ui/media-skip-backward.png", texture_creator);
    let shuffle_on_image = gen_image("ui/media-random-albums-amarok.png", texture_creator);
    let shuffle_off_image = gen_image("ui/media-playlist-shuffle.png", texture_creator);

    let muted_audio_image = gen_image("ui/notification-audio-volume-muted.png", texture_creator);
    let low_audio_image = gen_image("ui/notification-audio-volume-low.png", texture_creator);
    let medium_audio_image = gen_image("ui/notification-audio-volume-medium.png", texture_creator);
    let high_audio_image = gen_image("ui/notification-audio-volume-high.png", texture_creator);

    let volume_text_rect = Rect::new(VOLUME_TEXT_POSITION[0], VOLUME_TEXT_POSITION[1], 25, 25);
        
    return (volume_text_rect, playing_image, paused_image, next_image, previous_image, shuffle_on_image, shuffle_off_image, muted_audio_image, low_audio_image, medium_audio_image, high_audio_image);
}



//=====================================================================================//
//-----------------------------------THE VOLUME BAR DATA-------------------------------//
//=====================================================================================//
fn volume_icon(mut volume_command_string: String) -> (bool, bool, bool, bool)
{
    let mut audio_is_muted = false;
    let mut audio_is_low = false;
    let mut audio_is_medium = false;
    let mut audio_is_high = false;

    while volume_command_string.len() >= 4 { volume_command_string.pop(); }
    if volume_command_string == "0.0" { audio_is_muted = true; audio_is_low = false; audio_is_medium = false; audio_is_high = false;};
    if volume_command_string == "0.1" || volume_command_string == "0.2" || volume_command_string == "0.3" || volume_command_string == "0.4" { audio_is_low = true; audio_is_medium = false; audio_is_high = false; audio_is_muted = false; };
    if volume_command_string == "0.5" || volume_command_string == "0.6" || volume_command_string == "0.7" { audio_is_medium = true; audio_is_low = false; audio_is_high = false; audio_is_muted = false; };
    if volume_command_string == "0.8" || volume_command_string == "0.9" || volume_command_string == "1.0" { audio_is_high = true; audio_is_low = false; audio_is_medium = false; audio_is_muted = false; };

    return(audio_is_muted, audio_is_low, audio_is_medium, audio_is_high);
}



fn volume_bar(volume_command_string: String) -> (Rect, Rect)
{
    let mut volume_level_bar = Rect::new(VOLUME_LEVEL_BAR_POSITION[0], VOLUME_LEVEL_BAR_POSITION[1], DEFAULT_VOLUME_BAR_SIZE[0], DEFAULT_VOLUME_BAR_SIZE[1]);
    let under_volume_bar = Rect::new(VOLUME_BAR_BACKGROUND_POSITION[0], VOLUME_BAR_BACKGROUND_POSITION[1], DEFAULT_VOLUME_BAR_BACKGROUND_SIZE[0], DEFAULT_VOLUME_BAR_BACKGROUND_SIZE[1]);

    if volume_command_string == "0.0" { volume_level_bar.w = 0; };
    if volume_command_string == "0.1" { volume_level_bar.w = 3; };
    if volume_command_string == "0.2" { volume_level_bar.w = 5; }; 
    if volume_command_string == "0.3" { volume_level_bar.w = 7; };
    if volume_command_string == "0.4" { volume_level_bar.w = 10; };
    if volume_command_string == "0.5" { volume_level_bar.w = 12; };
    if volume_command_string == "0.6" { volume_level_bar.w = 17; };
    if volume_command_string == "0.7" { volume_level_bar.w = 22; };
    if volume_command_string == "0.8" { volume_level_bar.w = 28; };
    if volume_command_string == "0.9" { volume_level_bar.w = 30; };
    if volume_command_string == "1.0" { volume_level_bar.w = 35; };

    return (volume_level_bar, under_volume_bar);
}



//=====================================================================================//
//-------------------------------THE PROGRESS BAR DATA---------------------------------//
//=====================================================================================//
fn progress_bar() -> (Rect, Rect, String)
{
    let (_, _, _, _, _, _, _, _, _, _, _, mut get_time, mut get_current_time, mut get_time_remaining, _, _, _,) = commands();

    let music_total_time_u32 = extract_string_of_command_and_convert_to_u32(&mut get_time);
    let music_current_time_u32 = extract_string_of_command_and_convert_to_u32(&mut get_current_time);
    let music_current_time_percentage = get_percentage(music_current_time_u32, music_total_time_u32);
    let music_time_remaining_string = extract_or_update_string_of_command(&mut get_time_remaining);

    let progress_bar_background_rect = Rect::new(PROGRESS_BAR_BACKGROUND_POSITION[0], PROGRESS_BAR_BACKGROUND_POSITION[1], PROGRESS_BAR_SIZE[0], PROGRESS_BAR_SIZE[1]);
    let progress_rect = Rect::new(PROGRESS_BAR_POSITION[0], PROGRESS_BAR_POSITION[1], music_current_time_percentage as u32 * PROGRESS_BAR_SIZE[0] / 100, PROGRESS_BAR_SIZE[1]);


    return (progress_bar_background_rect, progress_rect, music_time_remaining_string);
}



//=====================================================================================//
//----------------------------------THE BACKGROUNDS DATA-------------------------------//
//=====================================================================================//
fn song_picture<'a>(texture_creator: &'a TextureCreator<WindowContext>, get_background_link: &mut Command) -> (Texture<'a>, Texture<'a>, Texture<'a>, Rect)
{
    let image_name = get_background_info(get_background_link);
    download_image(get_background_link);

    let album_picture_rect = Rect::new(MINI_ALBUM_PICTURE_POSITION[0], MINI_ALBUM_PICTURE_POSITION[1], MINI_ALBUM_PICTURE_SIZE[0], MINI_ALBUM_PICTURE_SIZE[1]);

    let background = texture_creator
    .load_texture(&image_name)
    .unwrap();

    let album_picture = texture_creator
    .load_texture(&image_name)
    .unwrap();

    let effect = texture_creator
    .load_texture(".background/system/effect.png")
    .unwrap();

    return (background, album_picture, effect, album_picture_rect);
}



//=====================================================================================//
//------------------------------------THE BUTTONS DATA---------------------------------//
//=====================================================================================//
fn buttons() -> (Rect, Rect, Rect, Rect)
{
    let shuffle_button_rect = Rect::new(SMALL_BUTTON_MEDIA_POSITION[0] - SMALL_BUTTON_PADDING, SMALL_BUTTON_MEDIA_POSITION[1], SMALL_BUTTON_SIZE[0].try_into().unwrap(), SMALL_BUTTON_SIZE[1].try_into().unwrap());
    let previous_rect = Rect::new(DEFAULT_BUTTON_MEDIA_POSITION[0] - DEFAULT_BUTTON_PADDING, DEFAULT_BUTTON_MEDIA_POSITION[1], DEFAULT_BUTTON_SIZE[0].try_into().unwrap(), DEFAULT_BUTTON_SIZE[1].try_into().unwrap());
    let pause_rect = Rect::new(DEFAULT_BUTTON_MEDIA_POSITION[0], DEFAULT_BUTTON_MEDIA_POSITION[1], DEFAULT_BUTTON_SIZE[0].try_into().unwrap(), DEFAULT_BUTTON_SIZE[1].try_into().unwrap());
    let next_rect = Rect::new(DEFAULT_BUTTON_MEDIA_POSITION[0] + DEFAULT_BUTTON_PADDING, DEFAULT_BUTTON_MEDIA_POSITION[1], DEFAULT_BUTTON_SIZE[0].try_into().unwrap(), DEFAULT_BUTTON_SIZE[1].try_into().unwrap());

    return (previous_rect, pause_rect, next_rect, shuffle_button_rect);
}



//=====================================================================================//
//-------------------------------THE FONTS DATA----------------------------------------//
//=====================================================================================//
fn fonts<'a>(volume_command_string: String, music_time_remaining_string: String, music_artist_string: String, music_album_string: String, texture_creator: &'a TextureCreator<WindowContext>, music_name_string: String) -> (Texture<'a>, Rect, Texture<'a>, Rect, Texture<'a>, Rect, Texture<'a>, Rect, Texture<'a>, Rect)
{
let (music_artist_text, music_artist_rect) = font_generator(" ", &texture_creator, SMALL_FONT_SIZE, music_artist_string, MUSIC_ARTIST_NAME_FONT_POSITION[0], MUSIC_ARTIST_NAME_FONT_POSITION[1]);
let (music_album_text, music_album_rect) = font_generator(" ", &texture_creator, SMALL_FONT_SIZE, music_album_string, MUSIC_ALBUM_NAME_FONT_POSITION[0], MUSIC_ALBUM_NAME_FONT_POSITION[1]);
let (time_remaining_text, time_remaining_rect) = font_generator(" ", &texture_creator, DEFAULT_FONT_SIZE, music_time_remaining_string, PROGRESS_BAR_TIME_REMAINING_POSITION[0], PROGRESS_BAR_TIME_REMAINING_POSITION[1]);
let (volume_integer_text, volume_integer_rect) = font_generator(" ", &texture_creator, DEFAULT_FONT_SIZE, volume_command_string, VOLUME_INTEGER_FONT_POSITION[0], VOLUME_INTEGER_FONT_POSITION[1]);
let (music_name_text, music_name_rect) = font_generator(" ", &texture_creator, DEFAULT_FONT_SIZE, music_name_string, MUSIC_NAME_FONT_POSITION[0], MUSIC_NAME_FONT_POSITION[1]);

return (music_artist_text, music_artist_rect, music_album_text, music_album_rect, time_remaining_text, time_remaining_rect, volume_integer_text, volume_integer_rect, music_name_text, music_name_rect);
}



//=====================================================================================//
//-----------------------------THE COMMANDS DATA---------------------------------------//
//=====================================================================================//
fn commands() -> (Command, Command, Command, Command, Command, Command, Command, Command, Command, Command, Command, Command, Command, Command, Command, Command, Command)
{
    let default_shell: &str = "bash";
    let default_argument: &str = "-c"; 

    let volume_info = gen_command_with_output(default_shell, default_argument, "playerctl --player=spotify -s volume && exit");
    let status_info = gen_command_with_output(default_shell, default_argument, "playerctl --player=spotify -s status && exit");
    let music_name_info = gen_command_with_output(default_shell, default_argument, "playerctl --player=spotify -s metadata xesam:title && exit");
    let shuffle_info = gen_command_with_output(default_shell, default_argument, "playerctl --player=spotify -s shuffle && exit");
    let shuffle_on = gen_command(default_shell, default_argument, "playerctl --player=spotify -s shuffle on && exit"); 
    let shuffle_off = gen_command(default_shell, default_argument, "playerctl --player=spotify -s shuffle off && exit"); 
    let next = gen_command(default_shell, default_argument, "playerctl --player=spotify -s next && exit"); 
    let previous = gen_command(default_shell, default_argument, "playerctl --player=spotify -s previous && exit"); 
    let pause_play = gen_command(default_shell, default_argument, "playerctl --player=spotify -s play-pause && exit"); 
    let volume_up = gen_command(default_shell, default_argument, "playerctl --player=spotify -s volume 0.1+ && exit"); 
    let volume_down = gen_command(default_shell, default_argument, "playerctl --player=spotify -s volume 0.1- && exit"); 
    let get_time = gen_command_with_output(default_shell, default_argument, "playerctl --player=spotify -s metadata --format '{{ duration(mpris:length) }}' && exit");
    let get_current_time = gen_command_with_output(default_shell, default_argument, "playerctl --player=spotify -s position --format '{{ duration(position)}}' && exit");
    let get_time_remaining = gen_command_with_output(default_shell, default_argument, "playerctl --player=spotify -s metadata --format '{{ duration(mpris:length - position) }}' && exit");
    let music_artist_info = gen_command_with_output(default_shell, default_argument, "playerctl --player=spotify -s metadata --format '{{ artist }}' && exit");
    let music_album_info = gen_command_with_output(default_shell, default_argument, "playerctl --player=spotify -s metadata --format '{{ album }}' && exit");
    let get_background_link = gen_command_with_output(default_shell, default_argument, "playerctl --player=spotify -s metadata mpris:artUrl && exit");

    return(
            volume_info,
            status_info,
            music_name_info,
            shuffle_info,
            shuffle_on,
            shuffle_off,
            next,
            previous, 
            pause_play,
            volume_up,
            volume_down,
            get_time,
            get_current_time,
            get_time_remaining,
            music_artist_info,
            music_album_info, 
            get_background_link,
            );
}










//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------THE SYSTEM'S THAT USES THE DATA---------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//

//=====================================================================================//
//---------------------------THE WINDOW DATA AND INICIALIZATOR-------------------------//
//=====================================================================================//
fn window() -> (Canvas<Window>, TextureCreator<WindowContext>, sdl2::Sdl)
{       
    let sdl_started = sdl2::init().unwrap(); 
    let video_system = sdl_started.video().unwrap();
    let window = video_system
    .window("Media", WINDOW_WIDTH, WINDOW_HEIGHT)
    .position_centered()
    .build()
    .map_err(|e| e.to_string())
    .unwrap();
    
    let mut canvas = window.into_canvas()
    .accelerated()
    .build()
    .map_err(|e| e.to_string())
    .unwrap();
    canvas.set_logical_size(WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();

    let texture_creator = canvas.texture_creator();

    return(canvas, texture_creator, sdl_started);
}



//=====================================================================================//
//---------------------------------THE WINDOW RENDER-----------------------------------//
//=====================================================================================//
fn render_scene(music_progress_bar_background: Rect, music_progress_bar_rect: Rect, volume_rect: Rect, playing_image: &Texture, paused_image: &Texture, next_image: &Texture, previous_image: &Texture, shuffle_on_image: &Texture, shuffle_off_image: &Texture, muted_audio_image: &Texture, low_audio_image: &Texture, medium_audio_image: &Texture, high_audio_image: &Texture, previous_rect: Rect, pause_rect: Rect, next_rect: Rect, shuffle_button_rect: Rect, audio_is_muted: bool, audio_is_low: bool, audio_is_medium: bool, audio_is_high: bool, volume_level_bar: Rect, under_volume_bar: Rect, background: Texture, album_picture: Texture, effect: Texture, album_picture_rect: Rect, music_artist_text: Texture, music_artist_rect: Rect, music_album_text: Texture, music_album_rect: Rect, time_remaining_text: Texture, time_remaining_rect: Rect, volume_integer_text: Texture, volume_integer_rect: Rect, music_name_text: Texture, music_name_rect: Rect, status_bool: bool, shuffle_bool: bool, canvas: &mut Canvas<Window>)
{
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
    canvas.copy(&time_remaining_text, None, time_remaining_rect).unwrap();

    // media buttons
    canvas.copy(next_image, None, next_rect).unwrap();
    canvas.copy(previous_image, None, previous_rect).unwrap();
    if audio_is_muted { canvas.copy(muted_audio_image, None, volume_rect).unwrap(); }
    if audio_is_low { canvas.copy(low_audio_image, None, volume_rect).unwrap(); }
    if audio_is_medium { canvas.copy(medium_audio_image, None, volume_rect).unwrap(); }
    if audio_is_high { canvas.copy(high_audio_image, None, volume_rect).unwrap(); }
    if shuffle_bool { canvas.copy(shuffle_on_image, None, shuffle_button_rect).unwrap(); }
    if status_bool { canvas.copy(paused_image, None, pause_rect).unwrap(); }
    if !shuffle_bool { canvas.copy(shuffle_off_image, None, shuffle_button_rect).unwrap(); }
    if !status_bool { canvas.copy(playing_image, None, pause_rect).unwrap(); }

    // fonts
    canvas.copy(&music_name_text, None, music_name_rect).unwrap();
    canvas.copy(&music_artist_text, None, music_artist_rect).unwrap();
    canvas.copy(&music_album_text, None, music_album_rect).unwrap();
    canvas.copy(&volume_integer_text, None, volume_integer_rect).unwrap();

    canvas.present();
}



//=====================================================================================//
//----------------------------------THE MAIN SYSTEM------------------------------------//
//=====================================================================================//
fn system()
{
//===================================TIMER SETUP FOR THE KEYCHECKER=======================================
    let mut keychecker_timer_activator = true;
    let keychecker_timer_duration_1 = Duration::from_millis(2);
    let keychecker_timer_duration_2 = Duration::from_millis(3);
    let mut keychecker_timer_1 = Instant::now();
    let mut keychecker_timer_2 = Instant::now();



//=============================================================================================//
//-----------------------------------THE FUNCTIONS EXECUTOR------------------------------------//
//=============================================================================================//
            //window
            let (mut canvas, texture_creator, sdl_started) = window();
            //commands
            let (mut volume_info, mut status_info, mut music_name_info, mut shuffle_info, mut shuffle_on, mut shuffle_off, mut next, mut previous, mut pause_play, mut volume_up, mut volume_down, _, _, _, mut music_artist_info, mut music_album_info, mut get_background_link) = commands();
            //buttons
            let (previous_rect, pause_rect, next_rect, shuffle_button_rect) = buttons();
            //basic ui
            let (volume_rect, playing_image, paused_image, next_image, previous_image, shuffle_on_image, shuffle_off_image, muted_audio_image, low_audio_image, medium_audio_image, high_audio_image) = basic_ui(&texture_creator);

//===========================================THE LOOP START================================================
let mut event_pump = sdl_started.event_pump().unwrap(); 'running: loop { std::thread::sleep(Duration::from_millis(32)); 
            
            //get music name
            let music_name_string = check_music_name(&mut music_name_info);
            //get music artist
            let music_artist_string = check_music_artist(&mut music_artist_info);
            //get music album
            let music_album_string = check_music_album(&mut music_album_info);
            //get music status
            let status_bool = check_status(&mut status_info);
            //get reproduction list shuffle status
            let shuffle_bool = check_shuffle(&mut shuffle_info);
            //volume bar body, progress and icons
            let volume_command_string = get_volume_value_in_string(&mut volume_info);
            let (audio_is_muted, audio_is_low, audio_is_medium, audio_is_high) = volume_icon(volume_command_string.clone());
            let (volume_level_bar, under_volume_bar) = volume_bar(volume_command_string.clone());
            //progress bar
            let (music_progress_bar_background_rect, music_progress_bar_rect, music_time_remaining_string) = progress_bar();
            //background and album picture
            let (background, album_picture, effect, album_picture_rect) = song_picture(&texture_creator, &mut get_background_link);
            //fonts
            let (music_artist_text, music_artist_rect, music_album_text, music_album_rect, time_remaining_text, time_remaining_rect, volume_integer_text, volume_integer_rect, music_name_text, music_name_rect) = fonts(volume_command_string.clone(), music_time_remaining_string, music_artist_string.clone(), music_album_string.clone(), &texture_creator, music_name_string.clone(),);
            //render
            render_scene(music_progress_bar_background_rect, music_progress_bar_rect, volume_rect, &playing_image, &paused_image, &next_image, &previous_image, &shuffle_on_image, &shuffle_off_image, &muted_audio_image, &low_audio_image, &medium_audio_image, &high_audio_image, previous_rect, pause_rect, next_rect, shuffle_button_rect, audio_is_muted, audio_is_low, audio_is_medium, audio_is_high, volume_level_bar, under_volume_bar, background, album_picture, effect, album_picture_rect, music_artist_text, music_artist_rect, music_album_text, music_album_rect, time_remaining_text, time_remaining_rect, volume_integer_text, volume_integer_rect, music_name_text, music_name_rect, status_bool, shuffle_bool, &mut canvas);

        

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
                        pause_play.spawn().unwrap();
                    }

                    if x >= next_rect.x && x <= next_rect.x + DEFAULT_BUTTON_SIZE[0] && y >= next_rect.y && y <= next_rect.y + DEFAULT_BUTTON_SIZE[1] as i32
                    {
                        next.spawn().unwrap();
                    }

                    if x >= previous_rect.x && x <= previous_rect.x + DEFAULT_BUTTON_SIZE[0] && y >= previous_rect.y && y <= previous_rect.y + DEFAULT_BUTTON_SIZE[1] as i32
                    {
                        previous.spawn().unwrap();
                    }

//===========================================SHUFFLE KEYCHECKER (MOUSE)============================================
                    if x >= shuffle_button_rect.x && x <= shuffle_button_rect.x + SMALL_BUTTON_SIZE[1] && y >= shuffle_button_rect.y && y <= shuffle_button_rect.y + SMALL_BUTTON_SIZE[1] as i32
                    {
                        if keychecker_timer_1.elapsed() > keychecker_timer_duration_1 && !keychecker_timer_activator
                        {   
                            shuffle_on.spawn().unwrap();
    
                            keychecker_timer_activator = true;
                            keychecker_timer_1 = Instant::now();
                            keychecker_timer_2 = Instant::now();
                        }
    
                        if keychecker_timer_2.elapsed() > keychecker_timer_duration_2 && keychecker_timer_activator
                        {
                            shuffle_off.spawn().unwrap();
      
                            keychecker_timer_activator = false;
                            keychecker_timer_1 = Instant::now();
                            keychecker_timer_2 = Instant::now();
                        }
                    }
                }



//===============================================================================================================//
//------------------------------------------MEDIA KEYCHECKER (KEYBOARD)------------------------------------------//
//===============================================================================================================//
                Event::KeyDown { keycode: Some(Keycode::Space), .. } | Event::KeyDown { keycode: Some(Keycode::Return), .. } =>
                {
                    pause_play.spawn().unwrap(); 
                }        
                
                Event::KeyDown { keycode: Some(Keycode::L), .. } | Event::KeyDown { keycode: Some(Keycode::Right), .. } =>
                {
                    next.spawn().unwrap();
                }

                Event::KeyDown { keycode: Some(Keycode::H), .. } | Event::KeyDown { keycode: Some(Keycode::Left), .. } =>
                {
                    previous.spawn().unwrap();
                }        

//========================================SHUFFLE KEYCHECKER (KEYBOARD)============================================
            Event::KeyDown { keycode: Some(Keycode::I), .. } =>
            {
                    if keychecker_timer_1.elapsed() > keychecker_timer_duration_1 && !keychecker_timer_activator
                    {   
                        shuffle_on.spawn().unwrap();

                        keychecker_timer_activator = true;
                        keychecker_timer_1 = Instant::now();
                        keychecker_timer_2 = Instant::now();
                    }

                    if keychecker_timer_2.elapsed() > keychecker_timer_duration_2 && keychecker_timer_activator
                    {
                        shuffle_off.spawn().unwrap();
  
                        keychecker_timer_activator = false;
                        keychecker_timer_1 = Instant::now();
                        keychecker_timer_2 = Instant::now();
                    }
            }

//============================================AUDIO KEYCHECKER (KEYBOARD)==========================================
                Event::KeyDown { keycode: Some(Keycode::K), .. } | Event::KeyDown { keycode: Some(Keycode::Up), .. } | Event::KeyDown { keycode: Some(Keycode::Plus), .. } => 
                {   
                        volume_up.spawn().unwrap();
                }     

                Event::KeyDown { keycode: Some(Keycode::J), .. } | Event::KeyDown { keycode: Some(Keycode::Down), .. } | Event::KeyDown { keycode: Some(Keycode::Minus), .. } => 
                {
                        volume_down.spawn().unwrap();
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
