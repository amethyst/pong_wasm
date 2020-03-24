//! Pong

mod audio;
mod bundle;
mod pong;
mod systems;

use amethyst::{
    audio::{AudioBundle, DjSystemDesc},
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    ecs::{Component, DenseVecStorage},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        rendy::hal::command::ClearColor,
        types::DefaultBackend,
        RenderingBundle
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    window::{DisplayConfig, EventLoop},
};

use crate::{audio::Music, bundle::PongBundle};
use std::time::Duration;

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
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

fn main() -> amethyst::Result<()> {
    #[cfg(not(feature = "wasm"))]
    amethyst::start_logger(Default::default());

    use crate::pong::Pong;

    log::debug!("before `application_root_dir()`");
    let app_root = application_root_dir()?;
    log::debug!("{:?}", app_root);

    let key_bindings_path = {
        if cfg!(feature = "sdl_controller") {
            app_root.join("config/input_controller.ron")
        } else {
            app_root.join("config/input.ron")
        }
    };
    log::debug!("{:?}", key_bindings_path);

    let assets_dir = app_root.join("assets");

    log::debug!("`EventLoop::new()`");
    let event_loop = EventLoop::new();

    log::debug!("`DisplayConfig::load()`");
    #[cfg(not(feature = "wasm"))]
    let display_config = DisplayConfig::load(
        app_root.join("config/display.ron")
    )?;
    #[cfg(feature = "wasm")]
    let display_config = DisplayConfig::default();

    let game_data = GameDataBuilder::default()
        // Add the transform bundle which handles tracking entity positions
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new()/*.with_bindings_from_file(key_bindings_path)?*/,
        )?
        // .with_bundle(PongBundle)?
        // .with_bundle(AudioBundle::default())?
        // .with_system_desc(
        //     DjSystemDesc::new(|music: &mut Music| music.music.next()),
        //     "dj_system",
        //     &[],
        // )
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new(display_config, &event_loop)
                // The RenderToWindow plugin provides all the scaffolding for opening a window and
                // drawing on it
                .with_plugin(RenderToWindow::new().with_clear(ClearColor {
                    float32: [0.34, 0.36, 0.52, 1.0],
                }))
                .with_plugin(RenderFlat2D::default())
                // .with_plugin(RenderUi::default())
        )?;

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

#[cfg(feature = "wasm")]
mod wasm {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn run() {
        // Make panic return a stack trace
        crate::init_panic_hook();

        wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

        log::debug!("run()");

        let res = super::main();
        match res {
            Ok(_) => log::info!("Exited without error"),
            Err(e) => log::error!("Main returned an error: {:?}", e),
        }
    }
}
