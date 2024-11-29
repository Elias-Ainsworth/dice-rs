use clap::Parser;
use dice_rs::*;
use dioxus::{desktop::Config, prelude::LaunchBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = DiceArgs::parse();
    if let Some(app) = args.launch_app {
        match app {
            true => {
                LaunchBuilder::desktop()
                    .with_cfg(
                        Config::new()
                            .with_background_color((30, 30, 46, 255))
                            .with_menu(None)
                            // disable on release builds
                            .with_disable_context_menu(!cfg!(debug_assertions))
                            .with_custom_index(
                                r#"<!DOCTYPE html>
                                    <html>
                                        <head>
                                            <title>Dioxus app</title>
                                            <meta name="viewport" content="width=device-width, initial-scale=1.0">
                                            <link rel="stylesheet" href="public/tailwind.css">
                                        </head>
                                        <body>
                                            <div id="main" style="height: 100vh;"></div>
                                        </body>
                                    </html>"#.to_string(),
                        ),
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
