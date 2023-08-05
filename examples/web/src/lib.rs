use minifb::{Window, WindowOptions};
use rusterer::texture::Texture;
use std::cell::RefCell;
use std::panic;
use std::rc::Rc;
use instant::Instant;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use lazy_static::lazy_static;
use rusterer::framebuffer::Framebuffer;
use rusterer::geometry::Mesh;
use rusterer::geometry::AnimatedMesh;
use minifb::Key;
use rusterer::renderer::Renderer;
use rusterer::texture_helper::get_texture_from_bmp;
const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;
lazy_static!
    {
        static ref MESH_TEXTURE: Texture = get_texture_from_bmp(include_bytes!("../../../demo_objects/floating_islands_demo_texture.bmp"));
        static ref MECH_TEXTURE: Texture = get_texture_from_bmp(include_bytes!("../../../demo_objects/mech/baked_mech_texture.bmp"));

    }


fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
pub fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut framebuffer: Framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let mut mesh_list: Vec<Mesh> = Vec::new();
    mesh_list.push(Mesh::new(&MESH_TEXTURE, include_bytes!("../../../demo_objects/floating_islands_demo.obj")));

    let mut animated_mesh_list: Vec<AnimatedMesh> = Vec::new();
    let mut animated_mech: Vec<Vec<u8>> = Vec::new();
    let mech_frame_1 = include_bytes!("../../../demo_objects/mech/mech1.obj");
    let mech_frame_2 = include_bytes!("../../../demo_objects/mech/mech2.obj");
    animated_mech.push(mech_frame_1.to_vec());
    animated_mech.push(mech_frame_2.to_vec());
    animated_mesh_list.push(AnimatedMesh::new(&MECH_TEXTURE,animated_mech.clone(),3.0, true));

    let mut renderer = Renderer::new(mesh_list, animated_mesh_list.clone(), WIDTH, HEIGHT, 0xffe6ac00);

    renderer.render(0.1, &mut framebuffer);
    let mut now = Instant::now();
    let mut window = Window::new(
        "Render Test",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    window
        .update_with_buffer(&framebuffer.get_framebuffer(), WIDTH, HEIGHT).unwrap();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {

        let elapsed_time = now.elapsed();
        now = Instant::now();
        let time_elapsed = elapsed_time.as_secs_f32();

        //Get Control Inputs
        if window.is_key_down(Key::Up)
        {
            renderer.translate_camera_y(8.0*time_elapsed);
        }
        if window.is_key_down(Key::Down)
        {
            renderer.translate_camera_y(-8.0*time_elapsed);
        }
        if window.is_key_down(Key::W)
        {
            renderer.translate_camera_forward(8.0, time_elapsed);
        }
        if window.is_key_down(Key::S)
        {
            renderer.translate_camera_backward(8.0, time_elapsed);
        }
        if window.is_key_down(Key::A)
        {
            renderer.translate_yaw(-2.0*time_elapsed);
        }
        if window.is_key_down(Key::D)
        {
            renderer.translate_yaw(2.0*time_elapsed);
        }

        renderer.render(time_elapsed, &mut framebuffer);
        window.update_with_buffer(&framebuffer.get_framebuffer(), WIDTH, HEIGHT).unwrap();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut() + 'static>));

    

    // start the animation loop
    request_animation_frame(g.borrow().as_ref().unwrap());
}
