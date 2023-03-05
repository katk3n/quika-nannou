use super::Scene;
use crate::model::Model;
use nannou::prelude::*;

struct Particle {
    distance: f32,
    theta: f32,
    radius: f32,
    lightness: f32,
}

impl Particle {
    fn get_theta(&self) -> f32 {
        self.theta
    }

    fn set_theta(&mut self, theta: f32) {
        self.theta = theta;
    }
}

pub struct CircularParticles {
    particles: Vec<Particle>,
}

impl CircularParticles {
    pub fn new(num_particles: u32) -> Self {
        let mut particles = vec![];
        let radius = 10.0;
        for _ in 0..num_particles {
            let lightness = random_f32() * 0.9 + 0.1;
            let distance = random_f32() * 250.0 + 100.0;
            let theta = random_f32() * 2.0 * PI;
            let particle = Particle {
                distance,
                theta,
                radius,
                lightness,
            };
            particles.push(particle);
        }
        Self { particles }
    }
}

impl Scene for CircularParticles {
    fn view(&self, app: &App, model: &Model, frame: Frame) {
        let draw = app.draw();
        draw.blend(BLEND_ADD);

        // to draw shadow
        let window = app.window_rect();
        draw.rect().wh(window.wh()).rgba(0.0, 0.0, 0.0, 0.2);
        draw.to_frame(app, &frame).unwrap();

        let num_particles = self.particles.len();
        let max_index = model.spectrum.max_frequency as usize % num_particles;
        let max_amp = model.spectrum.max_amplitude;

        for (i, particle) in self.particles.iter().enumerate() {
            let point = pt2(particle.theta.cos(), particle.theta.sin()) * particle.distance;
            let radius = if i == max_index {
                particle.radius + max_amp * 100.0
            } else {
                particle.radius
            };
            draw.ellipse()
                .color(hsla(0.0, 0.0, particle.lightness, 0.4))
                .w_h(radius, radius)
                .x_y(point.x, point.y);
        }

        draw.to_frame(app, &frame).unwrap();
    }

    fn update(&mut self, _app: &App, _update: Update) {
        for particle in &mut self.particles {
            let theta = (particle.get_theta() + 0.01) % (2.0 * PI);
            particle.set_theta(theta);
        }
    }
}
