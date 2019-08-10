use fart::geom::line;
use fart::prelude::*;

fn main() {
    fart::generate(|_cfg| {
        let mut canvas = Canvas::new(Aabb::new(point2(0, 0), point2(1000, 1000)));

        let x_dist = Uniform::new(0, 1000);
        let y_dist = Uniform::new(0, 1000);
        let rng = &mut fart::rng();

        let mut random_point = || point2(x_dist.sample(rng), y_dist.sample(rng));

        let random_line = line(random_point(), random_point());

        canvas.draw(&random_line);

        Ok(canvas.create_svg(Inches(7.0), Inches(7.0)))
    });
}
