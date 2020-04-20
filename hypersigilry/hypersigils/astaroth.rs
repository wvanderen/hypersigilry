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
    let radius = 20.0;
    let points = (0..=360).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .points_closed(points);    
    // Construct corner circles and connecting lines
    let radius = 10.0;
    let connector_x = 47.0;
    let connector_y = 110.0;;
    let points = (0..=360).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });    
    draw.polyline()
        .weight(3.0)
        .points_closed(points)
        .x_y(connector_x, connector_y); 
    
    let points = (0..=360).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()
        .weight(3.0)
        .points_closed(points)
        .x_y(connector_x, -connector_y);     

    let points = (0..=360).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });    
    draw.polyline()
        .weight(3.0)
        .points_closed(points)
        .x_y(-connector_x, -connector_y); 

    let points = (0..=360).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()
        .weight(3.0)
        .points_closed(points)
        .x_y(-connector_x, connector_y);   



    draw.line()
        .weight(3.0)
        .points(pt2(connector_x, connector_y - 10.0), pt2(connector_x, -connector_y + 10.0));

    draw.line()
        .weight(3.0)
        .points(pt2(-connector_x, connector_y - 10.0), pt2(-connector_x, -connector_y + 10.0));

    // Draw curling pieces
    let connector_y = -15.0;
    let stand_length = 60.0;
    draw.line()
        .weight(3.0)
        .points(pt2(connector_x, connector_y), pt2(connector_x + stand_length, connector_y));

    draw.line()
        .weight(3.0)
        .points(pt2(-connector_x, connector_y), pt2(-connector_x - stand_length, connector_y));
    let connector_x = connector_x + stand_length;
    
    let radius = 10.0;
    let points = (0..=360).map(|i| {      
        let degree = i as f32;
        let radian = deg_to_rad((degree % 360.0));
        let x = 2.0 * radian.sin() * radius - degree / 40.0;    
        let y = radian.cos() * radius + degree / 40.0;      
        pt2(x,y)             
    });
    draw.polyline()
        .weight(3.0)
        .points(points)
        .x_y((-connector_x + 1.1 * radius) - 0.0, connector_y - 18.8);   

    let points = (0..=360).map(|i| {      
        let degree = i as f32;
        let radian = deg_to_rad((degree % 360.0));
        let x = -2.0 * radian.sin() * radius - degree / 60.0;    
        let y = -radian.cos() * radius + degree / 60.0;      
        pt2(x,y)             
    });


    draw.polyline()
        .weight(3.0)
        .points(points)
        .x_y((-connector_x + 2.2 * radius) - 12.0, connector_y + 10.0);   


        let points = (0..=360).map(|i| {      
            let degree = -i as f32;
            let radian = deg_to_rad((degree % 360.0));
            let x = 2.0 * radian.sin() * radius - degree / 40.0;    
            let y = radian.cos() * radius - degree / 40.0;      
            pt2(x,y)             
        });
        draw.polyline()
            .weight(3.0)
            .points(points)
            .x_y(-(-connector_x + 2.1 * radius) + 10.0, connector_y - 19.5);   
    
        let points = (0..=360).map(|i| {      
            let degree = -i as f32;
            let radian = deg_to_rad((degree % 360.0));
            let x = -2.0 * radian.sin() * radius - degree / 40.0;    
            let y = -radian.cos() * radius - degree / 40.0;      
            pt2(x,y)             
        });
        draw.polyline()
            .weight(3.0)
            .points(points)
            .x_y(-((-connector_x + 2.2 * radius)) + 18.0, connector_y + 10.0);   
    

        

    




    let text_xy = 123.743; 
    let font_size = 32;
    draw.text("A")
        .color(WHITE)
        .font_size(font_size)
        .x_y(0.0, 175.0);

    draw.text("S")
        .color(WHITE)
        .font_size(font_size)
        .x_y(text_xy, text_xy);
        // .rotate(45.0);
    
    draw.text("T")
        .color(WHITE)
        .font_size(font_size)
        .x_y(175.0, 0.0);

    draw.text("A")
        .color(WHITE)
        .font_size(font_size)
        .x_y(text_xy, -text_xy);

    draw.text("R")
        .color(WHITE)
        .font_size(font_size)
        .x_y(0.0, -175.0);
        
    draw.text("O")
        .color(WHITE)
        .font_size(font_size)
        .x_y(-text_xy, -text_xy);

    draw.text("T")
        .color(WHITE)
        .font_size(font_size)
        .x_y(-175.0, 0.0);

    draw.text("H")
        .color(WHITE)
        .font_size(font_size)
        .x_y(-text_xy, text_xy);



        
        // Line from small circle down to bottom of circle
        draw.line()
        .weight(3.0)
        .points(pt2(0.0, -20.0), pt2(0.0, -130.0));
        
        // Line for cross at bottom center
        draw.line()
        .weight(3.0)
        .points(pt2(-10.0, -120.0), pt2(10.0, -120.0));

        // Construct tip points
        let radius = 140.0;   
        for element in 0..5 {
        let radian = deg_to_rad(((element as f32) * 360.0 / 5.0) % 360.0);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius; 
        
        draw.ellipse()
            .radius(5.0)
            .x_y(x, y);

        let x_left = (deg_to_rad(-5.0) + radian).sin() * (radius - 10.0);
        let y_left = (deg_to_rad(-5.0) + radian).cos() * (radius- 10.0);

        draw.ellipse()
            .radius(5.0)
            .x_y(x_left, y_left);

        let x_right = (deg_to_rad(5.0) + radian).sin() * (radius - 10.0);
        let y_right = (deg_to_rad(5.0) + radian).cos() * (radius- 10.0);

        draw.ellipse()
            .radius(5.0)
            .x_y(x_right, y_right);

        // draw.ellipse()
        //     .radius(5.0)
        //     .x_y(10.0, 130.0);

        
    }
    // Construct a pentagram
    let radius = 130.0;
    let points = (0..=1080).step_by(216).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .points_closed(points);          // tell our PathStroke Builder to draw lines connecting our array of points

    // Construct a Circle around the pentagram
    let radius = 150.0;
    let points = (0..=360).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .points_closed(points);          // tell our PathStroke Builder to draw lines connecting our array of points

    // Construct the outer circle
    let radius = 200.0;
    let points = (0..=360).map(|i| {      
        let radian = deg_to_rad((i % 360) as f32);
        let x = radian.sin() * radius;    
        let y = radian.cos() * radius;    
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(3.0)
        .points_closed(points);          // tell our PathStroke Builder to draw lines connecting our array of points






    // Construct a qliphothic star
    let points = (0..=23760).step_by(2160).map(|i| {      
        let radian = deg_to_rad(((i as f32) / 11.0) % 360.0);
        // ### Setting: Expand to infinite
        // let x = radian.sin() / t.sin() * 0.8 * radius;    
        // let y = radian.cos() / t.sin() * 0.8 * radius;   

        let x = radian.sin() * 7.0 * radius;    
        let y = radian.cos() * 7.0 * radius;   
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(2.0)
        .color(RED)
        .points_closed(points)
        .rotate(t);  


    let points = (0..=23760).step_by(2160).map(|i| {      
        let radian = deg_to_rad(((i as f32) / 11.0) % 360.0);
        // ### Setting: Expand to infinite
        // let x = radian.sin() / t.sin() * 0.8 * radius;    
        // let y = radian.cos() / t.sin() * 0.8 * radius;   

        let x = radian.sin() * 7.0 * radius;    
        let y = radian.cos() * 7.0 * radius;   
        pt2(x,y)             
    });
    draw.polyline()                       // create a PathStroke Builder object
        .weight(2.0)
        .color(GOLD)
        .points_closed(points)
        .rotate(-t);  

    
    

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
