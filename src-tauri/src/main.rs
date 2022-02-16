#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;

fn main() {
    tauri::Builder::default()
        .on_page_load(move |window, _| {
            let bootstrap = Bootstrap {
                msg: String::from("Absolutely neccessary config for JS side"),
            };

            window.emit("bootstrap", bootstrap.clone()).unwrap();
            println!("Rust properly sent <bootstrap> message");

            // Uncommenting this code "fixes" the issue
            // std::thread::spawn(move || {
            //     let bootstrap = Bootstrap {
            //         msg: String::from("Absolutely neccessary config for JS side (with delay)"),
            //     };

            //     let delay = 400;
            //     std::thread::sleep(std::time::Duration::from_millis(delay));
            //     window.emit("bootstrap", bootstrap.clone()).unwrap();
            //     println!("Rust properly sent <bootstrap> message (after {}ms)", delay);
            // });
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Clone, Serialize)]
struct Bootstrap {
    msg: String,
}
