use crate::audio::audio_manager::RtgcAudioManager;
use crate::core::app_state::AppState;
use crate::core::timer::FrameTimer;
use crate::graphics::game_renderer::GameSceneRenderer;
use crate::graphics::rhi::opengl::device::GlDevice;
use crate::graphics::ui_renderer::batch::DrawBatch;
use crate::graphics::CommandBuffer;
use crate::graphics::UiRenderer;
use crate::platform::input::InputState;
use crate::platform::paths::AppPaths;
use crate::platform::window::GlSurface;
use crate::screens::character_creation::character_creation_screen::CharacterCreationScreen;
use crate::screens::loading::loading_screen::LoadingScreen;
use crate::screens::main_menu::main_menu_screen::MainMenuScreen;
use crate::screens::save_select::save_select_screen::SaveSelectScreen;
use crate::screens::settings::settings_screen::SettingsScreen;
use crate::screens::splash::splash_screen::SplashScreen;
use anyhow::Result;
use glow::HasContext;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::{Key, KeyCode, NamedKey};
use winit::window::WindowId;
use winit::window::CursorGrabMode;

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}
    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {}
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.update();
        self.render();
    }
    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                self.should_exit = true;
            }
            WindowEvent::Resized(size) => {
                self.screen_width = size.width as f32;
                self.screen_height = size.height as f32;
                unsafe {
                    self.gl_context
                        .viewport(0, 0, size.width as i32, size.height as i32);
                }
                if let Some(ref mut scene) = self.game_scene {
                    scene.resize(self.screen_width, self.screen_height);
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let Key::Named(NamedKey::Escape) = &event.logical_key {
                    match &self.state {
                        AppState::Playing => {
                            self.state = AppState::PauseMenu;
                        }
                        AppState::PauseMenu => {
                            self.state = AppState::Playing;
                        }
                        AppState::Settings { return_to } => {
                            self.state = *return_to.clone();
                        }
                        AppState::CharacterCreation => {
                            self.state = AppState::MainMenu;
                        }
                        _ => {}
                    }
                }
                self.input.handle_key_event(&event);
            }
            other => {
                self.input.handle_window_event(&other);
            }
        }
    }
}

pub struct App {
    pub state: AppState,
    pub timer: FrameTimer,
    pub input: InputState,
    pub paths: AppPaths,
    pub gl_device: GlDevice,
    pub gl_surface: GlSurface,
    pub gl_context: Arc<glow::Context>,
    pub audio: RtgcAudioManager,
    pub draw_batch: DrawBatch,
    pub command_buffer: CommandBuffer,
    pub ui_renderer: UiRenderer,
    window: Option<Arc<std::sync::Mutex<winit::window::Window>>>,
    
    splash: SplashScreen,
    main_menu: Option<MainMenuScreen>,
    settings: Option<SettingsScreen>,
    character_creation: Option<CharacterCreationScreen>,
    loading: Option<LoadingScreen>,
    save_select: Option<SaveSelectScreen>,

    game_scene: Option<GameSceneRenderer>,
    character_height: f32,

    screen_width: f32,
    screen_height: f32,
    pub should_exit: bool,
}

impl App {
    pub fn new(
        gl_context: Arc<glow::Context>,
        gl_surface: GlSurface,
        audio: RtgcAudioManager,
        paths: AppPaths,
        width: u32,
        height: u32,
_window: Option<Arc<std::sync::Mutex<winit::window::Window>>>,
    ) -> Result<Self> {
        let gl_device = GlDevice::new(gl_context.clone());
        let ui_renderer = UiRenderer::new(gl_context.clone(), &paths.fonts_dir)
            .expect("Failed to create UI renderer");

        let game_scene = GameSceneRenderer::new(&gl_context, paths.assets_dir.clone())
            .map_err(|e| anyhow::anyhow!("Failed to create game scene: {}", e))
            .ok();

        Ok(Self {
            state: AppState::Splash,
            timer: FrameTimer::new(),
            input: InputState::new(),
            paths,
            gl_device,
            gl_surface,
            gl_context,
            audio,
            draw_batch: DrawBatch::new(),
            command_buffer: CommandBuffer::new(),
            ui_renderer,
            window: _window,
            splash: SplashScreen::new(),
            main_menu: None,
            settings: None,
            character_creation: None,
            loading: None,
            save_select: None,
            game_scene,
            character_height: 1.8,
            screen_width: width as f32,
            screen_height: height as f32,
            should_exit: false,
        })
    }

