// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use interpreter::Entry;
use rlua::{Lua, Table};

mod interpreter;

#[tauri::command]
fn interpret_contents(contents: &str) -> Vec<Entry> {
    let mut entries = Vec::new();

    // create lua environment
    let lua = Lua::new();
    lua.context(|ctx| {
        // immitate global fivem functions commonly used in configs
        ctx
            .load("function GetHashKey() end")
            .exec()
            .unwrap();

        // load config file
        ctx.load(&contents)
            .set_name("imported config")
            .unwrap()
            .exec()
            .unwrap();

        // get config table
        // TODO: search for all globals in loaded file or let user select which table is the config
        let config: Table = ctx.globals().get("Config").unwrap();
        interpreter::walk_table(&mut entries, config);
    });

    entries
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![interpret_contents])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
