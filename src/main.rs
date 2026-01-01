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
    let project_root = env!("CARGO_MANIFEST_DIR");
    let path = PathBuf::from(project_root).join("callables.local.json");
    let file = File::open(path).expect("callables.local.json does not exist...");
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
    let callables: Vec<(&str, (&str, i32))> = vec![
        (";hm", ("hello mole!", 0)),
        (";ht", ("hi ted!", 0)),
        (";qy", ("qualify row_number() over (partition by ) = 1", 5)),
        (";ac", ("git add -A && git commit -m \"\"", 1)),
        (
            ";sc",
            (
                "count(*)/1000000 n,\ncount(*) / sum(count(*)) over() pct,\nsum(count(*)) over (order by count(*) desc) / sum(count(*)) over() cum_pct",
                0,
            ),
        ),
    ];
    let map: HashMap<Vec<Key>, (String, i32)> = callables
        .into_iter()
        .map(|(k, v)| (k.chars().map(char_to_key).collect(), (v.0.to_string(), v.1)))
        .collect();
    map
}

fn current_date() -> () {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    let date = Local::now().date_naive().to_string();
    typer.text(&date).unwrap();
}

fn to_lowercase() -> () {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    let mut clipboard = Clipboard::new().unwrap();
    let result = clipboard.get_text().unwrap();
    typer.text(&result.to_lowercase());
}

fn to_uppercase() -> () {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    let mut clipboard = Clipboard::new().unwrap();
    let result = clipboard.get_text().unwrap();
    typer.text(&result.to_uppercase());
}

fn to_snake_case() -> () {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    let mut clipboard = Clipboard::new().unwrap();
    let result = clipboard.get_text().unwrap();
    typer.text(&result.to_snake_case());
}

fn remove_blanklines() -> () {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    let mut clipboard = Clipboard::new().unwrap();
    let result = clipboard.get_text().unwrap();
    typer.text(
        &result
            .lines()
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>()
            .join("\n"),
    );
}

fn add_underline() -> () {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    let mut clipboard = Clipboard::new().unwrap();
    let result = clipboard.get_text().unwrap();
    let underline = "-".repeat(result.chars().count());
    typer.text(&format!("{result}\n{underline}"));
}

fn dash_center() -> () {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    let mut clipboard = Clipboard::new().unwrap();
    let result = clipboard.get_text().unwrap();
    typer.text(&format!("{result:-^88}"));
}

fn hash_center() -> () {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    let mut clipboard = Clipboard::new().unwrap();
    let result = clipboard.get_text().unwrap();
    typer.text(&format!("{result:#^88}"));
}

fn copy_selection() -> String {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    let mut clipboard = Clipboard::new().unwrap();
    typer.key(enigo::Key::Meta, enigo::Direction::Press);
    typer.key(enigo::Key::Unicode('c'), enigo::Direction::Click);
    typer.key(enigo::Key::Meta, enigo::Direction::Release);
    clipboard.get_text().unwrap()
}

fn select_word() {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    typer.key(enigo::Key::Alt, enigo::Direction::Press);
    typer.key(enigo::Key::Shift, enigo::Direction::Press);
    typer.key(enigo::Key::LeftArrow, enigo::Direction::Click);
    typer.key(enigo::Key::Shift, enigo::Direction::Release);
    typer.key(enigo::Key::Alt, enigo::Direction::Release);
}

fn select_line() {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    typer.key(enigo::Key::Meta, enigo::Direction::Press);
    typer.key(enigo::Key::Shift, enigo::Direction::Press);
    typer.key(enigo::Key::LeftArrow, enigo::Direction::Click);
    typer.key(enigo::Key::Shift, enigo::Direction::Release);
    typer.key(enigo::Key::Meta, enigo::Direction::Release);
}

fn sql_count_distinct() {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    select_word();
    let column = copy_selection();
    typer.text(&format!("count(distinct {column}) n_{column},"));
}

fn sql_count_distinct_millions() {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    select_word();
    let column = copy_selection();
    typer.text(&format!("count(distinct {column})/1000000 n_{column},"));
}

fn read_key_to_func() -> HashMap<Vec<Key>, fn()> {
    let mut key_to_func: HashMap<&str, fn()> = HashMap::new();
    key_to_func.insert(";de", current_date);
    key_to_func.insert(";2l", to_lowercase);
    key_to_func.insert(";2u", to_uppercase);
    key_to_func.insert(";2s", to_snake_case);
    key_to_func.insert(";ul", add_underline);
    key_to_func.insert(";dc", dash_center);
    key_to_func.insert(";hc", hash_center);
    key_to_func.insert(";sw", select_word);
    key_to_func.insert(";sl", select_line);
    key_to_func.insert(";dd", sql_count_distinct);
    key_to_func.insert(";dt", sql_count_distinct_millions);
    key_to_func
        .into_iter()
        .map(|(k, v)| (k.chars().map(char_to_key).collect(), v))
        .collect()
}

fn main() {
    let mut history: VecDeque<Key> = VecDeque::new();
    let mut callables = read_callables();
    let user_callables = read_user_callables();
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
