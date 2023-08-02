use crate::{framebuffer::Framebuffer, texture::Texture};

pub fn draw_line(
    x0_param: i32,
    y0_param: i32,
    x1_param: i32,
    y1_param: i32,
    framebuffer: &mut Framebuffer,
    color: u32,
) {
    let mut x0 = x0_param;
    let mut y0 = y0_param;
    let mut x1 = x1_param;
    let mut y1 = y1_param;
    let mut steep: bool = false;
    if x0.abs_diff(x1) < y0.abs_diff(y1) {
        (x0, y0) = (y0, x0);
        (x1, y1) = (y1, x1);
        steep = true;
    }
    if x0 > x1 {
        (x0, x1) = (x1, x0);
        (y0, y1) = (y1, y0);
    }
    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror2: i32 = (dy.abs()) * 2;
    let mut error2: i32 = 0;
    let mut y = y0;
    for x in x0..(x1 + 1) {
        if steep {
            framebuffer.set_color_at(&(y as u32), &(x as u32), color);
        } else {
            framebuffer.set_color_at(&(x as u32), &(y as u32), color);
        }
        error2 += derror2;
        if error2 > dx {
            if y1 > y0 {
                y += 1
            } else {
                y += -1
            };
            error2 -= dx * 2;
        }
    }
}

