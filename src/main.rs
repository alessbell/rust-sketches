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

fn main() {
    fart::generate(|cfg| {
        let mut scene = Scene::new(Aabb::new(point2(0, 0), point2(0, 1000)));

        let x_dist = Uniform::new(0, 1000);
        let y_dist = Uniform::new(0, 1000);

        scene.add(shape::Triangle {
            a: point2(x_dist.sample(cfg.rng()), y_dist.sample(cfg.rng())),
            b: point2(x_dist.sample(cfg.rng()), y_dist.sample(cfg.rng())),
            c: point2(x_dist.sample(cfg.rng()), y_dist.sample(cfg.rng())),
        });

        Ok(scene.create_svg(Inches(7.0), Inches(7.0)))
    });
}
