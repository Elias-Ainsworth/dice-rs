use rand::Rng;
use serde::Deserialize;

use crate::FacesArgs;

#[derive(Deserialize)]
struct DiceFaces {
    faces: std::collections::HashMap<String, String>,
}

const FACES_JSON: &str = include_str!("./faces.json");

fn load_dice_faces(file_path: &str) -> Result<DiceFaces, Box<dyn std::error::Error>> {
    let dice_faces: DiceFaces = serde_json::from_str(file_path)?;
    Ok(dice_faces)
}

fn print_figlet(roll_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let font = figlet_rs::FIGfont::standard()?;
    let figure = font.convert(roll_str);
    figure
        .map(|fig| format!("{}", fig))
        .ok_or_else(|| "Failed to conver string into figlet.".into())
}

fn print_ascii(roll_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let dice_faces = load_dice_faces(FACES_JSON)?;
    dice_faces
        .faces
        .get(roll_str)
        .map(|face| format!("{}", face))
        .ok_or_else(|| "Face not found".into())
}

pub fn roll_dice(arg: &FacesArgs) -> Result<String, Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let faces = match arg.faces {
        Some(faces) => faces,
        None => 6,
    };

    let roll = rng.gen_range(1..=faces);
    let roll_string = format!("{}", roll);
    let roll_str: &str = &roll_string;

    match arg.disable_ascii {
        Some(true) => print_figlet(roll_str),
        _ => {
            if faces <= 6 {
                print_ascii(roll_str)
            } else {
                print_figlet(roll_str)
            }
        }
    }
}
