use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use ringbuf::{Consumer, Producer, RingBuffer};

use crate::spectrum;
use crate::view::view;

const MIN_FREQUENCY: f32 = 27.0;
const MAX_FREQUENCY: f32 = 2000.0;
const NUM_SAMPLES: usize = 2048;

pub struct Model {
    _stream: audio::Stream<InputModel>,
    consumer: Consumer<f32>,
    pub min_frequency: f32,
    pub max_frequency: f32,
    pub spectrum: Vec<(f32, f32)>, // (frequency, amplitude)
}

struct InputModel {
    pub producer: Producer<f32>,
}

pub fn model(app: &App) -> Model {
    // Create a window to receive key pressed events.
    app.new_window().view(view).build().unwrap();

    // Initialise the audio host so we can spawn an audio stream.
    let audio_host = audio::Host::new();

    // Create a ring buffer and split it into producer and consumer
    let ring_buffer = RingBuffer::<f32>::new(NUM_SAMPLES * 2); // Add some latency
    let (mut prod, cons) = ring_buffer.split();
    for _ in 0..NUM_SAMPLES {
        // The ring buffer has twice as much space as necessary to add latency here,
        // so this should never fail
        prod.push(0.0).unwrap();
    }

    // Create input model and input stream using that model
    let in_model = InputModel { producer: prod };
    let stream = audio_host
        .new_input_stream(in_model)
        .capture(pass_in)
        .build()
        .unwrap();

    stream.play().unwrap();

    let spectrum = vec![];

    Model {
        _stream: stream,
        consumer: cons,
        min_frequency: MIN_FREQUENCY,
        max_frequency: MAX_FREQUENCY,
        spectrum,
    }
}

fn pass_in(model: &mut InputModel, buffer: &Buffer) {
    for frame in buffer.frames() {
        // frame has 2 channels (stereo)
        model.producer.push(frame[0]).ok();
    }
}

pub fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut samples = vec![];
    if model.consumer.len() < NUM_SAMPLES {
        return;
    }

    for _ in 0..NUM_SAMPLES {
        let sample = model.consumer.pop().unwrap();
        samples.push(sample);
    }

    model.spectrum = spectrum::calc_spectrum(&samples, model.min_frequency, model.max_frequency);
}
