#![allow(unused)]
use arboard::Clipboard;
use chrono::Local;
use enigo::{self, Direction::Click, Keyboard, Mouse};
use heck::ToSnakeCase;
use rdev::{Event, EventType, Key, listen};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
    vec,
};

mod formatters;

fn char_to_key(char: char) -> Key {
    match char {
        // Punctuation
        ';' => Some(Key::SemiColon),
        ' ' => Some(Key::Space),
        // Alphabetical
        'a' => Some(Key::KeyA),
        'b' => Some(Key::KeyB),
        'c' => Some(Key::KeyC),
        'd' => Some(Key::KeyD),
        'e' => Some(Key::KeyE),
        'f' => Some(Key::KeyF),
        'g' => Some(Key::KeyG),
        'h' => Some(Key::KeyH),
        'i' => Some(Key::KeyI),
        'j' => Some(Key::KeyJ),
        'k' => Some(Key::KeyK),
        'l' => Some(Key::KeyL),
        'm' => Some(Key::KeyM),
        'n' => Some(Key::KeyN),
        'o' => Some(Key::KeyO),
        'p' => Some(Key::KeyP),
        'q' => Some(Key::KeyQ),
        'r' => Some(Key::KeyR),
        's' => Some(Key::KeyS),
        't' => Some(Key::KeyT),
        'u' => Some(Key::KeyU),
        'v' => Some(Key::KeyV),
        'w' => Some(Key::KeyW),
        'x' => Some(Key::KeyX),
        'y' => Some(Key::KeyY),
        'z' => Some(Key::KeyZ),
        // Numbers
        '0' => Some(Key::Num0),
        '1' => Some(Key::Num1),
        '2' => Some(Key::Num2),
        '3' => Some(Key::Num3),
        '4' => Some(Key::Num4),
        '5' => Some(Key::Num5),
        '6' => Some(Key::Num6),
        '7' => Some(Key::Num7),
        '8' => Some(Key::Num8),
        '9' => Some(Key::Num9),
        _ => None,
    }
    .expect("Invalid key detected")
}

fn read_callables(fname: &str) -> HashMap<Vec<Key>, (String, i32)> {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let path = PathBuf::from(project_root).join(fname);
    let file = File::open(path).expect("{fname} does not exist...");
    let reader = BufReader::new(file);
    let map: HashMap<String, (String, i32)> =
        serde_json::from_reader(reader).expect("{fname} is invalid JSON...");
    let keys_to_call: HashMap<Vec<Key>, (String, i32)> = map
        .into_iter()
        .map(|(k, v)| (k.chars().map(char_to_key).collect(), v))
        .collect();
    keys_to_call
}

fn read_key_to_func() -> HashMap<Vec<Key>, fn()> {
    let mut key_to_func: HashMap<&str, fn()> = HashMap::new();
    key_to_func.insert(";de", formatters::current_date);
    key_to_func.insert(";2l", formatters::to_lowercase);
    key_to_func.insert(";2u", formatters::to_uppercase);
    key_to_func.insert(";2s", formatters::to_snake_case);
    key_to_func.insert(";ul", formatters::add_underline);
    key_to_func.insert(";dc", formatters::dash_center);
    key_to_func.insert(";hc", formatters::hash_center);
    key_to_func.insert(";sw", formatters::select_word);
    key_to_func.insert(";sl", formatters::select_line);
    key_to_func.insert(";dd", formatters::sql_count_distinct);
    key_to_func.insert(";dt", formatters::sql_count_distinct_millions);
    key_to_func
        .into_iter()
        .map(|(k, v)| (k.chars().map(char_to_key).collect(), v))
        .collect()
}

fn main() {
    let mut history: VecDeque<Key> = VecDeque::new();
    let mut callables = read_callables("callables.json");
    let user_callables = read_callables("callables.local.json");
    callables.extend(user_callables);
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();

    let key_to_func = read_key_to_func();

    let callback = move |event: Event| {
        if let EventType::KeyPress(key) = event.event_type {
            history.push_back(key);
            if history.len() > 3 {
                history.pop_front();
            };
            println!("{:?}", history);

            if history.len() == 3 {
                let lookup: Vec<Key> = history.iter().cloned().collect();

                if let Some(result) = callables.get(&lookup) {
                    println!("{result:?}");
                    for _ in 0..3 {
                        typer.key(enigo::Key::Backspace, Click);
                    }
                    let lines = result.0.lines();
                    for (i, line) in lines.clone().enumerate() {
                        typer.text(line).unwrap();
                        if i + 1 != lines.clone().count() {
                            typer.key(enigo::Key::Return, Click);
                        }
                    }
                    for _ in 0..result.1 {
                        typer.key(enigo::Key::LeftArrow, Click);
                    }
                }

                if let Some(result) = key_to_func.get(&lookup) {
                    println!("{result:?}");
                    for _ in 0..3 {
                        typer.key(enigo::Key::Backspace, Click);
                    }
                    result()
                }
            }
        }
    };

    if let Err(error) = listen(callback) {
        println!("{:?}", error);
    }
}
