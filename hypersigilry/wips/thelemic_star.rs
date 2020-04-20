use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(BLACK);

    let win = app.window_rect();
    // Draw an ellipse to follow the mouse.
    let t = app.time;

    // Construct the small circle
    let radius = 200.0;

    // Outer hex star
    let points = (0..=1080)
        .step_by(60)
        .into_iter()
        .map(|i| {
            match i {
                0 | 120 | 300 | 600 | 780 | 960 | 1080 => {
                    let radian = deg_to_rad((i as f32) % 360.0);
                    let x = radian.sin() * radius;    
                    let y = radian.cos() * radius;   
                    pt2(x,y)             
                }
                60 | 240 | 540 | 720 | 900 | 1020 => {
                    let radian = deg_to_rad((i as f32) + 60.0 % 360.0);
                    let x = radian.sin() * radius;    
                    let y = radian.cos() * radius;   
                    pt2(x,y)             
                }
                180 | 480 | 660 | 840 => {
                    let radian = deg_to_rad((i as f32) + 120.0 % 360.0);
                    let x = radian.sin() * radius;    
                    let y = radian.cos() * radius;   
                    pt2(x,y)             
                }
                360 | 980 => {
                    let radian = deg_to_rad((i as f32) + 180.0 % 360.0);
                    let x = radian.sin() * radius;    
                    let y = radian.cos() * radius;   
                    pt2(x,y)             
                }
                _ => {
                    let radian = deg_to_rad((i as f32) % 360.0);
                    let x = radian.sin() * radius;    
                    let y = radian.cos() * radius;   
                    pt2(x,y)             
                }
            }
        });
    
    draw.polyline()                       // create a PathStroke Builder object
        .weight(4.0)
        .color(WHITE)
        .points_closed(points);

    

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
