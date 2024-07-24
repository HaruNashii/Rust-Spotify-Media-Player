use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::Rect;

use crate::playerctl_extra::*;
use crate::DEFAULT_BUTTON_SIZE;
use crate::SMALL_BUTTON_SIZE;
use playerctl::PlayerCtl;



const AUDIO_STEP: f32 = 0.1;



pub fn sdl_events( previous_rect: Rect, pause_rect: Rect, next_rect: Rect, shuffle_button_rect: Rect, event_pump: &mut sdl2::EventPump) {
    for event in event_pump.poll_iter() {
        match event {

            //===============================================================================================================//
            //------------------------------------------------MEDIA MOUSE CHECKER--------------------------------------------//
            //===============================================================================================================//
            Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, ..} => {
                if x >= pause_rect.x && x <= pause_rect.x + DEFAULT_BUTTON_SIZE[0] && y >= pause_rect.y && y <= pause_rect.y + DEFAULT_BUTTON_SIZE[1] {
                    PlayerCtl::play_pause();
                }

                if x >= next_rect.x && x <= next_rect.x + DEFAULT_BUTTON_SIZE[0] && y >= next_rect.y && y <= next_rect.y + DEFAULT_BUTTON_SIZE[1] {
                    PlayerCtl::next();
                }

                if x >= previous_rect.x && x <= previous_rect.x + DEFAULT_BUTTON_SIZE[0] && y >= previous_rect.y && y <= previous_rect.y + DEFAULT_BUTTON_SIZE[1] {
                    shuffle_toggle_set();
                }

                if x >= shuffle_button_rect.x && x <= shuffle_button_rect.x + SMALL_BUTTON_SIZE[1] && y >= shuffle_button_rect.y && y <= shuffle_button_rect.y + SMALL_BUTTON_SIZE[1] {
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
                panic!("App Exited.");
            }

            _ => {}
        }
    }
}
