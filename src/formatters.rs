#![allow(unused)]
use arboard::Clipboard;
use chrono::Local;
use enigo::{self, Direction::Click, Keyboard, Mouse};
use heck::ToSnakeCase;
use rdev::{Event, EventType, Key, listen};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
    vec,
};

pub fn current_date() -> () {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    let date = Local::now().date_naive().to_string();
    typer.text(&date).unwrap();
}

pub fn to_lowercase() -> () {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    let mut clipboard = Clipboard::new().unwrap();
    let result = clipboard.get_text().unwrap();
    typer.text(&result.trim().to_lowercase());
}

pub fn to_uppercase() -> () {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    let mut clipboard = Clipboard::new().unwrap();
    let result = clipboard.get_text().unwrap();
    typer.text(&result.trim().to_uppercase());
}

pub fn to_snake_case() -> () {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    let mut clipboard = Clipboard::new().unwrap();
    let result = clipboard.get_text().unwrap();
    typer.text(&result.trim().to_snake_case());
}

pub fn remove_blanklines() -> () {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    let mut clipboard = Clipboard::new().unwrap();
    let result = clipboard.get_text().unwrap();
    typer.text(
        &result
            .trim()
            .lines()
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>()
            .join("\n"),
    );
}

pub fn add_underline() -> () {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    let mut clipboard = Clipboard::new().unwrap();
    let result = clipboard.get_text().unwrap();
    let cleaned = result.trim();
    let underline = "-".repeat(cleaned.chars().count());
    typer.text(&format!("{cleaned}\n{underline}"));
}

pub fn dash_center() -> () {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    let mut clipboard = Clipboard::new().unwrap();
    let result = clipboard.get_text().unwrap();
    let cleaned = result.trim();
    typer.text(&format!("{cleaned:-^88}"));
}

pub fn hash_center() -> () {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    let mut clipboard = Clipboard::new().unwrap();
    let result = clipboard.get_text().unwrap();
    let cleaned = result.trim();
    typer.text(&format!("{cleaned:#^88}"));
}

pub fn copy_selection() -> String {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    let mut clipboard = Clipboard::new().unwrap();
    let original = clipboard.get_text().unwrap().trim().to_string();

    typer.key(enigo::Key::Meta, enigo::Direction::Press);
    typer.key(enigo::Key::Unicode('c'), enigo::Direction::Click);
    typer.key(enigo::Key::Meta, enigo::Direction::Release);

    for i in 0..5 {
        println!("Sleeping {} - {:.0} millis", i + 1, i + 1 * 10);
        thread::sleep(Duration::from_millis(10));
        let new = clipboard.get_text().unwrap().trim().to_string();
        if original != new {
            return new;
        }
    }
    clipboard.get_text().unwrap().trim().to_string()
}

pub fn select_word() {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    typer.key(enigo::Key::Alt, enigo::Direction::Press);
    typer.key(enigo::Key::Shift, enigo::Direction::Press);
    typer.key(enigo::Key::LeftArrow, enigo::Direction::Click);
    typer.key(enigo::Key::Shift, enigo::Direction::Release);
    typer.key(enigo::Key::Alt, enigo::Direction::Release);
}

pub fn select_line() {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    typer.key(enigo::Key::Meta, enigo::Direction::Press);
    typer.key(enigo::Key::Shift, enigo::Direction::Press);
    typer.key(enigo::Key::LeftArrow, enigo::Direction::Click);
    typer.key(enigo::Key::Shift, enigo::Direction::Release);
    typer.key(enigo::Key::Meta, enigo::Direction::Release);
    copy_selection();
}

pub fn sql_count_distinct() {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    select_word();
    let column = copy_selection();
    typer.text(&format!("count(distinct {column}) n_{column},"));
}

pub fn sql_count_distinct_millions() {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    select_word();
    let column = copy_selection();
    typer.text(&format!("count(distinct {column})/1000000 n_{column},"));
}

pub fn sql_count_nulls() {
    let mut typer = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    select_word();
    let column = copy_selection();
    typer.text(&format!(
        "countif({column} is null) / count(*) pct_null_{column},"
    ));
}
