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



//=====================================================================================//
//-------------------------------------------------------------------------------------//
//---------------------------------THE GENERATORS--------------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//
fn font_generator<'a>(additional_text: &str, texture_creator: &'a TextureCreator<sdl2::video::WindowContext>, size: u16, color: Color, path: &str, text: String, x: i32, y: i32) -> (Texture<'a>, Rect)
{
    let ttf_context = sdl2::ttf::init().unwrap();
    let font = ttf_context.load_font(path, size).unwrap();
    let surface = font
    .render(&format! ("{}{}", additional_text, text))
    .blended(color)
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
//---------------------------THE DATA CREATOR FUNCTIONS--------------------------------//
//-------------------------------------------------------------------------------------//
//=====================================================================================//

//=====================================================================================//
//----------------------------THE BACKGROUNDS DATA-------------------------------------//
//=====================================================================================//
fn download_image() 
{
    let mut download_image = gen_command("bash", "-c", "./scripts/background.sh");
    download_image.spawn().unwrap();
}


fn ui<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> (Rect, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>, Texture<'a>,  Texture<'a>)
{
    //all this ui images localitions is using the buttons rect so to change the position of the image you will need to change the position of the buttons
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

    let volume_rect = Rect::new(WINDOW_WIDTH_I32 - 275, WINDOW_HEIGHT_I32 - 65, 25, 25);
        
    return (volume_rect, playing_image, paused_image, next_image, previous_image, shuffle_on_image, shuffle_off_image, muted_audio_image, low_audio_image, medium_audio_image, high_audio_image);
}



fn volume_bar(volume_string: String) -> (Rect, Rect)
{
    let under_bar = Rect::new(WINDOW_WIDTH_I32 - 240, WINDOW_HEIGHT_I32 - 57, 35, 7);
    let mut bar_progress = Rect::new(WINDOW_WIDTH_I32 - 240, WINDOW_HEIGHT_I32 - 57, 0, 7);

    if volume_string == "0.0" { bar_progress.w = 0; };
    if volume_string == "0.1" { bar_progress.w = 3; };
    if volume_string == "0.2" { bar_progress.w = 5; }; 
    if volume_string == "0.3" { bar_progress.w = 7; };
    if volume_string == "0.4" { bar_progress.w = 10; };
    if volume_string == "0.5" { bar_progress.w = 12; };
    if volume_string == "0.6" { bar_progress.w = 17; };
    if volume_string == "0.7" { bar_progress.w = 22; };
    if volume_string == "0.8" { bar_progress.w = 28; };
    if volume_string == "0.9" { bar_progress.w = 30; };
    if volume_string == "1.0" { bar_progress.w = 35; };

    return (under_bar, bar_progress);
}






fn progress_bar<'a>(texture_creator: &'a TextureCreator<WindowContext>, get_time: &mut Command, get_current_time: &mut Command, get_time_remaining: &mut Command, mut music_time_string: String, mut music_current_time_string: String, mut music_time_remaining_string: String) -> (Texture<'a>, Rect, Rect, Rect)
{
    music_time_string.clear();
    get_time.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut music_time_string).unwrap();
    let mut remove1 = music_time_string.replace(":", "");
    remove1.pop();
    let music_total_time: i32 = remove1.parse().unwrap();
    let music_total_time_u32: u32 = remove1.parse().unwrap();

    music_current_time_string.clear();
    get_current_time.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut music_current_time_string).unwrap();
    let mut remove_1 = music_current_time_string.replace(":", "");
    remove_1.pop();
    let music_current_time: i32 = remove_1.parse().unwrap();


    music_time_remaining_string.clear();
    get_time_remaining.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut music_time_remaining_string).unwrap();
    music_time_remaining_string.pop();


    let u32_value: u32 = 40;
    let (time_remaining_texture, time_remaining_rect) = font_generator(" ", &texture_creator, 15, Color::RGB(255, 255, 255), "fonts/JetBrainsMonoNLNerdFontMono-Bold.ttf", music_time_remaining_string, (music_total_time_u32 + u32_value).try_into().unwrap(), 450);
    let progress_bar_background = Rect::new(40, 450, music_total_time.try_into().unwrap(), 20);
    let progress_rect = Rect::new(40, 450, music_current_time.try_into().unwrap(), 20);


    return (time_remaining_texture, time_remaining_rect, progress_bar_background, progress_rect);
}













