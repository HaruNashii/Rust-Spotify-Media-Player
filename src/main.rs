//======================================THE CRATES============================================
use std::fs::File;
use std::io::{Write, Read};
use std::time::{Duration, Instant};
use std::process::{Command, Stdio};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;



//======================================THE STRUCT THAT IS USED PER EVERY COMMAND============================================
struct CommandValues <'a>
{
    style: &'a str,
    argument: &'a str,
    exec: &'a str,
}










fn command()
{
//=============================THE DEFAULT CONFIG OF THE STRUCT USED PER EVERY COMMAND========================================
    let default_values = CommandValues
    {
        style: "bash",
        argument: "-c", 
        exec: "None",
    };



//===============================================INFO FOR RENDER COMMANDS============================================
    let info_values = CommandValues
    {
        exec: "clear && echo Status: && playerctl status && echo Music Name: && playerctl metadata xesam:title && echo Volume: && playerctl volume && echo Shuffle: && playerctl shuffle",
        ..default_values
    };
    let mut display_info = Command::new(info_values.style);
    display_info.arg(info_values.argument);
    display_info.arg(info_values.exec);



//================================================SHUFFLE COMMANDS============================================
    let shuffle_value = CommandValues
    {
        exec : "playerctl shuffle",
        ..default_values
    };
    let mut shuffle_info = Command::new(shuffle_value.style);
    shuffle_info.arg(shuffle_value.argument);
    shuffle_info.arg(shuffle_value.exec);
    shuffle_info.stdout(Stdio::piped());


    let shuffle_on_value = CommandValues
    {
        exec: "playerctl shuffle on",
        ..default_values
    };
    let mut shuffle_on = Command::new(shuffle_on_value.style);
    shuffle_on.arg(shuffle_on_value.argument);
    shuffle_on.arg(shuffle_on_value.exec);


    let shuffle_off_value = CommandValues
    {
        exec: "playerctl shuffle off",
        ..default_values
    };
    let mut shuffle_off = Command::new(shuffle_off_value.style);
    shuffle_off.arg(shuffle_off_value.argument);
    shuffle_off.arg(shuffle_off_value.exec);



//================================================BASIC MEDIA COMMANDS============================================
    let next_value = CommandValues 
    {
        exec: "playerctl next",
        ..default_values
    };
    let mut next = Command::new(next_value.style);
    next.arg(next_value.argument);
    next.arg(next_value.exec);


    let previous_value = CommandValues 
    {
        exec: "playerctl previous",
        ..default_values
    };
    let mut previous = Command::new(previous_value.style);
    previous.arg(previous_value.argument);
    previous.arg(previous_value.exec);


    let pause_play_value = CommandValues 
    {
        exec: "playerctl play-pause",
        ..default_values
    };
    let mut pause_play = Command::new(pause_play_value.style);
    pause_play.arg(pause_play_value.argument);
    pause_play.arg(pause_play_value.exec);



//================================================VOLUME COMMANDS============================================
    let volume_value = CommandValues
    {
        exec: "./scripts/volume.sh",
        ..default_values
    };
    let mut volume = Command::new(volume_value.style);
    volume.arg(volume_value.argument);
    volume.arg(volume_value.exec);



//=========================================PASS THE COMMANDS TO OTHER FUNCTION==========================================
window(&mut volume, &mut shuffle_info, &mut shuffle_on, &mut shuffle_off, &mut display_info, &mut next, &mut previous, &mut pause_play);
}










//================================================WINDOW============================================
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn window(volume: &mut Command, shuffle_info: &mut Command, shuffle_on: &mut Command, shuffle_off: &mut Command, display_info: &mut Command, next: &mut Command, previous: &mut Command, pause: &mut Command)
{
    let sdl_inicializator = sdl2::init().unwrap();
    let video_system = sdl_inicializator.video().unwrap();
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



//================================================WINDOW============================================
    let mut shuffle_info_string = String::from("");
    shuffle_info_string.clear();
    shuffle_info.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut shuffle_info_string).unwrap();
    shuffle_info_string.pop();



//================================================VOLUME SETUP============================================
    let volume_change: f64 = 0.1;
    let mut volume_float: f64 = 1.0;



//=======================================TIMER SETUP FOR DISPLAY INFO AND THE KEYCHECKER=======================================
    let timer_activator = true;
    let mini_timer_duration_1 = Duration::from_millis(16);
    let mini_timer_duration_2 = Duration::from_millis(116);
    let timer_duration = Duration::from_millis(350);
    let mut timer = Instant::now();
    let mut mini_timer_1 = Instant::now();
    let mut mini_timer_2 = Instant::now();



//================================================LOOP============================================
    let mut event_pump = sdl2::init().unwrap().event_pump().unwrap();
    'running: loop
    {
        std::thread::sleep(Duration::from_millis(16));
  


//================================================WINDOW============================================
        let volume_string = String::from("playerctl volume ");
        let float_string = volume_float.to_string();
        let volume_combined = volume_string + &float_string;



//================================================TIMER FOR DISPLAY INFO============================================
        if timer.elapsed() >= timer_duration && timer_activator
        {
            display_info.spawn().unwrap();
            timer = Instant::now();
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
                    if shuffle_info_string.len() == 3 
                    {
                        if mini_timer_1.elapsed() > mini_timer_duration_1 
                        {   
                            shuffle_on.spawn().unwrap();
                            shuffle_info_string.clear();
                            shuffle_info.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut shuffle_info_string).unwrap();
                            shuffle_info_string.pop();
                            mini_timer_1 = Instant::now();
                            mini_timer_2 = Instant::now();
                        }
                    }
                    
                
                    if shuffle_info_string.len() == 2  
                    {
                        if mini_timer_2.elapsed() > mini_timer_duration_2 
                        {
                            shuffle_off.spawn().unwrap();
                            shuffle_info_string.clear();
                            shuffle_info.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut shuffle_info_string).unwrap();
                            shuffle_info_string.pop();
                            mini_timer_1 = Instant::now();
                            mini_timer_2 = Instant::now();
                        }
                    }
                }



//================================================AUDIO KEYCHECKER============================================
                Event::KeyDown { keycode: Some(Keycode::K), .. } => 
                {   
                    if volume_float <= 1.0
                    {
                        volume_float += &volume_change;
                        File::create("scripts/volume.sh").unwrap().write_all(volume_combined.as_bytes()).unwrap();
                        volume.spawn().unwrap();
                    }
                }     


                Event::KeyDown { keycode: Some(Keycode::J), .. } => 
                {
                    if volume_float >= volume_change
                    {
                        volume_float -= &volume_change;
                        File::create("scripts/volume.sh").unwrap().write_all(volume_combined.as_bytes()).unwrap();
                        volume.spawn().unwrap();
                    }
                }        
                

                Event::Quit { .. } => break 'running,
                _=> {}
            }
        }



//================================================RENDER SCREEN============================================
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
    }
}



//================================================MAIN============================================










fn main() 
{
    command();
}


