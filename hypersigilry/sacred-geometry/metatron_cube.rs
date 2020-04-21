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

    // Innermost circle
    let radius = 50.0;
    let points = (0..=360).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .points_closed(points);    


    // Inner hexagon
    let radius = 100.0;
    let points = (0..=360).step_by(60).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .hsv(t, 1.0, 1.0)
        .rotate(2.0 * t)
        .points_closed(points);    
    let points = (0..=360).step_by(60).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .hsv(t, 1.0, 1.0)
        .rotate(-2.0 * t)
        .points_closed(points);    
    let points = (0..=360).step_by(60).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .points_closed(points);    

    // Inner Trianges
    let radius = 100.0;
    let points = (0..=360).step_by(120).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .hsv(t, 1.0, 1.0)
        .rotate(3.0 * t)
        .points_closed(points);    
    let points = (0..=360).step_by(120).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .rotate(deg_to_rad(180.0))
        .hsv(t, 1.0, 1.0)
        .rotate(-3.0 * t)
        .points_closed(points);    
    let points = (0..=360).step_by(120).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .points_closed(points);    
    let points = (0..=360).step_by(120).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .rotate(deg_to_rad(180.0))
        .points_closed(points);  
        
    // Outer Trianges
    let radius = 200.0;
    let points = (0..=360).step_by(120).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .rotate(t)
        .hsv(t, 1.0, 1.0)
        .points_closed(points);    
    let points = (0..=360).step_by(120).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .rotate(deg_to_rad(180.0))
        .rotate(-t)
        .hsv(t, 1.0, 1.0)
        .points_closed(points);    
    let points = (0..=360).step_by(120).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .points_closed(points);    
    let points = (0..=360).step_by(120).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .rotate(deg_to_rad(180.0))
        .points_closed(points);  

    // Outer hexagon
    let points = (0..=360).step_by(60).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .rotate(t)
        .hsv(t, 1.0, 1.0)
        .points_closed(points);  
    let points = (0..=360).step_by(60).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .rotate(-t)
        .hsv(t, 1.0, 1.0)
        .points_closed(points);    
    let points = (0..=360).step_by(60).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .points_closed(points);    


  

    for j in (0..=360_i32).step_by(60) {      
        let radian = deg_to_rad((j % 360) as f32);
        let x_1 = radian.sin() * radius;    
        let y_1 = radian.cos() * radius;    
        let x_2 =  (radian + deg_to_rad(180.0)).sin() * radius;    
        let y_2 = (radian + deg_to_rad(180.0)).cos() * radius;  
        let x_3 = (radian + deg_to_rad(120.0)).sin() * radius / 2.0;
        let y_3 = (radian + deg_to_rad(120.0)).cos() * radius / 2.0;
        let x_4 = (radian + deg_to_rad(240.0)).sin() * radius / 2.0;
        let y_4 = (radian + deg_to_rad(240.0)).cos() * radius / 2.0;
        
        
        draw.line()
            .weight(3.0)
            .color(WHITE)
            .points(pt2(x_1, y_1), pt2(x_2, y_2));

        // Draw skinny triangles
        let radius = 200.0;
        let points = (0..=360).step_by(120).map(|i| {   
            if i == 0 || i == 360 {
                let radian = deg_to_rad((i % 360) as f32);
                let x = (radian + deg_to_rad(j as f32)).sin() * radius;    
                let y = (radian + deg_to_rad(j as f32)).cos() * radius;    
                pt2(x,y)             
            } else {
                let radian = deg_to_rad((i % 360) as f32);
                let x = (radian + deg_to_rad(j as f32)).sin() * radius / 2.0;    
                let y = (radian + deg_to_rad(j as f32)).cos() * radius / 2.0;                   
                pt2(x,y)             
            }  
        });
        draw.polyline()                       // create a PathStroke Builder object
            .weight(3.0)
            .color(WHITE)
            .points_closed(points);    

        
        // Outermost small circles
        let radius = 50.0;
        let points = (0..=360).map(|i| {      
            let radian = deg_to_rad((i % 360) as f32);
            let x = radian.sin() * radius;    
            let y = radian.cos() * radius;    
            pt2(x,y)             
        });
        draw.polyline()                       // create a PathStroke Builder object
            .weight(3.0)
            .x_y(x_1, y_1)
            .points_closed(points);    
        
    };

    let radius = 100.0;
    for i in (0..=360_i32).step_by(60) {      
        let radian = deg_to_rad((i % 360) as f32);
        let x_1 = radian.sin() * radius;    
        let y_1 = radian.cos() * radius;    

        
        // Outermost 2 circles
        let radius = 50.0;
        let points = (0..=360).map(|i| {      
            let radian = deg_to_rad((i % 360) as f32);
            let x = radian.sin() * radius;    
            let y = radian.cos() * radius;    
            pt2(x,y)             
        });
        draw.polyline()                       // create a PathStroke Builder object
            .weight(3.0)
            .x_y(x_1, y_1)
            .points_closed(points);    
        
    };

    // Outer Circle
    let radius = 250.0;
    let points = (0..=360).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .points_closed(points);  

            // Outer Circle
    let radius = 250.0;
    let points = (0..=360).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .points_closed(points);  

    let points = (0..=360).map(|i| {      
        let u = t % 600.0;
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius  * u % (10000.0 * u.sin());    
        let y = radian.cos() * radius  * u % (10000.0 * u.cos());    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .hsv(t, 1.0, 1.0)
        .points_closed(points);  


    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