fn background<'a>(get_background: &'a mut Command,texture_creator: &'a TextureCreator<WindowContext>) -> (Texture<'a>, Texture<'a>, Texture<'a>, Rect)
{
    download_image();

    let mut background_url = String::new();
    background_url.clear();

    get_background.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut background_url).unwrap();    
    let str: &str = &background_url;
    let image_name = str.replace("\n", "");
    while image_name == String::from("$PWD/.background/holder.png")
    {
    get_background.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut background_url).unwrap();    
    };

    let album_picture_rect = Rect::new(40, WINDOW_HEIGHT_I32 - 400, 200, 200);

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
//------------------------------THE BUTTONS DATA---------------------------------------//
//=====================================================================================//
fn buttons() -> (Rect, Rect, Rect, Rect, Color, i32, i32, i32, i32)
{
    let default_button_color = Color::RGB(0, 153, 107);
    let default_width = WINDOW_WIDTH_I32 - 750;
    let default_height = WINDOW_WIDTH_I32 - 750;
    let small_width = WINDOW_WIDTH_I32 - 765;
    let small_height = WINDOW_WIDTH_I32 - 765;
    let middle_screen_x_default = WINDOW_WIDTH_I32 / 2 - default_width / 2;
    let middle_screen_y_default = WINDOW_HEIGHT_I32 / 2 - default_height / 2;
    let middle_screen_x_small = WINDOW_WIDTH_I32 / 2 - small_width / 2;
    let middle_screen_y_small = WINDOW_HEIGHT_I32 / 2 - small_height / 2;

    let default_y = middle_screen_y_default + 250;
    let small_y = middle_screen_y_small + 250;
    let padding = WINDOW_WIDTH_I32 - 725;

    let shuffle_button_rect = gen_button(middle_screen_x_small - padding*2 + small_width / 2, small_y, small_width, small_height);
    let previous_rect = gen_button(middle_screen_x_default - padding, default_y, default_width, default_height);
    let pause_rect = gen_button(middle_screen_x_default, default_y, default_width, default_height);
    let next_rect = gen_button(middle_screen_x_default + padding, default_y, default_width, default_height);

    return (previous_rect, pause_rect, next_rect, shuffle_button_rect, default_button_color, default_width, default_height, small_width, small_height);
}



//=====================================================================================//
//-------------------------------THE FONTS DATA----------------------------------------//
//=====================================================================================//
fn fonts(music_artist_string: String, music_album_string: String, time_remaining_text: Texture, time_remaining_rect: Rect, music_progress_bar_background: Rect, music_progress_bar_rect: Rect, under_bar: Rect, bar_progress: Rect, audio_is_muted: bool, audio_is_low: bool, audio_is_medium: bool, audio_is_high: bool, status_bool: bool, shuffle_bool: bool, background: Texture, album_picture: Texture, blur: Texture, album_picture_rect: Rect, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, music_name_string: String, volume_command_string: String) 
{
let default_path = "fonts/JetBrainsMonoNLNerdFontMono-Bold.ttf";
let default_color = Color::RGB(255, 255, 255);
let default_size: u16 = 20;
let small_size: u16 = 13;

let volume_rect_pos = [WINDOW_WIDTH_I32 - 200, WINDOW_HEIGHT_I32 - 68];
let music_name_rect_pos = [WINDOW_WIDTH_I32 - 550, WINDOW_HEIGHT_I32 - 360];
let music_artist_rect_pos = [WINDOW_WIDTH_I32 - 545, WINDOW_HEIGHT_I32 - 325];
let music_album_rect_pos = [WINDOW_WIDTH_I32 - 545, WINDOW_HEIGHT_I32 - 300];

let (volume_name_text, volume_name_rect) = font_generator(" ", &texture_creator, default_size, default_color, default_path, volume_command_string, volume_rect_pos[0], volume_rect_pos[1]);
let (music_name_text, music_name_rect) = font_generator(" ", &texture_creator, default_size, default_color, default_path, music_name_string, music_name_rect_pos[0], music_name_rect_pos[1]);
let (music_artist_text, music_artist_rect) = font_generator(" ", &texture_creator, small_size, default_color, default_path, music_artist_string, music_artist_rect_pos[0], music_artist_rect_pos[1]);
let (music_album_text, music_album_rect) = font_generator(" ", &texture_creator, small_size, default_color, default_path, music_album_string, music_album_rect_pos[0], music_album_rect_pos[1]);


render_scene(music_artist_text, music_artist_rect, music_album_text, music_album_rect, time_remaining_text, time_remaining_rect, music_progress_bar_background, music_progress_bar_rect, under_bar, bar_progress, texture_creator, audio_is_muted, audio_is_low, audio_is_medium, audio_is_high, status_bool, shuffle_bool, background, album_picture, blur, album_picture_rect, canvas, volume_name_text, music_name_text, volume_name_rect, music_name_rect);
}



