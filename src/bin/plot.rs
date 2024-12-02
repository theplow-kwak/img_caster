use plotly::color::NamedColor;
use plotly::layout::{
    Axis, Layout, Shape, ShapeLine, ShapeType,
};
use plotly::common::Mode;
use plotly::{Plot, Scatter};

fn vertical_and_horizontal_lines_positioned_relative_to_axes(show: bool) {
    let trace = Scatter::new(vec![2.0, 3.5, 6.0], vec![1.0, 1.5, 1.0])
        .text_array(vec![
            "Vertical Line",
            "Horizontal Dashed Line",
            "Diagonal dotted Line",
        ])
        .mode(Mode::Text);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let mut layout = Layout::new()
        .x_axis(Axis::new().range(vec![0.0, 7.0]))
        .y_axis(Axis::new().range(vec![0.0, 2.5]));

    layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Line)
            .x0(1)
            .y0(0)
            .x1(1)
            .y1(2)
            .line(ShapeLine::new().color(NamedColor::RoyalBlue).width(3.)),
    );
    layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Line)
            .x0(2)
            .y0(2)
            .x1(5)
            .y1(2)
            .line(ShapeLine::new().color(NamedColor::LightSeaGreen).width(3.)),
    );
    layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Line)
            .x0(4)
            .y0(0)
            .x1(6)
            .y1(2)
            .line(ShapeLine::new().color(NamedColor::MediumPurple).width(3.)),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some(
            "vertical_and_horizontal_lines_positioned_relative_to_axes"
        ))
    );
}

// fn multiple_ndarray_traces_over_columns(show: bool) {
//     let t: Array<f64, Ix1> = Array::range(0., 10., 5.);
//     let mut ys: Array<f64, Ix2> = Array::zeros((5, 11));
//     let mut count = 0.;
//     for mut row in ys.columns_mut() {
//         for index in 0..row.len() {
//             row[index] = count + (index as f64).powf(3.);
//         }
//         count += 1.;
//     }
//     println!("{}", ys);

//     let traces =
//         Scatter::default()
//             .mode(Mode::LinesMarkers)
//             .to_traces(t, ys, ArrayTraces::OverRows);

//     let mut plot = Plot::new();
//     plot.add_traces(traces);
//     if show {
//         plot.show();
//     }
//     println!(
//         "{}",
//         plot.to_inline_html(Some("multiple_ndarray_traces_over_columns"))
//     );
// }

fn main() {
    vertical_and_horizontal_lines_positioned_relative_to_axes(true);
    // multiple_ndarray_traces_over_columns(true);
}
