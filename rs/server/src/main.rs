use kira::tween::Tween;
use kira::{
    manager::{
        AudioManager, AudioManagerSettings,
        backend::cpal::CpalBackend,
    },
    track::TrackId,    
    dsp::Frame,    
    clock::clock_info::ClockInfoProvider,
    sound::{SoundData,Sound},
    
};

use glicol_server::kira_and_glicol::*;

use std::thread;
use std::time::Duration;


pub struct GlicolKiraPlayer {
    manager: AudioManager,
    handle: Option<MyGlicolSoundHandle>,
}

impl GlicolKiraPlayer {

    fn new() -> GlicolKiraPlayer {
        GlicolKiraPlayer {
            manager: AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).expect("expect manager"),
            handle: None,
        }
    }

    fn send_code() {

    }

    fn play(&mut self) {

        let sound_data = MyGlicolSoundData::new(MyGlicolSoundSettings::default(), "o: sin 110 >> mul 0.25");
        // Play the sound using the AudioManager, and handle any errors
        match self.manager.play(sound_data) {
            Ok(handle) => {
                self.handle = Some(handle);
                // Sound started playing successfully
                // Do something with the handle, e.g. stop the sound later
            },
            Err(e) => {
                println!("{}", e);
            }
            // Err(PlaySoundError::AlreadyPlaying(handle)) => {
            //     // Sound is already playing, handle it appropriately
            // },
            // Err(PlaySoundError::AudioManagerClosed) => {
            //     // The AudioManager was closed before the sound could start playing
            // },
            // Err(PlaySoundError::Data(handle, error)) => {
            //     // There was an error playing the sound data
            //     // Handle the error appropriately
            // },
        };
    }

    fn panic() {
        
    }

    fn stop() {
        
    }


    fn load(&mut self, code: &str) {
        match &mut self.handle {
            Some(handle) => {
                handle.derp(code);

            },
            None => todo!()
        }
    }
    
    
    fn pause(&mut self) {
      let tween = Tween {
         start_time: kira::StartTime::Immediate,
         duration: Duration::new(5, 0),
         easing: kira::tween::Easing::Linear,
      };
      
      self.manager.pause(tween);
    }
     

}
fn main() {

    // let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).expect("expect manager");
    // let sound_data_3 = MyGlicolSoundData::new(MyGlicolSoundSettings::default(), "o: sin 440");
    
    // // manager.play(sound_data);
    // // manager.play(sound_data_2);
    // let my_sound = manager.play(sound_data_3).expect("asdasd");

    let mut player = GlicolKiraPlayer::new();
    player.play();

    println!("Hello world!");

    thread::sleep(Duration::from_secs(10));

    player.load("o: tri 210 >> mul 0.55");

    thread::sleep(Duration::from_secs(10));


    player.pause();
}
