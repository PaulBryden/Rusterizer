use std::{f32::consts::PI, fs::read_to_string};

#[derive(Default, Debug, Clone)]
struct Vec3d
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

use crate::{texture::Texture};

#[derive(Default, Debug, Clone)]
struct Vec2d
{
    pub u: f32,
    pub v: f32,
    pub w: f32
}
#[derive(Default, Debug, Clone)]
struct Triangle
{
    pub points: [Vec3d; 3],
    pub texture_points: [Vec2d; 3]
}

#[derive(Clone)]
struct Mesh<'a>
{
    pub tris: Vec<Triangle>,
    pub texture: &'a Texture
}

#[derive(Default, Debug, Clone)]
struct Mat4x4
{
    pub m: [[f32; 4]; 4],
}


impl Mesh<'_> {
    fn read_lines(filename: &str) -> Vec<String> {
        let mut result = Vec::new();
        for line in read_to_string(filename).unwrap().lines() {
            result.push(line.to_string())
        }
        result
    }

    pub fn LoadFromObjectFile(&mut self, filename: &str, bHasTexture: bool) {
        let mut verts: Vec<Vec3d> = Vec::new();
        let mut texs: Vec<Vec2d> = Vec::new();
        let lines = Self::read_lines(filename);
        for line in lines {
            if (line.starts_with("v ")) {
                let mut parts = line.split(" ");
                let mut vert: Vec3d = Vec3d::default();
                parts.next();
                vert.x = parts.next().unwrap().parse::<f32>().unwrap();
                vert.y = parts.next().unwrap().parse::<f32>().unwrap();
                vert.z = parts.next().unwrap().parse::<f32>().unwrap();
                verts.push(vert);
            }
            if (line.starts_with("vt")) {
                let mut parts = line.split(" ");
                let mut vert: Vec2d = Vec2d::default();
                parts.next();
                vert.u = parts.next().unwrap().parse::<f32>().unwrap();
                vert.v = parts.next().unwrap().parse::<f32>().unwrap();
                texs.push(vert);
            }
            if (!bHasTexture) {
                if (line.starts_with("f ")) {
                    let mut parts = line.split(" ");
                    let mut f: [usize; 3] = [0, 0, 0];
                    parts.next();
                    f[0] = parts.next().unwrap().parse::<usize>().unwrap();
                    f[1] = parts.next().unwrap().parse::<usize>().unwrap();
                    f[2] = parts.next().unwrap().parse::<usize>().unwrap();
                    self.tris.push(Triangle {
                        points: [verts[f[0] - 1].clone(), verts[f[1] - 1].clone(), verts[f[2] - 1].clone()],
                        ..Default::default()
                    })
                }
            } else {
                if (line.starts_with("f ")) {
                    let mut parts = line.split(" ");
                    let mut f: [usize; 3] = [0, 0, 0];
                    let mut textoks: [usize; 3] = [0, 0, 0];
                    parts.next();
                    let mut counter: usize = 0;
                    for i in parts {
                        let mut vec_comps = i.split("/");
                        f[counter] = vec_comps.next().unwrap().parse::<usize>().unwrap();
                        textoks[counter] = vec_comps.next().unwrap().parse::<usize>().unwrap();
                        counter += 1;
                    }
                    self.tris.push(Triangle {
                        points: [verts[f[0] - 1].clone(), verts[f[1] - 1].clone(), verts[f[2] - 1].clone()],
                        texture_points: [
                            texs[textoks[0] - 1].clone(),
                            texs[textoks[1] - 1].clone(),
                            texs[textoks[2] - 1].clone(),
                        ],
                        ..Default::default()
                    })
                }
            }
        }
    }
}