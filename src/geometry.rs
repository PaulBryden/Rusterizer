use std::f32::consts::PI;

/// A 3D vector object with a W component, normalized and set to 1.0 by default.
#[derive(Debug, Clone)]
pub struct Vec3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl Default for Vec3d {
    fn default() -> Vec3d {
        Vec3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
}

use crate::texture::Texture;

/// A 2D vector object with a W component, normalized and set to 1.0 by default.
#[derive(Debug, Clone)]
pub struct Vec2d {
    pub u: f32,
    pub v: f32,
    pub w: f32,
}
impl Default for Vec2d {
    fn default() -> Vec2d {
        Vec2d {
            u: 0.0,
            v: 0.0,
            w: 1.0,
        }
    }
}
/// A Triangle object holding 3 Vec3d coordinates and 3 Vec2d texture coordinates.
#[derive(Default, Debug, Clone)]
pub struct Triangle {
    pub points: [Vec3d; 3],
    pub texture_points: [Vec2d; 3],
}

/// A Mesh object holding a vector of Triangle objects and a reference to a texture.
#[derive(Clone)]
pub struct Mesh<'a> {
    pub tris: Vec<Triangle>,
    pub texture: &'a Texture,
}

/// An Animated Mesh object holding a vector of keyframe meshes and a reference to a texture.
#[derive(Clone)]
pub struct AnimatedMesh<'a> {
    pub meshes: Vec<Mesh<'a>>,
    pub texture: &'a Texture,
    pub current_frame: Mesh<'a>,
    pub frames_per_second: f32,
    pub running: bool,
    current_frame_numeric: f32,
}

impl AnimatedMesh<'_> {
    /// Creates a new `AnimatedMesh`.
    pub fn new<'a>(
        tex: &'a Texture,
        files: Vec<Vec<u8>>,
        fps: f32,
        running: bool,
    ) -> AnimatedMesh<'a> {
        let mut meshes: Vec<Mesh<'a>> = Vec::new();
        let running = running;
        files.into_iter().for_each(|i| {
            let mut mesh = Mesh {
                tris: Vec::new(),
                texture: tex,
            };
            mesh.load_from_object_file(&i, true);
            meshes.push(mesh.clone());
        });
        let current_frame = meshes.get(0).unwrap().clone();
        AnimatedMesh {
            meshes: meshes,
            texture: tex,
            current_frame: current_frame,
            frames_per_second: fps,
            running: running,
            current_frame_numeric: 0.0,
        }
    }

    /// Interpolates the 'current_frame' object based on the 'time_elapsed_seconds'.
    pub fn tick(&mut self, mut time_elapsed_seconds: f32) {
        if self.running {
            let interpolation_factor = self.frames_per_second * time_elapsed_seconds;
            let total_number_of_frames: usize = self.meshes.len();
            let mut new_frame_time = self.current_frame_numeric + interpolation_factor;
            while new_frame_time >= total_number_of_frames as f32 {
                new_frame_time = 0.0 + time_elapsed_seconds;
                time_elapsed_seconds -= new_frame_time;
            }
            let base_frame = new_frame_time.floor() as usize;
            let mut next_frame = base_frame + 1;
            if next_frame >= total_number_of_frames {
                next_frame = 0;
            }
            let interpolation_amount = new_frame_time - new_frame_time.floor();
            let mut current_frame_mesh: Mesh<'_> = self.meshes.get(base_frame).unwrap().clone();
            let mut next_frame_mesh = self.meshes.get(next_frame).unwrap().clone();

            for i in 0..current_frame_mesh.tris.len() {
                for j in 0..3 {
                    let mut x_diff = next_frame_mesh.tris.get_mut(i).unwrap().points[j].x
                        - current_frame_mesh.tris.get_mut(i).unwrap().points[j].x;
                    let mut y_diff = next_frame_mesh.tris.get_mut(i).unwrap().points[j].y
                        - current_frame_mesh.tris.get_mut(i).unwrap().points[j].y;
                    let mut z_diff = next_frame_mesh.tris.get_mut(i).unwrap().points[j].z
                        - current_frame_mesh.tris.get_mut(i).unwrap().points[j].z;

                    x_diff = x_diff * interpolation_amount;
                    y_diff = y_diff * interpolation_amount;
                    z_diff = z_diff * interpolation_amount;

                    current_frame_mesh.tris.get_mut(i).unwrap().points[j].x += x_diff;
                    current_frame_mesh.tris.get_mut(i).unwrap().points[j].y += y_diff;
                    current_frame_mesh.tris.get_mut(i).unwrap().points[j].z += z_diff;
                }
            }
            self.current_frame = current_frame_mesh;
            self.current_frame_numeric = new_frame_time;
        }
    }
}

