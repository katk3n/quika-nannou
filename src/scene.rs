use crate::model::Model;
use nannou::prelude::*;

pub mod simple_spectrum;

pub trait Scene {
    fn view(&self, app: &App, model: &Model, frame: Frame);
}
