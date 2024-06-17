use std::io;

struct Data {
    weight: f32,
    height: f32,
}

fn main() {
    // 1.87 metres is approx 6' 1.5"
    //

    let data = get_input().unwrap();
    let bmi = data.weight / (data.height * data.height);

    show_bmi(bmi);
}

fn get_input() -> Result<Data, String> {
    let mut weight = String::new();
    let mut height = String::new();

    println!("Enter your weight in kg:");
    io::stdin().read_line(&mut weight).unwrap();
    let weight: f32 = weight.trim().parse().unwrap();

    println!("Enter your height in meters:");
    io::stdin().read_line(&mut height).unwrap();
    let height: f32 = height.trim().parse().unwrap();

    if height <= 0.0 {
        Err("Height must be greater than 0!".to_string())
    }
    else{
        Ok(Data {weight, height})
    }
}

fn show_bmi(bmi: f32) {
    let txt = "\n\
        .----------.--------------------.---------------.---------------.-------------.\n\
        |under     |healthy             |Overweight     |severe ow      |obese        |\n\
        |          |18.5                |25             |30             |35           |\n\
        .----------.--------------------.---------------.---------------.-------------.";

    let pos = (80. * (bmi - 15.) / (40. - 15.)) as usize;
    let you = format!("^ You ({bmi:.1})\n");

    //println!("pos is {pos}");
    println!("{txt}");
    println!("{}{}", " ".repeat(pos), you);
}
