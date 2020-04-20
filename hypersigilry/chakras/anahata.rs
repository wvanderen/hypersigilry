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

    // Construct the tescellating pattern
    let radius = 200.0;
    let points = (0..=360).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius * t % 1200.0; // + radian.cos() * radius;    
        let y = radian.cos() * radius * t % 1200.0; //  + radian.sin() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(4.0)
        .rgb(t.sin().abs(), 1.0, t.sin().abs())
        // .hsv((0.2 * t) % 1000.0, 1.0, 1.0)
        // .color(GREEN)
        .points_closed(points);  
    
    // Construct the main circle
    let points = (0..=360).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(5.0)
        .color(WHITE)
        .points_closed(points); 

    // Pink Hexagons
    let radius = 240.0;
    let points = (0..=360).step_by(60).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(5.0)
        .color(PINK)
        .rotate(2.0 * t)
        .points_closed(points); 

    let points = (0..=360).step_by(60).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(5.0)
        .color(PINK)
        .rotate(-2.0 * t)
        .points_closed(points); 

    let points = (0..=360).step_by(120).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius + t.sin();    
        let y = radian.cos() *  radius + t.sin();    
        pt2(x,y)             
    });
    draw.polygon()                       // create a PathStroke Builder object
        .stroke_weight(5.0)
        .color(rgba(0.0, 255.0, 0.0, 0.5))
        .rotate(t)
        .points(points);           
    let points = (0..=360).step_by(120).map(|i| {      
        let radian = deg_to_rad((i + 180 % 360) as f32);
        let x = radian.sin() * radius + t.sin();    
        let y = radian.cos() * radius + t.sin();    
        pt2(x,y)             
    });
    draw.polygon()                       // create a PathStroke Builder object
        .stroke_weight(5.0)
        .color(rgba(0.0, 255.0, 0.0, 0.5))
        .rotate(-t)
        .points(points);        

    let radius = 100.0;
    let points = (0..=360).step_by(120).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius + t.sin();    
        let y = radian.cos() *  radius + t.sin();    
        pt2(x,y)             
    });
    draw.polygon()                       // create a PathStroke Builder object
        .stroke_weight(5.0)
        .color(rgba(0.0, 255.0, 0.0, 0.5))
        .rotate(2.0 * t)
        .points(points);           
    let points = (0..=360).step_by(120).map(|i| {      
        let radian = deg_to_rad((i + 180 % 360) as f32);
        let x = radian.sin() * radius + t.sin();    
        let y = radian.cos() * radius + t.sin();    
        pt2(x,y)             
    });
    draw.polygon()                       // create a PathStroke Builder object
        .stroke_weight(5.0)
        .color(rgba(0.0, 255.0, 0.0, 0.5))
        .rotate(2.0 * t)
        .points(points);    

    let radius = 100.0;
    let points = (0..=360).step_by(60).map(|i| {      
        let radian = deg_to_rad((i + 180 % 360) as f32);
        let x = (t * radian).sin() * 2.0 * radius;    
        let y = (t * radian).cos() * 2.0 * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .stroke_weight(5.0)
        .color(PINK)
        .rotate(2.0 * (0.5 * t).sin())
        .points(points);        
    
    let radius = 50.0;
    let points = (0..=360).step_by(60).map(|i| {      
        let radian = deg_to_rad((i + 180 % 360) as f32);
        let x = (-t * radian).sin() * 2.0 * radius;    
        let y = (-t * radian).cos() * 2.0 * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .stroke_weight(3.0)
        .color(PINK)
        .rotate(-4.0 * (0.5 * t).sin())
        .points(points);   
        
    let radius = 240.0;
            

 
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
