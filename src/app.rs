#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{roll_dice, FacesArgs};

pub fn App() -> Element {
    let mut roll_result = use_signal(|| "Roll the die!".to_string());
    let mut user_faces = use_signal(|| String::new());

    rsx! {
        div {
            h1 { "dice-rs" }
            pre { "{roll_result}" }
            div {
                input {
                    placeholder: "Enter number of faces (default: 6)",
                    value: "{user_faces}",
                    oninput: move |event| user_faces.set(event.value()),
                }
            }

            button {
                onclick: move |_| {
                    let faces = user_faces.read().parse::<u32>().ok().unwrap_or(6);

                    let args = FacesArgs {
                        faces: Some(faces),
                        disable_ascii: Some(false),
                    };

                    if let Ok(result) = roll_dice(&args) {
                        roll_result.set(result);
                    } else {
                        roll_result.set("Error rolling the die!".to_string());
                    }
                },
                "Roll the die"
            }
        }
    }
}
