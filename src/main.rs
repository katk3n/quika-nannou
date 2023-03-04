use crate::model::{model, update};

pub mod model;
pub mod scene;
pub mod spectrum;
pub mod view;

fn main() {
    nannou::app(model).update(update).run();
}
