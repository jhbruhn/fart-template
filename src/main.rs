use fart::prelude::*;

fn main() {
    fart::generate(|_cfg| {
        let mut canvas = Canvas::new(Aabb::new(point2(0, 0), point2(1000, 1000)));

        let x_dist = Uniform::new(0, 1000);
        let y_dist = Uniform::new(0, 1000);
        let rng = &mut fart::rng();

        let mut triangle = || {
            let mut vs = vec![
                point2(x_dist.sample(rng), y_dist.sample(rng)),
                point2(x_dist.sample(rng), y_dist.sample(rng)),
                point2(x_dist.sample(rng), y_dist.sample(rng)),
            ];
            if !fart::geom::is_counter_clockwise(&vs) {
                vs.reverse();
            }
            fart::geom::Polygon::new(vs)
        };

        fart::user_const! {
            const TRIS: usize = 5;
        }

        for _ in 0..*TRIS {
            canvas.draw(&triangle());
        }

        Ok(canvas.create_svg(Inches(7.0), Inches(7.0)))
    });
}
