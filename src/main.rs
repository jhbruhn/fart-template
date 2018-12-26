use fart::svg::{self, node::element};

fn gen(config: &mut fart::Config, document: svg::Document) -> fart::Result<()> {
    let data = element::Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();

    let path = element::Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data);

    document.set("viewBox", (0, 0, 70, 70)).add(path)
}

fn main() {
    fart::generate(gen);
}
