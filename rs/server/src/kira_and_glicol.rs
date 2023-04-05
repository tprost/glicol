use glicol::Engine;
use glicol_synth::Buffer;

use std::{
	sync::{
		atomic::AtomicU64,
		Arc,
	},
};

use ringbuf::HeapRb;
use ringbuf::HeapProducer;
use ringbuf::HeapConsumer;

use kira::{
    manager::{
        AudioManager, AudioManagerSettings,
        backend::cpal::CpalBackend,
    },
    track::TrackId,    
    dsp::Frame,    
    clock::clock_info::ClockInfoProvider,
    sound::{SoundData,Sound}
};

const COMMAND_BUFFER_CAPACITY: usize = 8;

pub struct Shared {
	// state: AtomicU8,
	pub position: AtomicU64,
}

pub struct Command {
  pub glicol_snippet: String,
}

pub struct MyGlicolSound {

  command_consumer: HeapConsumer<Command>,
  data: MyGlicolSoundData,

  // current_frame_index: usize,
  left_blocks: Vec<Buffer<8>>,
  right_blocks: Vec<Buffer<8>>,

  current_block: usize,

  current_index_left: usize,
  current_index_right: usize,
  shared: Arc<Shared>,
    
}

impl MyGlicolSound {
  pub fn new(data: MyGlicolSoundData, command_consumer: HeapConsumer<Command>) -> Self {
    Self { command_consumer,
           data,
           // current_frame_index: 8,
           left_blocks: Vec::from([]),
           right_blocks: Vec::from([]),
           // current_block_left: Buffer::SILENT,
           // current_block_right: Buffer::SILENT,
           current_block: 0,
           current_index_left: 0,
           current_index_right: 0,
           shared: Arc::new(Shared {
				     // state: AtomicU8::new(PlaybackState::Playing as u8),
				     position: AtomicU64::new(0) // position.to_bits()),
			     }),
    }
  }

  pub fn shared(&self) -> Arc<Shared> {
		self.shared.clone()
	}
}

impl Sound for MyGlicolSound {
    
  fn track(&mut self) -> TrackId {
		self.data.settings.track
	}

	fn on_start_processing(&mut self) {

    // println!("on_start_processing");      

    while let Some(command) = self.command_consumer.pop() {
        // xprintln!("b");      
        self.data.engine.update_with_code(&command.glicol_snippet);
        self.data.engine.update().ok();        
    }

    
      
    if self.current_block > 0 {
        // println!("on_start_processing (cleaning)");
        self.left_blocks.drain(0..self.current_block);
        self.right_blocks.drain(0..self.current_block);
        self.current_block = 0;
    }

    
    while self.left_blocks.len() < 32 {
        // println!("a");      
        // println!("on_start_p rocessing (pushing)");
        
        let block = self.data.engine.next_block(Vec::new());
        self.left_blocks.push(block.0[0].clone());
        self.right_blocks.push(block.0[1].clone());        
    }
	}

	fn process(&mut self, _dt: f64, _clock_info_provider: &ClockInfoProvider) -> Frame {

    // xprintln!("process {}", _dt);      

    // let l: f32 = rand::random::<f32>();
    // let r: f32 = rand::random::<f32>();

    // return Frame { left: l, right: r };
      

    while self.left_blocks.len() - self.current_block < 1 {
        // println!("on_start_p rocessing (pushing)");
        
        let block = self.data.engine.next_block(Vec::new());
        self.left_blocks.push(block.0[0].clone());
        self.right_blocks.push(block.0[1].clone());        
    }

    

      
    

    // let l: f32 = self.current_block_left[self.current_frame_index];
    // let r: f32 = self.current_block_right[self.current_frame_index];

    if self.left_blocks.len() <= self.current_block {
        return Frame{ left: 0.0, right: 0.0 }
    }
      
    let l: f32 = self.left_blocks.get(self.current_block).unwrap()[self.current_index_left];
    let r: f32 = self.right_blocks.get(self.current_block).unwrap()[self.current_index_right];      

    self.current_index_left += 1;
    self.current_index_right += 1;

    if self.current_index_left >= 8 {      
      self.current_index_left = 0;
      self.current_index_right = 0;
      self.current_block += 1;
    }
      
    // Frame{ left: l, right: r }
    Frame{ left: l, right: r }
	}

	fn finished(&self) -> bool {
      false
		// self.state == PlaybackState::Stopped && self.resampler.outputting_silence()
	}

}

/// Controls a static sound.
pub struct MyGlicolSoundHandle {
	pub command_producer: HeapProducer<Command>,

    
	pub shared: Arc<Shared>,
}

impl MyGlicolSoundHandle {
    pub fn derp(&mut self, glicol_snippet: &str) {
        self.command_producer.push(Command{glicol_snippet: glicol_snippet.to_string()}).ok();

        // self.engine.update_with_code(r#"o: tri 440"#);
    
    }
}

pub struct MyGlicolSoundSettings {
    pub track: TrackId,
}

impl MyGlicolSoundSettings {
	/// Creates a new [`StaticSoundSettings`] with the default settings.
	pub fn new() -> Self {
		Self {
			track: TrackId::Main,
    }
  }
}
            

impl Default for MyGlicolSoundSettings {
	fn default() -> Self {
		Self::new()
	}
}

// #[derive(Clone)]
pub struct MyGlicolSoundData{
    
  /// Settings for the sound.
	pub settings: MyGlicolSoundSettings,

  pub engine: Engine<8>,

}

impl MyGlicolSoundData {

    pub fn new(settings: MyGlicolSoundSettings, glicol_song: &str) -> Self {

        let mut engine = Engine::<8>::new();        
        engine.update_with_code(glicol_song);
        // engine.update_with_code(r#"o: sin 440"#);
        // engine.update_with_code(r#"o: tri 500"#);
        // engine.update_with_code(OCEAN_SONG);
        // engine.update_with_code(r#"o: sin 220 >> mul 0.6 >> add 0.1"#);
        
        // engine.update_with_code(r#"o: tri 440"#);
        // println!("next block {:?}", engine.next_block(vec![]));
        
        // let mut context = AudioContextBuilder::<8>::new()
        // .sr(44100)
        // .channels(2)
        // .build();

        // let node_a = context.add_mono_node(SinOsc::new().freq(440.0));
        // let node_b = context.add_stereo_node(Mul::new(0.1));

        // context.connect(node_a, node_b);
        // context.connect(node_b, context.destination);

        println!("updated engine with code");    
        
        Self { settings: settings, engine: engine }
    }

    
    pub fn split(self) -> (MyGlicolSound, MyGlicolSoundHandle) {
		    let (command_producer, command_consumer) = HeapRb::new(COMMAND_BUFFER_CAPACITY).split();
		    let sound = MyGlicolSound::new(self, command_consumer);
		    let shared = sound.shared();
		    (
			      sound,
			      MyGlicolSoundHandle {
				        command_producer,
				        shared,
			      },
		    )
	  }    

}

impl SoundData for MyGlicolSoundData {

  type Error = i32;
	type Handle = MyGlicolSoundHandle;

  // #[allow(clippy::type_complexity)]
	fn into_sound(self) -> Result<(Box<dyn Sound>, Self::Handle), Self::Error> {
    // let sound = MyGlicolSound::new(self);
    // let handle = MyGlicolSoundHandle{};
		let (sound, handle) = self.split();
		Ok((Box::new(sound), handle))
    // return Err(1);
	}

}

// #[derive(Resource)]
// struct GlicolKiraResource {
//     pub manager: AudioManager::<CpalBackend>
// }


// impl GlicolKiraComponent {

// }
