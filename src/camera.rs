use nalgebra as nl;
use nl::{matrix, vector, Matrix4, Point3, Rotation3, Vector3};

use crate::triangle::Triangle;

pub struct Camera {
    window_width: u32,
    window_height: u32,
    pub position: Vector3<f32>,
    looking_at: Vector3<f32>,
    projection_matrix: Matrix4<f32>,
}

impl Camera {
    pub fn new(
        window_width: u32,
        window_height: u32,
        f_near: f32,
        f_far: f32,
        f_fov: f32,
        f_aspect_ratio: f32,
    ) -> Camera {
        let f_fov_rad = 1. / ((f_fov / 2.).to_radians().tan());

        let proj_mat = matrix![f_aspect_ratio * f_fov_rad, 0., 0., 0.;
                                   0., f_fov_rad, 0., 0.;
                                   0., 0., f_far / (f_far - f_near), (-f_far * f_near) / (f_far - f_near);
                                   0., 0., 1., 0.;];
        Camera {
            window_width,
            window_height,
            position: vector![0., 0., -10.],
            looking_at: vector![0., 0., 1.],
            projection_matrix: proj_mat,
        }
    }

    pub fn move_looking_at(&mut self, x_am: f32, y_am: f32) {
        // let x = Rotation3::from_axis_angle(&Vector3::x_axis(), x_am);
        let y = Rotation3::from_axis_angle(&Vector3::y_axis(), y_am);
        self.looking_at = y * self.looking_at;
        self.looking_at += vector![0., -x_am, 0.];
        self.looking_at = self.looking_at.normalize();
    }

    pub fn move_forward(&mut self, amount: f32) {
        self.position += &self.looking_at * amount
    }

    pub fn move_up(&mut self, amount: f32) {
        self.position[1] += amount
    }

    pub fn move_left(&mut self, amount: f32) {
        let left = self.looking_at.cross(&Vector3::y_axis());
        self.position += left * amount;
    }

    pub fn project_triangle(&self, triangle: &Triangle) -> Vec<Triangle> {
        let camera_mat = Matrix4::look_at_rh(
            &Point3::from(self.position),
            &Point3::from(self.position + self.looking_at),
            &vector![0., 1., 0.],
        );

        let mat_view = camera_mat;

        let viewed = Triangle::new(mat_view * triangle.points);

        let clipped_triangles =
            self.clip_triangle(&vector![0., 0., -0.1], &vector![0., 0., -1.], viewed);

        clipped_triangles.iter().map(|clipped| {
            let projected = self.projection_matrix * clipped.points;

            let z1 = if projected[(3, 0)] == 0. {
                1.
            } else {
                projected[(3, 0)]
            };
            let z2 = if projected[(3, 1)] == 0. {
                1.
            } else {
                projected[(3, 1)]
            };
            let z3 = if projected[(3, 2)] == 0. {
                1.
            } else {
                projected[(3, 2)]
            };

            let mut tri = Triangle::new(matrix![
                          projected[(0, 0)] / z1, projected[(0, 1)] / z2, projected[(0, 2)] / z3;
                          projected[(1, 0)] / z1, projected[(1, 1)] / z2, projected[(1, 2)] / z3;
                          projected[(2, 0)] / z1, projected[(2, 1)] / z2, projected[(2, 2)] / z3;
                          1., 1., 1.
            ]);

            tri.translate(1., 1., 0.);

            tri.points[(0, 0)] *= 0.5 * self.window_width as f32;
            tri.points[(0, 1)] *= 0.5 * self.window_width as f32;
            tri.points[(0, 2)] *= 0.5 * self.window_width as f32;

            tri.points[(1, 0)] *= 0.5 * self.window_height as f32;
            tri.points[(1, 1)] *= 0.5 * self.window_height as f32;
            tri.points[(1, 2)] *= 0.5 * self.window_height as f32;

            tri
        })
        .collect()
    }

    pub fn is_triangle_visible(&self, triangle: &Triangle) -> bool {
        let line = vector![
            triangle.points[(0, 0)],
            triangle.points[(1, 0)],
            triangle.points[(2, 0)]
        ];
        let normal = triangle.normal();
        normal.dot(&(line - self.position)) < 0.
    }

    fn vector_plane_intersection(
        &self,
        plane_point: &Vector3<f32>,
        plane_normal: &Vector3<f32>,
        line_start: &Vector3<f32>,
        line_end: &Vector3<f32>,
    ) -> Vector3<f32> {
        let plane_d = -plane_normal.dot(&plane_point);
        let ad = line_start.dot(&plane_normal);
        let bd = line_end.dot(&plane_normal);
        let t = (-plane_d - ad) / (bd - ad);
        let line_start_to_end = line_end - line_start;
        let line_to_intersect = line_start_to_end * t;
        line_start + line_to_intersect
    }
    fn distance_point_plane(
        &self,
        point: &Vector3<f32>,
        plane_normal: &Vector3<f32>,
        plane_point: &Vector3<f32>,
    ) -> f32 {
        plane_normal.dot(&point) - plane_normal.dot(&plane_point)
    }

    pub fn clip_triangle(
        &self,
        plane_point: &Vector3<f32>,
        plane_normal: &Vector3<f32>,
        triangle: Triangle,
    ) -> Vec<Triangle> {
        let mut inside_points: Vec<Vector3<f32>> = Vec::new();
        let mut outside_points: Vec<Vector3<f32>> = Vec::new();

        let d0 = self.distance_point_plane(&triangle.point3(0), &plane_normal, &plane_point);
        let d1 = self.distance_point_plane(&triangle.point3(1), &plane_normal, &plane_point);
        let d2 = self.distance_point_plane(&triangle.point3(2), &plane_normal, &plane_point);

        if d0 >= 0. {
            inside_points.push(triangle.point3(0))
        } else {
            outside_points.push(triangle.point3(0))
        }
        if d1 >= 0. {
            inside_points.push(triangle.point3(1))
        } else {
            outside_points.push(triangle.point3(1))
        }
        if d2 >= 0. {
            inside_points.push(triangle.point3(2))
        } else {
            outside_points.push(triangle.point3(2))
        }

        if inside_points.len() == 3 {
            vec![triangle]
        } else if inside_points.len() == 1 && outside_points.len() == 2 {
            let p0 = inside_points[0];
            let p1 =
                self.vector_plane_intersection(plane_point, plane_normal, &p0, &outside_points[0]);
            let p2 =
                self.vector_plane_intersection(plane_point, plane_normal, &p0, &outside_points[1]);
            let mut new_triangle = Triangle::from_vectors(&p0, &p1, &p2);
            new_triangle.col = triangle.col;
            vec![new_triangle]
        } else if inside_points.len() == 2 && outside_points.len() == 1 {
            // first triangle
            let f0 = inside_points[0];
            let f1 = inside_points[1];
            let f2 =
                self.vector_plane_intersection(plane_point, plane_normal, &f0, &outside_points[0]);

            // second triangle
            let s0 = inside_points[1];
            let s1 = f2;
            let s2 =
                self.vector_plane_intersection(plane_point, plane_normal, &f1, &outside_points[0]);

            let mut tri1 = Triangle::from_vectors(&f0, &f1, &f2);
            let mut tri2 = Triangle::from_vectors(&s0, &s1, &s2);

            tri1.col = triangle.col;
            tri2.col = triangle.col;

            vec![tri1, tri2]
        } else {
            vec![]
        }
    }
}
