use glam::Vec2;
use std::collections::HashSet;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::KeyCode;

#[derive(Debug, Clone)]
pub struct InputState {
    pub mouse_pos: Vec2,
    pub mouse_delta: Vec2,
    pub scroll_delta: f32,
    mouse_buttons: [bool; 5],
    prev_mouse_buttons: [bool; 5],
    keys_curr: HashSet<KeyCode>,
    keys_prev: HashSet<KeyCode>,
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

impl InputState {
    pub fn new() -> Self {
        Self {
            mouse_pos: Vec2::ZERO,
            mouse_delta: Vec2::ZERO,
            scroll_delta: 0.0,
            mouse_buttons: [false; 5],
            prev_mouse_buttons: [false; 5],
            keys_curr: HashSet::new(),
            keys_prev: HashSet::new(),
        }
    }

    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let new_pos = Vec2::new(position.x as f32, position.y as f32);
                self.mouse_delta = new_pos - self.mouse_pos;
                self.mouse_pos = new_pos;
            }
            WindowEvent::MouseInput { button, state, .. } => {
                let idx = match button {
                    MouseButton::Left => 0,
                    MouseButton::Right => 1,
                    MouseButton::Middle => 2,
                    MouseButton::Back => 3,
                    MouseButton::Forward => 4,
                    _ => return,
                };
                self.mouse_buttons[idx] = *state == ElementState::Pressed;
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.scroll_delta = match delta {
                    MouseScrollDelta::LineDelta(_, y) => *y,
                    MouseScrollDelta::PixelDelta(pos) => pos.y as f32 / 20.0,
                };
            }
            WindowEvent::KeyboardInput { .. } => {
            }
            _ => {}
        }
    }

    pub fn handle_key_event(&mut self, event: &winit::event::KeyEvent) {
        if let winit::keyboard::PhysicalKey::Code(code) = event.physical_key {
            match event.state {
                ElementState::Pressed => {
                    self.keys_curr.insert(code);
                }
                ElementState::Released => {
                    self.keys_curr.remove(&code);
                }
            }
        }
    }

    pub fn is_mouse_button_pressed(&self, button: usize) -> bool {
        button < 5 && self.mouse_buttons[button]
    }

    pub fn is_mouse_button_just_pressed(&self, button: usize) -> bool {
        button < 5 && self.mouse_buttons[button] && !self.prev_mouse_buttons[button]
    }

    pub fn is_key_down(&self, k: KeyCode) -> bool {
        self.keys_curr.contains(&k)
    }

    pub fn is_key_pressed(&self, k: KeyCode) -> bool {
        self.keys_curr.contains(&k) && !self.keys_prev.contains(&k)
    }

    pub fn is_key_just_pressed(&self, k: KeyCode) -> bool {
        self.keys_curr.contains(&k) && !self.keys_prev.contains(&k)
    }

    pub fn is_key_just_released(&self, k: KeyCode) -> bool {
        !self.keys_curr.contains(&k) && self.keys_prev.contains(&k)
    }

    pub fn clear_frame_end(&mut self) {
        self.scroll_delta = 0.0;
        self.mouse_delta = Vec2::ZERO;
        self.prev_mouse_buttons = self.mouse_buttons;
        self.keys_prev = self.keys_curr.clone();
    }
}
