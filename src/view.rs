use crate::model::Model;
use crate::scene::Scene;
use crate::scene::SimpleSpectrum;
use nannou::prelude::*;

pub fn view(app: &App, model: &Model, frame: Frame) {
    let scene = SimpleSpectrum {};
    scene.view(app, model, frame);
}
