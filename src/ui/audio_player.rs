use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;

pub struct AudioPlayer {
    sink: Option<Sink>,
    _stream: Option<OutputStream>,
}

impl AudioPlayer {
    pub fn new() -> Self {
        if let Ok((_stream, handle)) = OutputStream::try_default() {
            if let Ok(sink) = Sink::try_new(&handle) {
                return Self {
                    sink: Some(sink),
                    _stream: Some(_stream),
                };
            }
        }
        Self {
            sink: None,
            _stream: None,
        }
    }

    pub fn play_loop(&self) {
        if let Some(sink) = &self.sink {
            if let Ok(file) = File::open("sounds/Tetris.mp3") {
                if let Ok(source) = Decoder::new(BufReader::new(file)) {
                    sink.append(source.repeat_infinite());
                }
            }
        }
    }

    pub fn stop(&self) {
        if let Some(sink) = &self.sink {
            sink.stop();
        }
    }
}
