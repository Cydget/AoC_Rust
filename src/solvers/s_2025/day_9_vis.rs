use svg::node::element::{Path, Rectangle, path::Data};
use svg::Document;
use std::error::Error;
use svg::node::element::Circle;
use svg::node::element::{Line, Text};
use svg::node::Text as TextNode;
const GLOBAL_STROKE:f64= 0.2;
//const GLOBAL_STROKE:f64= 1.0;


use rand::Rng;

/// Returns a random color like "#184a7cff"
fn random_color() -> String {
    let mut rng = rand::rng();
    format!(
        "#{:02X}{:02X}{:02X}",
        rng.random::<u8>(),
        rng.random::<u8>(),
        rng.random::<u8>(),
    )
}
pub fn create_svg(
    points: &Vec<(i64, i64)>,
    width: i64,
    height: i64,
    output_path: &str,
    solution_paths:&Vec<Path>
) -> Result<(), Box<dyn Error>> {
    if points.len() < 2 {
        return Err("Need at least two points".into());
    }

    // White background
    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", width)
        .set("height", height)
        .set("fill", "white");

    let mut data = Data::new().move_to((points[0].0, points[0].1));
    for &(x, y) in &points[1..] {
        data = data.line_to((x, y));
    }

    let path: Path = Path::new()
        .set("fill", "Gray")
        .set("fill-opacity", 0.4)
        .set("stroke", "black")
        .set("stroke-width", GLOBAL_STROKE)
        .set("d", data);


    let dots: Vec<Circle> = points.iter().map(|&(x, y)| {
        Circle::new()
            .set("cx", x)
            .set("cy", y)
            .set("r", 0.5)
            .set("fill", "Red")
    }).collect();

    let mut document = Document::new()
        .set("viewBox", (0, 0, width, height))
        .set("width", width)
        .set("height", height)
        // background MUST be added first
        .add(background)
        .add(path);




    for sol in solution_paths{
        let p  = sol.clone();
        document = document.clone().add(p);
    }
    for circle in dots {
        document = document.add(circle);
    }
    //for elem in draw_grid_and_axes(width, height, 1) {
    //    document = document.add(elem);
    //}
    svg::save(output_path, &document)?;
    Ok(())
}

pub fn get_square_path(    points: &Vec<(i64, i64)>)->Path{
    let stroke_color = random_color();

    let mut data = Data::new().move_to((points[0].0, points[0].1));
    for &(x, y) in &points[1..] {
        data = data.line_to((x, y));
    }
    let path = Path::new()
        .set("fill", stroke_color.clone())
        .set("fill-opacity", 0.4)

        .set("stroke", stroke_color)
        .set("stroke-width", GLOBAL_STROKE)
        .set("d", data);
    path
}

fn draw_grid_and_axes(
    width: i64,
    height: i64,
    step: i64,
) -> Vec<svg::node::element::Element> {
    let mut elements: Vec<svg::node::element::Element> = Vec::new();

    let grid_color = "#4b1414ff";
    let axis_color = "#5600a7ff";

    // Vertical grid lines + X ticks
    for x in (0..=width).step_by(step as usize) {
        // Grid line
        elements.push(
            Line::new()
                .set("x1", x)
                .set("y1", 0)
                .set("x2", x)
                .set("y2", height)
                .set("stroke", grid_color)
                .set("stroke-width", 0.1)
                .into(),
        );

        // Tick label
        elements.push(
            Text::new("")
                .set("x", x)
                .set("y", height + 4)
                .set("font-size", 3)
                .set("text-anchor", "middle")
                .add(TextNode::new(x.to_string()))
                .into(),
        );
    }

    // Horizontal grid lines + Y ticks
    for y in (0..=height).step_by(step as usize) {
        elements.push(
            Line::new()
                .set("x1", 0)
                .set("y1", y)
                .set("x2", width)
                .set("y2", y)
                .set("stroke", grid_color)
                .set("stroke-width", 1)
                .into(),
        );

        elements.push(
            Text::new("")
                .set("x", -2)
                .set("y", y + 1)
                .set("font-size", 3)
                .set("text-anchor", "end")
                .add(TextNode::new(y.to_string()))
                .into(),
        );
    }

    // X axis
    elements.push(
        Line::new()
            .set("x1", 0)
            .set("y1", 0)
            .set("x2", width)
            .set("y2", 0)
            .set("stroke", axis_color)
            .set("stroke-width", 1)
            .into(),
    );

    // Y axis
    elements.push(
        Line::new()
            .set("x1", 0)
            .set("y1", 0)
            .set("x2", 0)
            .set("y2", height)
            .set("stroke", axis_color)
            .set("stroke-width", 1)
            .into(),
    );

    elements
}