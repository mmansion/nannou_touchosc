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

    // add button inputs to the client
    touchosc.add_button("/my-toggle", true); //toggle button, initialized as true
    touchosc.add_button("/my-momentary", false); //momentary button, initialized as false
    
    Model { win, touchosc }
}

fn update(a: &App, m: &mut Model, _update: Update) {
    m.touchosc.update(); //receive touchosc messages
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();

    if m.touchosc.button("/my-momentary") {
        draw.background().color(BLUE);
    } else {
        draw.background().color(PLUM);
    }

    if m.touchosc.button("/my-toggle") {
        draw.ellipse().color(STEELBLUE);
    }

    draw.to_frame(app, &frame).unwrap();
}
