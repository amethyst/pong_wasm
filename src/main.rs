//! Pong

mod audio;
mod bundle;
mod event;
mod pong;
mod systems;

use std::{path::Path, time::Duration};

use amethyst::{
    audio::{AudioBundle, DjSystemDesc},
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    ecs::{Component, DenseVecStorage},
    input::{Bindings, InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        rendy::hal::command::ClearColor,
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    window::EventLoop,
};
#[cfg(not(feature = "wasm"))]
use amethyst::{config::Config, window::DisplayConfig};

use crate::{audio::Music, bundle::PongBundle};

const ARENA_HEIGHT: f32 = 100.0;
const ARENA_WIDTH: f32 = 100.0;
const PADDLE_HEIGHT: f32 = 16.0;
const PADDLE_WIDTH: f32 = 4.0;
const PADDLE_VELOCITY: f32 = 75.0;

const BALL_VELOCITY_X: f32 = 75.0;
const BALL_VELOCITY_Y: f32 = 50.0;
const BALL_RADIUS: f32 = 2.0;

const AUDIO_MUSIC: &[&str] = &[
    "audio/Computer_Music_All-Stars_-_Wheres_My_Jetpack.ogg",
    "audio/Computer_Music_All-Stars_-_Albatross_v2.ogg",
];
const AUDIO_BOUNCE: &str = "audio/bounce.ogg";
const AUDIO_SCORE: &str = "audio/score.ogg";

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[cfg(not(feature = "wasm"))]
fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let setup_fn = |app_root: &Path, event_loop: &EventLoop<()>| {
        let key_bindings_path = {
            if cfg!(feature = "sdl_controller") {
                app_root.join("config/input_controller.ron")
            } else {
                app_root.join("config/input.ron")
            }
        };
        let bindings = <Bindings<StringBindings> as Config>::load(key_bindings_path)?;

        let display_config = DisplayConfig::load(app_root.join("config/display.ron"))?;
        let rendering_bundle = RenderingBundle::<DefaultBackend>::new(display_config, event_loop);

        Ok((bindings, rendering_bundle))
    };

    run_application(setup_fn)
}

#[allow(unused)]
#[cfg(feature = "wasm")]
fn main() {}

#[cfg(feature = "wasm")]
mod wasm {
    use std::path::Path;

    use amethyst::{
        config::Config,
        input::{Axis, Bindings, Button, StringBindings},
        renderer::{types::DefaultBackend, RenderingBundle},
        window::{DisplayConfig, EventLoop},
        winit::event::VirtualKeyCode,
        LoggerConfig,
    };
    use wasm_bindgen::prelude::*;
    use web_sys::HtmlCanvasElement;

    /// Pong application builder.
    #[wasm_bindgen]
    #[derive(Debug, Default)]
    pub struct PongAppBuilder {
        /// User supplied canvas, if any.
        canvas_element: Option<HtmlCanvasElement>,
        /// Input bindings data.
        input_bindings_str: Option<String>,
    }

    #[wasm_bindgen]
    impl PongAppBuilder {
        /// Returns a new `PongAppBuilder`.
        pub fn new() -> Self {
            Self::default()
        }

        /// Sets the canvas element for the `PongAppBuilder`.
        pub fn with_canvas(mut self, canvas: HtmlCanvasElement) -> Self {
            self.canvas_element = Some(canvas);
            self
        }

        /// Sets the canvas element for the `PongAppBuilder`.
        pub fn with_input_bindings(mut self, input_bindings_str: String) -> Self {
            self.input_bindings_str = Some(input_bindings_str);
            self
        }

        pub fn run(self) {
            // Make panic return a stack trace
            crate::init_panic_hook();

            amethyst::start_logger(LoggerConfig {
                allow_env_override: false,
                ..Default::default()
            });

            log::debug!("canvas element: {:?}", self.canvas_element);

            let dimensions = self
                .canvas_element
                .as_ref()
                .map(|canvas_element| (canvas_element.width(), canvas_element.height()));
            log::debug!("dimensions: {:?}", dimensions);

            let display_config = DisplayConfig {
                dimensions,
                ..Default::default()
            };

            let bindings = if let Some(input_bindings_str) = self.input_bindings_str.as_ref() {
                <Bindings<StringBindings> as Config>::load_bytes(input_bindings_str.as_bytes())
                    .expect("Failed to deserialize input bindings.")
            } else {
                // Hard coded bindings
                log::debug!("Using built in bindings.");

                let mut bindings = Bindings::<StringBindings>::new();
                let left_paddle_axis = Axis::Emulated {
                    pos: Button::Key(VirtualKeyCode::W),
                    neg: Button::Key(VirtualKeyCode::S),
                };
                let _ = bindings.insert_axis("left_paddle", left_paddle_axis);
                let right_paddle_axis = Axis::Emulated {
                    pos: Button::Key(VirtualKeyCode::Up),
                    neg: Button::Key(VirtualKeyCode::Down),
                };
                let _ = bindings.insert_axis("right_paddle", right_paddle_axis);

                bindings
            };

            let setup_fn = move |_: &Path, event_loop: &EventLoop<()>| {
                let rendering_bundle = RenderingBundle::<DefaultBackend>::new(
                    display_config,
                    event_loop,
                    self.canvas_element,
                );

                Ok((bindings, rendering_bundle))
            };

            let res = super::run_application(setup_fn);
            match res {
                Ok(_) => log::info!("Exited without error"),
                Err(e) => log::error!("Main returned an error: {:?}", e),
            }
        }
    }
}

fn run_application<FnSetupBundle>(setup_fn: FnSetupBundle) -> amethyst::Result<()>
where
    FnSetupBundle:
        FnOnce(
            &Path,
            &EventLoop<()>,
        )
            -> amethyst::Result<(Bindings<StringBindings>, RenderingBundle<DefaultBackend>)>,
{
    use crate::pong::Pong;

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");

    let event_loop = EventLoop::new();

    let (bindings, rendering_bundle) = setup_fn(&app_root, &event_loop)?;

    let game_data = GameDataBuilder::default()
        // Add the transform bundle which handles tracking entity positions
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings(bindings))?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            rendering_bundle
                // The RenderToWindow plugin provides all the scaffolding for opening a window and
                // drawing on it
                .with_plugin(RenderToWindow::new().with_clear(ClearColor {
                    float32: [0.34, 0.36, 0.52, 1.0],
                }))
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?;

    // Sound is currently not supported on wasm target
    let game_data = game_data
        .with_bundle(AudioBundle::default())?
        .with_system_desc(
            DjSystemDesc::new(|music: &mut Music| music.music.next()),
            "dj_system",
            &[],
        );

    let game_data = game_data.with_bundle(PongBundle)?;

    let game = Application::build(assets_dir, Pong::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .build(game_data)?;

    log::debug!("Before `run_winit_loop`.");
    game.run_winit_loop(event_loop);
}

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub velocity: f32,
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    pub fn new(side: Side) -> Paddle {
        Paddle {
            velocity: 1.0,
            side,
            width: 1.0,
            height: 1.0,
        }
    }
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct ScoreBoard {
    score_left: i32,
    score_right: i32,
}

impl ScoreBoard {
    pub fn new() -> ScoreBoard {
        ScoreBoard {
            score_left: 0,
            score_right: 0,
        }
    }
}
