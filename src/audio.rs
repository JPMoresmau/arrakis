use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{WavFormat, SourceHandle,output::Output, Source,},
    ecs::{World, WorldExt},
};

const WALL_SOUND: &str = "audio/wall.wav";
const FOUNTAIN_SOUND: &str = "audio/fountain.wav";
const ARMOURER_SOUND: &str = "audio/armourer.wav";
const MAGICIAN_SOUND: &str = "audio/magician.wav";
const CHARISMA_SOUND: &str = "audio/charisma.wav";
const MAGIC_SOUND: &str = "audio/magic.wav";
const POWER_SOUND: &str = "audio/power.wav";

/// different sounds
pub struct Sounds {
    pub wall_sfx: SourceHandle,
    pub fountain_sfx: SourceHandle,
    pub armourer_sfx: SourceHandle,
    pub magician_sfx: SourceHandle,
    pub charisma_sfx: SourceHandle,
    pub magic_sfx: SourceHandle,
    pub power_sfx: SourceHandle,
}

/// load a wav audio track
fn load_wav_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, WavFormat, (), &world.read_resource())
}

/// Initialise audio in the world
pub fn initialize_audio(world: &mut World) {
     let sound_effects = {
        let loader = world.read_resource::<Loader>();

        let sound = Sounds {
            wall_sfx: load_wav_track(&loader, &world, WALL_SOUND),
            fountain_sfx: load_wav_track(&loader, &world, FOUNTAIN_SOUND),
            armourer_sfx: load_wav_track(&loader, &world, ARMOURER_SOUND),
            magician_sfx: load_wav_track(&loader, &world, MAGICIAN_SOUND),
            charisma_sfx: load_wav_track(&loader, &world, CHARISMA_SOUND),
            magic_sfx: load_wav_track(&loader, &world, MAGIC_SOUND),
            power_sfx: load_wav_track(&loader, &world, POWER_SOUND),

        };

        sound
    };

    world.insert(sound_effects);
}

/// play the wall sound
pub fn play_wall_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
   play_sound(sounds, storage, output, &|s| &s.wall_sfx);
}

/// play a generic sound by passing which handle from Sounds to use
pub fn play_sound<'a>(sounds: &'a Sounds, storage: &AssetStorage<Source>, output: Option<&Output>,
    f: &dyn Fn(&'a Sounds) -> &'a SourceHandle) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(f(sounds)) {
            output.play_once(sound, 1.0);
        }
    }
}

/// no two closures have the same type, they said
/// wrap them up, they said
pub struct SoundHandler<'a>{
    pub handle_func: Box<&'a dyn Fn(&'a Sounds) -> &'a SourceHandle>,
}

/// at least hide the boxing
impl <'a> SoundHandler<'a> {
    pub fn new(func: &'a dyn Fn(&'a Sounds) -> &'a SourceHandle) -> SoundHandler<'a> {
        SoundHandler{
            handle_func: Box::new(func),
        }
    }
}
