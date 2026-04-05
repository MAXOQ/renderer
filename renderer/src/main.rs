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
    window.set_target_fps(10);

    let triangles: Vec<Triangle> = vec![
        Triangle {points: [(100, 100, 0), (100, 0, 0), (0, 100, 0)]}
    ];

    while window.is_open() && !window.is_key_down(Key::Escape) {

        for pixel in 0..buffer.len(){
            let point = index_to_point(pixel);
            buffer[pixel] = rgb(((point.0 * 255) / WIDTH as i32) as u8, ((point.1 * 255) / WIDTH as i32) as u8, 0);
        }

        for pixel in 0..buffer.len() {
            //buffer[pixel] = rgb(30, 30, 60);
            for x in &triangles{
                if point_in_triangle(*x, index_to_point(pixel)){
                    buffer[pixel] = rgb(0, 0, 255);
                    break;
                }
            }
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("Failed to update window");
    }
}

fn rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

fn index_to_point(index: usize) -> (i32, i32) {
    ((index % WIDTH) as i32, (index / WIDTH) as i32)
}

#[derive(Copy, Clone)]
struct Triangle{
    points: [(i32, i32, i32); 3],
}

fn point_in_triangle(triangle: Triangle, point: (i32, i32)) -> bool {
    let mut found_pos = false;
    let mut found_neg = false;

    let mut cross = cross_z(
        point,
        (triangle.points[0].0, triangle.points[0].1),
        (triangle.points[1].0, triangle.points[1].1),
    );
    if cross > 0 { found_pos = true; } else if cross < 0 { found_neg = true; }

    cross = cross_z(
        point,
        (triangle.points[1].0, triangle.points[1].1),
        (triangle.points[2].0, triangle.points[2].1),
    );
    if cross > 0 { found_pos = true; } else if cross < 0 { found_neg = true; }

    cross = cross_z(
        point,
        (triangle.points[2].0, triangle.points[2].1),
        (triangle.points[0].0, triangle.points[0].1),
    );
    if cross > 0 { found_pos = true; } else if cross < 0 { found_neg = true; }

    !(found_neg && found_pos)
}

// Z-component of the cross product of vectors (edge_b - edge_a) and (point - edge_a)
fn cross_z(point: (i32, i32), edge_a: (i32, i32), edge_b: (i32, i32)) -> i32 {
    let ab: (i32, i32) = (edge_b.0 - edge_a.0, edge_b.1 - edge_a.1);
    let ap: (i32, i32) = (point.0  - edge_a.0, point.1  - edge_a.1);
    ab.0 * ap.1 - ab.1 * ap.0
}