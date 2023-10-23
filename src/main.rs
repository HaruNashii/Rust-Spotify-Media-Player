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
    let mut download_image = Command::new("bash");
    download_image.arg("./scripts/background.sh");
    download_image.spawn().unwrap();

}


fn background<'a>(get_background: &'a mut Command,texture_creator: &'a TextureCreator<WindowContext>) -> (Texture<'a>, Texture<'a>, Texture<'a>, Rect)
{
    download_image();

    let mut background_url = String::new();
    background_url.clear();

    get_background.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut background_url).unwrap();    
    let str: &str = &background_url;
    let image_name = str.replace("\n", "");
    while image_name == String::from("$PWD/.pictures/holder.png")
    {
    get_background.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut background_url).unwrap();    
    };

    let album_picture_rect = Rect::new(60, 400, 350, 350);

    let background = texture_creator
    .load_texture(&image_name)
    .unwrap();

    let album_picture = texture_creator
    .load_texture(&image_name)
    .unwrap();

    let blur = texture_creator
    .load_texture(".pictures/blur.png")
    .unwrap();

    return (background, album_picture, blur, album_picture_rect);
}



//=====================================================================================//
//------------------------------THE BUTTONS DATA---------------------------------------//
//=====================================================================================//
fn buttons() -> (Rect, Rect, Rect, Rect, Color, i32, i32, i32, i32)
{
    let default_button_color = Color::RGB(255, 255, 255);
    let default_width = 100;
    let default_height = 100;
    let small_width = 50;
    let small_height = 50;
    let middle_screen_x_default = WINDOW_WIDTH_I32 / 2 - default_width / 2;
    let middle_screen_y_default = WINDOW_HEIGHT_I32 / 2 - default_height / 2;
    let middle_screen_x_small = WINDOW_WIDTH_I32 / 2 - small_width / 2;
    let middle_screen_y_small = WINDOW_HEIGHT_I32 / 2 - small_height / 2;

    let default_y = middle_screen_y_default + 450;
    let small_y = middle_screen_y_small + 450;
    let padding = 200;

    let shuffle_button_rect = gen_button(middle_screen_x_small - padding*2 + 25, small_y, small_width, small_height);
    let previous_rect = gen_button(middle_screen_x_default - padding, default_y, default_width, default_height);
    let pause_rect = gen_button(middle_screen_x_default, default_y, default_width, default_height);
    let next_rect = gen_button(middle_screen_x_default + padding, default_y, default_width, default_height);

    return (previous_rect, pause_rect, next_rect, shuffle_button_rect, default_button_color, default_width, default_height, small_width, small_height);
}



