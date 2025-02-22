use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

const WIDTH: usize = 200;
const HEIGHT: usize = 200;
const FRACTAL_DEPTH: u32 = 64;
const GENERATION_INFINITY: f64 = 16.;

pub fn fractal() {
    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Fractal example - press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale: Scale::X4,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create the window");

    window.set_target_fps(60);

    let range = 2.0;
    let x_min = 0. - range;
    let y_min = 0. - range;

    let x_max = 0. + range;
    let y_max = 0. + range;

    let mut angle: f64 = 0.0;

    window.set_background_color(0, 0, 20);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, pixel) in buffer.iter_mut().enumerate() {
            let mut real = map((i % WIDTH) as f64, 0., WIDTH as f64, x_min, x_max);
            let mut imag = map((i / HEIGHT) as f64, 0., HEIGHT as f64, y_min, y_max);

            let mut depth = 0;
            while depth < FRACTAL_DEPTH {
                let re = real.powf(2.) - imag.powf(2.);
                let im = 2. * real * imag;

                real = re + angle.cos();
                imag = im + angle.sin();

                if (real + imag).abs() > GENERATION_INFINITY {
                    break; // Leave when achieve infinity
                }
                depth += 1;
            }

            *pixel = if depth == FRACTAL_DEPTH {
                0x00
            } else {
                depth * 32 % 255
            };
        }

        angle += 0.1;

        // We unwrap here as we want this code to exit if it fails
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn map(val: f64, start1: f64, stop1: f64, start2: f64, stop2: f64) -> f64 {
    start2 + (stop2 - start2) * ((val - start1) / (stop1 - start1))
}
