#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Vector {
    x: i32,
    y: i32,
}


#[derive(Debug)]
struct Player {
    center: Point,
    radius: i8,
    velocity: Vector,
    acceleration: Vector,
}

#[derive(Debug)]
struct GameMap {
    height: i32,
    width: i32,
} 

fn main() -> eframe::Result {

    let game_map =  GameMap {
        height: 200,
        width: 300,
    };

    let p1 = Player {
        center: Point { x: 10, y: 10 },
        radius: 5,
        velocity: Vector { x: 3, y: 1},
        acceleration: Vector { x: 1, y: -1},
    };


    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.image(egui::include_image!(
                "../ferris.png"
            ));
        });
    }
}
