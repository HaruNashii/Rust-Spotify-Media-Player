//======================================THE CRATES============================================
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
//=========(ELEMENTS SCALES)==========//
//====================================//
// fonts
const DEFAULT_FONT_SIZE: u16 = 20;
const SMALL_FONT_SIZE: u16 = 13;

//buttons 

const DEFAULT_BUTTON_SIZE: [i32;2] = [50, 50];
const SMALL_BUTTON_SIZE: [i32;2] = [35, 35];






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






//====================================//
//========(ELEMENTS POSITION)=========//
//====================================//
// volume bar
const VOLUME_TEXT_POSITION: [i32;2] = [525, 535];
const VOLUME_BAR_BACKGROUND_POSITION: [i32;2] = [560, 543];
const VOLUME_LEVEL_BAR_POSITION: [i32;2] = [560, 543];

// progress bar
const PROGRESS_BAR_BACKGROUND_POSITION: [i32;2] = [40, 450];
const PROGRESS_BAR_POSITION: [i32;2] = [40, 450];
const PROGRESS_BAR_TIME_REMAINING_POSITION: [i32;2] = [740, 445];

// album picture / the background album image have the position based on the screen size (that's why it's not here)
const ALBUM_MINI_PICTURE_POSITION: [i32;2] = [40, 200];

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









//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------THE GENERATORS--------------------------------------//
//-------------------------------------------------------------------------------------//
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



fn gen_button(x: i32, y: i32, width: i32, height: i32) -> Rect 
{
let rect = Rect::new(x, y, width.try_into().unwrap(), height.try_into().unwrap());

return rect;
}




fn gen_image<'a>(path: &'a str, texture_creator: &'a TextureCreator<WindowContext>) -> Texture<'a>
{
    let image = texture_creator
    .load_texture(path)
    .unwrap();


    return image;
}










//=====================================================================================//
//-------------------------------------------------------------------------------------//
//-----------------------------------THE TOOLS FUNCTIONS-------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
fn update_string_progress_bar(command: &mut Command, mut string: String) -> String
{
    if string.len() > 1 { string.clear(); };
    command.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut string).unwrap();
    string.pop();


    return string;
}



fn update_and_convert_string_to_u32_progress_bar(command: &mut Command, mut string: String) -> u32
{
    if string.len() > 1 { string.clear(); };
    command.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut string).unwrap();
    let u32_returned = convert_to_u32(string);

    return u32_returned;
}



fn convert_to_u32(string: String) -> u32
{
    let mut string = string.replace(":", "");
    string.pop();
    if string.len() < 1 { string.push_str("0")};
    let u32_to_return: u32 = string.parse().unwrap();

    return u32_to_return;
}



fn download_image_fn(download_image: &mut Command) 
{
    download_image.spawn().unwrap();
}



fn get_background_info(command: &mut Command) -> String
{

    let mut string = String::new();
    if string.len() > 1 { string.clear(); };
    command.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut string).unwrap();   
    
    if string.len() >= 24 {  string = (&string[24..]).to_string(); };

    let str: &str = &string;
    let mut image_name: String = str.replace("\n", "");
    let image_name_str: &str = &image_name;

    let path_to_check = format!("{}{}{}", ".background/", image_name_str, ".png");
    if Path::new(&path_to_check).exists() 
    {
        image_name = path_to_check;
    }
    else 
    {
        image_name.clear();
        image_name.push_str(".background/holder.jpg");
    };
     

    return image_name;
}












//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------THE DATA CREATOR FUNCTIONS--------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//


