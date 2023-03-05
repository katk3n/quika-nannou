use crate::model::Model;
use nannou::prelude::*;

pub fn view(app: &App, model: &Model, frame: Frame) {
    model.scenes[model.current_scene].view(app, model, frame);
}
