use std::process;
use std::env;

use coffee::{Game, Result, Timer};
use coffee::graphics::{
    Batch, Color, Frame, Image, Point, Rectangle, Sprite, Vector, Window,
    WindowSettings,
};
use coffee::input::{keyboard, KeyboardAndMouse};
use coffee::load::{Join, loading_screen::ProgressBar, Task};
use primes::PrimeSet;
use rayon::prelude::*;
use std::cmp::max;

fn main() -> Result<()> {
    PolarOxides::run(WindowSettings {
        title: String::from("Polar Oxides"),
        size: (1280, 800),
        resizable: true,
        fullscreen: false,
    })
}

#[derive(Debug, Clone)]
struct Particle {
    position: Point,
    is_prime: bool,
}

impl Particle {
    pub fn new(number: f32, prime_tester: &PrimeSet) -> Particle {
        Particle {
            position: Point::new(
                number * number.cos(),
                number * number.sin(),
            ),
            is_prime: prime_tester
                .find_vec(number as u64)
                .map(|(_, n)| n == number as u64)
                .unwrap_or_else(|| false),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Configs {
    zoom_level: i32,
    draw_nonprimes: bool,
}

struct PolarOxides  {
    particles: Vec<Particle>,
    batch: Batch,
    configs: Configs,
    prev_frame_configs: Configs,
}

impl PolarOxides  {
    const DEFAULT_MAX_NUMBER: u64 = 50_000;
    const BASE_PIXEL_RATE: f32 = 10.0;
    const MAX_ZOOM_LEVEL: i32 = 1000;

    pub fn generate_particles() -> Task<Vec<Particle>> {
        let args: Vec<String> = env::args().collect();
        let max_number:u64 = if args.len() > 1 {
            match args[1].trim().parse::<u64>() {
                Ok(i) => { i }
                Err(_) => { Self::DEFAULT_MAX_NUMBER }
            }
        } else {
            Self::DEFAULT_MAX_NUMBER
        };
        Task::new(move || {
            let mut prime_tester = PrimeSet::new();
            let (_, _) = prime_tester.find(max_number);

            (1..max_number).into_par_iter()
                .map(|number| Particle::new(number as f32, &prime_tester))
                .collect()
        })
    }

    pub fn generate_image() -> Task<Image> {
        Task::using_gpu( |gpu| Image::from_colors(gpu, &COLORS))
    }
}

impl Game for PolarOxides {
    type Input = KeyboardAndMouse;
    type LoadingScreen = ProgressBar;

    fn load(_window: &Window) -> Task<PolarOxides> {
        (
            Task::stage(
                "Finding primes and generating points...",
                Self::generate_particles(),
            ),
            Task::stage(
                "Generating image...",
                Self::generate_image()
            )
        )
        .join()
        .map(|(particles, image)| PolarOxides {
            particles,
            batch: Batch::new(image),
            configs: Configs {
                zoom_level: 0,
                draw_nonprimes: true,
            },
            prev_frame_configs: Configs {
                zoom_level: -1,
                draw_nonprimes: true,
            },
        })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(PolarOxideColors::BLACK);

        // Only update things if zoom has changed
        if self.configs != self.prev_frame_configs {
            let x_origin = frame.width() / 2.0;
            let y_origin = frame.height() / 2.0;

            let pixel_rate = Self::BASE_PIXEL_RATE / 1.02_f32.powi(self.configs.zoom_level);
            let centralize_vector = Vector::new(x_origin, y_origin);

            let draw_nonprime = self.configs.draw_nonprimes;
            let frame_bound = max(frame.width() as i32, frame.height() as i32) as f32;

            let sprites = self.particles.par_iter()
                .filter(|particle| {
                    let max_dim = max((particle.position * pixel_rate).x.abs() as i32,
                                      (particle.position * pixel_rate).y.abs() as i32) as f32;
                    max_dim >= 1.0 && max_dim / 2.0 <= frame_bound && (particle.is_prime || draw_nonprime)
                })
                .map(|particle| {
                    Sprite {
                        source: Rectangle {
                            x: if particle.is_prime {
                                    PolarOxideColors::index_of(PolarOxideColors::BLUE)
                                } else {
                                    PolarOxideColors::index_of(PolarOxideColors::YELLOW)
                                },
                            y: 0,
                            width: 1,
                            height: 1,
                        },
                        position: particle.position * pixel_rate + centralize_vector,
                        scale: (2.0, 2.0)
                    }
                });

            self.batch.clear();
            self.batch.par_extend(sprites);
        }
        self.batch.draw(&mut frame.as_target());
        self.prev_frame_configs = self.configs
    }

    fn interact(&mut self, input: &mut KeyboardAndMouse, window: &mut Window) {
        if input.is_key_pressed(keyboard::KeyCode::W) {
            if self.configs.zoom_level > 0 {
                self.configs.zoom_level -= 1;
            }
        }

        if input.is_key_pressed(keyboard::KeyCode::S) {
            if self.configs.zoom_level <= Self::MAX_ZOOM_LEVEL {
                self.configs.zoom_level += 1;
            }
        }

        if input.was_key_released(keyboard::KeyCode::F) {
            window.toggle_fullscreen();
            self.configs.zoom_level += 1;
        }

        if input.was_key_released(keyboard::KeyCode::D) {
            self.configs.draw_nonprimes = !self.configs.draw_nonprimes;
        }

        if input.was_key_released(keyboard::KeyCode::Escape) {
            process::exit(0);
        }
    }
}

struct PolarOxideColors { }

impl PolarOxideColors {
    const BLACK: Color = Color {r: 0.0, g: 0.0, b: 0.0, a: 1.0};
    const YELLOW: Color = Color {r: 0.91, g: 0.92, b: 0.18, a: 1.0};
    const BLUE: Color = Color {r: 0.36, g: 0.82, b: 0.69, a: 1.0};

    pub fn index_of(c: Color) -> u16 {
        match COLORS.iter().position(|color| color.eq(&c)) {
            Some(i) => { i as u16 }
            None => { 0 } // Black if we can't find a color
        }
    }
}

const COLORS: [Color; 3] = [
    PolarOxideColors::BLACK,
    PolarOxideColors::YELLOW,
    PolarOxideColors::BLUE,
];