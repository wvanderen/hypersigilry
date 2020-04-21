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
    let radius = 100.0;
    let points = (0..=360).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .hsv((0.4 * t) % 1.0, 1.0, 1.0)
        .points_closed(points);    

    for i in (0..=360_i32).step_by(60) {      
        let radian = deg_to_rad((i % 360) as f32);
        let x_1 = radian.sin() * radius;    
        let y_1 = radian.cos() * radius;    

        
        // Outermost 2 circles
        let points = (0..=360).map(|i| {      
            let radian = deg_to_rad((i % 360) as f32);
            let x = radian.sin() * radius;    
            let y = radian.cos() * radius;    
            pt2(x,y)             
        });
        draw.polyline()                       // create a PathStroke Builder object
            .weight(3.0)
            .x_y(x_1, y_1)
            .hsv((0.2 * t + 0.25) % 1.0, 1.0, 1.0)
            .points(points); 
            
            let outer_radian = radian;
            
        for i in (0..=60_i32).step_by(60) {      
            let radian = outer_radian + deg_to_rad((i % 360) as f32);
            let x_2 = radian.sin() * radius + x_1;    
            let y_2 = radian.cos() * radius + y_1;    
    
            
            // Outermost 2 circles
            let points = (0..=360).map(|i| {      
                let radian = deg_to_rad((i % 360) as f32);
                let x = radian.sin() * radius;    
                let y = radian.cos() * radius;    
                pt2(x,y)             
            });
            draw.polyline()                       // create a PathStroke Builder object
                .weight(3.0)
                .x_y(x_2, y_2)
                .hsv((0.1 * t + 0.5) % 1.0, 1.0, 1.0)
                .points(points);    
            let outer_radian = radian;
            for i in (0..=60_i32).step_by(60) {      
            let radian = outer_radian + deg_to_rad((i % 360) as f32);
            let x_3= radian.sin() * radius + x_2 * 0.01 * radius * (0.5 * t).cos();    
            let y_3 = radian.cos() * radius + y_2 * 0.01 * radius * (0.5 * t).cos();    
    
            
            // Outermost 2 circles
            let points = (0..=360).map(|i| {      
                let radian = deg_to_rad((i % 360) as f32);
                // let x = radian.sin() * radius + radius * t.sin() / 0.29 * (0.2 * t).sin();    
                // let y = radian.cos() * radius + radius * t.cos() / 0.29 * (0.2 * t).sin();    
                let x = radian.sin() * radius;    
                let y = radian.cos() * radius;    
                pt2(x,y)             
            });
            draw.polyline()                       // create a PathStroke Builder object
                .weight(3.0)
                .hsv((0.05 * t + 0.75) % 1.0, 1.0, 1.0)
                .x_y(x_3 , y_3)
                .points(points);    
                let outer_radian = radian;
                for i in (0..=60_i32).step_by(60) {      
                    let radian = outer_radian + deg_to_rad((i % 360) as f32);
                    let x_4 = radian.sin() * radius + x_3 * 0.01 * radius * (0.2 * t).cos();    
                    let y_4 = radian.cos() * radius + y_3 * 0.01 * radius * (0.2 * t).cos();    
            
                    
                    // Outermost 2 circles
                    let points = (0..=360).map(|i| {      
                        let radian = deg_to_rad((i % 360) as f32);
                        // let x = radian.sin() * radius + radius * t.sin() / 0.29 * (0.2 * t).sin();    
                        // let y = radian.cos() * radius + radius * t.cos() / 0.29 * (0.2 * t).sin();    
                        let x = radian.sin() * radius;    
                        let y = radian.cos() * radius;    
                        pt2(x,y)             
                    });
                    draw.polyline()                       // create a PathStroke Builder object
                        .weight(3.0)
                        .hsv((0.05 * t + 0.9) % 1.0, 1.0, 1.0)
                        .x_y(x_4 , y_4)
                        .points(points);    
                            
                };
                    
            };
            
        };

                    
        

        
        
    };





    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
