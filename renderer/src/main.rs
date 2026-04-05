use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "minifb App - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("Unable to create window");

    // Limit to ~60 fps
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut triangles: Vec<Triangle> = vec![
            Triangle {points: vec![(0, 0, 0), (100, 0, 0), (0, 100, 0)]}
        ];

        for pixel in buffer.iter_mut() {
            *pixel = rgb(30, 30, 60);
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("Failed to update window");
    }
}

fn rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

struct Triangle{
    points: Vec<(u32, u32, u32)>,
}