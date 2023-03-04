use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use ringbuf::{Consumer, Producer, RingBuffer};

pub mod spectrum;

const MIN_FREQUENCY: f32 = 27.0;
const MAX_FREQUENCY: f32 = 2000.0;
const NUM_SAMPLES: usize = 2048;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _stream: audio::Stream<InputModel>,
    consumer: Consumer<f32>,
    spectrum: Vec<(f32, f32)>, // (frequency, amplitude)
}

struct InputModel {
    pub producer: Producer<f32>,
}

fn model(app: &App) -> Model {
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
        spectrum,
    }
}

fn pass_in(model: &mut InputModel, buffer: &Buffer) {
    for frame in buffer.frames() {
        // frame has 2 channels (stereo)
        model.producer.push(frame[0]).ok();
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut samples = vec![];
    if model.consumer.len() < NUM_SAMPLES {
        return;
    }

    for _ in 0..NUM_SAMPLES {
        let sample = model.consumer.pop().unwrap();
        samples.push(sample);
    }

    model.spectrum = spectrum::calc_spectrum(&samples, MIN_FREQUENCY, MAX_FREQUENCY);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let boundary = app.window_rect();
    let min_x = boundary.left();
    let max_x = boundary.right();

    for (fr, amp) in model.spectrum.iter() {
        let x = map_range(*fr, MIN_FREQUENCY, MAX_FREQUENCY, min_x, max_x);
        let height = amp * 1000.0;
        let width = 10.0;
        let hue = map_range(*fr, MIN_FREQUENCY, MAX_FREQUENCY, 0.0, 1.0);
        draw.ellipse()
            .color(hsla(hue, 1.0, 0.5, 0.2))
            .w_h(width, height)
            .x_y(x, 0.0);
    }
    draw.to_frame(app, &frame).unwrap();
}
