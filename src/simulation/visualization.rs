use super::experiment::AgentReport;
use crate::house_properties::{building_type::BuildingType, position::Position};
use plotters::prelude::*;
use std::{error::Error, path::Path};

pub(crate) fn terminal_map(
    map: &[Vec<BuildingType>],
    trail: &[Position],
    collected: &[Position],
    unseen: &[Position],
) -> String {
    let positions = |items: &[Position]| {
        items
            .iter()
            .map(|p| (p.y, p.x))
            .collect::<std::collections::HashSet<_>>()
    };
    let visited = positions(trail);
    let collected = positions(collected);
    let unseen = positions(unseen);
    let mut output = String::from(
        ". empty  H house  S school  $ shop  P park  F factory\n+ visited  C collected  ? unseen test house  O start  A final position\n",
    );
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let p = (y, x);
            let symbol = if trail.last().is_some_and(|q| (q.y, q.x) == p) {
                'A'
            } else if !trail.is_empty() && p == (0, 0) {
                'O'
            } else if collected.contains(&p) {
                'C'
            } else if unseen.contains(&p) {
                '?'
            } else if visited.contains(&p) {
                '+'
            } else {
                match cell {
                    BuildingType::Empty => '.',
                    BuildingType::House => 'H',
                    BuildingType::School => 'S',
                    BuildingType::Shop => '$',
                    BuildingType::Park => 'P',
                    BuildingType::Factory => 'F',
                }
            };
            output.push(symbol);
            output.push(' ');
        }
        output.push('\n');
    }
    output
}

pub(crate) fn comparison_chart(
    agents: &[AgentReport],
    path: impl AsRef<Path>,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(path.as_ref(), (1500, 900)).into_drawing_area();
    root.fill(&WHITE)?;
    let panels = root.split_evenly((2, 3));
    let colours = [BLUE, RGBColor(230, 150, 30), RGBColor(55, 160, 90)];
    for (index, caption) in [
        "MAE (lower is better)",
        "RMSE (lower is better)",
        "R2 (higher is better)",
        "Collected houses",
        "Movement energy used",
    ]
    .iter()
    .enumerate()
    {
        let values: Vec<Option<f64>> = agents
            .iter()
            .map(|a| match index {
                0 => a.metrics.as_ref().map(|m| m.mae),
                1 => a.metrics.as_ref().map(|m| m.rmse),
                2 => a.metrics.as_ref().and_then(|m| m.r_squared),
                3 => Some(a.houses as f64),
                _ => Some(a.steps as f64),
            })
            .collect();
        let low = values
            .iter()
            .flatten()
            .copied()
            .fold(0.0, f64::min)
            .min(0.0);
        let high = values.iter().flatten().copied().fold(1.0, f64::max);
        let pad = (high - low) * 0.12;
        let mut chart = ChartBuilder::on(&panels[index])
            .caption(*caption, ("sans-serif", 23))
            .margin(25)
            .x_label_area_size(40)
            .y_label_area_size(85)
            .build_cartesian_2d(-0.5f64..2.5f64, (low - pad)..(high + pad))?;
        chart
            .configure_mesh()
            .disable_x_mesh()
            .x_labels(3)
            .x_label_formatter(&|v| agents[v.round().clamp(0.0, 2.0) as usize].strategy.clone())
            .y_label_formatter(&|v| format!("{v:.1}"))
            .draw()?;
        for (i, value) in values.iter().enumerate() {
            if let Some(value) = value {
                chart.draw_series(std::iter::once(Rectangle::new(
                    [(i as f64 - 0.35, 0.0), (i as f64 + 0.35, *value)],
                    colours[i].filled(),
                )))?;
            } else {
                chart.draw_series(std::iter::once(Text::new(
                    "N/A",
                    (i as f64 - 0.2, 0.0),
                    ("sans-serif", 18),
                )))?;
            }
        }
    }
    panels[5].draw(&Text::new(
        "Same unseen houses for every model",
        (25, 60),
        ("sans-serif", 22),
    ))?;
    panels[5].draw(&Text::new(
        "Details: report.json and predictions.csv",
        (25, 100),
        ("sans-serif", 20),
    ))?;
    root.present()?;
    Ok(())
}

