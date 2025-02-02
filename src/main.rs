use raylib::prelude::*;

const GRAPH_X_OFFSET: i32 = 50; // Left margin
const GRAPH_Y_OFFSET: i32 = 50; // Top margin

fn main() {
    //

    let graph_width: i32 = 1600; // Width of the graph
    let graph_height: i32 = 900; // Height of the graph

    let (mut rl, thread) = raylib::init()
        .size(
            graph_width - 2 * GRAPH_X_OFFSET,
            graph_height - 2 * GRAPH_Y_OFFSET,
        )
        .title("Static Graph: Simulation Results")
        .resizable()
        .build();

    // Simulated data for a time range of 20–50 minutes
    // let time_range = (20..=50).collect::<Vec<i32>>();
    // let results = time_range
    //     .iter()
    //     .map(|minute| simulate_value(*minute))
    //     .collect::<Vec<f32>>();

    let screen_width = rl.get_screen_width();
    let screen_height = rl.get_screen_height();

    // Calculate center position
    let center_x = (get_monitor_width(0) - screen_width) / 2;
    let center_y = (get_monitor_height(0) - screen_height) / 2;

    // Set window position to the center
    rl.set_window_position(center_x, center_y);

    // rl.toggle_fullscreen();

    // Find the min and max values for scaling
    // let min_value = results.iter().cloned().fold(f32::INFINITY, f32::min);
    // let max_value = results.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        // Draw graph axes
        d.draw_line(
            GRAPH_X_OFFSET,
            GRAPH_Y_OFFSET + graph_height,
            GRAPH_X_OFFSET + graph_width,
            GRAPH_Y_OFFSET + graph_height,
            Color::BLACK,
        ); // X-axis
        d.draw_line(
            GRAPH_X_OFFSET,
            GRAPH_Y_OFFSET,
            GRAPH_X_OFFSET,
            GRAPH_Y_OFFSET + graph_height,
            Color::BLACK,
        ); // Y-axis

        let foo = get_current_monitor();
        d.draw_text(&foo.to_string(), 10, 40, 40, Color::BLUE);

        d.draw_text(&graph_width.to_string(), 30, 30, 30, Color::GREEN);
        d.draw_text(&graph_height.to_string(), 90, 90, 90, Color::GREEN);
        // Draw labels
        d.draw_text(
            "Simulation Results (Value)",
            10,
            GRAPH_Y_OFFSET - 20,
            20,
            Color::GRAY,
        );
        d.draw_text(
            "Time (Minutes)",
            GRAPH_X_OFFSET + graph_width / 2 - 20,
            GRAPH_Y_OFFSET + graph_height + 10,
            20,
            Color::GRAY,
        );

        // Plot the graph
        // for i in 1..results.len() {
        //     let x1 = GRAPH_X_OFFSET
        //         + ((i - 1) as f32 / (time_range.len() - 1) as f32 * GRAPH_WIDTH as f32) as i32;
        //     let x2 = GRAPH_X_OFFSET
        //         + (i as f32 / (time_range.len() - 1) as f32 * GRAPH_WIDTH as f32) as i32;

        //     let y1 = GRAPH_Y_OFFSET + GRAPH_HEIGHT
        //         - (((results[i - 1] - min_value) / (max_value - min_value)) * GRAPH_HEIGHT as f32)
        //             as i32;
        //     let y2 = GRAPH_Y_OFFSET + GRAPH_HEIGHT
        //         - (((results[i] - min_value) / (max_value - min_value)) * GRAPH_HEIGHT as f32)
        //             as i32;

        //     d.draw_line(x1, y1, x2, y2, Color::BLUE); // Connect data points with a line
        // }

        // // Draw points for better visibility
        // for (i, value) in results.iter().enumerate() {
        //     let x = GRAPH_X_OFFSET
        //         + (i as f32 / (time_range.len() - 1) as f32 * GRAPH_WIDTH as f32) as i32;
        //     let y = GRAPH_Y_OFFSET + GRAPH_HEIGHT
        //         - (((value - min_value) / (max_value - min_value)) * GRAPH_HEIGHT as f32) as i32;
        // }

        // Display current min/max values
        // d.draw_text(
        //     &format!("Min Value: {:.2}", min_value),
        //     GRAPH_X_OFFSET + 10,
        //     GRAPH_Y_OFFSET + GRAPH_HEIGHT + 30,
        //     20,
        //     Color::BLACK,
        // );
        // d.draw_text(
        //     &format!("Max Value: {:.2}", max_value),
        //     GRAPH_X_OFFSET + 200,
        //     GRAPH_Y_OFFSET + GRAPH_HEIGHT + 30,
        //     20,
        //     Color::BLACK,
        // );
    }
}

// fn plot_value_group(name: &str, values: &[f32] ) {
//     let
// }
// Function to simulate some values based on time
// fn simulate_value(minute: i32) -> f32 {
//     // Example: A sine wave pattern with an increasing trend
//     let trend = minute as f32 * 0.1; // Linear trend
//     let fluctuation = (minute as f32 * 0.5).sin() * 5.0; // Sine wave fluctuation
//     trend + fluctuation
// }
