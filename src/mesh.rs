use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::triangle::Triangle;

pub struct Mesh {
    triangles: Vec<Triangle>,
}

pub struct IterMesh<'a> {
    inner: &'a Mesh,
    pos: usize,
}

impl Mesh {
    pub fn load(file: &str) -> Result<Mesh, std::io::Error> {
        let file = File::open(&file)?;

        let mut reader = BufReader::new(file);
        let mut line = String::new();

        let mut points = Vec::<[f32; 3]>::new();
        let mut triangles = Vec::<Triangle>::new();

        loop {
            match reader.read_line(&mut line) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        break;
                    };
                    let vec = line.trim().split_whitespace().collect::<Vec<&str>>();
                    if vec.len() > 0 {
                        match vec[0] {
                            "v" => points.push([
                                vec[1].parse().unwrap(),
                                vec[2].parse().unwrap(),
                                vec[3].parse().unwrap(),
                            ]),
                            "f" => triangles.push(Triangle::from_vercices(
                                points[vec[1].parse::<usize>().unwrap() - 1],
                                points[vec[2].parse::<usize>().unwrap() - 1],
                                points[vec[3].parse::<usize>().unwrap() - 1],
                            )),
                            _ => {}
                        }
                    }
                    line.clear();
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Ok(Mesh { triangles })
    }

    pub fn iter<'a>(&'a self) -> IterMesh<'a> {
        IterMesh {
            inner: self,
            pos: 0,
        }
    }

    pub fn unit_cube() -> Mesh {
        Mesh {
            triangles: vec![
                // South
                Triangle::from_points(0., 0., 0., 0., 1., 0., 1., 1., 0.),
                Triangle::from_points(0., 0., 0., 1., 1., 0., 1., 0., 0.),
                // East
                Triangle::from_points(1., 0., 0., 1., 1., 0., 1., 1., 1.),
                Triangle::from_points(1., 0., 0., 1., 1., 1., 1., 0., 1.),
                // North
                Triangle::from_points(1., 0., 1., 1., 1., 1., 0., 1., 1.),
                Triangle::from_points(1., 0., 1., 0., 1., 1., 0., 0., 1.),
                // West
                Triangle::from_points(0., 0., 1., 0., 1., 1., 0., 1., 0.),
                Triangle::from_points(0., 0., 1., 0., 1., 0., 0., 0., 0.),
                // Top
                Triangle::from_points(0., 1., 0., 0., 1., 1., 1., 1., 1.),
                Triangle::from_points(0., 1., 0., 1., 1., 1., 1., 1., 0.),
                // Bottom
                Triangle::from_points(1., 0., 1., 0., 0., 1., 0., 0., 0.),
                Triangle::from_points(1., 0., 1., 0., 0., 0., 1., 0., 0.),
            ],
        }
    }
}

impl<'a> Iterator for IterMesh<'a> {
    type Item = &'a Triangle;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.inner.triangles.len() {
            None
        } else {
            self.pos += 1;
            Some(&self.inner.triangles[self.pos - 1])
        }
    }
}