pub(crate) fn export_map(
    map: &[Vec<BuildingType>],
    path: impl AsRef<Path>,
    trail: &[Position],
) -> Result<(), Box<dyn Error>> {
    let height = map.len();
    let width = map.first().map_or(0, Vec::len);
    if width == 0 || height == 0 || map.iter().any(|r| r.len() != width) {
        return Err("Map must be nonempty and rectangular".into());
    }
    let (image_width, image_height, tile) = map_image_dimensions(width, height);
    let legend_x = 60 + width as i32 * tile;
    let root = BitMapBackend::new(path.as_ref(), (image_width, image_height)).into_drawing_area();
    root.fill(&WHITE)?;
    root.draw(&Text::new(
        "Neighbourhood — origin (0,0) at top left",
        (30, 30),
        ("sans-serif", 26),
    ))?;
    let colours = [
        RGBColor(235, 229, 213),
        RGBColor(69, 135, 210),
        RGBColor(245, 185, 65),
        RGBColor(174, 100, 200),
        RGBColor(88, 175, 98),
        RGBColor(110, 115, 125),
    ];
    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let (x, y) = (30 + x as i32 * tile, 60 + y as i32 * tile);
            root.draw(&Rectangle::new(
                [(x, y), (x + tile - 1, y + tile - 1)],
                colours[*cell as usize].filled(),
            ))?;
        }
    }
    root.draw(&PathElement::new(
        trail
            .iter()
            .map(|p| {
                (
                    30 + p.x as i32 * tile + tile / 2,
                    60 + p.y as i32 * tile + tile / 2,
                )
            })
            .collect::<Vec<_>>(),
        RED.stroke_width(2),
    ))?;
    for (i, label) in ["Empty", "House", "School", "Shop", "Park", "Factory"]
        .iter()
        .enumerate()
    {
        let y = 90 + i as i32 * 40;
        root.draw(&Rectangle::new(
            [(legend_x, y), (legend_x + 20, y + 20)],
            colours[i].filled(),
        ))?;
        root.draw(&Text::new(
            *label,
            (legend_x + 30, y + 16),
            ("sans-serif", 18),
        ))?;
    }
    root.present()?;
    Ok(())
}

fn map_image_dimensions(width: usize, height: usize) -> (u32, u32, i32) {
    let tile = (900 / width.max(height)).max(8);
    (
        (width * tile + 210).max(750) as u32,
        (height * tile + 100).max(400) as u32,
        tile as i32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn large_map_tiles_and_legend_fit_canvas() {
        for size in [1, 30, 100, 150, 200] {
            let (width, height, tile) = map_image_dimensions(size, size);
            assert!(tile >= 8);
            assert!(30 + size as u32 * tile as u32 + 150 < width);
            assert!(60 + size as u32 * (tile as u32) < height);
        }
        assert_eq!(map_image_dimensions(200, 200), (1810, 1700, 8));
    }

    #[test]
    fn exports_maximum_size_map_as_png() {
        let mut map = vec![vec![BuildingType::Empty; 200]; 200];
        map[0][199] = BuildingType::House;
        map[199][0] = BuildingType::Park;
        map[199][199] = BuildingType::Factory;
        let path =
            std::env::temp_dir().join(format!("urban_learn_large_map_{}.png", std::process::id()));
        export_map(&map, &path, &[]).unwrap();
        let png = std::fs::read(&path).unwrap();
        assert_eq!(&png[..8], b"\x89PNG\r\n\x1a\n");
        assert_eq!(u32::from_be_bytes(png[16..20].try_into().unwrap()), 1810);
        assert_eq!(u32::from_be_bytes(png[20..24].try_into().unwrap()), 1700);
        std::fs::remove_file(path).unwrap();
    }
}
