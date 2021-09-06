use std::f64;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn cardioid_times_table(
    total_points: u32,
    times: f64,
    context: &web_sys::CanvasRenderingContext2d,
    r: f64,
) {
    context.begin_path();

    let points = get_points(r, total_points);

    // draw lines
    for i in 0..points.len() {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i as f64 * times) as usize % total_points as usize];
        context.move_to(x1, y1);
        context.line_to(x2, y2);
    }

    // draw points
    for (x, y) in points {
        context.move_to(x, y);
        context.arc(x, y, 1., 0., f64::consts::PI * 2.).unwrap();
    }
    context.stroke();
}

fn get_points(r: f64, total_points: u32) -> Vec<(f64, f64)> {
    let mut result = Vec::new();
    for i in 0..total_points {
        let theta = (f64::consts::PI * 2.) / (total_points as f64);
        let angle = theta * (i as f64);
        let x = r * angle.cos() + r;
        let y = r * angle.sin() + r;

        result.push((x, y));
    }

    result
}