    fn update(&mut self) {
        self.timer.tick();

        let mouse_x = self.input.mouse_pos.x;
        let mouse_y = self.input.mouse_pos.y;
        let mouse_just_pressed = self.input.is_mouse_button_just_pressed(0);
        let mouse_held = self.input.is_mouse_button_pressed(0);
        let dt = self.timer.dt as f32;

        let next_state: Option<AppState> = match &mut self.state {
            AppState::Splash => self.splash.update(dt, &self.input),
            AppState::MainMenu => self.main_menu.as_mut().and_then(|menu| {
                menu.update(
                    mouse_x,
                    mouse_y,
                    mouse_just_pressed,
                    dt,
                    self.screen_width,
                    self.screen_height,
                )
            }),
            AppState::Settings { .. } => {
                if let Some(ref mut settings) = self.settings {
                    settings.update(
                        mouse_x,
                        mouse_y,
                        mouse_just_pressed,
                        mouse_held,
                        self.screen_width,
                        self.screen_height,
                    )
                } else {
                    None
                }
            }
            AppState::CharacterCreation => self.character_creation.as_mut().and_then(|s| {
                s.update(
                    mouse_x,
                    mouse_y,
                    mouse_just_pressed,
                    mouse_held,
                    self.screen_width,
                    self.screen_height,
                )
            }),
            AppState::Loading { .. } => self.loading.as_mut().and_then(|l| l.update(dt)),
            AppState::SaveSelect => self
                .save_select
                .as_mut()
                .and_then(|s| s.update(mouse_x, mouse_y, mouse_just_pressed)),
            AppState::Playing => {
                if let Some(ref mut scene) = self.game_scene {
                    scene.update(dt);
                }
                let forward = self.input.is_key_down(KeyCode::ArrowUp)
                    || self.input.is_key_down(KeyCode::KeyW);
                let back = self.input.is_key_down(KeyCode::ArrowDown)
                    || self.input.is_key_down(KeyCode::KeyS);
                let left = self.input.is_key_down(KeyCode::ArrowLeft)
                    || self.input.is_key_down(KeyCode::KeyA);
                let right = self.input.is_key_down(KeyCode::ArrowRight)
                    || self.input.is_key_down(KeyCode::KeyD);

                let mouse_dx = self.input.mouse_delta.x;
                let mouse_dy = self.input.mouse_delta.y;

                if forward || back || left || right {
                    tracing::debug!("Movement: f={} b={} l={} r={}", forward, back, left, right);
                }
                if mouse_dx != 0.0 || mouse_dy != 0.0 {
                    tracing::debug!("Mouse: dx={:.1}, dy={:.1}", mouse_dx, mouse_dy);
                }

                if let Some(ref mut scene) = self.game_scene {
                    if forward {
                        scene.move_camera(-1.5 * dt, 0.0);
                    }
                    if back {
                        scene.move_camera(1.5 * dt, 0.0);
                    }
                    if left {
                        scene.move_camera(0.0, -1.5 * dt);
                    }
                    if right {
                        scene.move_camera(0.0, 1.5 * dt);
                    }
                    if mouse_dx != 0.0 || mouse_dy != 0.0 {
                        scene.rotate_camera(mouse_dy * 0.003, mouse_dx * 0.003);
                    }
                }
                None
            }
            AppState::PauseMenu => None,
        };

        if let Some(next) = next_state {
            self.transition_to(next);
        }

        self.input.clear_frame_end();
    }

    fn transition_to(&mut self, next: AppState) {
        tracing::info!("Transitioning to {:?}", next);
        match &next {
            AppState::MainMenu => {
                self.main_menu = Some(MainMenuScreen::new(self.screen_width, self.screen_height));
            }
            AppState::Settings { return_to } => {
                self.settings = Some(SettingsScreen::new(
                    *return_to.clone(),
                    self.screen_width,
                    self.screen_height,
                ));
            }
            AppState::CharacterCreation => {
                self.character_creation = Some(CharacterCreationScreen::new());
            }
            AppState::Loading { character_data } => {
                self.loading = Some(LoadingScreen::new(
                    *character_data.clone(),
                    self.screen_width,
                    self.screen_height,
                ));
                self.character_height = character_data.height_m as f32;
                
                self.unlock_mouse();
            }
            AppState::Playing => {
                if let Some(ref mut scene) = self.game_scene {
                    scene.set_character_height(self.character_height);
                }
                self.lock_mouse();
            }
            AppState::PauseMenu => {
                self.unlock_mouse();
            }
            AppState::SaveSelect => {
                self.save_select =
                    Some(SaveSelectScreen::new(self.screen_width, self.screen_height));
            }
            _ => {}
        }
        self.state = next;
    }