//=====================================================================================//
//---------------------------------------THE UI DATA-----------------------------------//
//=====================================================================================//
fn ui<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> (Rect, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>,  Texture<'a>)
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
fn volume_bar(volume_string: String) -> (Rect, Rect)
{
    let under_volume_bar = Rect::new(VOLUME_BAR_BACKGROUND_POSITION[0], VOLUME_BAR_BACKGROUND_POSITION[1], 35, 7);
    let mut volume_level_bar = Rect::new(VOLUME_LEVEL_BAR_POSITION[0], VOLUME_LEVEL_BAR_POSITION[1], 0, 7);

    if volume_string == "0.0" { volume_level_bar.w = 0; };
    if volume_string == "0.1" { volume_level_bar.w = 3; };
    if volume_string == "0.2" { volume_level_bar.w = 5; }; 
    if volume_string == "0.3" { volume_level_bar.w = 7; };
    if volume_string == "0.4" { volume_level_bar.w = 10; };
    if volume_string == "0.5" { volume_level_bar.w = 12; };
    if volume_string == "0.6" { volume_level_bar.w = 17; };
    if volume_string == "0.7" { volume_level_bar.w = 22; };
    if volume_string == "0.8" { volume_level_bar.w = 28; };
    if volume_string == "0.9" { volume_level_bar.w = 30; };
    if volume_string == "1.0" { volume_level_bar.w = 35; };

    return (under_volume_bar, volume_level_bar);
}





//=====================================================================================//
//-------------------------------THE PROGRESS BAR DATA---------------------------------//
//=====================================================================================//
fn progress_bar<'a>(texture_creator: &'a TextureCreator<WindowContext>, get_time: &mut Command, get_current_time: &mut Command, get_time_remaining: &mut Command, music_time_string: String, music_current_time_string: String, music_time_remaining_string: String) -> (Texture<'a>, Rect, Rect, Rect)
{
    let music_total_time_u32 = update_and_convert_string_to_u32_progress_bar(get_time, music_time_string);

    let music_current_time_u32 = update_and_convert_string_to_u32_progress_bar(get_current_time, music_current_time_string);

    let music_current_time_percentage: f32 = music_current_time_u32 as f32 / music_total_time_u32 as f32 * 100.0;

    let music_time_remaining_string = update_string_progress_bar(get_time_remaining, music_time_remaining_string);

    let progress_bar_background = Rect::new(PROGRESS_BAR_BACKGROUND_POSITION[0], PROGRESS_BAR_BACKGROUND_POSITION[1], 700, 20);

    let progress_rect = Rect::new(PROGRESS_BAR_POSITION[0], PROGRESS_BAR_POSITION[1], music_current_time_percentage as u32 * 7 , 20);

    let (time_remaining_texture, time_remaining_rect) = font_generator(" ", &texture_creator, DEFAULT_FONT_SIZE, music_time_remaining_string, PROGRESS_BAR_TIME_REMAINING_POSITION[0], PROGRESS_BAR_TIME_REMAINING_POSITION[1]);

    return (time_remaining_texture, time_remaining_rect, progress_bar_background, progress_rect);
}





//=====================================================================================//
//----------------------------------THE BACKGROUNDS DATA-------------------------------//
//=====================================================================================//
fn background<'a>(download_image: &'a mut Command, get_background_path: &'a mut Command, texture_creator: &'a TextureCreator<WindowContext>) -> (Texture<'a>, Texture<'a>, Texture<'a>, Rect)
{
    download_image_fn(download_image);

    let image_name = get_background_info(get_background_path);

    let album_picture_rect = Rect::new(ALBUM_MINI_PICTURE_POSITION[0], ALBUM_MINI_PICTURE_POSITION[1], 200, 200);

    let background = texture_creator
    .load_texture(&image_name)
    .unwrap();

    let album_picture = texture_creator
    .load_texture(&image_name)
    .unwrap();

    let blur = texture_creator
    .load_texture(".background/blur.png")
    .unwrap();

    return (background, album_picture, blur, album_picture_rect);
}





//=====================================================================================//
//------------------------------------THE BUTTONS DATA---------------------------------//
//=====================================================================================//
fn buttons() -> (Rect, Rect, Rect, Rect)
{
    let shuffle_button_rect = gen_button(SMALL_BUTTON_MEDIA_POSITION[0] - SMALL_BUTTON_PADDING, SMALL_BUTTON_MEDIA_POSITION[1], SMALL_BUTTON_SIZE[0], SMALL_BUTTON_SIZE[1]);
    let previous_rect = gen_button(DEFAULT_BUTTON_MEDIA_POSITION[0] - DEFAULT_BUTTON_PADDING, DEFAULT_BUTTON_MEDIA_POSITION[1], DEFAULT_BUTTON_SIZE[0], DEFAULT_BUTTON_SIZE[1]);
    let pause_rect = gen_button(DEFAULT_BUTTON_MEDIA_POSITION[0], DEFAULT_BUTTON_MEDIA_POSITION[1], DEFAULT_BUTTON_SIZE[0], DEFAULT_BUTTON_SIZE[1]);
    let next_rect = gen_button(DEFAULT_BUTTON_MEDIA_POSITION[0] + DEFAULT_BUTTON_PADDING, DEFAULT_BUTTON_MEDIA_POSITION[1], DEFAULT_BUTTON_SIZE[0], DEFAULT_BUTTON_SIZE[1]);

    return (previous_rect, pause_rect, next_rect, shuffle_button_rect);
}





//=====================================================================================//
//-------------------------------THE FONTS DATA----------------------------------------//
//=====================================================================================//
fn fonts(music_artist_string: String, music_album_string: String, time_remaining_text: Texture, time_remaining_rect: Rect, music_progress_bar_background: Rect, music_progress_bar_rect: Rect, under_volume_bar: Rect, volume_level_bar: Rect, audio_is_muted: bool, audio_is_low: bool, audio_is_medium: bool, audio_is_high: bool, status_bool: bool, shuffle_bool: bool, background: Texture, album_picture: Texture, blur: Texture, album_picture_rect: Rect, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, music_name_string: String, volume_command_string: String) 
{

let (volume_integer_text, volume_integer_rect) = font_generator(" ", &texture_creator, DEFAULT_FONT_SIZE, volume_command_string, VOLUME_INTEGER_FONT_POSITION[0], VOLUME_INTEGER_FONT_POSITION[1]);

let (music_name_text, music_name_rect) = font_generator(" ", &texture_creator, DEFAULT_FONT_SIZE, music_name_string, MUSIC_NAME_FONT_POSITION[0], MUSIC_NAME_FONT_POSITION[1]);

let (music_artist_text, music_artist_rect) = font_generator(" ", &texture_creator, SMALL_FONT_SIZE, music_artist_string, MUSIC_ARTIST_NAME_FONT_POSITION[0], MUSIC_ARTIST_NAME_FONT_POSITION[1]);

let (music_album_text, music_album_rect) = font_generator(" ", &texture_creator, SMALL_FONT_SIZE, music_album_string, MUSIC_ALBUM_NAME_FONT_POSITION[0], MUSIC_ALBUM_NAME_FONT_POSITION[1]);


render_scene(music_artist_text, music_artist_rect, music_album_text, music_album_rect, time_remaining_text, time_remaining_rect, music_progress_bar_background, music_progress_bar_rect, under_volume_bar, volume_level_bar, texture_creator, audio_is_muted, audio_is_low, audio_is_medium, audio_is_high, status_bool, shuffle_bool, background, album_picture, blur, album_picture_rect, canvas, volume_integer_text, music_name_text, volume_integer_rect, music_name_rect);
}





//=====================================================================================//
//-----------------------------THE COMMANDS DATA---------------------------------------//
//=====================================================================================//
fn commands(canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, sdl_started: sdl2::Sdl) 
{
    let default_shell: &str = "bash";
    let default_argument: &str = "-c"; 


    let mut volume_info = gen_command_with_output(default_shell, default_argument, "playerctl -s volume && exit");

    let mut status_info = gen_command_with_output(default_shell, default_argument, "playerctl -s status && exit");

    let mut music_name_info = gen_command_with_output(default_shell, default_argument, "playerctl -s metadata xesam:title && exit");
    let mut shuffle_info = gen_command_with_output(default_shell, default_argument, "playerctl -s shuffle && exit");
    let mut shuffle_on = gen_command(default_shell, default_argument, "playerctl -s shuffle on && exit"); 
    let mut shuffle_off = gen_command(default_shell, default_argument, "playerctl -s shuffle off && exit"); 
    let mut next = gen_command(default_shell, default_argument, "playerctl -s next && exit"); 
    let mut previous = gen_command(default_shell, default_argument, "playerctl -s previous && exit"); 
    let mut pause_play = gen_command(default_shell, default_argument, "playerctl -s play-pause && exit"); 
    let mut volume_up = gen_command(default_shell, default_argument, "playerctl -s volume 0.1+ && exit"); 
    let mut volume_down = gen_command(default_shell, default_argument, "playerctl -s volume 0.1- && exit"); 
    let mut get_time = gen_command_with_output(default_shell, default_argument, "playerctl -s metadata --format '{{ duration(mpris:length) }}' && exit");
    let mut get_current_time = gen_command_with_output(default_shell, default_argument, "playerctl -s position --format '{{ duration(position)}}' && exit");
    let mut get_time_remaining = gen_command_with_output(default_shell, default_argument, "playerctl -s metadata --format '{{ duration(mpris:length - position) }}' && exit");
    let mut music_artist_info = gen_command_with_output(default_shell, default_argument, "playerctl -s metadata --format '{{ artist }}' && exit");
    let mut music_album_info = gen_command_with_output(default_shell, default_argument, "playerctl -s metadata --format '{{ album }}' && exit");

    let mut download_image = gen_command("sh", default_argument, "./scripts/background.sh && exit");

    let mut get_background_path = gen_command_with_output(default_shell, default_argument, "playerctl -s metadata mpris:artUrl && exit");

    system 
    (   
        &mut download_image,
        &mut get_background_path,
        &mut volume_info,
        &mut status_info,
        &mut music_name_info,
        &mut shuffle_info,
        &mut volume_up,
        &mut volume_down,
        &mut shuffle_off,
        &mut shuffle_on,
        &mut next,
        &mut previous,
        &mut pause_play,
        &mut get_time,
        &mut get_current_time, 
        &mut get_time_remaining,
        &mut music_artist_info,
        &mut music_album_info,
        canvas,
        texture_creator,
        sdl_started
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
fn window()
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

    commands(&mut canvas, &texture_creator, sdl_started);
}





//=====================================================================================//
//---------------------------------THE WINDOW RENDER-----------------------------------//
//=====================================================================================//
fn render_scene(music_artist_text: Texture, music_artist_rect: Rect, music_album_text: Texture, music_album_rect: Rect, time_remaining_text: Texture, time_remaining_rect: Rect, music_progress_bar_background: Rect, music_progress_bar_rect: Rect, under_volume_bar: Rect, volume_level_bar: Rect, texture_creator: &TextureCreator<WindowContext>,audio_is_muted: bool, audio_is_low: bool, audio_is_medium: bool, audio_is_high: bool, status_bool: bool, shuffle_bool: bool, background: Texture, album_picture: Texture, blur: Texture, album_picture_rect: Rect, canvas: &mut Canvas<Window>, volume_integer_text: Texture, music_name_text: Texture, volume_integer_rect: Rect, music_name_rect: Rect)
{
    let (previous_rect, pause_rect, next_rect, shuffle_button_rect) = buttons();
    let (volume_rect, playing_image, paused_image, next_image, previous_image, shuffle_on_image, shuffle_off_image,muted_audio_image, low_audio_image, medium_audio_image, high_audio_image) = ui(texture_creator);


    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    
    // background
    canvas.copy(&background, None, None).unwrap();
    canvas.copy(&blur, None, None).unwrap();

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
    canvas.copy(&next_image, None, next_rect).unwrap();
    canvas.copy(&previous_image, None, previous_rect).unwrap();
    if shuffle_bool { canvas.copy(&shuffle_on_image, None, shuffle_button_rect).unwrap(); }
    if !shuffle_bool { canvas.copy(&shuffle_off_image, None, shuffle_button_rect).unwrap(); }
    if !status_bool { canvas.copy(&playing_image, None, pause_rect).unwrap(); }
    if status_bool { canvas.copy(&paused_image, None, pause_rect).unwrap(); }
    if audio_is_muted { canvas.copy(&muted_audio_image, None, volume_rect).unwrap(); }
    if audio_is_low { canvas.copy(&low_audio_image, None, volume_rect).unwrap(); }
    if audio_is_medium { canvas.copy(&medium_audio_image, None, volume_rect).unwrap(); }
    if audio_is_high { canvas.copy(&high_audio_image, None, volume_rect).unwrap(); }



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
fn system(download_image: &mut Command, get_background_path: &mut Command, volume_info: &mut Command, status_info: &mut Command, music_name_info: &mut Command, shuffle_info: &mut Command, volume_up: &mut Command, volume_down: &mut Command, shuffle_off: &mut Command, shuffle_on: &mut Command, next: &mut Command, previous: &mut Command, pause: &mut Command, get_time: &mut Command, get_current_time: &mut Command, get_time_remaining: &mut Command, music_artist_info: &mut Command, music_album_info: &mut Command, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, sdl_started: sdl2::Sdl)
{
    let (previous_rect, pause_rect, next_rect, shuffle_button_rect) = buttons();


//===================================TIMER SETUP FOR DISPLAY INFO AND FOR THE KEYCHECKER=======================================
    let display_timer_activator = true;
    let display_timer_duration = Duration::from_millis(100);
    let mut display_timer = Instant::now();
    
    let mut keychecker_timer_activator = true;
    let keychecker_timer_duration_1 = Duration::from_millis(150);
    let keychecker_timer_duration_2 = Duration::from_millis(190);
    let mut keychecker_timer_1 = Instant::now();
    let mut keychecker_timer_2 = Instant::now();
 

//===========================================COMMANDS STRINGS============================================
    let mut music_name_string = String::new();
    let mut status_info_string = String::new();
    let mut shuffle_string = String::new();
    let mut volume_command_string = String::new();
    let music_time_string = String::new();
    let music_current_time_string = String::new();
    let music_time_remaining_string = String::new();
    let mut music_artist_string = String::new();
    let mut music_album_string = String::new();

    let mut shuffle_bool = false;
    let mut status_bool = false;

    let mut audio_is_muted = false;
    let mut audio_is_low = false;
    let mut audio_is_medium = false;
    let mut audio_is_high = false;

//===========================================THE LOOP============================================
    let mut event_pump = sdl_started.event_pump().unwrap();
    'running: loop 
    {
        std::thread::sleep(Duration::from_millis(16));


        let (time_remaining_text, time_remaining_rect, music_progress_bar_background, music_progress_bar_rect) = progress_bar(texture_creator, get_time, get_current_time, get_time_remaining, music_time_string.clone(), music_current_time_string.clone(), music_time_remaining_string.clone());


    

        while volume_command_string.len() >= 4 { volume_command_string.pop(); }
        if shuffle_string == String::from("On") { shuffle_bool = true; };
        if shuffle_string == String::from("Off") { shuffle_bool = false; };
        if status_info_string == String::from("Playing") { status_bool = true; };
        if status_info_string == String::from("Paused") { status_bool = false; };
        if volume_command_string == "0.0" { audio_is_muted = true; audio_is_low = false; audio_is_medium = false; audio_is_high = false;};
        if volume_command_string == "0.1" || volume_command_string == "0.2" || volume_command_string == "0.3" || volume_command_string == "0.4" { audio_is_low = true; audio_is_medium = false; audio_is_high = false; audio_is_muted = false; };

        if volume_command_string == "0.5" || volume_command_string == "0.6" || volume_command_string == "0.7" { audio_is_medium = true; audio_is_low = false; audio_is_high = false; audio_is_muted = false; };

        if volume_command_string == "0.8" || volume_command_string == "0.9" || volume_command_string == "1.0" { audio_is_high = true; audio_is_low = false; audio_is_medium = false; audio_is_muted = false; };

        let (under_volume_bar, volume_level_bar) = volume_bar(volume_command_string.clone());


//=======================================RENDER SCENE ACTIVATOR AND STRING PASSING============================================
        let (background, album_picture, blur, album_picture_rect) = background(download_image, get_background_path, texture_creator);
        fonts(music_artist_string.clone(), music_album_string.clone(), time_remaining_text, time_remaining_rect, music_progress_bar_background, music_progress_bar_rect, under_volume_bar, volume_level_bar, audio_is_muted, audio_is_low, audio_is_medium, audio_is_high, status_bool, shuffle_bool, background, album_picture, blur, album_picture_rect, canvas, texture_creator, music_name_string.clone(), volume_command_string.clone());
        

//===============================================ACTIVATOR OF THE COMMANDS IN LOOP============================================
        if display_timer.elapsed() >= display_timer_duration && display_timer_activator
        {
            music_name_string.clear();
            music_name_info.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut music_name_string).unwrap();
            music_name_string.pop();

            music_artist_string.clear();
            music_artist_info.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut music_artist_string).unwrap();
            music_artist_string.pop();
            
            music_album_string.clear();
            music_album_info.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut music_album_string).unwrap();
            music_album_string.pop();
            
            status_info_string.clear();
            status_info.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut status_info_string).unwrap();
            status_info_string.pop();
                        
            shuffle_string.clear();
            shuffle_info.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut shuffle_string).unwrap();
            shuffle_string.pop();
            
            volume_command_string.clear();
            volume_info.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut volume_command_string).unwrap();
            volume_command_string.pop();
            
            display_timer = Instant::now();
        }
        


