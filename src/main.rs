#![allow(unused)]
use enigo::{self, Direction::Click, Keyboard, Mouse};
use rdev::{Event, EventType, Key, listen};
use std::fs::File;
use std::io::BufReader;
use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
    vec,
};

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

fn read_user_callables() -> HashMap<Vec<Key>, (String, i32)> {
    let file = File::open("callables.local.json").expect("callables.local.json does not exist...");
    let reader = BufReader::new(file);
    let map: HashMap<String, (String, i32)> =
        serde_json::from_reader(reader).expect("callables.local.json is invalid JSON...");
    let keys_to_call: HashMap<Vec<Key>, (String, i32)> = map
        .into_iter()
        .map(|(k, v)| (k.chars().map(char_to_key).collect(), v))
        .collect();
    keys_to_call
}

fn read_callables() -> HashMap<Vec<Key>, (String, i32)> {
    let callables: Vec<(&str, (String, i32))> = vec![
        (";hm", ("hello mole!".to_string(), 0)),
        (";ht", ("hi ted!".to_string(), 0)),
        (
            ";qy",
            (
                "qualify row_number() over (partition by ) = 1".to_string(),
                5,
            ),
        ),
        (";ac", ("git add -A && git commit -m \"\"".to_string(), 1)),
    ];
    let map: HashMap<Vec<Key>, (String, i32)> = callables
        .into_iter()
        .map(|(k, v)| (k.chars().map(char_to_key).collect(), v))
        .collect();
    map
}

fn main() {
    let mut history: VecDeque<Key> = VecDeque::new();
    let mut callables = read_callables();
    let user_callables = read_user_callables();
    callables.extend(user_callables);

    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();

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
                    let lines = result.0.split("\n");
                    for line in lines {
                        typer.text(line).unwrap();
                        typer.key(enigo::Key::Return, Click);
                    }
                    typer.key(enigo::Key::Backspace, Click);
                    for _ in 0..result.1 {
                        typer.key(enigo::Key::LeftArrow, Click);
                    }
                }
            }
        }
    };

    if let Err(error) = listen(callback) {
        println!("{:?}", error);
    }
}
