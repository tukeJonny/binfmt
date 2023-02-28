use std::fmt::Write;
use std::fs::File;
use anyhow::Result;

mod drawer;
mod field;

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
            println!("{}", &buf[..size+1]);
        }

    }
    Ok(())
}

fn main() -> Result<()> {
    let spec_file = File::open("examples/test1.yaml")?;
    let spec: FieldSpec = serde_yaml::from_reader(spec_file)?;

    let mut drawer = drawer::FieldDrawer::new();
    for field in spec.fields {
        drawer.draw_field(field)?;
    }

    write_header();
    write_border(BorderSize::Default)?;

    let mut remaining_size = drawer.total_size * 2;
    let canvas: drawer::SlicedCanvas = drawer.into();
    for mut row in canvas.into_iter() {
        if !str::starts_with(&row, drawer::DELIMITER) {
            print!("{}", drawer::DELIMITER);
            row.remove(0); // drop one whitespace
        }

        if str::ends_with(row.as_str(), drawer::DELIMITER) {
            row.pop();
            row.push(' ');
            row.push_str(drawer::DELIMITER);
        } else {
            row.push_str(drawer::DELIMITER);
        }

        if remaining_size < WIDTH {
            row.pop(); // drop two whitespaces
            row.pop();
            print!("{row}");
            println!("{}", drawer::DELIMITER); 
            write_border(BorderSize::Sized(remaining_size as usize))?;
        } else {
            println!("{row}");
            write_border(BorderSize::Default)?;
        }

        remaining_size = remaining_size.saturating_sub(WIDTH);
    }

    Ok(())
}
