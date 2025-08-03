
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;

pub struct AudioPlayer {
    sink: Sink,
    _stream: OutputStream,
}

impl AudioPlayer {
    pub fn new() -> Self {
        let (_stream, handle) = OutputStream::try_default().expect("Failed to get audio output stream");
        let sink = Sink::try_new(&handle).expect("Failed to create audio sink");
        Self { sink, _stream }
    }

    pub fn play_loop(&self) {
        let file = File::open("sounds/Tetris.mp3").expect("Failed to open audio file");
        let source = Decoder::new(BufReader::new(file)).expect("Failed to decode audio file");
        self.sink.append(source.repeat_infinite());
    }

    pub fn stop(&self) {
        self.sink.stop();
    }
}
