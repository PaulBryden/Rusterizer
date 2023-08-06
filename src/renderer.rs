use std::collections::VecDeque;

use crate::{
    draw::draw_textured_triangle,
    framebuffer::Framebuffer,
    geometry::{
        matrix_make_projection, matrix_make_rotation_x, matrix_make_rotation_y,
        matrix_make_rotation_z, matrix_make_translation, matrix_multiply_matrix,
        matrix_multiply_vector, matrix_point_at, matrix_quick_inverse, triangle_clip_against_plane,
        vector_add, vector_cross_product, vector_div, vector_dot_product, vector_mul, vector_sub,
        Mat4x4, Mesh, Triangle, Vec3d, AnimatedMesh,
    },
};

/// A Renderer object. This is the object responsible for performing the geometric calculations
/// to output a 2d camera perspective of the 3D environment passed in via the mesh lists.
pub struct Renderer<'a> {
    meshes: Vec<Mesh<'a>>,
    animated_meshes: Vec<AnimatedMesh<'a>>,
    view_width: usize,
    view_height: usize,
    framebuffer_clear_color: u32,
    mat_projection: Mat4x4,
    vec_camera: Vec3d,
    vec_target: Vec3d,
    vec_look_dir: Vec3d,
    vec_up: Vec3d,
    yaw: f32,
    depth_buffer: Vec<f32>,
}

