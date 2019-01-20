use fart::svg::{self, node::element};

fn main() {
    fart::generate(gen);
}

fn gen(cfg: &mut fart::Config, document: svg::Document) -> fart::Result<svg::Document> {
    let data = element::path::Data::new()
        .move_to((250, 250))
        .line_by((0, 500))
        .line_by((500, 0))
        .line_by((0, -500))
        .close();

    let path = element::Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data);

    Ok(document.set("viewBox", (0, 0, 1000, 1000)).add(path))
}
