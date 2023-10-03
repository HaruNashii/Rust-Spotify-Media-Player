use std::fs::File;
use std::io::{Write, Read};
use std::time::{Duration, Instant};
use std::process::{Command, Stdio};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

struct CommandValues <'a>
{
    style: &'a str,
    argument: &'a str,
    exec: &'a str,
}



fn command()
{
    let default_values = CommandValues
    {
        style: "bash",
        argument: "-c", 
        exec: "hope that don't print it",
    };




    let info_values = CommandValues
    {
        exec: "clear && echo Status: && playerctl status && echo Music Name: && playerctl metadata xesam:title && echo Volume: && playerctl volume && echo Shuffle: && playerctl shuffle",
        ..default_values
    };
    let mut display_info = Command::new(info_values.style);
    display_info.arg(info_values.argument);
    display_info.arg(info_values.exec);



    let shuffle_value = CommandValues
    {
        exec: "playerctl shuffle",
        ..default_values
    };
    let mut shuffle = Command::new(shuffle_value.style);
    shuffle.arg(shuffle_value.argument);
    shuffle.stdout(Stdio::piped());


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





    window(&mut shuffle, &mut display_info, &mut next, &mut previous, &mut pause_play);
}














const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn window(shuffle: &mut Command, display_info: &mut Command, next: &mut Command, previous: &mut Command, pause: &mut Command)
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


    let mut shuffle_string = String::from("");

    shuffle.arg("playerctl shuffle");
    shuffle_string.clear();
    shuffle.spawn().unwrap().stdout.take().unwrap().read_to_string(&mut shuffle_string).unwrap();
    shuffle_string.pop();

    let volume_change: f64 = 0.1;
    let mut volume = Command::new("bash");
    volume.arg("-c");
    volume.arg("./scripts/volume.sh");

    let mut volume_float: f64 = 1.0;

    let timer_activator = true;
    let timer_duration = Duration::from_millis(500);
    let mini_timer_duration = Duration::from_millis(250);
    let mini_timer_duration_2 = Duration::from_millis(100);
    let mut mini_timer = Instant::now();
    let mut mini_timer2 = Instant::now();
    let mut timer = Instant::now();


    let mut event_pump = sdl2::init().unwrap().event_pump().unwrap();
    'running: loop
    {
    std::thread::sleep(Duration::from_millis(5));
    
    println!("{}", shuffle_string.len());
    println!("{}", shuffle_string);

    let volume_string = String::from("playerctl volume ");
    let float_string = volume_float.to_string();
    let volume_combined = volume_string + &float_string;

    if timer.elapsed() >= timer_duration && timer_activator
    {
    display_info.spawn().unwrap();
    timer = Instant::now();
    }


        for event in event_pump.poll_iter()
        {
            match event 
            {
                Event::KeyDown { keycode: Some(Keycode::L), .. } => { next.spawn().unwrap(); }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => { pause.spawn().unwrap(); }        
                Event::KeyDown { keycode: Some(Keycode::H), .. } => { previous.spawn().unwrap(); }    
                
                Event::KeyDown { keycode: Some(Keycode::I), .. } =>
                {
                    if mini_timer.elapsed() > mini_timer_duration && timer_activator
                    {
                        if shuffle_string.len() == 3 
                        {
                            println!("have more or 3");
                            shuffle.arg("playerctl shuffle on");
                            shuffle_string.clear();
                            shuffle_string.push_str("on");
                            shuffle.spawn().unwrap();
                        }
                        mini_timer = Instant::now();
                    }

                    if mini_timer2.elapsed() > mini_timer_duration_2 && timer_activator
                    {
                        if shuffle_string.len() == 2
                        {
                            println!("have less or 2");
                            shuffle.arg("playerctl shuffle off");
                            shuffle_string.clear();
                            shuffle_string.push_str("off");
                            shuffle.spawn().unwrap();
                        }
                        mini_timer2 = Instant::now();
                    }
                }


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


        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
    }

}


fn main() 
{
    command();
}


