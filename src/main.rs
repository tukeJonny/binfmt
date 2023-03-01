use anyhow::Result;
use clap::{crate_version, App, AppSettings, Arg};
use std::env;
use std::fmt::Write;
use std::fs::File;

mod drawer;
mod field;

use drawer::{FieldDrawer, SlicedCanvas, DELIMITER};
use field::FieldSpec;

const WIDTH: u64 = 32 * 2;

enum BorderSize {
    Default,
    Sized(usize),
}

fn write_header() {
    print!(" ");
    for num in (0..10).cycle().take(32) {
        print!("{num} ");
    }
    println!();
}

fn write_border(bs: BorderSize) -> Result<()> {
    match bs {
        BorderSize::Default => {
            print!("{}", "+-".repeat(32));
            println!("+");
        }
        BorderSize::Sized(size) => {
            let mut buf = String::new();
            write!(buf, "{}", "+-".repeat(32))?;
            write!(buf, "+")?;
            println!("{}", &buf[..size + 1]);
        }
    }
    Ok(())
}

fn build_app() -> App<'static, 'static> {
    let app = App::new("binfmt")
        .version(crate_version!())
        .usage("binfmt [FLAGS/OPTIONS] [<file-path>]")
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::with_name("file-path")
                .help("path to spec yaml file")
                .required(true),
        );
    app
}

fn main() {
    let matches = build_app().get_matches_from(env::args_os());
    let path = matches
        .value_of("file-path")
        .expect("Failed to get spec file path");

    let spec_file = File::open(path).expect("Failed to open spec file");
    let spec: FieldSpec =
        serde_yaml::from_reader(spec_file).expect("Failed to read spec file as yaml file");

    let mut drawer = FieldDrawer::new();
    for field in spec.fields {
        drawer.draw_field(field).expect("Failed to draw field");
    }

    write_header();
    write_border(BorderSize::Default).expect("Failed to write border");

    let mut remaining_size = drawer.total_size * 2;
    let canvas: SlicedCanvas = drawer.into();
    for mut row in canvas.into_iter() {
        if !str::starts_with(&row, DELIMITER) {
            print!("{DELIMITER}");
            row.remove(0); // drop one whitespace
        }

        if str::ends_with(row.as_str(), DELIMITER) {
            row.pop();
            row.push(' ');
            row.push_str(DELIMITER);
        } else {
            row.push_str(DELIMITER);
        }

        if remaining_size < WIDTH {
            row.pop(); // drop two whitespaces
            row.pop();
            print!("{row}");
            println!("{DELIMITER}");
            write_border(BorderSize::Sized(remaining_size as usize))
                .expect("Failed to write border");
        } else {
            println!("{row}");
            write_border(BorderSize::Default).expect("Failed to write border");
        }

        remaining_size = remaining_size.saturating_sub(WIDTH);
    }
}
