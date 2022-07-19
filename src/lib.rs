mod camera;
mod mesh;
mod triangle;

use std::time::{Instant, SystemTime};

use camera::Camera;
use mesh::Mesh;
use nalgebra as nl;
use nl::vector;
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    prelude,
};
use sdl2::{
    event::Event, gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, sys::CurrentTime,
    video::Window,
};
use triangle::Triangle;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

pub fn run() {
    //SDL setup
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // Projection Matrix

    let f_near: f32 = 0.1;
    let f_far: f32 = 1000.;
    let f_fov: f32 = 90.;
    let f_aspect_ratio = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;

    let mut camera = Camera::new(
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        f_near,
        f_far,
        f_fov,
        f_aspect_ratio,
    );

    // let pot = Mesh::load("xyzrgb_dragon.obj").unwrap();
    // let pot = Mesh::load("teapot.obj").unwrap();
    // let pot = Mesh::load("african_head.obj").unwrap();
    // let pot = Mesh::load("mountains.obj").unwrap();
    // let pot = Mesh::load("axis.obj").unwrap();
    let pot = Mesh::unit_cube();

    let time = Instant::now();
    let mut last_time = 0;

    'mainloop: loop {
        let temp = time.elapsed().as_millis();

        println!("{:?}", 1000 / (temp - last_time).max(1));

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'mainloop,
                _ => (),
            }
        }

        for key in event_pump.keyboard_state().pressed_scancodes().into_iter() {
            match key.to_string().as_str() {
                "Space" => camera.move_up(0.01 * (temp - last_time) as f32),
                "Left Shift" => camera.move_up(-0.01 * (temp - last_time) as f32),
                "A" => camera.move_left(0.01 * (temp - last_time) as f32),
                "D" => camera.move_left(-0.01 * (temp - last_time) as f32),
                "W" => camera.move_forward(0.01 * (temp - last_time) as f32),
                "S" => camera.move_forward(-0.01 * (temp - last_time) as f32),
                "Left" => camera.move_looking_at(0., -0.001 * (temp - last_time) as f32),
                "Right" => camera.move_looking_at(0., 0.001 * (temp - last_time) as f32),
                "Up" => camera.move_looking_at(-0.001 * (temp - last_time) as f32, 0.),
                "Down" => camera.move_looking_at(0.001 * (temp - last_time) as f32, 0.),
                _ => {}
            }
        }

        last_time = temp;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let mut triangles_to_draw = Vec::<Triangle>::new();

        let a = pot
            .triangles
            .par_iter()
            .map(|tri| {
                let normal = tri.normal();

                if camera.is_triangle_visible(&tri) {
                    let mut light_direction = nl::vector![0., 1., -1.];
                    light_direction = light_direction * 1. / light_direction.norm();

                    let dp = normal.dot(&light_direction).max(0.1);

                    let mut proj_tri = camera.project_triangle(&tri);

                    for tri in &mut proj_tri {
                        tri.col = [(255. * dp) as u8, (255. * dp) as u8, (255. * dp) as u8];
                    }
                    proj_tri
                } else {
                    vec![]
                }
            })
            .collect::<Vec<Vec<Triangle>>>();

        for mut b in a {
            triangles_to_draw.append(&mut b)
        }

        triangles_to_draw.sort_unstable_by(|a, b| {
            (a.mid[2] - camera.position[2])
                .partial_cmp(&(b.mid[2] - camera.position[2]))
                .unwrap()
        });

        for i in 0..4 {
            let mut temp: Vec<Triangle> = vec![];
            for t in &triangles_to_draw {
                let mut clipped = match i {
                    0 => camera.clip_triangle(&vector![0., 0., 0.], &vector![0., 1., 0.], *t),
                    1 => camera.clip_triangle(
                        &vector![0., WINDOW_HEIGHT as f32 - 1., 0.],
                        &vector![0., -1., 0.],
                        *t,
                    ),
                    2 => camera.clip_triangle(&vector![0., 0., 0.], &vector![1., 0., 0.], *t),
                    3 => camera.clip_triangle(
                        &vector![WINDOW_WIDTH as f32 - 1., 0., 0.],
                        &vector![-1., 0., 0.],
                        *t,
                    ),
                    _ => panic!(),
                };
                temp.append(&mut clipped)
            }
            triangles_to_draw = temp
        }

        for tri in triangles_to_draw {
            draw_triangle(
                &mut canvas,
                &tri,
                Color::RGB(tri.col[0], tri.col[1], tri.col[2]),
            );
        }
        canvas.present();
    }
}
fn draw_triangle(canvas: &mut Canvas<Window>, tri: &Triangle, colour: Color) {
    canvas
        .filled_trigon(
            tri.points[(0, 0)] as i16,
            tri.points[(1, 0)] as i16,
            tri.points[(0, 1)] as i16,
            tri.points[(1, 1)] as i16,
            tri.points[(0, 2)] as i16,
            tri.points[(1, 2)] as i16,
            colour,
        )
        .unwrap();
    // canvas
    //     .aa_trigon(
    //         tri.points[(0, 0)] as i16,
    //         tri.points[(1, 0)] as i16,
    //         tri.points[(0, 1)] as i16,
    //         tri.points[(1, 1)] as i16,
    //         tri.points[(0, 2)] as i16,
    //         tri.points[(1, 2)] as i16,
    //         Color::RGB(0, 0, 0),
    //     )
    //     .unwrap()
}
