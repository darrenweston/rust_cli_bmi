#![warn(clippy::all, clippy::pedantic)]

use std::io;

struct Data {
    weight: f32,
    height: f32,
}

fn main() -> Result<(), io::Error> {
    // 1.87 metres is approx 6' 1.5"
    //
    let do_horizontal_chart = std::env::args().nth(1).is_some_and(|a| a.contains("-h"));

    clear_screen();
    let data = get_input()?;
    
    let bmi = data.weight / (data.height * data.height);

    if do_horizontal_chart {
        show_bmi(bmi);
    } else {
        show_vertical_bmi(bmi);
    }

    Ok(())
}

fn get_input() -> Result<Data, io::Error> {
    use std::io::Write;

    let mut weight: f32;
    let mut height: f32;

    loop {
        let mut weight_str = String::new();
        print!("Enter your weight in kg: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut weight_str)?;
        weight = match weight_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if weight <= 0.0 {
            println!("Weight must be greater than 0!");
            continue;
        }

        break;
    }

    loop {
        let mut height_str = String::new();
        print!("Enter your height in meters: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut height_str)?;
        height = match height_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if height <= 0.0 {
            println!("Height must be greater than 0!");
            continue;
        }

        break;
    }

    Ok(Data { weight, height })
}

fn show_bmi(bmi: f32) {
    let txt = "\n\
.----------.--------------------.---------------.---------------.-------------.\n\
|under     |healthy             |overweight     |severe ow      |obese        |\n\
|          |18.5                |25             |30             |35           |\n\
.----------.--------------------.---------------.---------------.-------------.";

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    let pos = (80. * (bmi - 15.) / (40. - 15.)) as usize;
    let you = format!("^ You ({bmi:.1})\n");

    //println!("pos is {pos}");
    println!("{txt}");
    println!("{}{}", " ".repeat(pos), you);
}

fn show_vertical_bmi(bmi: f32) {
    const REPS: usize = 4;
    let spacer = "|\n".repeat(REPS).trim_end().to_string();
    let spacer2 = "|\n".repeat(REPS - 2).trim_end().to_string();
    let txt = format!(
        "\
-underweight
{spacer2}
|-healthy (18.5)
{spacer}
|
|
|-overweight (25.0)
{spacer}
|-severe overweight (30.0)
{spacer}
|-obese (35.0)
{spacer}
"
    );

    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_precision_loss)]
    let pos = (((REPS + 1) as f32) * 5.0 * (bmi - 15.) / (40. - 15.)) as usize;
    let you = format!("You ({bmi:04.1}) -> |-");

    let mapper = |s| format!("              |-{s}");

    let mut txt_iter = txt.split('|');

    let pre: String = txt_iter.by_ref().take(pos).map(mapper).collect();
    let mid: String = txt_iter.by_ref().take(1).collect();
    let post: String = txt_iter.map(mapper).collect();

    let txt = format!("{pre}{you}{mid}{post}");

    //println!("{}", txt.split('\n').rev().map(|s| format!("{s}\n")).collect::<String>());
    let output = txt
        .split('\n')
        .rev()
        .fold(String::new(), |acc, s| acc + s + "\n");
    println!("{output}");
}

fn clear_screen() {
    use std::process::Command;

    let output = if cfg!(target_os = "windows") {
        Command::new("cls").status()
    } else {
        Command::new("clear").status()
    };

    if let Ok(status) = output {
        if !status.success() {
            eprintln!("Failed to clear screen");
        }
    } else {
        eprintln!("Failed to execute command");
    }
}