pub fn draw_textured_triangle(
    x1_param: i64,
    y1_param: i64,
    u1_param: f32,
    v1_param: f32,
    w1_param: f32,
    x2_param: i64,
    y2_param: i64,
    u2_param: f32,
    v2_param: f32,
    w2_param: f32,
    x3_param: i64,
    y3_param: i64,
    u3_param: f32,
    v3_param: f32,
    w3_param: f32,
    tex: &Texture,
    framebuffer: &mut Framebuffer,
    p_depth_buffer: &mut Vec<f32>,
    screen_width: &i64,
) {
    let mut x1 = x1_param;
    let mut y1 = y1_param;
    let mut u1 = u1_param;
    let mut v1 = v1_param;
    let mut w1 = w1_param;
    let mut x2 = x2_param;
    let mut y2 = y2_param;
    let mut u2 = u2_param;
    let mut v2 = v2_param;
    let mut w2 = w2_param;
    let mut x3 = x3_param;
    let mut y3 = y3_param;
    let mut u3 = u3_param;
    let mut v3 = v3_param;
    let mut w3 = w3_param;

    if y2 < y1 {
        (y1, y2) = (y2, y1);
        (x1, x2) = (x2, x1);
        (u1, u2) = (u2, u1);
        (v1, v2) = (v2, v1);
        (w1, w2) = (w2, w1);
    }

    if y3 < y1 {
        (y1, y3) = (y3, y1);
        (x1, x3) = (x3, x1);
        (u1, u3) = (u3, u1);
        (v1, v3) = (v3, v1);
        (w1, w3) = (w3, w1);
    }

    if y3 < y2 {
        (y2, y3) = (y3, y2);
        (x2, x3) = (x3, x2);
        (u2, u3) = (u3, u2);
        (v2, v3) = (v3, v2);
        (w2, w3) = (w3, w2);
    }

    let mut dy1: i64 = y2 - y1;
    let mut dx1: i64 = x2 - x1;
    let mut dv1: f32 = v2 - v1;
    let mut du1: f32 = u2 - u1;
    let mut dw1: f32 = w2 - w1;

    let dy2: i64 = y3 - y1;
    let dx2: i64 = x3 - x1;
    let dv2: f32 = v3 - v1;
    let du2: f32 = u3 - u1;
    let dw2: f32 = w3 - w1;

    let mut tex_u: f32;
    let mut tex_v: f32;
    let mut tex_w: f32;

    let mut dax_step: f32 = 0.0;
    let mut dbx_step: f32 = 0.0;
    let mut du1_step: f32 = 0.0;
    let mut dv1_step: f32 = 0.0;
    let mut du2_step: f32 = 0.0;
    let mut dv2_step: f32 = 0.0;
    let mut dw1_step: f32 = 0.0;
    let mut dw2_step: f32 = 0.0;

    if dy1 != 0 {
        dax_step = (dx1 as f32) / ((dy1).abs() as f32)
    };
    if dy2 != 0 {
        dbx_step = (dx2 as f32) / ((dy2).abs() as f32)
    };

    if dy1 != 0 {
        du1_step = du1 / ((dy1).abs() as f32)
    };
    if dy1 != 0 {
        dv1_step = dv1 / ((dy1).abs() as f32)
    };
    if dy1 != 0 {
        dw1_step = dw1 / ((dy1).abs() as f32)
    };

    if dy2 != 0 {
        du2_step = du2 / ((dy2).abs() as f32)
    };
    if dy2 != 0 {
        dv2_step = dv2 / ((dy2).abs() as f32)
    };
    if dy2 != 0 {
        dw2_step = dw2 / ((dy2).abs() as f32)
    };

    if dy1 != 0 {
        for i in y1..(y2) {
            let mut ax: i64 = (x1 as f32 + ((i - y1) as f32) * dax_step).round() as i64;
            let mut bx: i64 = (x1 as f32 + ((i - y1) as f32) * dbx_step).round() as i64;

            let mut tex_su: f32 = u1 + ((i - y1) as f32) * du1_step;
            let mut tex_sv: f32 = v1 + ((i - y1) as f32) * dv1_step;
            let mut tex_sw: f32 = w1 + ((i - y1) as f32) * dw1_step;

            let mut tex_eu: f32 = u1 + ((i - y1) as f32) * du2_step;
            let mut tex_ev: f32 = v1 + ((i - y1) as f32) * dv2_step;
            let mut tex_ew: f32 = w1 + ((i - y1) as f32) * dw2_step;

            if ax > bx {
                (ax, bx) = (bx, ax);
                (tex_su, tex_eu) = (tex_eu, tex_su);
                (tex_sv, tex_ev) = (tex_ev, tex_sv);
                (tex_sw, tex_ew) = (tex_ew, tex_sw);
            }

            let tstep: f32 = 1.0 / ((bx - ax) as f32);
            let mut t: f32 = 0.0;

            for j in ax..bx {
                tex_u = (1.0 - t) * tex_su + t * tex_eu;
                tex_v = (1.0 - t) * tex_sv + t * tex_ev;
                tex_w = (1.0 - t) * tex_sw + t * tex_ew;
                if tex_w > p_depth_buffer[(i * screen_width + j) as usize] {
                    framebuffer.set_color_at(
                        &(j as u32),
                        &(i as u32),
                        *tex.get_color_at_normalized_coord(&(tex_u / tex_w), &(tex_v / tex_w)),
                    );

                    p_depth_buffer[(i * screen_width + j) as usize] = tex_w;
                }
                t += tstep;
            }
        }
    }

    dy1 = y3 - y2;
    dx1 = x3 - x2;
    dv1 = v3 - v2;
    du1 = u3 - u2;
    dw1 = w3 - w2;

    if dy1 != 0 {
        dax_step = (dx1 as f32) / ((dy1).abs() as f32)
    };
    if dy2 != 0 {
        dbx_step = (dx2 as f32) / ((dy2).abs() as f32)
    };

    du1_step = 0.0;
    dv1_step = 0.0;
    if dy1 != 0 {
        du1_step = du1 / (dy1.abs() as f32)
    };
    if dy1 != 0 {
        dv1_step = dv1 / (dy1.abs() as f32)
    };
    if dy1 != 0 {
        dw1_step = dw1 / (dy1.abs() as f32)
    };

    if dy1 != 0 {
        for i in y2..(y3) {
            let mut ax: i64 = (x2 as f32 + ((i - y2) as f32) * dax_step).round() as i64;
            let mut bx: i64 = (x1 as f32 + ((i - y1) as f32) * dbx_step).round() as i64;

            let mut tex_su: f32 = u2 + ((i - y2) as f32) * du1_step;
            let mut tex_sv: f32 = v2 + ((i - y2) as f32) * dv1_step;
            let mut tex_sw: f32 = w2 + ((i - y2) as f32) * dw1_step;

            let mut tex_eu: f32 = u1 + ((i - y1) as f32) * du2_step;
            let mut tex_ev: f32 = v1 + ((i - y1) as f32) * dv2_step;
            let mut tex_ew: f32 = w1 + ((i - y1) as f32) * dw2_step;

            if ax > bx {
                (ax, bx) = (bx, ax);
                (tex_su, tex_eu) = (tex_eu, tex_su);
                (tex_sv, tex_ev) = (tex_ev, tex_sv);
                (tex_sw, tex_ew) = (tex_ew, tex_sw);
            }

            let tstep: f32 = 1.0 / ((bx - ax) as f32);
            let mut t: f32 = 0.0;

            for j in ax..bx {
                tex_u = (1.0 - t) * tex_su + t * tex_eu;
                tex_v = (1.0 - t) * tex_sv + t * tex_ev;
                tex_w = (1.0 - t) * tex_sw + t * tex_ew;

                if tex_w > p_depth_buffer[(i * screen_width + j) as usize] {
                    framebuffer.set_color_at(
                        &(j as u32),
                        &(i as u32),
                        *tex.get_color_at_normalized_coord(&(tex_u / tex_w), &(tex_v / tex_w)),
                    );
                    p_depth_buffer[(i * screen_width + j) as usize] = tex_w;
                }
                t += tstep;
            }
        }
    }
}
