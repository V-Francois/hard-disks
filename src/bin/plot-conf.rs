use draw::*;
use std::env;
use std::fs::read_to_string;
use std::path;
use std::process;

fn main() -> process::ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} path-to-position-file", args[0]);
        return process::ExitCode::from(1);
    }

    let file_path = &args[1];
    if !path::Path::new(file_path).exists() {
        eprintln!("Path {} doesn’t exist", file_path);
        return process::ExitCode::from(1);
    }

    let mut lines = Vec::new();
    for line in read_to_string(file_path).unwrap().lines() {
        lines.push(line.to_string())
    }

    let mut sigma: f32 = 0.0;
    let mut lx: f32 = 0.0;
    let mut ly: f32 = 0.0;
    for (id, string) in lines[0].split_whitespace().enumerate() {
        if id == 1 {
            sigma = string.parse().unwrap();
        } else if id == 2 {
            lx = string.parse().unwrap();
        } else if id == 3 {
            ly = string.parse().unwrap();
        }
    }

    let scaling = 10.0;
    let mut canvas = Canvas::new((lx * scaling) as u32, (ly * scaling) as u32);

    for string in lines[1..lines.len()].iter() {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        for (id, string) in string.split_whitespace().enumerate() {
            if id == 0 {
                x = string.parse().unwrap();
            } else if id == 1 {
                y = string.parse().unwrap();
            }
        }
        let circle = Drawing::new()
            .with_shape(Shape::Circle {
                radius: (sigma * scaling) as u32,
            })
            .with_xy(x * scaling, y * scaling)
            .with_style(Style::stroked(1, Color::black()));

        canvas.display_list.add(circle);
    }

    render::save(&canvas, "image.svg", SvgRenderer::new()).expect("Failed to save");

    process::ExitCode::SUCCESS
}
