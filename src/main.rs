use plotly::charts::Scatter;
use plotly::Plot;

fn main() {
    // Create data for the Gantt chart
    let task_data = vec![
        (1, 901, 910),
        (2, 905, 915),
        (3, 912, 920),
        (3, 812, 820),
        (2, 942, 950),
        (3, 612, 620),
        (1, 312, 330),
        (3, 543, 556),
        (3, 112, 132),
        (1, 815, 883),
        (2, 762, 860),
        (3, 232, 290),
    ];

    let mut plot = Plot::new();

    // Create a scatter plot for each task
    for (task, start_date, end_date) in task_data {
        let trace = Scatter::new(vec![start_date, end_date], vec![task.clone(), task.clone()]);
        plot.add_trace(trace);
    }

    // Display the chart (in a Jupyter notebook or as an HTML file)
    plot.show();
}
