use std::io;

struct Data {
    weight: f32,
    height: f32,
}

fn main() {
    // 1.87 metres is approx 6' 1.5"
    //
    let hor = std::env::args().nth(1).is_some_and(|a| a.contains("-h"));

    clear_screen();
    let data = get_input().unwrap();
    let bmi = data.weight / (data.height * data.height);

    if hor {
        show_bmi(bmi);
    } else {
        show_vertical_bmi(bmi);
    }
}

fn get_input() -> Result<Data, String> {
    use std::io::Write;

    let mut weight = String::new();
    let mut height = String::new();

    print!("Enter your weight in kg: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut weight).unwrap();
    let weight: f32 = weight.trim().parse().unwrap();

    print!("Enter your height in meters: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut height).unwrap();
    let height: f32 = height.trim().parse().unwrap();

    if height <= 0.0 {
        Err("Height must be greater than 0!".to_string())
    } else {
        Ok(Data { weight, height })
    }
}

fn show_bmi(bmi: f32) {
    let txt = "\n\
.----------.--------------------.---------------.---------------.-------------.\n\
|under     |healthy             |overweight     |severe ow      |obese        |\n\
|          |18.5                |25             |30             |35           |\n\
.----------.--------------------.---------------.---------------.-------------.";

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

    let pos = (((REPS + 1) as f32) * 5.0 * (bmi - 15.) / (40. - 15.)) as usize;
    let you = format!("You ({bmi:04.1}) -> |-");

    let mapper = |s| format!("              |-{s}");

    let pre: String = txt.split('|').take(pos).map(mapper).collect();
    let mid: String = txt.split('|').skip(pos).take(1).collect();
    let post: String = txt.split('|').skip(pos + 1).map(mapper).collect();

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
