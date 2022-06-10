// nannou_touchosc example
// Mikhail Mansion
// https://mikhailmansion.art

use nannou::prelude::*;
use nannou_touchosc::TouchOscClient;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    touchosc: TouchOscClient,
}

fn model(app: &App) -> Model {
    app.new_window().size(600, 600).view(view).build().unwrap();

    // EXAMPLE: Initializing the client.
    // To initialize a TouchOscClient you instantiate it as "mutable"
    // and pass the OSC port where messages will be received.
    let mut touchosc = TouchOscClient::new(6555);

    touchosc.verbose();

    // EXAMPLE: Adding client inputs.
    // Any type of TouchOSC controller inputs can be added to the TouchOscClient instance.
    // Inputs are initialized by calling their respective add_ method, and passing initialization values.
    // See the README documentaiton for a breakdown of the init values used for each type of TouchOSC controller.
    touchosc.add_button("/show_points", true);
    touchosc.add_radio("/invert", 2, 0);
    touchosc.add_grid("/grid", 2, 3.0, 24.0, 10.0);
    touchosc.add_encoder("/rotate", 0.0, PI * 2.0, 0.0);
    touchosc.add_radial("/offset", 0.0, 10.0, 0.0);
    touchosc.add_fader("/color_r", 0.0, 1.0, 1.0);
    touchosc.add_fader("/color_g", 0.0, 1.0, 0.0);
    touchosc.add_fader("/color_b", 0.0, 1.0, 1.0);
    touchosc.add_fader("/color_a", 0.0, 1.0, 1.0);
    touchosc.add_xy("/scale", 0.1, 3.0, 1.0);
    touchosc.add_fader("/stroke_width", 1.0, 10.0, 2.0);
    touchosc.add_fader("/vertices", 3.0, 8.0, 3.0);
    touchosc.add_radar("/scale_rotate", (0.1, 1.0, 1.0), (0.0, PI * 2.0, PI / 4.0));

    Model { touchosc }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    // EXAMPLE: Updating values.
    // To receive OSC values from the TouchOSC controller, run the update function.
    // If messages available, they'll be routed to the associated TouchOSC client input and saved.
    // Note that values do persist after the application is terminated.
    m.touchosc.update();
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();

    // EXAMPLE: Accessing client inputs.
    // To read values from client inputs, call the respective method and pass the registered address.
    // The most recent stored value will be returned back

    //example: for "radio" inputs, an i32 is returned reprenting a radio index (0-n)
    let invert = m.touchosc.radio("/invert");

    draw.background().color(match invert {
        0 => WHITE,
        1 => BLACK,
        _ => WHITE,
    });

    let win_w = app.window_rect().w();
    let win_h = app.window_rect().h();

    // example: for "grid" inputs, a fader is accessed via its address followed by a position number (1-n)
    let rows = m.touchosc.grid("/grid/1").ceil();
    let cols = m.touchosc.grid("/grid/2").ceil();

    // example: "fader" inputs return f32
    let stroke_width = m.touchosc.fader("/stroke_width");

    let grid_margin = map_range(stroke_width, 1.0, 10.0, 100.0, 0.0);

    // example: "encoder" inputs work like faders, returning f32
    let grid_rotate = m.touchosc.encoder("/rotate");
    let grid_points = m.touchosc.button("/show_points");

    let x_space = (win_w - grid_margin) / cols;
    let y_space = (win_h - grid_margin) / rows;
    let x_off = -win_w / 2.0 + grid_margin / 2.0;
    let y_off = -win_h / 2.0 + grid_margin / 2.0;

    //example: for "xy" inputs, a vec2 is returned
    let x_scale = m.touchosc.xy("/scale").x;
    let y_scale = m.touchosc.xy("/scale").y;

    //example: "radar" inputs work like "xy", returning vec2
    let scale = m.touchosc.radar("/scale_rotate").x;
    let rotate = m.touchosc.radar("/scale_rotate").y;

    // example: "radial" inputs work like faders, returning f32
    let offset = m.touchosc.radial("/offset");

    let vertices = m.touchosc.fader("/vertices").round() as usize;

    let stroke_color = match invert {
        0 => BLACK,
        1 => WHITE,
        _ => BLACK,
    };
    let fill_color = rgba(
        m.touchosc.fader("/color_r"),
        m.touchosc.fader("/color_g"),
        m.touchosc.fader("/color_b"),
        m.touchosc.fader("/color_a"),
    );

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

            let radius = 20.0;
            let points = (0..=360).step_by(360 / vertices).map(|i| {
                let radian = deg_to_rad(i as f32);
                let x = radian.sin() * radius;
                let y = radian.cos() * radius;
                pt2(x * x_scale, y * y_scale)
            });
            draw.translate(pt3(x, y, 0.0))
                .rotate(rotation)
                .scale(scale * stroke_width * 0.1 + 1.0)
                .polygon()
                .stroke_weight(stroke_width)
                .stroke_color(stroke_color)
                .color(fill_color)
                .points(points);

            if grid_points {
                draw.ellipse().color(BLACK).x_y(x, y).radius(10.0);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
