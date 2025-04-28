//
//
//#[derive(Debug)]
//struct Point {
//    x: i32,
//    y: i32,
//}
//
//#[derive(Debug)]
//struct Vector {
//    x: i32,
//    y: i32,
//}
//
//
//#[derive(Debug)]
//struct Player {
//    center: Point,
//    radius: i8,
//    velocity: Vector,
//    acceleration: Vector,
//}
//
//#[derive(Debug)]
//struct GameMap {
//    height: i32,
//    width: i32,
//}    



//fn main() {
//    println!("yeet");
//    
//    let game_map: GameMap {
//        height: 200,
//        width: 300,
//    };
//
//    let p1: Player = Player {
//        center: Point { x: 10, y: 10 },
//        radius: 5,
//        velocity: Vector { x: 3, y: 1},
//        acceleration: Vector { x: 1, y: -1},
//    };
//
//
//    
//
//    println!("{:?}", p1);
//}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    // Our application state:
    let mut name = "Arthur".to_owned();
    let mut age = 42;

    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                age += 1;
            }
            ui.label(format!("Hello '{name}', age {age}"));
        });
    })
}
