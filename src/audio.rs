use std::iter::Cycle;
use std::vec::IntoIter;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::audio::output::Output;
use amethyst::audio::{AudioSink, Source, SourceHandle};
use amethyst::ecs::prelude::World;
use amethyst::audio::OggFormat;

use super::constants::{AUDIO_MUSICS, AUDIO_BRICK_BROKEN, AUDIO_GAME_OVER};


pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>
}

pub struct Sounds {
    pub brick_broken_sfx: SourceHandle,
    pub game_over_sfx: SourceHandle,
}

/// Loads an ogg audio track.
fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(
        file, OggFormat,
        (),
        (),
        &world.read_resource())
}

/// Initialise audio in the world. This includes the background track and the
/// sound effects.
pub fn initialise_audio(world: &mut World) {

    let (music, sound_effects) = {
        let loader = world.read_resource::<Loader>();

        let music = AUDIO_MUSICS
            .iter()
            .map(|file| load_audio_track(&loader, &world, file))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        let music = Music { music };


        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.25); // Music is a bit loud, reduce the volume.

        let sound = Sounds {
            brick_broken_sfx: load_audio_track(&loader, &world, AUDIO_BRICK_BROKEN),
            game_over_sfx: load_audio_track(&loader, &world, AUDIO_GAME_OVER),
        };

        (music, sound)
    };

    // Add sound effects to the world. We have to do this in another scope because
    // world won't let us insert new resources as long as `Loader` is borrowed.
    world.add_resource(music);
    world.add_resource(sound_effects);
}

/// Plays the bounce sound when a ball hits a side or a paddle.
pub fn play_brick_broken(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.brick_broken_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

/// Plays the bounce sound when a ball hits a side or a paddle.
pub fn play_game_over(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.game_over_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}
