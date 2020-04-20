use nannou::prelude::*;
use nannou::noise::{NoiseFn, Perlin};
use nannou_audio as audio;
use nannou_audio::Buffer;
use std::f64::consts::PI;

fn main() {
    nannou::sketch(view).run()
}

struct Model {
    stream: audio::Stream<Audio>,
}

struct Audio {
    phase: f64,
    hz: f64,
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let win = app.window_rect();
    let t = app.time;
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(BLACK);

    let perlin = Perlin::new();
    let val = perlin.get([42.4, 37.7, 2.8]);

    // Create two rotating squares, rotating in opposite directions with pulsing colors
    let n_points = 4;

    // Create squares inside those squares
    let radius = (win.w().min(win.h()).powf(2.0) / 2.0).sqrt();
    //  + (t.sin() * win.w().min(win.h()) * 0.5 / 2.0.sqrt());
    let points = (0..n_points).map(|i| {
        let fract = i as f32 / n_points as f32;
        let phase = fract;
        let x = radius * (TAU * phase).cos();
        let y = radius * (TAU * phase).sin();
        pt2(x, y)
    });
    draw.polygon()
        .stroke(WHITE)
        .stroke_weight(10.0)
        .join_round()
        .hsva(-t / 8.0 , 1.0, 0.5, -2.0 * t.cos())
        .rotate(16.0 * (0.125 * t).cos())
        .points(points);

    let points = (0..n_points).map(|i| {
        let fract = i as f32 / n_points as f32;
        let phase = fract;
        let x = radius  * (TAU * phase).cos();
        let y = radius * (TAU * phase).sin();
        pt2(x, y)
    });
    draw.polygon()
        .stroke(WHITE)
        .stroke_weight(10.0)
        .join_round()
        .hsva(t / 8.0 , 1.0, 0.5, -2.0 * -t.cos())
        .rotate(-8.0 * (0.125 * t).cos())
        .points(points);
    let radius = (win.w().min(win.h()).powf(2.0) / 4.0).sqrt();
    //  + (t.sin() * win.w().min(win.h()) * 0.5 / 2.0.sqrt());
    let points = (0..n_points).map(|i| {
        let fract = i as f32 / n_points as f32;
        let phase = fract;
        let x = radius * (TAU * phase).cos();
        let y = radius * (TAU * phase).sin();
        pt2(x, y)
    });
    draw.polygon()
        .stroke(WHITE)
        .stroke_weight(10.0)
        .join_round()
        .hsva(-t / 8.0 , 1.0, 0.75, -2.0 * t.sin())
        .rotate(16.0 * (0.25 * t).sin())
        .points(points);

    let points = (0..n_points).map(|i| {
        let fract = i as f32 / n_points as f32;
        let phase = fract;
        let x = radius * (TAU * phase).cos();
        let y = radius * (TAU * phase).sin();
        pt2(x, y)
    });

    //  + (t.sin() * win.w().min(win.h()) * 0.5 / 2.0.sqrt());
    let points = (0..n_points).map(|i| {
        let fract = i as f32 / n_points as f32;
        let phase = fract;
        let x = radius * (TAU * phase).cos();
        let y = radius * (TAU * phase).sin();
        pt2(x, y)
    });
    draw.polygon()
        .stroke(WHITE)
        .stroke_weight(10.0)
        .join_round()
        .hsva(t / 4.0 , 0.9, 0.9, -2.0 * t.cos())
        .rotate(t * 0.8)
        .points(points);

    let points = (0..n_points).map(|i| {
        let fract = i as f32 / n_points as f32;
        let phase = fract;
        let x = radius * (TAU * phase).cos();
        let y = radius * (TAU * phase).sin();
        pt2(x, y)
    });
    draw.polygon()
        .stroke(WHITE)
        .stroke_weight(10.0)
        .join_round()
        .hsva(-t / 4.0 , 0.9, 0.9, -2.0 * -t.cos())
        .rotate(t * -0.4)
        .points(points);

    
    let radius = (win.w().min(win.h()) * 0.25);
    let points = (0..n_points).map(|i| {
        let fract = i as f32 / n_points as f32;
        let phase = fract;
        let x = radius * (TAU * phase).cos();
        let y = radius * (TAU * phase).sin();
        pt2(x, y)
    });
    draw.polygon()
        .stroke(WHITE)
        .stroke_weight(10.0)
        .join_round()
        .hsva(t / 8.0 , 1.0, 0.75, -2.0 * -t.sin())
        .rotate(16.0 * (-0.25 * t).sin())
        .points(points);

