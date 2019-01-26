use fart::{aabb, path, scene, shape, svg, Point2};

fn main() {
    fart::generate(gen);
}

fn gen(_cfg: &mut fart::Config) -> fart::Result<svg::Document> {
    let mut scene = scene::Scene::new(aabb::AxisAlignedBoundingBox::new(
        Point2::new(0.0, 0.0),
        Point2::new(1000.0, 1000.0),
    ));

    scene.add(Triangle(
        Point2::new(500.0, 250.0),
        Point2::new(750.0, 750.0),
        Point2::new(250.0, 750.0),
    ));

    Ok(scene.create_svg(scene::Inches(7.0), scene::Inches(7.0)))
}

#[derive(Debug)]
struct Triangle(Point2, Point2, Point2);

impl shape::Shape for Triangle {
    fn paths(&self) -> Vec<path::Path> {
        vec![path::Path::with_commands(vec![
            path::LineCommand::MoveTo(self.0),
            path::LineCommand::LineTo(self.1),
            path::LineCommand::LineTo(self.2),
            path::LineCommand::Close,
        ])]
    }

    fn aabb(&self) -> aabb::AxisAlignedBoundingBox {
        use std::f64;
        let mut min = self.0;
        let mut max = self.0;
        for p in [self.1, self.2].iter() {
            min.x = f64::min(min.x, p.x);
            min.y = f64::min(min.y, p.y);
            max.x = f64::max(max.x, p.x);
            max.y = f64::max(max.y, p.y);
        }
        aabb::AxisAlignedBoundingBox::new(min, max)
    }
}
