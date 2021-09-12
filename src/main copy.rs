use macroquad::prelude::*;

const LENGTH1: f32 = 100.0;
const LENGTH2: f32 = 100.0;
const MASS1: f32 = 10.0;
const MASS2: f32 = 10.0;
const STROKEWEIGHT: f32 = 2.0;
const GRAVITY: f32 = 1.0;

fn draw() {}
fn setup() {}

struct Point(f32, f32);

fn translate(x: f32, y: f32) -> Point {
    Point(translate_x(x), translate_x(y))
}

fn translate_x(x: f32) -> f32 {
    x + screen_width() / 2.0
}
fn translate_y(y: f32) -> f32 {
    y + screen_height() / 2.0
}

#[macroquad::main("Double Pendulum")]
async fn main() {
    println!("Width: {}, Height: {}", screen_width(), screen_height());

    let mut a1 = std::f32::consts::PI / 2.0;
    let mut a2 = std::f32::consts::PI / 2.0;
    let mut a1_v = 0.0;
    let mut a2_v = 0.0;
    let mut a1_a = 0.00;
    let mut a2_a = 0.00;
    let mut trail_points = vec![];

    loop {
        clear_background(LIGHTGRAY);

        let x1 = LENGTH1 * f32::sin(a1);
        let y1 = LENGTH1 * f32::cos(a1);
        let x2 = x1 + LENGTH2 * f32::sin(a2);
        let y2 = y1 + LENGTH2 * f32::cos(a2);

        let p1_numerator1 = -GRAVITY * (2.0 * MASS1 + MASS2) * f32::sin(a1);
        let p1_numerator2 = -MASS2 * GRAVITY * f32::sin(a1 - 2.0 * a2);
        let p1_numerator3 = -2.0 * f32::sin(a1 - a2) * MASS2;
        let p1_numerator4 = a2_v * a2_v * LENGTH2 + a1_v * a1_v * LENGTH1 * f32::cos(a1 - a2);
        let p1_denominator =
            LENGTH1 * (2.0 * MASS1 + MASS2 - MASS2 * f32::cos(2.0 * a1 - 2.0 * a2));
        a1_a = (p1_numerator1 + p1_numerator2 + p1_numerator3 * p1_numerator4) / p1_denominator;

        //let p2_numerator1 = 2.0 * f32::sin(a1 - a2);
        //let p2_numerator2 = a1_v * a1_v * LENGTH1 * (MASS1 + MASS2);
        //let p2_numerator3 = GRAVITY * (MASS1 + MASS2) * f32::cos(a1);
        //let p2_numerator4 = a2_v * a2_v * LENGTH2 * MASS2 * f32::cos(a1 - a2);
        //a2_a = p2_numerator1 * (p2_numerator2 + p2_numerator3 + p2_numerator4)
        //    / (LENGTH2 * denominator);

        // draw pendulum 1
        draw_line(
            translate_x(0.0),
            translate_y(0.0),
            translate_x(x1),
            translate_y(y1),
            STROKEWEIGHT,
            BLACK,
        );
        draw_circle(translate_x(x1), translate_y(y1), MASS1, BLACK);

        // draw pendulum 2
        draw_line(
            translate_x(x1),
            translate_y(y1),
            translate_x(x2),
            translate_y(y2),
            STROKEWEIGHT,
            BLACK,
        );
        draw_circle(translate_x(x2), translate_y(y2), MASS2, BLACK);

        // draw the trail
        // keep the vector small
        if trail_points.len() > 1000 {
            trail_points.remove(0);
        }

        trail_points.push(Point(x2, y2));

        draw_circle(translate_x(x2), translate_y(y2), 1.0, BLACK);
        for (index, _) in trail_points.iter().enumerate() {
            //draw_circle(
            //    translate_x(trail_points[index].0),
            //    translate_y(trail_points[index].1),
            //    1.0,
            //    BLACK,
            //);

            draw_line(
                translate_x(trail_points[index.saturating_sub(1)].0),
                translate_y(trail_points[index.saturating_sub(1)].1),
                translate_x(trail_points[index].0),
                translate_y(trail_points[index].1),
                0.5,
                BLACK,
            );
        }

        /*         let test = translate(0.0, 0.0);
        println!("{},{}", test.0, test.1); */
        a1_v += a1_a;
        a2_v += a2_a;
        a1 += a1_v;
        a2 -= a2_v;

        //a1_v *= 0.9;
        //a2_v *= 0.9;

        next_frame().await
    }
}
