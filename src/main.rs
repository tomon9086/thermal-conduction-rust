use color::temp_to_rgb;
use nannou::color::rgb;
use nannou::{color::*, event::Update, App, Frame};

mod color;

const MIN_TEMP: f64 = 0.0;
const MAX_TEMP: f64 = 800.0;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

#[allow(dead_code)]
struct Model {
    length: i32,
    dx: f64,
    dt: f64,
    l: usize,
    kappa: f64,
    density: f64,
    cp: f64,
    alpha: f64,
    gamma: f64,
    u: Vec<f64>,
}

fn model(_app: &App) -> Model {
    let length = 2;
    let dx = 0.01;
    let dt = 0.01;
    let l = (length as f64 / dx) as usize;
    let kappa = 71.6;
    let density = 21.45;
    let cp = 25.86 * 195.084;
    let alpha = kappa / (density * cp);

    Model {
        length,
        dx,
        dt,
        l,
        kappa,
        density,
        cp,
        alpha,
        gamma: alpha * dt / (dx * dx),
        u: vec![0.0; l],
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.u[0] = 773.0;

    for i in 1..model.l - 1 {
        model.u[i] = diff(model.gamma, model.u[i - 1], model.u[i], model.u[i + 1]);
    }
    model.u[model.l - 1] = model.u[model.l - 2];
}

fn view(app: &App, model: &Model, frame: Frame) {
    let window_rect = app.window_rect();
    let draw = app.draw();
    let rect_dim = (500.0, 20.0);

    update_color(app, rect_dim, model.l, &model.u);

    draw.text(&app.elapsed_frames().to_string())
        .font_size(24)
        .x(window_rect.left() + 50.0)
        .y(window_rect.top() - 10.0)
        .color(WHITE);
    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}

fn update_color(app: &App, rect_dim: (f32, f32), n: usize, temp: &Vec<f64>) {
    let draw = app.draw();
    let w = rect_dim.0 / n as f32;
    for (i, t) in temp.iter().enumerate() {
        draw.rect()
            .w_h(w, rect_dim.1)
            .color(temp_to_rgb(MIN_TEMP, MAX_TEMP, *t))
            .x_y(-rect_dim.0 / 2.0 + (i as f32 * w), 0.0);
    }
}

fn diff(gamma: f64, u0: f64, u1: f64, u2: f64) -> f64 {
    return gamma * u2 + (1.0 - 2.0 * gamma) * u1 + gamma * u0;
}
