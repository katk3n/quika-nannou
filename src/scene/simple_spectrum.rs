use super::Scene;
use crate::model::Model;
use nannou::prelude::*;

pub struct SimpleSpectrum {}

impl Scene for SimpleSpectrum {
    fn view(&self, app: &App, model: &Model, frame: Frame) {
        let draw = app.draw();
        draw.background().color(BLACK);
        let boundary = app.window_rect();
        let min_x = boundary.left();
        let max_x = boundary.right();

        for (fr, amp) in model.spectrum.frequencies.iter() {
            let x = map_range(*fr, model.min_frequency, model.max_frequency, min_x, max_x);
            let height = amp * 1000.0;
            let width = 10.0;
            let hue = map_range(*fr, model.min_frequency, model.max_frequency, 0.0, 1.0);
            draw.ellipse()
                .color(hsla(hue, 1.0, 0.5, 0.2))
                .w_h(width, height)
                .x_y(x, 0.0);
        }
        draw.to_frame(app, &frame).unwrap();
    }

    fn update(&mut self, _app: &App, _update: Update) {}
}