/// A 4x4 matrix object.
#[derive(Default, Debug, Clone)]
pub struct Mat4x4 {
    pub m: [[f32; 4]; 4],
}

impl Mesh<'_> {
    /// Creates a new 'Mesh' Object
    pub fn new<'a>(tex: &'a Texture, file: &'a [u8]) -> Mesh<'a> {
        let mut mesh = Mesh {
            tris: Vec::new(),
            texture: tex,
        };
        mesh.load_from_object_file(file, true);
        mesh
    }

    /// Reads the obj data into a Vector containing lines.
    fn read_lines_from_file(file: &[u8]) -> Vec<String> {
        let mut result = Vec::new();
        for line in std::io::read_to_string(file).unwrap().lines() {
            result.push(line.to_string())
        }
        result
    }

    /// Populate the mesh object with vertices and texture coordinates from the obj file.
    pub fn load_from_object_file(&mut self, file: &[u8], b_has_texture: bool) {
        let mut verts: Vec<Vec3d> = Vec::new();
        let mut texs: Vec<Vec2d> = Vec::new();
        let lines = Self::read_lines_from_file(file);
        for line in lines {
            if line.starts_with("v ") {
                let mut parts = line.split(' ');
                let mut vert: Vec3d = Vec3d::default();
                parts.next();
                vert.x = parts.next().unwrap().parse::<f32>().unwrap();
                vert.y = parts.next().unwrap().parse::<f32>().unwrap();
                vert.z = parts.next().unwrap().parse::<f32>().unwrap();
                verts.push(vert);
            }
            if line.starts_with("vt") {
                let mut parts = line.split(' ');
                let mut vert: Vec2d = Vec2d::default();
                parts.next();
                vert.u = parts.next().unwrap().parse::<f32>().unwrap();
                vert.v = parts.next().unwrap().parse::<f32>().unwrap();
                texs.push(vert);
            }
            if !b_has_texture {
                if line.starts_with("f ") {
                    let mut parts = line.split(' ');
                    let mut f: [usize; 3] = [0, 0, 0];
                    parts.next();
                    f[0] = parts.next().unwrap().parse::<usize>().unwrap();
                    f[1] = parts.next().unwrap().parse::<usize>().unwrap();
                    f[2] = parts.next().unwrap().parse::<usize>().unwrap();
                    self.tris.push(Triangle {
                        points: [
                            verts[f[0] - 1].clone(),
                            verts[f[1] - 1].clone(),
                            verts[f[2] - 1].clone(),
                        ],
                        ..Default::default()
                    })
                }
            } else if line.starts_with("f ") {
                let mut parts = line.split(' ');
                let mut f: [usize; 3] = [0, 0, 0];
                let mut textoks: [usize; 3] = [0, 0, 0];
                parts.next();
                let mut counter: usize = 0;
                for i in parts {
                    let mut vec_comps = i.split('/');
                    f[counter] = vec_comps.next().unwrap().parse::<usize>().unwrap();
                    textoks[counter] = vec_comps.next().unwrap().parse::<usize>().unwrap();
                    counter += 1;
                }
                self.tris.push(Triangle {
                    points: [
                        verts[f[0] - 1].clone(),
                        verts[f[1] - 1].clone(),
                        verts[f[2] - 1].clone(),
                    ],
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

/// Multiply a 'Mat4x4' by a 'Vec3d'.
pub fn matrix_multiply_vector(m: &Mat4x4, i: &Vec3d) -> Vec3d {
    let mut v: Vec3d = Vec3d::default();
    v.x = i.x * m.m[0][0] + i.y * m.m[1][0] + i.z * m.m[2][0] + i.w * m.m[3][0];
    v.y = i.x * m.m[0][1] + i.y * m.m[1][1] + i.z * m.m[2][1] + i.w * m.m[3][1];
    v.z = i.x * m.m[0][2] + i.y * m.m[1][2] + i.z * m.m[2][2] + i.w * m.m[3][2];
    v.w = i.x * m.m[0][3] + i.y * m.m[1][3] + i.z * m.m[2][3] + i.w * m.m[3][3];
    v
}

/// Return a 'Mat4x4' identity matrix.
pub fn matrix_make_identity() -> Mat4x4 {
    let mut matrix: Mat4x4 = Mat4x4::default();
    matrix.m[0][0] = 1.0;
    matrix.m[1][1] = 1.0;
    matrix.m[2][2] = 1.0;
    matrix.m[3][3] = 1.0;
    matrix
}

/// Return a 'Mat4x4' x rotation matrix based on input angle 'f_angle_rad'.
pub fn matrix_make_rotation_x(f_angle_rad: &f32) -> Mat4x4 {
    let mut matrix: Mat4x4 = Mat4x4::default();
    matrix.m[0][0] = 1.0;
    matrix.m[1][1] = f_angle_rad.cos();
    matrix.m[1][2] = f_angle_rad.sin();
    matrix.m[2][1] = -(f_angle_rad.sin());
    matrix.m[2][2] = f_angle_rad.cos();
    matrix.m[3][3] = 1.0;
    matrix
}

/// Return a 'Mat4x4' y rotation matrix based on input angle 'f_angle_rad'.
pub fn matrix_make_rotation_y(f_angle_rad: &f32) -> Mat4x4 {
    let mut matrix: Mat4x4 = Mat4x4::default();
    matrix.m[0][0] = f_angle_rad.cos();
    matrix.m[0][2] = f_angle_rad.sin();
    matrix.m[2][0] = -(f_angle_rad.sin());
    matrix.m[1][1] = 1.0;
    matrix.m[2][2] = f_angle_rad.cos();
    matrix.m[3][3] = 1.0;
    matrix
}

/// Return a 'Mat4x4' z rotation matrix based on input angle 'f_angle_rad'.
pub fn matrix_make_rotation_z(f_angle_rad: &f32) -> Mat4x4 {
    let mut matrix: Mat4x4 = Mat4x4::default();
    matrix.m[0][0] = f_angle_rad.cos();
    matrix.m[0][1] = f_angle_rad.sin();
    matrix.m[1][0] = -(f_angle_rad.sin());
    matrix.m[1][1] = f_angle_rad.cos();
    matrix.m[2][2] = 1.0;
    matrix.m[3][3] = 1.0;
    matrix
}

/// Return a 'Mat4x4' translation matrix based on inputs 'x', 'y' and 'z'.
pub fn matrix_make_translation(x: f32, y: f32, z: f32) -> Mat4x4 {
    let mut matrix: Mat4x4 = matrix_make_identity();
    matrix.m[3][0] = x;
    matrix.m[3][1] = y;
    matrix.m[3][2] = z;
    matrix
}

/// Return a 'Mat4x4' projection matrix based on the viewport parameters.
pub fn matrix_make_projection(
    f_fov_degrees: f32,
    f_aspect_ratio: f32,
    f_near: f32,
    f_far: f32,
) -> Mat4x4 {
    let f_fov_rad: f32 = 1.0 / (f_fov_degrees * 0.5 / 180.0 * PI).tan();
    let mut matrix: Mat4x4 = Mat4x4::default();
    matrix.m[0][0] = f_aspect_ratio * f_fov_rad;
    matrix.m[1][1] = f_fov_rad;
    matrix.m[2][2] = f_far / (f_far - f_near);
    matrix.m[3][2] = (-f_far * f_near) / (f_far - f_near);
    matrix.m[2][3] = 1.0;
    matrix.m[3][3] = 0.0;
    matrix
}

/// Multiply a 'Mat4x4' matrix by a 'Mat4x4' matrix.
pub fn matrix_multiply_matrix(m1: &Mat4x4, m2: &Mat4x4) -> Mat4x4 {
    let mut matrix: Mat4x4 = Mat4x4::default();
    for c in 0..matrix.m.len() {
        for r in 0..matrix.m[c].len() {
            matrix.m[r][c] = m1.m[r][0] * m2.m[0][c]
                + m1.m[r][1] * m2.m[1][c]
                + m1.m[r][2] * m2.m[2][c]
                + m1.m[r][3] * m2.m[3][c];
        }
    }
    matrix
}

/// Return an aptly named point_at matrix for translating between a position and a target.
pub fn matrix_point_at(pos: &Vec3d, target: &Vec3d, up: &Vec3d) -> Mat4x4 {
    let mut new_forward: Vec3d = vector_sub(target, pos);
    new_forward = vector_normalize(&new_forward);

    let a: Vec3d = vector_mul(&new_forward, vector_dot_product(up, &new_forward));
    let mut new_up: Vec3d = vector_sub(up, &a);
    new_up = vector_normalize(&new_up);

    let new_right: Vec3d = vector_cross_product(&new_up, &new_forward);

    let mut matrix: Mat4x4 = Mat4x4::default();
    matrix.m[0][0] = new_right.x;
    matrix.m[0][1] = new_right.y;
    matrix.m[0][2] = new_right.z;
    matrix.m[0][3] = 0.0;
    matrix.m[1][0] = new_up.x;
    matrix.m[1][1] = new_up.y;
    matrix.m[1][2] = new_up.z;
    matrix.m[1][3] = 0.0;
    matrix.m[2][0] = new_forward.x;
    matrix.m[2][1] = new_forward.y;
    matrix.m[2][2] = new_forward.z;
    matrix.m[2][3] = 0.0;
    matrix.m[3][0] = pos.x;
    matrix.m[3][1] = pos.y;
    matrix.m[3][2] = pos.z;
    matrix.m[3][3] = 1.0;
    matrix
}

/// Quickly invert a 'Mat4x4' matrix.
pub fn matrix_quick_inverse(m: &Mat4x4) -> Mat4x4 {
    let mut matrix: Mat4x4 = Mat4x4::default();
    matrix.m[0][0] = m.m[0][0];
    matrix.m[0][1] = m.m[1][0];
    matrix.m[0][2] = m.m[2][0];
    matrix.m[0][3] = 0.0;
    matrix.m[1][0] = m.m[0][1];
    matrix.m[1][1] = m.m[1][1];
    matrix.m[1][2] = m.m[2][1];
    matrix.m[1][3] = 0.0;
    matrix.m[2][0] = m.m[0][2];
    matrix.m[2][1] = m.m[1][2];
    matrix.m[2][2] = m.m[2][2];
    matrix.m[2][3] = 0.0;
    matrix.m[3][0] =
        -(m.m[3][0] * matrix.m[0][0] + m.m[3][1] * matrix.m[1][0] + m.m[3][2] * matrix.m[2][0]);
    matrix.m[3][1] =
        -(m.m[3][0] * matrix.m[0][1] + m.m[3][1] * matrix.m[1][1] + m.m[3][2] * matrix.m[2][1]);
    matrix.m[3][2] =
        -(m.m[3][0] * matrix.m[0][2] + m.m[3][1] * matrix.m[1][2] + m.m[3][2] * matrix.m[2][2]);
    matrix.m[3][3] = 1.0;
    matrix
}

/// Add a 'Vec3d' to a 'Vec3d'.
pub fn vector_add(v1: &Vec3d, v2: &Vec3d) -> Vec3d {
    let mut v_result = Vec3d::default();
    v_result.x = v1.x + v2.x;
    v_result.y = v1.y + v2.y;
    v_result.z = v1.z + v2.z;
    v_result
}

/// Subtract a 'Vec3d' from a 'Vec3d'.
pub fn vector_sub(v1: &Vec3d, v2: &Vec3d) -> Vec3d {
    let mut v_result = Vec3d::default();
    v_result.x = v1.x - v2.x;
    v_result.y = v1.y - v2.y;
    v_result.z = v1.z - v2.z;
    v_result
}

/// Multiply a 'Vec3d' by a constant 'k'
pub fn vector_mul(v1: &Vec3d, k: f32) -> Vec3d {
    let mut v_result = Vec3d::default();
    v_result.x = v1.x * k;
    v_result.y = v1.y * k;
    v_result.z = v1.z * k;
    v_result
}

/// Divide a 'Vec3d' by a constant 'k'
pub fn vector_div(v1: &Vec3d, k: f32) -> Vec3d {
    let mut v_result = Vec3d::default();
    v_result.x = v1.x / k;
    v_result.y = v1.y / k;
    v_result.z = v1.z / k;
    v_result
}

/// Calculate the dot product of 2 'Vec3d' objects.
pub fn vector_dot_product(v1: &Vec3d, v2: &Vec3d) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

/// Calculate the length of a 'Vec3d' object.
pub fn vector_length(v1: &Vec3d) -> f32 {
    vector_dot_product(v1, v1).sqrt()
}

/// Normalize a 'Vec3d' object.
pub fn vector_normalize(v: &Vec3d) -> Vec3d {
    let l: f32 = vector_length(v);
    Vec3d {
        x: v.x / l,
        y: v.y / l,
        z: v.z / l,
        ..Default::default()
    }
}

/// Calculate the cross product of 2 'Vec3d' objects
pub fn vector_cross_product(v1: &Vec3d, v2: &Vec3d) -> Vec3d {
    let mut v: Vec3d = Vec3d::default();
    v.x = v1.y * v2.z - v1.z * v2.y;
    v.y = v1.z * v2.x - v1.x * v2.z;
    v.z = v1.x * v2.y - v1.y * v2.x;

    v
}

/// Calculate the distance between 2 'Vec3d' objects
pub fn dist(p: &Vec3d, plane_n: &Vec3d, plane_p: &Vec3d) -> f32 {
    let _n: Vec3d = vector_normalize(p);
    plane_n.x * p.x + plane_n.y * p.y + plane_n.z * p.z - vector_dot_product(plane_n, plane_p)
}

/// Clip a 'Triangle' object against a plane 'Vec3d' object. This function returns up to 2 new 'Triangle'
/// objects based on the result of the clipping operation.
pub fn triangle_clip_against_plane(
    plane_p: &Vec3d,
    plane_n_input: &Vec3d,
    in_tri: &Triangle,
    out_tri1: &mut Triangle,
    out_tri2: &mut Triangle,
) -> u64 {
    let mut plane_n: Vec3d = vector_normalize(plane_n_input);

    let mut inside_points: [Vec3d; 3] = [Vec3d::default(), Vec3d::default(), Vec3d::default()];
    let mut n_inside_point_count: i32 = 0;

    let mut outside_points: [Vec3d; 3] = [Vec3d::default(), Vec3d::default(), Vec3d::default()];
    let mut n_outside_point_count: i32 = 0;

    let mut inside_tex: [Vec2d; 3] = [Vec2d::default(), Vec2d::default(), Vec2d::default()];
    let mut n_inside_tex_count: i32 = 0;

    let mut outside_tex: [Vec2d; 3] = [Vec2d::default(), Vec2d::default(), Vec2d::default()];
    let mut n_outside_tex_count: i32 = 0;

    let d0: f32 = dist(&in_tri.points[0], &plane_n, plane_p);
    let d1: f32 = dist(&in_tri.points[1], &plane_n, plane_p);
    let d2: f32 = dist(&in_tri.points[2], &plane_n, plane_p);

    if d0 >= 0.0 {
        inside_points[n_inside_point_count as usize] = in_tri.points[0].clone();
        n_inside_point_count += 1;
        inside_tex[n_inside_tex_count as usize] = in_tri.texture_points[0].clone();
        n_inside_tex_count += 1;
    } else {
        outside_points[n_outside_point_count as usize] = in_tri.points[0].clone();
        n_outside_point_count += 1;
        outside_tex[n_outside_tex_count as usize] = in_tri.texture_points[0].clone();
        n_outside_tex_count += 1;
    }
    if d1 >= 0.0 {
        inside_points[n_inside_point_count as usize] = in_tri.points[1].clone();
        n_inside_point_count += 1;
        inside_tex[n_inside_tex_count as usize] = in_tri.texture_points[1].clone();
        n_inside_tex_count += 1;
    } else {
        outside_points[n_outside_point_count as usize] = in_tri.points[1].clone();
        n_outside_point_count += 1;
        outside_tex[n_outside_tex_count as usize] = in_tri.texture_points[1].clone();
        n_outside_tex_count += 1;
    }
    if d2 >= 0.0 {
        inside_points[n_inside_point_count as usize] = in_tri.points[2].clone();
        n_inside_point_count += 1;
        inside_tex[n_inside_tex_count as usize] = in_tri.texture_points[2].clone();
    } else {
        outside_points[n_outside_point_count as usize] = in_tri.points[2].clone();
        n_outside_point_count += 1;
        outside_tex[n_outside_tex_count as usize] = in_tri.texture_points[2].clone();
    }

    if n_inside_point_count == 0 {
        return 0;
    }

    if n_inside_point_count == 3 {
        *out_tri1 = in_tri.clone();

        return 1;
    }

    if n_inside_point_count == 1 && n_outside_point_count == 2 {
        out_tri1.points[0] = inside_points[0].clone();
        out_tri1.texture_points[0] = inside_tex[0].clone();

        let mut t: f32 = 0.0;
        out_tri1.points[1] = vector_intersect_plane(
            plane_p,
            &mut plane_n,
            inside_points[0].clone(),
            outside_points[0].clone(),
            &mut t,
        );
        out_tri1.texture_points[1].u = t * (outside_tex[0].u - inside_tex[0].u) + inside_tex[0].u;
        out_tri1.texture_points[1].v = t * (outside_tex[0].v - inside_tex[0].v) + inside_tex[0].v;
        out_tri1.texture_points[1].w = t * (outside_tex[0].w - inside_tex[0].w) + inside_tex[0].w;

        out_tri1.points[2] = vector_intersect_plane(
            plane_p,
            &mut plane_n,
            inside_points[0].clone(),
            outside_points[1].clone(),
            &mut t,
        );
        out_tri1.texture_points[2].u = t * (outside_tex[1].u - inside_tex[0].u) + inside_tex[0].u;
        out_tri1.texture_points[2].v = t * (outside_tex[1].v - inside_tex[0].v) + inside_tex[0].v;
        out_tri1.texture_points[2].w = t * (outside_tex[1].w - inside_tex[0].w) + inside_tex[0].w;

        return 1;
    }

    if n_inside_point_count == 2 && n_outside_point_count == 1 {
        out_tri1.points[0] = inside_points[0].clone();
        out_tri1.points[1] = inside_points[1].clone();
        out_tri1.texture_points[0] = inside_tex[0].clone();
        out_tri1.texture_points[1] = inside_tex[1].clone();

        let mut t: f32 = 0.0;
        out_tri1.points[2] = vector_intersect_plane(
            plane_p,
            &mut plane_n,
            inside_points[0].clone(),
            outside_points[0].clone(),
            &mut t,
        );
        out_tri1.texture_points[2].u = t * (outside_tex[0].u - inside_tex[0].u) + inside_tex[0].u;
        out_tri1.texture_points[2].v = t * (outside_tex[0].v - inside_tex[0].v) + inside_tex[0].v;
        out_tri1.texture_points[2].w = t * (outside_tex[0].w - inside_tex[0].w) + inside_tex[0].w;

        out_tri2.points[0] = inside_points[1].clone();
        out_tri2.texture_points[0] = inside_tex[1].clone();
        out_tri2.points[1] = out_tri1.points[2].clone();
        out_tri2.texture_points[1] = out_tri1.texture_points[2].clone();
        out_tri2.points[2] = vector_intersect_plane(
            plane_p,
            &mut plane_n,
            inside_points[1].clone(),
            outside_points[0].clone(),
            &mut t,
        );
        out_tri2.texture_points[2].u = t * (outside_tex[0].u - inside_tex[1].u) + inside_tex[1].u;
        out_tri2.texture_points[2].v = t * (outside_tex[0].v - inside_tex[1].v) + inside_tex[1].v;
        out_tri2.texture_points[2].w = t * (outside_tex[0].w - inside_tex[1].w) + inside_tex[1].w;

        2
    } else {
        0
    }
}

/// Get the 'Vec3d' at point of intersection. Also return 't' for calculating the new texture coordinates.
pub fn vector_intersect_plane(
    plane_p: &Vec3d,
    plane_n_input: &Vec3d,
    line_start: Vec3d,
    line_end: Vec3d,
    t: &mut f32,
) -> Vec3d {
    let plane_n = vector_normalize(plane_n_input);
    let plane_d: f32 = -vector_dot_product(&plane_n, plane_p);
    let ad: f32 = vector_dot_product(&line_start, &plane_n);
    let bd: f32 = vector_dot_product(&line_end, &plane_n);
    *t = (-plane_d - ad) / (bd - ad);
    let line_start_to_end: Vec3d = vector_sub(&line_end, &line_start);
    let line_to_intersect: Vec3d = vector_mul(&line_start_to_end, *t);
    vector_add(&line_start, &line_to_intersect)
}
