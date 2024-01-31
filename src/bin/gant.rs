use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 데이터 포인트 생성
    let tasks = vec![
        ("Task 1", (1, 5)),
        ("Task 2", (3, 7)),
        ("Task 3", (6, 10)),
    ];

    // 차트 생성
    let root = BitMapBackend::new("gantt_chart.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(5)
        .caption("Gantt Chart Example", ("sans-serif", 20))
        .build_cartesian_2d(0..12, 0..1)?;

    chart.configure_mesh().draw()?;

    // Gantt 바 그리기
    for (_task, (start, end)) in tasks {
        chart.draw_series(
            (start..end)
                .map(|x| {
                    Rectangle::new(
                        [(x, 0), (x + 1, 1)],
                        HSLColor(240.0, 1.0, 0.5).filled(),
                    )
                })
                .collect::<Vec<_>>(),
        )?;
    }

    Ok(())
}
