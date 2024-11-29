use clap::Parser;
use dice_rs::*;
use dioxus::{desktop::Config, prelude::LaunchBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = DiceArgs::parse();
    if let Some(webapp) = args.launch_webapp {
        match webapp {
            true => {
                LaunchBuilder::desktop()
                    .with_cfg(
                        Config::new()
                            .with_background_color((30, 30, 46, 255))
                            .with_menu(None),
                    )
                    .launch(App);
                Ok(())
            }
            false => match args.command.as_ref() {
                Some(command) => match command {
                    DiceSubcommand::Generate(args) => completions(args),
                    DiceSubcommand::Faces(args) => {
                        let die = roll_dice(args)?;
                        println!("{}", die);
                        Ok(())
                    }
                },
                None => {
                    let die = roll_dice(&FacesArgs {
                        faces: Some(6),
                        disable_ascii: Some(false),
                    })?;
                    println!("{}", die);
                    Ok(())
                }
            },
        }
    } else {
        Ok(())
    }
}
