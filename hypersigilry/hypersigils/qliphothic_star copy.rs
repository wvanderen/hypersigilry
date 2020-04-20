use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(BLACK);

    // Draw a purple triangle in the top left half of the window.
    let win = app.window_rect();
    // Draw an ellipse to follow the mouse.
    let t = app.time;
    // Construct a pentagram
    let radius = 150.0;              
    // let points = (0..=1080).step_by(216).map(|i| {      
    //     let radian = deg_to_rad((i % 360) as f32);
    //     let x = radian.sin() * radius;    
    //     let y = radian.cos() * radius;    
    //     pt2(x,y)             
    // });
    // draw.polyline()                       // create a PathStroke Builder object
    //     .weight(3.0)
    //     .points_closed(points);          // tell our PathStroke Builder to draw lines connecting our array of points

    // Construct a qliphothic star
    let points = (0..=23760).step_by(2160).map(|i| {      
        let radian = deg_to_rad(((i as f32) / 11.0) % 360.0);
        // ### Setting: Expand to infinite
        // let x = radian.sin() / t.sin() * 0.8 * radius;    
        // let y = radian.cos() / t.sin() * 0.8 * radius;   

        let x = (t * radian).sin() * 2.0 * radius;    
        let y = (t *radian).cos() * 2.0 * radius;   
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(4.0)
        .points_closed(points)
        .rotate(t);  


    let points = (0..=23760).step_by(2160).map(|i| {      
        let radian = deg_to_rad(((i as f32) / 11.0) % 360.0);
        // ### Setting: Expand to infinite
        // let x = radian.sin() / t.sin() * 0.8 * radius;    
        // let y = radian.cos() / t.sin() * 0.8 * radius;   

        let x = (t * radian).sin() * 3.0 * radius;    
        let y = (t * radian).cos() * 3.0 * radius;   
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(2.0 * t.sin().abs() + 1.0)
        .color(GOLD)
        .points_closed(points)
        .rotate(0.5 * t);  

    let points = (0..=23760).step_by(2160).map(|i| {      
        let radian = deg_to_rad(((i as f32) / 11.0) % 360.0);
        // ### Setting: Breathing
        let x = (0.5 * t).sin() * (radian % 360.0).sin() * 2.0 * radius;    
        let y = (0.5 * t).sin() * (radian % 360.0).cos() * 2.0 * radius;    
        // ### Setting: Dimensional Lightshow
        // let x = (0.5 * t).sin() * (t * radian % 360.0).sin() * 2.0 * radius;    
        // let y = (0.5 *t).sin() * (t * radian % 360.0).cos() * 2.0 * radius;    
        // ### Setting: Dimensional Scrying Lens
        // let x = (0.5 * t).sin() * (t.pow(2.0).sin() * radian % 360.0).sin() * 2.0 * radius;    
        // let y = (0.5 *t).sin() * (t.pow(2.0).sin() * radian % 360.0).cos() * 2.0 * radius;    
        (pt2(x,y), srgba((t.cos().abs()) % 1.0, 0.0, 0.0, 1.0))             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(4.0)
        .colored_points(points)
        .rotate(t.sin());  
    let points = (0..=23760).step_by(2160).map(|i| {      
        let radian = deg_to_rad(((i as f32) / 11.0) % 360.0);
        // ### Setting: Breathing
        let x = -(0.5 * -t).cos() * -(radian % 360.0).sin() * 2.0 * radius;    
        let y = -(0.5 * -t).cos() * -(radian % 360.0).cos() * 2.0 * radius;    
        // ### Setting: Dimensional Lightshow
        // let x = (0.5 * t).sin() * (t * radian % 360.0).sin() * 2.0 * radius;    
        // let y = (0.5 *t).sin() * (t * radian % 360.0).cos() * 2.0 * radius;    
        // ### Setting: Dimensional Scrying Lens
        // let x = (0.5 * t).sin() * (t.pow(2.0).sin() * radian % 360.0).sin() * 2.0 * radius;    
        // let y = (0.5 *t).sin() * (t.pow(2.0).sin() * radian % 360.0).cos() * 2.0 * radius;    
        (pt2(x,y), srgba((t * 0.3) % 1.0, 0.0, 0.0, 1.0))          
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(4.0)
        .colored_points(points)
        .rotate(-t.sin());         // tell our PathStroke Builder to draw lines connecting our array of points

    

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
