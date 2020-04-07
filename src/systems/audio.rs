use crate::audio::{play_bounce, play_score, Sounds};
use crate::event::PongEvent;

use amethyst::assets::AssetStorage;
use amethyst::audio::{output::OutputDevice, Source};
use amethyst::core::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, World, WorldExt};
use amethyst::shrev::{EventChannel, ReaderId};

/// Builds an `AudioSystem`.
#[derive(Default, Debug)]
pub struct AudioSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, AudioSystem> for AudioSystemDesc {
    fn build(self, world: &mut World) -> AudioSystem {
        <AudioSystem as System<'_>>::SystemData::setup(world);

        let output_device = OutputDevice::default();
        let pong_event_reader = world
            .write_resource::<EventChannel<PongEvent>>()
            .register_reader();

        AudioSystem::new(output_device, pong_event_reader)
    }
}

pub struct AudioSystem {
    output_device: OutputDevice,
    pong_event_reader: ReaderId<PongEvent>,
}

impl AudioSystem {
    fn new(output_device: OutputDevice, pong_event_reader: ReaderId<PongEvent>) -> Self {
        AudioSystem {
            output_device,
            pong_event_reader,
        }
    }
}

impl<'s> System<'s> for AudioSystem {
    type SystemData = (
        Read<'s, AssetStorage<Source>>,
        Option<Read<'s, Sounds>>,
        Read<'s, EventChannel<PongEvent>>,
    );

    fn run(&mut self, (storage, sounds, pong_events): Self::SystemData) {
        // Reads PongEvent, play sound accordingly
        if let Some(sounds) = sounds {
            pong_events
                .read(&mut self.pong_event_reader)
                .for_each(|ev| match ev {
                    PongEvent::Bounce => {
                        play_bounce(&*sounds, &storage, self.output_device.output())
                    }
                    PongEvent::Score => play_score(&*sounds, &storage, self.output_device.output()),
                });
        }
    }
}
