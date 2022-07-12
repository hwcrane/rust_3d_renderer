mod mesh;
mod triangle;

use std::f32::consts::PI;

use mesh::Mesh;
use nalgebra as nl;
use sdl2::{
    event::Event, gfx::primitives::DrawRenderer, keyboard::Keycode, pixels::Color, render::Canvas,
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
    let f_fov_rad = 1. / ((f_fov / 2.).to_radians().tan());

    let proj_mat = nl::matrix![f_aspect_ratio * f_fov_rad, 0., 0., 0.;
                               0., f_fov_rad, 0., 0.;
                               0., 0., f_far / (f_far - f_near), (-f_far * f_near) / (f_far - f_near);
                               0., 0., -1., 0.;];

    // let pot = Mesh::load("cow.obj").unwrap();
    let pot = Mesh::load("teapot.obj").unwrap();
    // let pot = Mesh::load("VideoShip.obj").unwrap();
    // let cube = Mesh::unit_cube();
    let vcamera = nl::Vector3::new(0., 0., 0.);

    let mut x_rotation = 0.;
    let mut y_rotation = 0.;
    let mut z_offset = 8.;

    'mainloop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'mainloop,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    x_rotation -= 0.1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    x_rotation += 0.1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    y_rotation += 0.1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    y_rotation -= 0.1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Num0),
                    ..
                } => {
                    z_offset += 0.1;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => {
                    z_offset -= 0.1;
                }
                _ => (),
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let mut triangles_to_draw = Vec::<Triangle>::new();

        for tri in pot.iter() {
            let mut translated = tri.clone();
            translated.rotate(&nl::Rotation3::new(nl::Vector3::y() * y_rotation * 2.));
            translated.rotate(&nl::Rotation3::new(nl::Vector3::x() * x_rotation * 2.));
            translated.translate(&nl::matrix![0., 0., 0.; 0., 0., 0.; z_offset, z_offset,z_offset]);

            let normal = translated.normal();

            if normal.dot(&(translated.points.column(1) - vcamera)) < 0. {
                let mut light_direction = nl::Vector3::new(0., 0., -1.);
                light_direction = light_direction * 1. / light_direction.norm();

                let dp = normal.dot(&light_direction);
                let mut proj_tri = translated.project(&proj_mat);

                proj_tri.translate(&nl::matrix![1., 1., 1.;
                                                1., 1., 1.;
                                                0., 0., 0.;]);

                proj_tri.enlarge_x(0.5 * WINDOW_WIDTH as f32);
                proj_tri.enlarge_y(0.5 * WINDOW_HEIGHT as f32);
                proj_tri.col = [(255. * dp) as u8, (255. * dp) as u8, (255. * dp) as u8];

                triangles_to_draw.push(proj_tri);
            }
        }

        triangles_to_draw.sort_unstable_by(|a, b| a.mid[2].partial_cmp(&b.mid[2]).unwrap());

        for tri in triangles_to_draw {
            // println!("{}", tri.mid[2]);
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
    // canvas.set_draw_color(colour);

    canvas
        .aa_trigon(
            tri.points[(0, 0)] as i16,
            tri.points[(1, 0)] as i16,
            tri.points[(0, 1)] as i16,
            tri.points[(1, 1)] as i16,
            tri.points[(0, 2)] as i16,
            tri.points[(1, 2)] as i16,
            colour,
        )
        .unwrap()
}
