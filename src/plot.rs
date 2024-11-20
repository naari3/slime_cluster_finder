use plotly::common::{Marker, Mode};
use plotly::layout::Axis;
use plotly::Scatter;
use plotly::{Layout, Plot};

pub fn plot(data: &[((i32, i32), usize)], seed: i64) -> Result<(), Box<dyn std::error::Error>> {
    // let mut filtered_data = Vec::new();

    // // データをフィルタリング
    // for ((x, z), count) in data {
    //     let mut is_near = false;
    //     for ((nx, nz), _) in &filtered_data {
    //         if (x - nx).abs() < 20 && (z - nz).abs() < 20 {
    //             is_near = true;
    //             break;
    //         }
    //     }
    //     if !is_near {
    //         filtered_data.push(((*x, *z), *count));
    //     }
    // }

    // let data = &filtered_data[..];

    let max_count = data.iter().map(|(_, count)| *count).max().unwrap_or(1) as f64;
    let min_count = data.iter().map(|(_, count)| *count).min().unwrap_or(0) as f64;

    let x_values: Vec<i32> = data.iter().map(|((x, _), _)| *x).collect();
    let z_values: Vec<i32> = data.iter().map(|((_, z), _)| *z).collect();
    let values: Vec<String> = data
        .iter()
        .map(|(chunk, count)| format!("x: {}, z: {}<br>{}", chunk.0 * 16, chunk.1 * 16, count,))
        .collect();
    let sizes: Vec<usize> = data
        .iter()
        .map(|(_, count)| {
            (((*count as f64 - min_count) / (max_count - min_count) * 30.0) + 5.0) as usize
        })
        .collect();
    let colors: Vec<String> = data
        .iter()
        .map(|(_, count)| {
            let intensity = ((*count as f64) / max_count * 255.0) as u8;
            format!("rgba(0, {}, 0, {:.2})", intensity, intensity as f64 / 255.0)
        })
        .collect();

    let scatter = Scatter::new(x_values, z_values)
        .mode(Mode::Markers)
        .marker(Marker::new().size_array(sizes).color_array(colors))
        .text_array(values);

    let layout = Layout::new()
        .title(format!("Slime Chunks at {}", seed))
        .width(1080)
        .height(1080)
        .x_axis(Axis::new().scale_anchor("y").title("Chunk X"))
        .y_axis(Axis::new().scale_anchor("x").title("Chunk Z"));

    let mut plot = Plot::new();
    plot.add_trace(scatter);
    plot.set_layout(layout);
    plot.show();

    Ok(())
}
