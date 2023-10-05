//======================================THE CRATES============================================
use std::fs::File;
use std::io::{Write, Read};
use std::time::{Duration, Instant};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::EventPump;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;



//================================================GENERATORS============================================
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






//================================================FONTS DATA============================================
fn fonts(canvas: &mut Canvas<Window>, shuffle_string: String, music_name_string: String, status_info_string: String, volume_command_string: String)
{
let texture_creator = canvas.texture_creator();
let default_path = "fonts/JetBrainsMonoNLNerdFontMono-Bold.ttf";
let default_color = Color::RGB(255, 255, 255);
let default_size: u16 = 25;

let (volume_text, volume_rect) = font_generator("Volume:", &texture_creator, default_size, default_color, default_path, volume_command_string, 0, 0);

let (shuffle_text, shuffle_rect) = font_generator("Shuffle:", &texture_creator, default_size, default_color, default_path, shuffle_string, 0, 50);

let (music_name_text, music_name_rect) = font_generator("Name:", &texture_creator, default_size, default_color, default_path, music_name_string, 0, 100);

let (status_text, status_rect) = font_generator("Status:", &texture_creator, default_size, default_color, default_path, status_info_string, 0, 150);


canvas.copy(&volume_text, None, volume_rect).unwrap();
canvas.copy(&shuffle_text, None, shuffle_rect).unwrap();
canvas.copy(&music_name_text, None, music_name_rect).unwrap();
canvas.copy(&status_text, None, status_rect).unwrap();
canvas.present();
}





//=============================THE COMMANDS DATA========================================
fn command(mut event_pump: &mut EventPump, canvas: &mut Canvas<Window>) 
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

    

    system 
    (
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
        &mut event_pump,
        canvas,
    );

}




//======================================THE WINDOW DATA AND INICIALIZATOR============================================
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
fn window()
{       
            let sdl_inicializator_creator = Arc::new(Mutex::new(sdl2::init().unwrap())); 
            let sdl_started = Arc::clone(&sdl_inicializator_creator);
            let sdl_guard = sdl_started.lock().unwrap();
            let video_system = sdl_guard.video().unwrap();
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


            let mut event_pump = sdl_guard.event_pump().unwrap();
            command(&mut event_pump, &mut canvas);
}
 




//================================================THE WINDOW RENDER============================================
fn render_scene(canvas: &mut Canvas<Window>, shuffle_info: String, music_name_string: String, status_info_string: String, volume_command_string: String)
{
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            fonts(canvas, shuffle_info, music_name_string, status_info_string, volume_command_string);
}





//================================================SYSTEM FUNCTION============================================
fn system(volume_info: &mut Command, status_info: &mut Command, music_name_info: &mut Command, shuffle_info: &mut Command, volume_up: &mut Command, volume_down: &mut Command, shuffle_on: &mut Command, shuffle_off: &mut Command, next: &mut Command, previous: &mut Command, pause: &mut Command,  event_pump: &mut EventPump, canvas: &mut Canvas<Window>)
{






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
    'running: loop 
    {
        while volume_command_string.len() >= 4 { volume_command_string.pop();}
        std::thread::sleep(Duration::from_millis(16));



//=======================================RENDER SCENE ACTIVATOR AND STRING PASSING============================================
        render_scene(canvas, shuffle_string.clone(), music_name_string.clone(), status_info_string.clone(), volume_command_string.clone());



//===============================================ACTIVATOR OF THE COMMANDS IN LOOP============================================
        if display_timer.elapsed() >= display_timer_duration && display_timer_activator
        {
            music_name_string.clear();
            music_name_info.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut music_name_string).unwrap();
            music_name_string.pop();
    
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
        


//================================================MEDIA KEYCHECKER============================================
        for event in event_pump.poll_iter()
        {
            match event 
            {
                Event::KeyDown { keycode: Some(Keycode::L), .. } => { next.spawn().unwrap(); }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => { pause.spawn().unwrap(); }        
                Event::KeyDown { keycode: Some(Keycode::H), .. } => { previous.spawn().unwrap(); }    
                


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
                Event::KeyDown { keycode: Some(Keycode::K), .. } => 
                {   
                        volume_up.spawn().unwrap();
                }     


                Event::KeyDown { keycode: Some(Keycode::J), .. } => 
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





//================================================MAIN============================================
fn main() 
{
//                                                gen_command()                         font_generator
//                                                     ^                                       ^
//                                                     ^                                       ^ 
//the sequal for function activation is (Window() > command() > system() > render_scene() > fonts())
//                                                     v
//                                                     v
//                                           gen_command_with_output()
window();
}