    let points = (0..n_points).map(|i| {
        let fract = i as f32 / n_points as f32;
        let phase = fract;
        let x = radius * (TAU * phase).cos();
        let y = radius * (TAU * phase).sin();
        pt2(x, y)
    });
    draw.polygon()
        .stroke(WHITE)
        .stroke_weight(10.0)
        .join_round()
        .hsva(t / 8.0 , 1.0, 0.75, -2.0 * -t.sin())
        .rotate(-16.0 * (-0.25 * t).sin())
        .points(points);


let radius = (win.w().min(win.h()) * 0.5 / 2.0.sqrt());
    //  + (win.w().min(win.h()) * 0.25 * t.sin());
    let points = (0..n_points).map(|i| {
        let fract = i as f32 / n_points as f32;
        let phase = fract;
        let x = radius * (TAU * phase).cos();
        let y = radius * (TAU * phase).sin();
        pt2(x, y)
    });
    draw.polygon()
        .stroke(WHITE)
        .stroke_weight(10.0)
        .hsva(-t.sin(), 1.0, 1.0, 2.0 * t.sin())
        .rotate(-t * 1.6)
        .points(points);

    let points = (0..n_points).map(|i| {
        let fract = i as f32 / n_points as f32;
        let phase = fract;
        let x = radius * (TAU * phase).cos();
        let y = radius * (TAU * phase).sin();
        pt2(x, y)
    });
    draw.polygon()
        .stroke(WHITE)
        .stroke_weight(10.0)
        .hsva(t.sin() , 1.0, 1.0, -2.0 * t.cos())
        .rotate(t * 1.6)
        .points(points);

    let radius = (win.w().min(win.h()) * 0.5 / 2.0.sqrt());

    draw.line()
        .weight(6.0)
        .points(pt2(0.0, radius), pt2(0.0, -radius))
        .hsva(t.sin() , 1.0, 0.75, 1.0)
        .rotate(t);
    draw.line()
        .weight(6.0)
        .hsva(t.cos() , 1.0, 0.75, 1.0)
        .points(pt2(0.0, radius), pt2(0.0, -radius))
        .rotate(-t);
    draw.line()
        .weight(6.0)
        .hsva(-t.sin() , 1.0, 0.75, 1.0)
        .points(pt2(radius, 0.0), pt2(-radius, 0.0))
        .rotate(t);
    draw.line()
        .weight(6.0)
        .hsva(-t.cos() , 1.0, 0.75, 1.0)
        .points(pt2(radius, 0.0), pt2(-radius, 0.0))
        .rotate(-t);


    //         // Create squares inside those squares
    // let radius = (win.w().min(win.h()) * 0.5 / 2.0.sqrt()) + (t.sin() * win.w().min(win.h()) * 0.5 / 2.0.sqrt());
    // let points = (0..n_points).map(|i| {
    //     let fract = i as f32 / n_points as f32;
    //     let phase = fract;
    //     let x = radius * (TAU * phase).cos();
    //     let y = radius * (TAU * phase).sin();
    //     pt2(x, y)
    // });
    // draw.polygon()
    //     .stroke(WHITE)
    //     .stroke_weight(10.0)
    //     .join_round()
    //     .hsva(t.sin() / 4.0 , 1.0, 1.0, -2.0 * t.cos())
    //     .rotate(t * 0.8)
    //     .points(points);

    // let points = (0..n_points).map(|i| {
    //     let fract = i as f32 / n_points as f32;
    //     let phase = fract;
    //     let x = radius * (TAU * phase).cos();
    //     let y = radius * (TAU * phase).sin();
    //     pt2(x, y)
    // });
    // draw.polygon()
    //     .stroke(WHITE)
    //     .stroke_weight(10.0)
    //     .join_round()
    //     .hsva(t.sin() / 4.0 , 1.0, 1.0, -2.0 * t.cos())
    //     .rotate(t * -0.4)
    //     .points(points);

    
    // let radius = (win.w().min(win.h()) * 0.25) + (win.w().min(win.h()) * 0.25 * -t.sin());
    // let points = (0..n_points).map(|i| {
    //     let fract = i as f32 / n_points as f32;
    //     let phase = fract;
    //     let x = radius * (TAU * phase).cos();
    //     let y = radius * (TAU * phase).sin();
    //     pt2(x, y)
    // });
    // draw.polygon()
    //     .stroke(WHITE)
    //     .stroke_weight(10.0)
    //     .hsva(-t.sin(), 1.0, 1.0, 2.0 * t.sin())
    //     .rotate(-t * 1.6)
    //     .points(points);

