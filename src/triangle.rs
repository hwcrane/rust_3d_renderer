// use crate::vec3::Vec3;
use nalgebra as nl;
// use std::ops::{Index, IndexMut};
#[derive(Clone, Copy)]
pub struct Triangle {
    pub points: nl::Matrix3<f32>,
    pub mid: nl::Vector3<f32>,
    pub col: [u8; 3],
}

impl Triangle {
    pub fn new(points: nl::Matrix3<f32>) -> Triangle {
        Triangle {
            points,
            mid: points.column_mean(),
            col: [255, 255, 255],
        }
    }

    pub fn from_vercices(v1: [f32; 3], v2: [f32; 3], v3: [f32; 3]) -> Triangle {
        let points = nl::matrix![v1[0], v2[0], v3[0];
                                v1[1], v2[1], v3[1];
                                v1[2], v2[2], v3[2]];
        Triangle {
            points,
            mid: points.column_mean(),
            col: [255, 255, 255],
        }
    }

    pub fn from_points(
        p1: f32,
        p2: f32,
        p3: f32,
        p4: f32,
        p5: f32,
        p6: f32,
        p7: f32,
        p8: f32,
        p9: f32,
    ) -> Triangle {
        let points = nl::matrix! [p1, p4, p7;
                                 p2, p5, p8;
                                 p3, p6, p9];
        Triangle {
            points,
            mid: points.column_mean(),
            col: [255, 255, 255],
        }
    }

    pub fn translate(&mut self, translation: &nl::Matrix3<f32>) {
        self.points += translation
    }

    pub fn transform(&mut self, transformation: &nl::Matrix3<f32>) {
        self.points = transformation * self.points
    }
    pub fn rotate(&mut self, transformation: &nl::Rotation3<f32>) {
        self.points = transformation * self.points
    }

    pub fn enlarge_x(&mut self, scale_factor: f32) {
        self.points[(0, 0)] *= scale_factor;
        self.points[(0, 1)] *= scale_factor;
        self.points[(0, 2)] *= scale_factor;
    }

    pub fn enlarge_y(&mut self, scale_factor: f32) {
        self.points[(1, 0)] *= scale_factor;
        self.points[(1, 1)] *= scale_factor;
        self.points[(1, 2)] *= scale_factor;
    }

    pub fn normal(&self) -> nl::Vector3<f32> {
        let line1 = self.points.column(1) - self.points.column(0);
        let line2 = self.points.column(2) - self.points.column(0);
        let nor = line1.cross(&line2);
        nor * 1. / nor.norm()
    }

    pub fn project(&self, project_matrix: &nl::Matrix4<f32>) -> Triangle {
        let temp_tri = nl::matrix![ self.points[(0, 0)], self.points[(0, 1)], self.points[(0, 2)];
                                    self.points[(1, 0)], self.points[(1, 1)], self.points[(1, 2)];
                                    self.points[(2, 0)], self.points[(2, 1)], self.points[(2, 2)];
                                    1.,             1.,             1.];

        let projected = project_matrix * temp_tri;

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

        Triangle::new(nl::matrix![
                      projected[(0, 0)] / z1, projected[(0, 1)] / z2, projected[(0, 2)] / z3;
                      projected[(1, 0)] / z1, projected[(1, 1)] / z2, projected[(1, 2)] / z3;
                      projected[(2, 0)] / z1, projected[(2, 1)] / z2, projected[(2, 2)] / z3;
        ])
    }
}
