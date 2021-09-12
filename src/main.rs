use macroquad::prelude::*;

const LENGTH1: f32 = 150.0;
const LENGTH2: f32 = 100.0;
const MASS1: f32 = 45.0;
const MASS2: f32 = 45.0;
const STROKEWEIGHT: f32 = 2.0;
const GRAVITY: f32 = 0.1;

struct Point(f32, f32);

// translate x from the corner to the middle
fn translate_x(x: f32) -> f32 {
    x + screen_width() / 2.0
}

// translate y from the corner to the middle
fn translate_y(y: f32) -> f32 {
    y + screen_height() / 2.0
}

#[macroquad::main("Double Pendulum")]
async fn main() {
    // angle of each pendulum
    let mut a1 = std::f32::consts::PI / 2.0;
    let mut a2 = std::f32::consts::PI / 1.8;

    // velocity of each pendulum
    let mut a1_v = 0.0;
    let mut a2_v = 0.0;

    // acceleration of each pendulum
    let mut a1_a: f32;
    let mut a2_a: f32;

    // vector of old points
    let mut trail_points = vec![];

    loop {
        // clear the canvas for this iteration
        clear_background(LIGHTGRAY);

        // draw center circle
        draw_circle(translate_x(0.0), translate_y(0.0), 2.0, BLACK);

        // set the co ordinates of the bobs
        let x1 = LENGTH1 * f32::sin(a1);
        let y1 = LENGTH1 * f32::cos(a1);
        let x2 = x1 + LENGTH2 * f32::sin(a2);
        let y2 = y1 + LENGTH2 * f32::cos(a2);

        // calculate the acceleration of pendulum 1
        let p1_numerator = (-GRAVITY * (2.0 * MASS1 + MASS2) * f32::sin(a1))
            + (-MASS2 * GRAVITY * f32::sin(a1 - 2.0 * a2))
            + (-2.0 * f32::sin(a1 - a2) * MASS2)
                * (a2_v * a2_v * LENGTH2 + a1_v * a1_v * LENGTH1 * f32::cos(a1 - a2));
        let p1_denominator =
            LENGTH1 * (2.0 * MASS1 + MASS2 - MASS2 * f32::cos(2.0 * a1 - 2.0 * a2));
        a1_a = p1_numerator / p1_denominator;

        // calculate the acceleration of pendulum 2
        let p2_numerator = (2.0 * f32::sin(a1 - a2))
            * ((a1_v * a1_v * LENGTH1 * (MASS1 + MASS2))
                + (GRAVITY * (MASS1 + MASS2) * f32::cos(a1))
                + (a2_v * a2_v * LENGTH2 * MASS2 * f32::cos(a1 - a2)));
        let p2_denominator =
            LENGTH2 * (2.0 * MASS1 + MASS2 - MASS2 * f32::cos(2.0 * a1 - 2.0 * a2));
        a2_a = p2_numerator / p2_denominator;

        // calculate the velocity and angle of both pendulums
        a1_v += a1_a;
        a2_v += a2_a;
        a1 += a1_v;
        a2 += a2_v;

        // draw pendulum 1
        draw_line(
            translate_x(0.0),
            translate_y(0.0),
            translate_x(x1),
            translate_y(y1),
            STROKEWEIGHT,
            BLACK,
        );
        draw_circle(translate_x(x1), translate_y(y1), MASS1 / 2.0, BLACK);

        // draw pendulum 2
        draw_line(
            translate_x(x1),
            translate_y(y1),
            translate_x(x2),
            translate_y(y2),
            STROKEWEIGHT,
            BLACK,
        );
        draw_circle(translate_x(x2), translate_y(y2), MASS2 / 2.0, BLACK);

        // keep the list short :)
        if trail_points.len() > 500 {
            trail_points.remove(0);
        }

        // add the latest bob to the list
        trail_points.push(Point(x2, y2));

        // draw the trails - old bob points
        for (index, _) in trail_points.iter().enumerate() {
            draw_circle(
                translate_x(trail_points[index].0),
                translate_y(trail_points[index].1),
                0.7,
                BLACK,
            );

            draw_line(
                translate_x(trail_points[index.saturating_sub(1)].0),
                translate_y(trail_points[index.saturating_sub(1)].1),
                translate_x(trail_points[index].0),
                translate_y(trail_points[index].1),
                0.5,
                BLACK,
            );
        }

        next_frame().await
    }
}
