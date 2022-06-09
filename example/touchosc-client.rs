use nannou::prelude::*;
use nannou_touchosc::TouchOscClient;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    win: window::Id,
    touchosc: TouchOscClient,
}

fn model(app: &App) -> Model {
    let win = app.new_window().size(400, 400).view(view).build().unwrap();
    let mut touchosc = TouchOscClient::new(6555);

    touchosc.add_button("/shape1", true);   //toggle button, initialized as true
    touchosc.add_radio("/invert", 2, 0); //add radio button with 2 options, initialized at zero index
    touchosc.add_grid("/grid", 2, 3.0, 24.0, 10.0);
    touchosc.add_encoder("/rotate", 0.0, PI*2.0, 0.0);
    touchosc.add_radial("/offset", 0.0, 10.0, 0.0);
    touchosc.add_fader("/color_r", 0.0, 1.0, 1.0);
    touchosc.add_fader("/color_g", 0.0, 1.0, 0.0);
    touchosc.add_fader("/color_b", 0.0, 1.0, 1.0);
    touchosc.add_fader("/color_a", 0.0, 1.0, 1.0);
    touchosc.add_fader("/polyline/points", 3.0, 8.0, 4.0);
    touchosc.add_xy("/scale",0.1, 3.0, 1.0);
    touchosc.add_fader("/rect/stroke_width", 1.0, 10.0, 2.0);
    touchosc.add_fader("/vertices", 3.0, 8.0, 3.0);
    touchosc.add_radar("/scale_rotate", (0.1, 1.0, 1.0), (0.0, PI*2.0, PI/4.0));
    
    Model { win, touchosc }
}

fn update(a: &App, m: &mut Model, _update: Update) {
    m.touchosc.update(); //receive touchosc messages
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();

    let invert = m.touchosc.radio("/invert");

    if invert > 0 {
        draw.background().color(BLACK);
    } else {
        draw.background().color(WHITE);
    }
    
    let win_w = app.window_rect().w();
    let win_h = app.window_rect().h();
    let scale = m.touchosc.radar("/scale_rotate").x;
    let rotate = m.touchosc.radar("/scale_rotate").y;
    let offset  = m.touchosc.radial("/offset");
    let shape1 = m.touchosc.button("/shape1");
    let grid_margin = 100.0;
    let grid_rotate = m.touchosc.encoder("/rotate");
    let rows = m.touchosc.grid("/grid/1");
    let cols = m.touchosc.grid("/grid/2");
    let x_space = ( (win_w) - grid_margin) / cols;
    let y_space = ( (win_h) - grid_margin) / rows;
    let x_off = -win_w/2.0 + grid_margin/2.0;
    let y_off = -win_h/2.0 + grid_margin/2.0;
    let r_stroke  = m.touchosc.fader("/rect/stroke_width");
    let x_scale = m.touchosc.xy("/scale").x;
    let y_scale = m.touchosc.xy("/scale").y;
    let steps = m.touchosc.fader("/vertices").round() as usize;

    let fill_color = rgba(
        m.touchosc.fader("/color_r"),
        m.touchosc.fader("/color_g"),
        m.touchosc.fader("/color_b"),
        m.touchosc.fader("/color_a")
    );
    let stroke_color = match m.touchosc.radio("/invert") { 0 => BLACK, 1 => WHITE, _ => BLACK };

    // let draw = draw.rotate(PI/3.0);
    let draw = draw.rotate(grid_rotate).translate(pt3(x_off, y_off, 0.0));
    
    for c in 1..cols as i32 {
        
        for r in 1..rows as i32 {
            let n = rows * cols;
            let f = (c * r) as f32 / n;
            let w = (rotate).sin() * (rotate + f * PI * 2.0).cos();
            let rotation = rotate + (w * offset);
            let x = x_space * c as f32;
            let y = y_space * r as f32;

            if shape1 {
                let radius = 20.0;
                let points = (0..=360).step_by(360/steps).map(|i| {
                    let radian = deg_to_rad(i as f32);
                    let x = radian.sin() * radius;
                    let y = radian.cos() * radius;
                    pt2(x*x_scale,y*y_scale)
                });
                draw
                .translate(pt3(x, y, 0.0))
                .rotate(rotation)
                .scale(scale)
                .polygon()
                .stroke_weight(r_stroke)
                .stroke_color(stroke_color)
                .color(fill_color)
                .points(points);
            }

        }
    }


    draw.to_frame(app, &frame).unwrap();
}
