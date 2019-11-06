// use nannou::prelude::*;

// fn main() {
//     nannou::app(model)
//         .update(update)
//         .simple_window(view)
//         .run();
// }

// struct Model {}

// fn model(_app: &App) -> Model {
//     Model {}
// }

// fn update(_app: &App, _model: &mut Model, _update: Update) {
// }

// fn view(_app: &App, _model: &Model, frame: &Frame){
//     frame.clear(PURPLE);
// }

use fart::prelude::*;
use fart::shape;
use fart::euclid::{point2, UnknownUnit};
use fart::fart_2d_geom::Polygon;

fn main() {
    fart::generate(|cfg| {
        let mut scene = Scene::new(Aabb::new(point2(0, 0), point2(1000, 1000)));

        let x_dist = Uniform::new(0, 1000);
        let y_dist = Uniform::new(0, 1000);

        let count = 5;

        for x in 0..count {
            for y in 0..count {
                // println!("{}", x); // x: i32
                // let result = if a > b { a } else { b };
                let u = { x as f64 * (count as f64 - 1.0) };
                let v = { y as f64 * (count as f64 - 1.0) };
                // (x_dist.sample(u), y_dist.sample(v)))

                // let triangle: Polygon<i32, UnknownUnit> = Polygon::new(vec![
                //     point2(x_dist.sample(cfg.rng()), y_dist.sample(cfg.rng())),
                //     point2(x_dist.sample(cfg.rng()), y_dist.sample(cfg.rng())),
                //     point2(x_dist.sample(cfg.rng()), y_dist.sample(cfg.rng())),
                // ]);

                // scene.add(shape::Shape {
                //     a: point2(x_dist.sample(cfg.rng()), y_dist.sample(cfg.rng())),
                //     b: point2(x_dist.sample(cfg.rng()), y_dist.sample(cfg.rng())),
                //     // b: 3, 4,
                // });
                // scene.add(triangle);
                scene.add(shape::Triangle {
                    a: point2(u as i64, v as i64),
                    b: point2(x_dist.sample(cfg.rng()), y_dist.sample(cfg.rng())),
                    c: point2(x_dist.sample(cfg.rng()), y_dist.sample(cfg.rng())),
                });
            }
        }

        Ok(scene.create_svg(Inches(10.0), Inches(10.0)))
    });
}