//=====================================================================================//
//-----------------------------THE COMMANDS DATA---------------------------------------//
//=====================================================================================//
fn commands(canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, sdl_started: sdl2::Sdl) 
{
//=============================THE DEFAULT CONFIG USED PER EVERY COMMAND========================================
    let default_shell: &str = "bash";
    let default_argument: &str = "-c"; 


//================================================COMMANDS============================================
    let mut volume_info = gen_command_with_output(default_shell, default_argument, "playerctl volume");
    let mut status_info = gen_command_with_output(default_shell, default_argument, "playerctl status");
    let mut music_name_info = gen_command_with_output(default_shell, default_argument, "playerctl metadata xesam:title");
    let mut shuffle_info = gen_command_with_output(default_shell, default_argument, "playerctl shuffle");
    let mut shuffle_on = gen_command(default_shell, default_argument, "playerctl shuffle on"); 
    let mut shuffle_off = gen_command(default_shell, default_argument, "playerctl shuffle off"); 
    let mut next = gen_command(default_shell, default_argument, "playerctl next"); 
    let mut previous = gen_command(default_shell, default_argument, "playerctl previous"); 
    let mut pause_play = gen_command(default_shell, default_argument, "playerctl play-pause"); 
    let mut volume_up = gen_command(default_shell, default_argument, "playerctl volume 0.1+"); 
    let mut volume_down = gen_command(default_shell, default_argument, "playerctl volume 0.1-"); 
    let mut get_background = gen_command_with_output(default_shell, default_argument, "./scripts/url.sh");
    let mut get_time = gen_command_with_output(default_shell, default_argument, "playerctl metadata playerctl metadata --format '{{ duration(mpris:length) }}'");
    let mut get_current_time = gen_command_with_output(default_shell, default_argument, "playerctl position --format '{{ duration(position)}}'");
    let mut get_time_remaining = gen_command_with_output(default_shell, default_argument, "playerctl metadata --format '{{ duration(mpris:length - position) }}'");
    let mut music_artist_info = gen_command_with_output(default_shell, default_argument, "playerctl metadata --format '{{ artist }}'");
    let mut music_album_info = gen_command_with_output(default_shell, default_argument, "playerctl metadata --format '{{ album }}'");



    system 
    (
        &mut get_background,
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
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_WIDTH_I32: i32 = 800;
const WINDOW_HEIGHT_I32: i32 = 600;
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
fn render_scene(music_artist_text: Texture, music_artist_rect: Rect, music_album_text: Texture, music_album_rect: Rect, time_remaining_text: Texture, time_remaining_rect: Rect, music_progress_bar_background: Rect, music_progress_bar_rect: Rect, under_bar: Rect, bar_progress: Rect, texture_creator: &TextureCreator<WindowContext>,audio_is_muted: bool, audio_is_low: bool, audio_is_medium: bool, audio_is_high: bool, status_bool: bool, shuffle_bool: bool, background: Texture, album_picture: Texture, blur: Texture, album_picture_rect: Rect, canvas: &mut Canvas<Window>, volume_name_text: Texture, music_name_text: Texture, volume_name_rect: Rect, music_name_rect: Rect)
{
    let (previous_rect, pause_rect, next_rect, shuffle_button_rect, default_button_color, _, _, _, _) = buttons();
    let (volume_rect, playing_image, paused_image, next_image, previous_image, shuffle_on_image, shuffle_off_image,muted_audio_image, low_audio_image, medium_audio_image, high_audio_image) = ui(texture_creator);


    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    

    canvas.copy(&background, None, None).unwrap();
    canvas.copy(&blur, None, None).unwrap();
    canvas.copy(&album_picture, None, album_picture_rect).unwrap();
    
    
    canvas.set_draw_color(default_button_color);
    canvas.fill_rect(shuffle_button_rect).unwrap();
    canvas.fill_rect(previous_rect).unwrap();
    canvas.fill_rect(pause_rect).unwrap();
    canvas.fill_rect(next_rect).unwrap();
    canvas.set_draw_color(Color::RGB(150, 150, 150));
    canvas.fill_rect(under_bar).unwrap();
    canvas.set_draw_color(default_button_color);
    canvas.fill_rect(bar_progress).unwrap();
   
    canvas.set_draw_color(Color::RGB(150, 150, 150));
    canvas.fill_rect(music_progress_bar_background).unwrap();
    canvas.set_draw_color(default_button_color);
    canvas.fill_rect(music_progress_bar_rect).unwrap();
    canvas.copy(&time_remaining_text, None, time_remaining_rect).unwrap();


    if shuffle_bool == true 
    {
        canvas.copy(&shuffle_on_image, None, shuffle_button_rect).unwrap();
    }

    if shuffle_bool == false
    {
        canvas.copy(&shuffle_off_image, None, shuffle_button_rect).unwrap();
    }




    if status_bool == true
    {
        canvas.copy(&paused_image, None, pause_rect).unwrap();
    }

    if status_bool == false
    {
        canvas.copy(&playing_image, None, pause_rect).unwrap();
    }




    if audio_is_muted
    {
        canvas.copy(&muted_audio_image, None, volume_rect).unwrap();
    }

    if audio_is_low
    {
        canvas.copy(&low_audio_image, None, volume_rect).unwrap();
    }

    if audio_is_medium
    {
        canvas.copy(&medium_audio_image, None, volume_rect).unwrap();
    }

    if audio_is_high
    {
        canvas.copy(&high_audio_image, None, volume_rect).unwrap();
    }



    canvas.copy(&next_image, None, next_rect).unwrap();
    canvas.copy(&previous_image, None, previous_rect).unwrap();
    canvas.copy(&music_name_text, None, music_name_rect).unwrap();
    canvas.copy(&music_artist_text, None, music_artist_rect).unwrap();
    canvas.copy(&music_album_text, None, music_album_rect).unwrap();
    canvas.copy(&volume_name_text, None, volume_name_rect).unwrap();

    canvas.present();
}




//=====================================================================================//
//----------------------------------THE MAIN SYSTEM------------------------------------//
//=====================================================================================//
fn system(get_background: &mut Command, volume_info: &mut Command, status_info: &mut Command, music_name_info: &mut Command, shuffle_info: &mut Command, volume_up: &mut Command, volume_down: &mut Command, shuffle_off: &mut Command, shuffle_on: &mut Command, next: &mut Command, previous: &mut Command, pause: &mut Command, get_time: &mut Command, get_current_time: &mut Command, get_time_remaining: &mut Command, music_artist_info: &mut Command, music_album_info: &mut Command, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, sdl_started: sdl2::Sdl)
{
    let (previous_rect, pause_rect, next_rect, shuffle_button_rect, _default_button_color, buttonsizex, buttonsizey, buttonsize_smallx, buttonsize_smally) = buttons();


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
        let (under_bar, bar_progress) = volume_bar(volume_command_string.clone());


//=======================================RENDER SCENE ACTIVATOR AND STRING PASSING============================================
        let (background, album_picture, blur, album_picture_rect) = background(get_background, texture_creator);
        fonts(music_artist_string.clone(), music_album_string.clone(), time_remaining_text, time_remaining_rect, music_progress_bar_background, music_progress_bar_rect, under_bar, bar_progress, audio_is_muted, audio_is_low, audio_is_medium, audio_is_high, status_bool, shuffle_bool, background, album_picture, blur, album_picture_rect, canvas, texture_creator, music_name_string.clone(), volume_command_string.clone());
        

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
                    if x >= pause_rect.x && x <= pause_rect.x + buttonsizex && y >= pause_rect.y && y <= pause_rect.y + buttonsizey as i32
                    {
                        pause.spawn().unwrap();
                    }

                    if x >= next_rect.x && x <= next_rect.x + buttonsizex && y >= next_rect.y && y <= next_rect.y + buttonsizey as i32
                    {
                        next.spawn().unwrap();
                    }

                    if x >= previous_rect.x && x <= previous_rect.x + buttonsizex && y >= previous_rect.y && y <= previous_rect.y + buttonsizey as i32
                    {
                        previous.spawn().unwrap();
                    }

                    if x >= shuffle_button_rect.x && x <= shuffle_button_rect.x + buttonsize_smallx && y >= shuffle_button_rect.y && y <= shuffle_button_rect.y + buttonsize_smally as i32
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
//
window();
}


