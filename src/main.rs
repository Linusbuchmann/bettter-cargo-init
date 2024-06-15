#![windows_subsystem = "windows"] 


use eframe::egui::{self, debug_text::print};
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Write;


fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let mut name = "my_project".to_owned();
    let mut egui_checked = false;
    let mut rand_checked = false;
    let mut egui_extras_checked = false;
    let mut image_checked = false;
    let mut bevy_checked = false;
    let mut mgf_checked = false;
    let mut amethyst_checked = false;
    let mut sfml_checked = false;
    let mut directory_var = "C:/dev/Rust".to_owned();

    eframe::run_simple_native("Better cargo init", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            // UI for project name
            ui.horizontal(|ui| {
                let name_label = ui.label("Project name: ");
                ui.text_edit_singleline(&mut name).labelled_by(name_label.id);
                });
            ui.horizontal(|ui| {
                let directory_label = ui.label("directory:         ");
                ui.text_edit_singleline(&mut directory_var).labelled_by(directory_label.id);
            });   
            // UI for checkboxes
            ui.horizontal(|ui| {
                ui.checkbox(&mut egui_checked, "egui");
                ui.checkbox(&mut rand_checked, "rand");
                ui.checkbox(&mut egui_extras_checked, "egui_extras");
                ui.checkbox(&mut image_checked, "image");
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut bevy_checked, "bevy");
                ui.checkbox(&mut mgf_checked, "mgf");
                ui.checkbox(&mut amethyst_checked, "amethyst");
                ui.checkbox(&mut sfml_checked, "sfml");
            });

            // Button to create project
            if ui.button("Create Project").clicked() {
                let path = format!("{}/{}", directory_var, name);
                match fs::create_dir_all(&path) {
                    Ok(_) => {
                        println!("Directory created successfully");

                        // Create .gitignore file
                        let gitignore_path = format!("{}/.gitignore", path);
                        match File::create(&gitignore_path) {
                            Ok(mut file) => {
                                match file.write_all(b"Hello, World!") {
                                    Ok(_) => println!(".gitignore created successfully"),
                                    Err(e) => println!("Failed to write to .gitignore: {}", e),
                                }
                            }
                            Err(e) => println!("Failed to create .gitignore: {}", e),
                        }

                        // Create Cargo.toml file with selected dependencies
                        let cargo_toml_path = format!("{}/Cargo.toml", path);
                        let mut cargo_toml_contents = format!(
                            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
"#,
                            name
                        );

                        if egui_checked {
                            cargo_toml_contents.push_str("egui = \"0.27.2\"\n");
                            cargo_toml_contents.push_str("eframe = \"0.27.2\"\n");
                        }
                        if rand_checked {
                            cargo_toml_contents.push_str("rand = \"0.8\"\n");
                        }
                        if egui_extras_checked {
                            cargo_toml_contents.push_str("egui_extras = \"0.27.2\"\n");
                        }
                        if image_checked {
                            cargo_toml_contents.push_str("image = \"0.25.1\"\n");
                        }
                        if bevy_checked {
                            cargo_toml_contents.push_str("bevy = \"0.13.2\"\n");
                        }
                        if mgf_checked {
                            cargo_toml_contents.push_str("mgf = \"1.4.0\"\n");
                        }
                        if amethyst_checked {
                            cargo_toml_contents.push_str("amethyst = \"0.15\"\n");
                        }
                        if sfml_checked {
                            cargo_toml_contents.push_str("sfml = \"0.21.0\"\n");
                        }

                        match File::create(&cargo_toml_path) {
                            Ok(mut file) => {
                                match file.write_all(cargo_toml_contents.as_bytes()) {
                                    Ok(_) => println!("Cargo.toml created successfully"),
                                    Err(e) => println!("Failed to write to Cargo.toml: {}", e),
                                }
                            }
                            Err(e) => println!("Failed to create Cargo.toml: {}", e),
                        }

                        // Create src directory and main.rs file with Hello World
                        let src_path = format!("{}/src", path);
                        match fs::create_dir_all(&src_path) {
                            Ok(_) => {
                                let main_rs_path = format!("{}/main.rs", src_path);
                                let main_rs_contents = r#"fn main() {
    println!("Hello, world!");
}
"#;
                                match File::create(&main_rs_path) {
                                    Ok(mut file) => {
                                        match file.write_all(main_rs_contents.as_bytes()) {
                                            Ok(_) => println!("src/main.rs created successfully"),
                                            Err(e) => println!("Failed to write to src/main.rs: {}", e),
                                        }
                                    }
                                    Err(e) => println!("Failed to create src/main.rs: {}", e),
                                }
                            }
                            Err(e) => println!("Failed to create src directory: {}", e),
                        }
                    }
                    Err(e) => println!("Failed to create directory: {}", e),
                }
            }
        });
    })
}