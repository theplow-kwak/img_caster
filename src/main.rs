use plotly::charts::{Layout, RgbColor, Line, Color, Dim, Marker, Mode, Scatter, Title, Axis};
use plotly::Plot;

fn colored_and_styled_scatter_plot() {
    let trace1 = Scatter::new(vec![52698, 43117], vec![53, 31])
        .mode(Mode::Markers)
        .name("North America")
        .text(vec!["United States".to_owned(), "Canada".to_owned()])
        .marker(Marker::new()
            .color(Dim::Scalar(Color::Rgb(RgbColor::new(164, 194, 244))))
            .size(Dim::Scalar(12))
            .line(Line::new().color(Color::White).width(0.5)));
    let trace2 = Scatter::new(vec![39317, 37236, 35650, 30066, 29570, 27159, 23557, 21046, 18007],
                              vec![33, 20, 13, 19, 27, 19, 49, 44, 38])
        .mode(Mode::Markers)
        .name("Europe")
        .text(vec!["Germany".to_owned(), "Britain".to_owned(), "France".to_owned(), "Spain".to_owned(),
                   "Italy".to_owned(), "Czech Rep.".to_owned(), "Greece".to_owned(), "Poland".to_owned()])
        .marker(Marker::new()
            .color(Dim::Scalar(Color::Rgb(RgbColor::new(255, 217, 102))))
            .size(Dim::Scalar(12)));
    let trace3 = Scatter::new(vec![42952, 37037, 33106, 17478, 9813, 5253, 4692, 3899],
                              vec![23, 42, 54, 89, 14, 99, 93, 70])
        .mode(Mode::Markers)
        .name("Asia/Pacific")
        .text(vec!["Australia".to_owned(), "Japan".to_owned(),
                   "South Korea".to_owned(), "Malaysia".to_owned(),
                   "China".to_owned(), "Indonesia".to_owned(), "Philippines".to_owned(), "India".to_owned()])
        .marker(Marker::new()
            .color(Dim::Scalar(Color::Rgb(RgbColor::new(234, 153, 153))))
            .size(Dim::Scalar(12)));
    let trace4 = Scatter::new(vec![19097, 18601, 15595, 13546, 12026, 7434, 5419],
                              vec![43, 47, 56, 80, 86, 93, 80])
        .mode(Mode::Markers)
        .name("Latin America")
        .text(vec!["Chile".to_owned(), "Argentina".to_owned(), "Mexico".to_owned(),
                   "Venezuela".to_owned(), "Venezuela".to_owned(), "El Salvador".to_owned(), "Bolivia".to_owned()])
        .marker(Marker::new()
            .color(Dim::Scalar(Color::Rgb(RgbColor::new(142, 124, 195))))
            .size(Dim::Scalar(12)));

    let layout = Layout::new()
        .title(Title::new("Quarter 1 Growth"))
        .xaxis(Axis::new().title(Title::new("GDP per Capita")).show_grid(false).zero_line(false))
        .yaxis(Axis::new().title(Title::new("Percent")).show_line(false));
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);
    plot.add_trace(trace4);
    plot.add_layout(layout);
    plot.show();
}

fn main() -> std::io::Result<()> {
    colored_and_styled_scatter_plot();
    Ok(())
}