//! Koch Snowflake — recursive generative art with Nannou
//! Each frame the recursion depth cycles 0 → 6, showing the fractal build up.

use nannou::prelude::*;

const MAX_DEPTH: u32 = 6;
const CYCLE_FRAMES: u32 = 90; // frames at each depth before advancing

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    depth: u32,
    frame_counter: u32,
}

fn model(app: &App) -> Model {
    app.new_window()
        .title("Koch Snowflake — Recursive Generative Art")
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();
    Model {
        depth: 0,
        frame_counter: 0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.frame_counter += 1;
    if model.frame_counter >= CYCLE_FRAMES {
        model.frame_counter = 0;
        model.depth = (model.depth + 1) % (MAX_DEPTH + 1);
    }
}

// --- Recursive Koch subdivision ---

/// Subdivide one edge into the Koch bump, appending points to `pts`.
fn koch_edge(a: Vec2, b: Vec2, depth: u32, pts: &mut Vec<Vec2>) {
    if depth == 0 {
        pts.push(b);
        return;
    }
    let diff = b - a;
    let p1 = a + diff / 3.0;
    let p2 = a + diff * 2.0 / 3.0;

    // Rotate the midpoint 60° outward to form the spike
    let angle = std::f32::consts::PI / 3.0;
    let spike = p1
        + Vec2::new(
            diff.x / 3.0 * angle.cos() - diff.y / 3.0 * angle.sin(),
            diff.x / 3.0 * angle.sin() + diff.y / 3.0 * angle.cos(),
        );

    koch_edge(a, p1, depth - 1, pts);
    koch_edge(p1, spike, depth - 1, pts);
    koch_edge(spike, p2, depth - 1, pts);
    koch_edge(p2, b, depth - 1, pts);
}

/// Build the full Koch snowflake polyline for the given depth.
fn build_snowflake(depth: u32, radius: f32) -> Vec<Vec2> {
    let a = Vec2::new(0.0, radius);
    let b = Vec2::new(
        radius * (std::f32::consts::PI * 7.0 / 6.0).cos(),
        radius * (std::f32::consts::PI * 7.0 / 6.0).sin(),
    );
    let c = Vec2::new(
        radius * (std::f32::consts::PI * 11.0 / 6.0).cos(),
        radius * (std::f32::consts::PI * 11.0 / 6.0).sin(),
    );

    let mut pts = vec![a];
    koch_edge(a, b, depth, &mut pts);
    koch_edge(b, c, depth, &mut pts);
    koch_edge(c, a, depth, &mut pts);
    pts
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(hsl(0.6, 0.15, 0.07));

    let t = app.time;

    // Draw 4 nested snowflakes at shrinking scales for a layered effect
    for layer in 0..4u32 {
        let radius = 300.0 * (1.0 - layer as f32 * 0.18);
        let pts = build_snowflake(model.depth, radius);

        let hue = (t * 0.05 + layer as f32 * 0.15) % 1.0;
        let alpha = 1.0 - layer as f32 * 0.2;

        draw.polyline()
            .weight(1.5 / (layer as f32 + 1.0))
            .points_closed(pts.iter().cloned())
            .color(hsla(hue, 0.8, 0.65, alpha));
    }

    draw.text(&format!("Recursion depth: {}", model.depth))
        .x_y(0.0, -370.0)
        .font_size(22)
        .color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}