    // let points = (0..n_points).map(|i| {
    //     let fract = i as f32 / n_points as f32;
    //     let phase = fract;
    //     let x = radius * (TAU * phase).cos();
    //     let y = radius * (TAU * phase).sin();
    //     pt2(x, y)
    // });
    // draw.polygon()
    //     .stroke(WHITE)
    //     .stroke_weight(10.0)
    //     .join_round()
    //     .hsva(t.sin() , 1.0, 1.0, -2.0 * t.cos())
    //     .rotate(t * 1.6)
    //     .points(points);


        let n_points = 3;
        let radius = win.w().min(win.h()) * 0.25 / 2.0.sqrt();
        let points = (0..n_points).map(|i| {
            let fract = i as f32 / n_points as f32;
            let phase = fract;
            let x = radius * (TAU * phase).cos();
            let y = radius * (TAU * phase).sin();
            pt2(x, y)
        });
        draw.polygon()
            .stroke(WHITE)
            .hsva(1.0, 1.0, 1.0, 0.0)
            .stroke_weight(10.0)
            .rotate(-t * 2.4)
            .points(points);
    
        let points = (0..n_points).map(|i| {
            let fract = i as f32 / n_points as f32;
            let phase = fract;
            let x = radius * (TAU * phase).cos();
            let y = radius * (TAU * phase).sin();
            pt2(x, y)
        });
        draw.polygon()
            .stroke(WHITE)
            .hsva(1.0, 1.0, 1.0, 0.0)
            .stroke_weight(10.0)
            .rotate(t * 2.4)
            .points(points);
    
        let points = (0..n_points).map(|i| {
            let fract = i as f32 / n_points as f32;
            let phase = fract;
            let x = radius * (TAU * phase).cos();
            let y = radius * (TAU * phase).sin();
            pt2(x, y)
        });
        draw.polygon()
            .stroke(WHITE)
            .hsva(1.0, 1.0, 1.0, 0.0)
            .stroke_weight(10.0)
            .rotate(-t * 1.2)
            .points(points);
    
        let points = (0..n_points).map(|i| {
            let fract = i as f32 / n_points as f32;
            let phase = fract;
            let x = radius * (TAU * phase).cos();
            let y = radius * (TAU * phase).sin();
            pt2(x, y)
        });
        draw.polygon()
            .stroke(WHITE)
            .hsva(1.0, 1.0, 1.0, 0.0)
            .stroke_weight(10.0)
            .rotate(t * 1.2)
            .points(points);
    
        let radius = win.w().min(win.h()) * 0.125 / 2.0.sqrt();
        let points = (0..n_points).map(|i| {
            let fract = i as f32 / n_points as f32;
            let phase = fract;
            let x = radius * (TAU * phase).cos();
            let y = radius * (TAU * phase).sin();
            pt2(x, y)
        });
        draw.polygon()
            .stroke(WHITE)
            .hsva(1.0, 1.0, 1.0, 0.0)
            .stroke_weight(5.0)
            .rotate(-t.sin() * 4.8)
            .points(points);
    
        let points = (0..n_points).map(|i| {
            let fract = i as f32 / n_points as f32;
            let phase = fract;
            let x = radius * (TAU * phase).cos();
            let y = radius * (TAU * phase).sin();
            pt2(x, y)
        });
        draw.polygon()
            .stroke(WHITE)
            .hsva(1.0, 1.0, 1.0, 0.0)
            .stroke_weight(5.0)
            .rotate(t.sin() * 4.8)
            .points(points);
    
        let points = (0..n_points).map(|i| {
            let fract = i as f32 / n_points as f32;
            let phase = fract;
            let x = radius * (TAU * phase).cos();
            let y = radius * (TAU * phase).sin();
            pt2(x, y)
        });
        draw.polygon()
            .stroke(WHITE)
            .hsva(1.0, 1.0, 1.0, 0.0)
            .stroke_weight(5.0)
            .rotate(-t.cos() * 3.2)
            .points(points);
    
        let points = (0..n_points).map(|i| {
            let fract = i as f32 / n_points as f32;
            let phase = fract;
            let x = radius * (TAU * phase).cos();
            let y = radius * (TAU * phase).sin();
            pt2(x, y)
        });
        draw.polygon()
            .stroke(WHITE)
            .hsva(1.0, 1.0, 1.0, 0.0)
            .stroke_weight(5.0)
            .rotate(t.cos() * 3.2)
            .points(points);


    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
