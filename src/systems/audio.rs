use crate::audio::{play_bounce, play_score, Sounds};
use crate::event::PongEvent;

use amethyst::assets::AssetStorage;
use amethyst::audio::{output::Output, Source};
use amethyst::ecs::{Join, Read, ReadExpect, System, SystemData, World};
use amethyst::shrev::{EventChannel, ReaderId};

use std::ops::Deref;

#[derive(Default)]
pub struct AudioSystem {
    pong_event_reader: Option<ReaderId<PongEvent>>,
}

impl<'s> System<'s> for AudioSystem {
    type SystemData = (
        Read<'s, AssetStorage<Source>>,
        Option<Read<'s, Sounds>>,
        Option<Read<'s, Output>>,
        Read<'s, EventChannel<PongEvent>>,
    );

    fn run(&mut self, (storage, sounds, audio_output, pong_events): Self::SystemData) {
        // Reads PongEvent, play sound accordingly
        let reader = self
            .pong_event_reader
            .as_mut()
            .expect("AudioSystem::setup has not been called");

        if let Some(sounds) = sounds {
            pong_events.read(reader).for_each(|ev| match ev {
                PongEvent::Bounce => {
                    play_bounce(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()))
                }
                PongEvent::Score => {
                    play_score(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()))
                }
            });
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.pong_event_reader = Some(
            world
                .fetch_mut::<EventChannel<PongEvent>>()
                .register_reader(),
        );
    }
}
