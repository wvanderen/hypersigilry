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

    // Inner hexagon
    let radius = 50.0;
    let points = (0..=360).step_by(60).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .points_closed(points);    



    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