//=====================================================================================//
//-------------------------------THE FONTS DATA----------------------------------------//
//=====================================================================================//
fn fonts(shuffle_bool: bool, background: Texture, album_picture: Texture, blur: Texture, album_picture_rect: Rect, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, shuffle_string: String, music_name_string: String, status_info_string: String, volume_command_string: String) 
{
let default_path = "fonts/JetBrainsMonoNLNerdFontMono-Bold.ttf";
let default_color = Color::RGB(255, 255, 255);
let default_size: u16 = 25;

let volume_rect_pos = [1360, 970];
let shuffle_rect_pos = [550, 970];
let music_name_rect_pos = [450, 500];
let status_rect_pos = [0, 150];

let (volume_text, volume_rect) = font_generator("Volume:", &texture_creator, default_size, default_color, default_path, volume_command_string, volume_rect_pos[0], volume_rect_pos[1]);
let (shuffle_text, shuffle_rect) = font_generator(" ", &texture_creator, default_size, default_color, default_path, shuffle_string, shuffle_rect_pos[0], shuffle_rect_pos[1]);
let (music_name_text, music_name_rect) = font_generator(" ", &texture_creator, default_size, default_color, default_path, music_name_string, music_name_rect_pos[0], music_name_rect_pos[1]);
let (status_text, status_rect) = font_generator("Status:", &texture_creator, default_size, default_color, default_path, status_info_string, status_rect_pos[0], status_rect_pos[1]);

render_scene(shuffle_bool, background, album_picture, blur, album_picture_rect, canvas, volume_text, shuffle_text, music_name_text, status_text, volume_rect, shuffle_rect, music_name_rect, status_rect);
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
const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;
const WINDOW_WIDTH_I32: i32 = 1920;
const WINDOW_HEIGHT_I32: i32 = 1080;
fn window()
{       
    let sdl_started = sdl2::init().unwrap(); 
    let video_system = sdl_started.video().unwrap();
    let window = video_system
    .window("Media", WINDOW_WIDTH, WINDOW_HEIGHT)
    .position_centered()
    .resizable()
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
fn render_scene(shuffle_bool: bool, background: Texture, album_picture: Texture, blur: Texture, album_picture_rect: Rect, canvas: &mut Canvas<Window>, volume_text: Texture, shuffle_text: Texture, music_name_text: Texture, status_text: Texture, volume_rect: Rect, shuffle_rect: Rect, music_name_rect: Rect, status_rect: Rect)
{
    let (previous_rect, pause_rect, next_rect, shuffle_button_rect, default_button_color, _, _, _, _) = buttons();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    
    canvas.copy(&background, None, None).unwrap();
    canvas.copy(&blur, None, None).unwrap();
    canvas.copy(&album_picture, None, album_picture_rect).unwrap();
    
    if shuffle_bool == true 
    {
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.fill_rect(shuffle_button_rect).unwrap();
    }
    if shuffle_bool == false
    {
        canvas.set_draw_color(Color::RGB(175, 175, 175));
        canvas.fill_rect(shuffle_button_rect).unwrap();
    }
    canvas.set_draw_color(default_button_color);
    canvas.fill_rect(previous_rect).unwrap();
    canvas.fill_rect(pause_rect).unwrap();
    canvas.fill_rect(next_rect).unwrap();

    canvas.copy(&volume_text, None, volume_rect).unwrap();
    canvas.copy(&shuffle_text, None, shuffle_rect).unwrap();
    canvas.copy(&music_name_text, None, music_name_rect).unwrap();
    canvas.copy(&status_text, None, status_rect).unwrap();
    canvas.present();
}




//=====================================================================================//
//----------------------------------THE MAIN SYSTEM------------------------------------//
//=====================================================================================//
fn system(get_background: &mut Command, volume_info: &mut Command, status_info: &mut Command, music_name_info: &mut Command, shuffle_info: &mut Command, volume_up: &mut Command, volume_down: &mut Command, shuffle_off: &mut Command, shuffle_on: &mut Command, next: &mut Command, previous: &mut Command, pause: &mut Command, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, sdl_started: sdl2::Sdl)
{
    let (previous_rect, pause_rect, next_rect, shuffle_button_rect, _default_button_color, buttonsizex, buttonsizey, buttonsize_smallx, buttonsize_smally) = buttons();


//===================================TIMER SETUP FOR DISPLAY INFO AND FOR THE KEYCHECKER=======================================
    let display_timer_activator = true;
    let display_timer_duration = Duration::from_millis(1);
    let mut display_timer = Instant::now();
    
    let mut keychecker_timer_activator = true;
    let keychecker_timer_duration_1 = Duration::from_millis(100);
    let keychecker_timer_duration_2 = Duration::from_millis(200);
    let mut keychecker_timer_1 = Instant::now();
    let mut keychecker_timer_2 = Instant::now();
 


//===========================================COMMANDS STRINGS============================================
    let mut music_name_string = String::from("");
    let mut status_info_string = String::from("");
    let mut shuffle_string = String::from("");
    let mut volume_command_string = String::from("");
    let mut shuffle_bool = false;
    


//===========================================THE LOOP============================================
    let mut event_pump = sdl_started.event_pump().unwrap();
    'running: loop 
    {
        while volume_command_string.len() >= 4 { volume_command_string.pop(); }
        if shuffle_string == String::from("On") { shuffle_bool = true; };
        if shuffle_string == String::from("Off") { shuffle_bool = false; };
        std::thread::sleep(Duration::from_millis(16));



//=======================================RENDER SCENE ACTIVATOR AND STRING PASSING============================================
        let (background, album_picture, blur, album_picture_rect) = background(get_background, texture_creator);
        fonts(shuffle_bool, background, album_picture, blur, album_picture_rect, canvas, texture_creator, shuffle_string.clone(), music_name_string.clone(), status_info_string.clone(), volume_command_string.clone());
        

//===============================================ACTIVATOR OF THE COMMANDS IN LOOP============================================
        if display_timer.elapsed() >= display_timer_duration && display_timer_activator
        {
            music_name_string.clear();
            music_name_info.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut music_name_string).unwrap();
            music_name_string.pop() ;
    

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