//================================================MEDIA MOUSE CHECKER============================================
        for event in event_pump.poll_iter()
        {
            match event 
            {
                Event::MouseButtonDown {mouse_btn: MouseButton::Left, x, y, .. } => 
                {
                    if x >= pause_rect.x && x <= pause_rect.x + DEFAULT_BUTTON_SIZE[0] && y >= pause_rect.y && y <= pause_rect.y + DEFAULT_BUTTON_SIZE[1] as i32
                    {
                        pause.spawn().unwrap();
                    }

                    if x >= next_rect.x && x <= next_rect.x + DEFAULT_BUTTON_SIZE[0] && y >= next_rect.y && y <= next_rect.y + DEFAULT_BUTTON_SIZE[1] as i32
                    {
                        next.spawn().unwrap();
                    }

                    if x >= previous_rect.x && x <= previous_rect.x + DEFAULT_BUTTON_SIZE[0] && y >= previous_rect.y && y <= previous_rect.y + DEFAULT_BUTTON_SIZE[1] as i32
                    {
                        previous.spawn().unwrap();
                    }

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




//================================================MEDIA KEYCHECKER============================================

                Event::KeyDown { keycode: Some(Keycode::Space), .. } | Event::KeyDown { keycode: Some(Keycode::Return), .. } =>
                {
                    pause.spawn().unwrap(); 
                }        
                
                Event::KeyDown { keycode: Some(Keycode::L), .. } | Event::KeyDown { keycode: Some(Keycode::Right), .. } =>
                {
                    next.spawn().unwrap();
                }

                Event::KeyDown { keycode: Some(Keycode::H), .. } | Event::KeyDown { keycode: Some(Keycode::Left), .. } =>
                {
                    previous.spawn().unwrap();
                }        


//================================================SHUFFLE KEYCHECKER============================================
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



//================================================AUDIO KEYCHECKER============================================
                Event::KeyDown { keycode: Some(Keycode::K), .. } | Event::KeyDown { keycode: Some(Keycode::Up), .. } | Event::KeyDown { keycode: Some(Keycode::Plus), .. } => 
                {   
                        volume_up.spawn().unwrap();
                }     

                Event::KeyDown { keycode: Some(Keycode::J), .. } | Event::KeyDown { keycode: Some(Keycode::Down), .. } | Event::KeyDown { keycode: Some(Keycode::Minus), .. } => 
                {
                        volume_down.spawn().unwrap();
                }


//================================================QUIT HANDLER============================================
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
//the sequal for function activation is :
//
//                         gen_buttons()
//                              ^
//                              ^
//                           buttons()             gen_button()
//                              ^                       ^
//                              ^                       ^
//           gen_command()      ^  font_generator()  buttons()
//                 ^            ^         ^             ^
//                 ^            ^         ^             ^
//   Window() > commands() > system() > fonts() > render_scene()
//                 v            v
//                 v            v
//   gen_command_with_output()  v
//                              v
//                        backgrounds()


window();
}