impl Renderer<'_> {
    
    /// Creates a new `Renderer`.
    pub fn new<'a>(
        meshes: Vec<Mesh<'a>>,
        animated_meshes: Vec<AnimatedMesh<'a>>,
        view_width: usize,
        view_height: usize,
        framebuffer_clear_color: u32,
    ) -> Renderer<'a> {
        //For first draft lets make some defaults for the projection matrix.
        let mat_projection =
            matrix_make_projection(90.0, view_height as f32 / view_width as f32, 0.1, 1000.0);
        let vec_camera: Vec3d = Vec3d::default();
        let vec_target: Vec3d = Vec3d {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            ..Default::default()
        };
        let vec_look_dir: Vec3d = Vec3d {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            ..Default::default()
        };
        let vec_up: Vec3d = Vec3d {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            ..Default::default()
        };
        let yaw: f32 = 0.0;
        let depth_buffer: Vec<f32> = vec![0.0; view_width * view_height];
        Renderer {
            meshes,
            animated_meshes,
            view_width,
            view_height,
            framebuffer_clear_color,
            mat_projection,
            vec_camera,
            vec_look_dir,
            vec_target,
            vec_up,
            yaw,
            depth_buffer,
        }
    }

    /// Translates the renderer camera yaw by 'yaw_adjustment'
    pub fn translate_yaw(&mut self, yaw_adjustment: f32) {
        self.yaw += yaw_adjustment;
    }
    
    /// Translates the renderer camera yaw by 'x_adjustment'
    pub fn translate_camera_x(&mut self, x_adjustment: f32) {
        self.vec_camera.x += x_adjustment;
    }
    /// Translates the renderer camera yaw by 'y_adjustment'
    pub fn translate_camera_y(&mut self, y_adjustment: f32) {
        self.vec_camera.y += y_adjustment;
    }
    /// Translates the renderer camera forward location by 'forward_adjustment' * 'time_elapsed'
    pub fn translate_camera_forward(&mut self, forward_adjustment: f32, time_elapsed: f32) {
        let vec_forward = vector_mul(&self.vec_look_dir, forward_adjustment * time_elapsed);
        self.vec_camera = vector_add(&self.vec_camera, &vec_forward);
    }
    /// Translates the renderer camera backward location by 'backward_adjustment' * 'time_elapsed'
    pub fn translate_camera_backward(&mut self, backward_adjustment: f32, time_elapsed: f32) {
        let vec_backward = vector_mul(&self.vec_look_dir, backward_adjustment * time_elapsed);
        self.vec_camera = vector_sub(&self.vec_camera, &vec_backward);
    }

    /// Performs the render function, translating the world meshes and camera location into a 2D frame.
    pub fn render(&mut self, time_elapsed: f32, framebuffer: &mut Framebuffer) {
        //Clear the depth buffer and frame buffer for pixel rendering
        for i in 0..self.view_width * self.view_height {
            self.depth_buffer[i] = 0.0;
        }

        framebuffer.clear_buffer_color(&self.framebuffer_clear_color);

        for i in 0..self.animated_meshes.len()
        {
            self.animated_meshes.get_mut(i).unwrap().tick(time_elapsed);
        }

        let mat_rot_z: Mat4x4 = matrix_make_rotation_z(&(0.0));
        let mat_rot_x: Mat4x4 = matrix_make_rotation_x(&0.0);
        let mat_trans: Mat4x4 = matrix_make_translation(0.0, 0.0, 5.0);
        let mut mat_world = matrix_multiply_matrix(&mat_rot_z, &mat_rot_x);
        mat_world = matrix_multiply_matrix(&mat_world, &mat_trans);

        self.vec_look_dir = Vec3d {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            ..Default::default()
        };
        self.vec_target = Vec3d {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            ..Default::default()
        };

        let mat_camera_rot = matrix_make_rotation_y(&self.yaw);
        self.vec_look_dir = matrix_multiply_vector(&mat_camera_rot, &self.vec_target);
        self.vec_target = vector_add(&self.vec_camera, &self.vec_look_dir);

        let mat_camera: Mat4x4 = matrix_point_at(&self.vec_camera, &self.vec_target, &self.vec_up);

        let mat_view: Mat4x4 = matrix_quick_inverse(&mat_camera);

        let mut render_list: Vec<Mesh<'_>> = self.meshes.clone();     

        for i in 0..self.animated_meshes.len()
        {
            render_list.push(self.animated_meshes.get(i).unwrap().current_frame.clone());
        }
        

        for mesh in render_list.iter() {
            let mut vec_triangles_to_raster: Vec<Triangle> = Vec::new();
            for tri in mesh.tris.iter() {
                let mut tri_projected: Triangle = Triangle::default();
                let mut tri_transformed: Triangle = Triangle::default();
                let mut tri_viewed: Triangle = Triangle::default();

                tri_transformed.points[0] = matrix_multiply_vector(&mat_world, &tri.points[0]);
                tri_transformed.points[1] = matrix_multiply_vector(&mat_world, &tri.points[1]);
                tri_transformed.points[2] = matrix_multiply_vector(&mat_world, &tri.points[2]);
                tri_transformed.texture_points[0] = tri.texture_points[0].clone();
                tri_transformed.texture_points[1] = tri.texture_points[1].clone();
                tri_transformed.texture_points[2] = tri.texture_points[2].clone();

                let line1 = vector_sub(&tri_transformed.points[1], &tri_transformed.points[0]);
                let line2 = vector_sub(&tri_transformed.points[2], &tri_transformed.points[0]);
                let normal = vector_cross_product(&line1, &line2);

                let v_camera_ray: Vec3d = vector_sub(&tri_transformed.points[0], &self.vec_camera);

                if vector_dot_product(&normal, &v_camera_ray) < 0.0 {
                    tri_viewed.points[0] =
                        matrix_multiply_vector(&mat_view, &tri_transformed.points[0]);
                    tri_viewed.points[1] =
                        matrix_multiply_vector(&mat_view, &tri_transformed.points[1]);
                    tri_viewed.points[2] =
                        matrix_multiply_vector(&mat_view, &tri_transformed.points[2]);
                    tri_viewed.texture_points[0] = tri_transformed.texture_points[0].clone();
                    tri_viewed.texture_points[1] = tri_transformed.texture_points[1].clone();
                    tri_viewed.texture_points[2] = tri_transformed.texture_points[2].clone();
                    let mut clipped_1: Triangle = Triangle::default();
                    let mut clipped_2: Triangle = Triangle::default();

                    let n_clipped_triangles = triangle_clip_against_plane(
                        &mut Vec3d {
                            x: 0.0,
                            y: 0.0,
                            z: 0.1,
                            ..Default::default()
                        },
                        &mut Vec3d {
                            x: 0.0,
                            y: 0.0,
                            z: 1.0,
                            ..Default::default()
                        },
                        &mut tri_viewed,
                        &mut clipped_1,
                        &mut clipped_2,
                    );

                    for i in 0..n_clipped_triangles {
                        if i == 0 {
                            // Project triangles
                            tri_projected.points[0] =
                                matrix_multiply_vector(&self.mat_projection, &clipped_1.points[0]);
                            tri_projected.points[1] =
                                matrix_multiply_vector(&self.mat_projection, &clipped_1.points[1]);
                            tri_projected.points[2] =
                                matrix_multiply_vector(&self.mat_projection, &clipped_1.points[2]);
                            tri_projected.texture_points[0] = clipped_1.texture_points[0].clone();
                            tri_projected.texture_points[1] = clipped_1.texture_points[1].clone();
                            tri_projected.texture_points[2] = clipped_1.texture_points[2].clone();
                        } else if i == 1 {
                            // Project triangles
                            tri_projected.points[0] =
                                matrix_multiply_vector(&self.mat_projection, &clipped_2.points[0]);
                            tri_projected.points[1] =
                                matrix_multiply_vector(&self.mat_projection, &clipped_2.points[1]);
                            tri_projected.points[2] =
                                matrix_multiply_vector(&self.mat_projection, &clipped_2.points[2]);
                            tri_projected.texture_points[0] = clipped_2.texture_points[0].clone();
                            tri_projected.texture_points[1] = clipped_2.texture_points[1].clone();
                            tri_projected.texture_points[2] = clipped_2.texture_points[2].clone();
                        }

                        tri_projected.texture_points[0].u /= tri_projected.points[0].w;
                        tri_projected.texture_points[1].u /= tri_projected.points[1].w;
                        tri_projected.texture_points[2].u /= tri_projected.points[2].w;

                        tri_projected.texture_points[0].v /= tri_projected.points[0].w;
                        tri_projected.texture_points[1].v /= tri_projected.points[1].w;
                        tri_projected.texture_points[2].v /= tri_projected.points[2].w;

                        tri_projected.texture_points[0].w = 1.0 / tri_projected.points[0].w;
                        tri_projected.texture_points[1].w = 1.0 / tri_projected.points[1].w;
                        tri_projected.texture_points[2].w = 1.0 / tri_projected.points[2].w;

                        //Scale/Normalize
                        tri_projected.points[0] =
                            vector_div(&tri_projected.points[0], tri_projected.points[0].w);
                        tri_projected.points[1] =
                            vector_div(&tri_projected.points[1], tri_projected.points[1].w);
                        tri_projected.points[2] =
                            vector_div(&tri_projected.points[2], tri_projected.points[2].w);

                        //X/Y are inverted so put them back
                        tri_projected.points[0].x *= -1.0;
                        tri_projected.points[1].x *= -1.0;
                        tri_projected.points[2].x *= -1.0;
                        tri_projected.points[0].y *= -1.0;
                        tri_projected.points[1].y *= -1.0;
                        tri_projected.points[2].y *= -1.0;

                        //offset Vertices
                        let v_offset_view: Vec3d = Vec3d {
                            x: 1.0,
                            y: 1.0,
                            ..Default::default()
                        };
                        tri_projected.points[0] =
                            vector_add(&tri_projected.points[0], &v_offset_view);
                        tri_projected.points[1] =
                            vector_add(&tri_projected.points[1], &v_offset_view);
                        tri_projected.points[2] =
                            vector_add(&tri_projected.points[2], &v_offset_view);

                        tri_projected.points[0].x *= 0.5 * (self.view_width as f32);
                        tri_projected.points[0].y *= 0.5 * (self.view_height as f32);
                        tri_projected.points[1].x *= 0.5 * (self.view_width as f32);
                        tri_projected.points[1].y *= 0.5 * (self.view_height as f32);
                        tri_projected.points[2].x *= 0.5 * (self.view_width as f32);
                        tri_projected.points[2].y *= 0.5 * (self.view_height as f32);
                        vec_triangles_to_raster.push(tri_projected.clone());
                    }
                }
            }
            for tri_to_raster in vec_triangles_to_raster {
                let mut clipped_1: Triangle = Triangle::default();
                let mut clipped_2: Triangle = Triangle::default();

                let mut list_triangles: VecDeque<Triangle> = VecDeque::new();

                list_triangles.push_back(tri_to_raster);
                let mut n_new_triangles: usize = 1;

                for p in 0..4 {
                    let mut n_tris_to_add: u64 = 0;
                    while n_new_triangles > 0 {
                        // Take triangle from front of list
                        let mut test: Triangle = list_triangles.front().unwrap().clone();
                        list_triangles.pop_front();
                        n_new_triangles -= 1;

                        // Clip it against a plane.
                        match p {
                            0 => {
                                n_tris_to_add = triangle_clip_against_plane(
                                    &mut Vec3d {
                                        x: 0.0,
                                        y: 0.0,
                                        z: 0.0,
                                        ..Default::default()
                                    },
                                    &mut Vec3d {
                                        x: 0.0,
                                        y: 1.0,
                                        z: 0.0,
                                        ..Default::default()
                                    },
                                    &mut test,
                                    &mut (clipped_1),
                                    &mut (clipped_2),
                                );
                            }
                            1 => {
                                n_tris_to_add = triangle_clip_against_plane(
                                    &mut Vec3d {
                                        x: 0.0,
                                        y: self.view_height as f32 - 1.0,
                                        z: 0.0,
                                        ..Default::default()
                                    },
                                    &mut Vec3d {
                                        x: 0.0,
                                        y: -1.0,
                                        z: 0.0,
                                        ..Default::default()
                                    },
                                    &mut test,
                                    &mut (clipped_1),
                                    &mut (clipped_2),
                                );
                            }
                            2 => {
                                n_tris_to_add = triangle_clip_against_plane(
                                    &mut Vec3d {
                                        x: 0.0,
                                        y: 0.0,
                                        z: 0.0,
                                        ..Default::default()
                                    },
                                    &mut Vec3d {
                                        x: 1.0,
                                        y: 0.0,
                                        z: 0.0,
                                        ..Default::default()
                                    },
                                    &mut test,
                                    &mut (clipped_1),
                                    &mut (clipped_2),
                                );
                            }
                            3 => {
                                n_tris_to_add = triangle_clip_against_plane(
                                    &mut Vec3d {
                                        x: self.view_width as f32 - 1.0,
                                        y: 0.0,
                                        z: 0.0,
                                        ..Default::default()
                                    },
                                    &mut Vec3d {
                                        x: -1.0,
                                        y: 0.0,
                                        z: 0.0,
                                        ..Default::default()
                                    },
                                    &mut test,
                                    &mut (clipped_1),
                                    &mut (clipped_2),
                                );
                            }
                            _ => {}
                        }

                        // Clipping may create more triangles, so add these to the queue for further clipping
                        for w in 0..n_tris_to_add {
                            if w == 0 {
                                list_triangles.push_back(clipped_1.clone());
                            } else if w == 1 {
                                list_triangles.push_back(clipped_2.clone());
                            }
                        }
                    }
                    n_new_triangles = list_triangles.len();
                }
                for t in list_triangles {
                    draw_textured_triangle(
                        t.points[0].x.round() as i64,
                        t.points[0].y.round() as i64,
                        t.texture_points[0].u,
                        t.texture_points[0].v,
                        t.texture_points[0].w,
                        t.points[1].x.round() as i64,
                        t.points[1].y.round() as i64,
                        t.texture_points[1].u,
                        t.texture_points[1].v,
                        t.texture_points[1].w,
                        t.points[2].x.round() as i64,
                        t.points[2].y.round() as i64,
                        t.texture_points[2].u,
                        t.texture_points[2].v,
                        t.texture_points[2].w,
                        mesh.texture,
                        framebuffer,
                        &mut self.depth_buffer,
                        &(self.view_width as i64),
                    );
                }
            }
        }
    }
}
