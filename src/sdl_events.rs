use std::process::exit;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;

use crate::playerctl_extra::*;
use crate::ui::DEFAULT_BUTTON_SIZE;
use crate::ui::SMALL_BUTTON_SIZE;
use playerctl::PlayerCtl;



const AUDIO_STEP: f32 = 0.1;



pub fn sdl_events(buttons_rect_vec: &Vec<Rect>, event_pump: &mut sdl2::EventPump) {
    for event in event_pump.poll_iter() {
        match event {

            //===============================================================================================================//
            //------------------------------------------------MEDIA MOUSE CHECKER--------------------------------------------//
            //===============================================================================================================//
            Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, ..} => {
                if x >= buttons_rect_vec[0].x && x <= buttons_rect_vec[0].x + DEFAULT_BUTTON_SIZE[0] && y >= buttons_rect_vec[0].y && y <= buttons_rect_vec[0].y + DEFAULT_BUTTON_SIZE[1] {
                    shuffle_toggle_set();
                }

                if x >= buttons_rect_vec[1].x && x <= buttons_rect_vec[1].x + DEFAULT_BUTTON_SIZE[0] && y >= buttons_rect_vec[1].y && y <= buttons_rect_vec[1].y + DEFAULT_BUTTON_SIZE[1] {
                    PlayerCtl::play_pause();
                }

                if x >= buttons_rect_vec[2].x && x <= buttons_rect_vec[2].x + DEFAULT_BUTTON_SIZE[0] && y >= buttons_rect_vec[2].y && y <= buttons_rect_vec[2].y + DEFAULT_BUTTON_SIZE[1] {
                    PlayerCtl::next();
                }

                if x >= buttons_rect_vec[3].x && x <= buttons_rect_vec[3].x + SMALL_BUTTON_SIZE[1] && y >= buttons_rect_vec[3].y && y <= buttons_rect_vec[3].y + SMALL_BUTTON_SIZE[1] {
                    shuffle_toggle_set();
                }
            }

            //===============================================================================================================//
            //------------------------------------------MEDIA KEYCHECKER (KEYBOARD)------------------------------------------//
            //===============================================================================================================//
            Event::KeyDown { keycode: Some(Keycode::Space), .. } | Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                PlayerCtl::play_pause();
            }

            Event::KeyDown { keycode: Some(Keycode::L), .. } | Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                PlayerCtl::next();
            }

            Event::KeyDown { keycode: Some(Keycode::H), .. } | Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                PlayerCtl::previous();
            }

            Event::KeyDown { keycode: Some(Keycode::I), .. } => {
                shuffle_toggle_set();
            }

            Event::KeyDown { keycode: Some(Keycode::K), .. } | Event::KeyDown { keycode: Some(Keycode::Up), .. } | Event::KeyDown { keycode: Some(Keycode::Plus), .. } => {
                volume_up(AUDIO_STEP);
            }

            Event::KeyDown { keycode: Some(Keycode::J), .. } | Event::KeyDown { keycode: Some(Keycode::Down), .. } | Event::KeyDown { keycode: Some(Keycode::Minus), .. } => {
                volume_down(AUDIO_STEP);
            }

            //===============================================================================================================//
            //------------------------------------QUIT EVENT & QUIT KEYCHECKER (KEYBOARD)------------------------------------//
            //===============================================================================================================//
            sdl2::event::Event::Quit { .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => {
                exit(0);
            }

            _ => {}
        }
    }
}