    fn render(&mut self) {
        let sw = self.screen_width;
        let sh = self.screen_height;

        match &self.state {
            AppState::Splash
            | AppState::MainMenu
            | AppState::Settings { .. }
            | AppState::CharacterCreation
            | AppState::Loading { .. }
            | AppState::SaveSelect => {
                unsafe {
                    self.gl_context.clear_color(0.04, 0.04, 0.06, 1.0);
                    self.gl_context
                        .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                }
                self.draw_batch.clear();
            }
            AppState::Playing | AppState::PauseMenu => {}
        }

        match &self.state {
            AppState::Splash => {
                self.splash
                    .render(&mut self.draw_batch, &self.ui_renderer, sw, sh);
            }
            AppState::MainMenu => {
                if let Some(ref menu) = self.main_menu {
                    menu.render(&mut self.draw_batch, &self.ui_renderer, sw, sh);
                }
            }
            AppState::Settings { .. } => {
                if let Some(ref mut settings) = self.settings {
                    settings.render(&mut self.draw_batch, &self.ui_renderer, sw, sh);
                }
            }
            AppState::CharacterCreation => {
                if let Some(ref screen) = self.character_creation {
                    screen.render(&mut self.draw_batch, &self.ui_renderer, sw, sh);
                }
            }
            AppState::Loading { .. } => {
                if let Some(ref loading) = self.loading {
                    loading.render(&mut self.draw_batch, &self.ui_renderer, sw, sh);
                }
            }
            AppState::SaveSelect => {
                if let Some(ref save_select) = self.save_select {
                    save_select.render(&mut self.draw_batch, &self.ui_renderer, sw, sh);
                }
            }
            AppState::Playing => {
                if let Some(ref mut scene) = self.game_scene {
                    scene.render(&self.gl_context);
                }
            }
            AppState::PauseMenu => {
                self.draw_batch.push_rect(
                    crate::graphics::ui_renderer::batch::Rect {
                        x: 0.0,
                        y: 0.0,
                        w: sw,
                        h: sh,
                    },
                    crate::graphics::ui_renderer::batch::Color::new(0.05, 0.07, 0.09, 0.9),
                    0.0,
                );
                let t = "ПАУЗА";
                let tw = self.ui_renderer.measure_text_width(t);
                self.ui_renderer.push_text(
                    &mut self.draw_batch,
                    t,
                    sw / 2.0 - tw / 2.0,
                    sh / 2.0,
                    crate::graphics::ui_renderer::batch::Color::new(0.85, 0.55, 0.15, 1.0),
                );
                let hint = "ESC — продолжить";
                let hw = self.ui_renderer.measure_text_width(hint);
                self.ui_renderer.push_text(
                    &mut self.draw_batch,
                    hint,
                    sw / 2.0 - hw / 2.0,
                    sh / 2.0 + 30.0,
                    crate::graphics::ui_renderer::batch::Color::RTGC_TEXT_DIM,
                );

                if let Some(ref mut scene) = self.game_scene {
                    scene.render(&self.gl_context);
                }
            }
        }

        if matches!(
            &self.state,
            AppState::Splash
                | AppState::MainMenu
                | AppState::Settings { .. }
                | AppState::CharacterCreation
                | AppState::Loading { .. }
                | AppState::SaveSelect
                | AppState::PauseMenu
        ) {
            self.ui_renderer.render(&self.draw_batch, sw, sh);
        }

        let fps_text = format!("{:.0} FPS", self.timer.fps);
        self.ui_renderer.push_text(
            &mut self.draw_batch,
            &fps_text,
            sw - 80.0,
            20.0,
            crate::graphics::ui_renderer::batch::Color::new(0.4, 0.8, 0.4, 0.8),
        );

        use glutin::prelude::GlSurface;
        let _ = self
            .gl_surface
            .surface
            .swap_buffers(&self.gl_surface.context)
            .ok();
    }

    fn lock_mouse(&self) {
        if let Some(ref win) = self.window {
            if let Ok(window) = win.lock() {
                window.set_cursor_grab(CursorGrabMode::Confined).ok();
                window.set_cursor_visible(false);
            }
        }
    }

    fn unlock_mouse(&self) {
        if let Some(ref win) = self.window {
            if let Ok(window) = win.lock() {
                window.set_cursor_grab(CursorGrabMode::None).ok();
                window.set_cursor_visible(true);
            }
        }
    }
}
