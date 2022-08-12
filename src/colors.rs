use std::collections::HashMap;
use std::num::ParseIntError;

use palette::{Srgb, Lch, FromColor, Gradient};

pub fn get_palette_map() -> HashMap<i32, String> {
    // Define a basic color palette
    let mut palette: HashMap<i32, String> = HashMap::new();
    palette.insert(0, String::from("#212121"));
    palette.insert(1, String::from("#99412f"));
    palette.insert(2, String::from("#8c9440"));
    palette.insert(3, String::from("#ca904f"));
    palette.insert(4, String::from("#32888b"));
    palette.insert(5, String::from("#815d7b"));
    palette.insert(6, String::from("#068488"));
    palette.insert(7, String::from("#707880"));
    palette.insert(8, String::from("#373b41"));
    palette.insert(9, String::from("#d6806e"));
    palette.insert(10, String::from("#b5bd68"));
    palette.insert(11, String::from("#fbb566"));
    palette.insert(12, String::from("#2aabbb"));
    palette.insert(13, String::from("#bc81b2"));
    palette.insert(14, String::from("#00c6cf"));
    palette.insert(15, String::from("#faf8ff"));
    return palette;
}

pub fn get_palette() -> Vec<String> {
    let palette_map = get_palette_map();
    let palette = vec![
        palette_map.get(&0).unwrap().to_owned(),
        palette_map.get(&1).unwrap().to_owned(),
        palette_map.get(&2).unwrap().to_owned(),
        palette_map.get(&3).unwrap().to_owned(),
        palette_map.get(&4).unwrap().to_owned(),
        palette_map.get(&5).unwrap().to_owned(),
        palette_map.get(&6).unwrap().to_owned(),
        palette_map.get(&7).unwrap().to_owned(),
        palette_map.get(&8).unwrap().to_owned(),
        palette_map.get(&9).unwrap().to_owned(),
        palette_map.get(&10).unwrap().to_owned(),
        palette_map.get(&11).unwrap().to_owned(),
        palette_map.get(&12).unwrap().to_owned(),
        palette_map.get(&13).unwrap().to_owned(),
        palette_map.get(&14).unwrap().to_owned(),
        palette_map.get(&15).unwrap().to_owned(),
    ];
    return palette;
}

pub fn get_colors() -> HashMap<i32, String> {
    let palette = get_palette_map();

    // Generate 256 Color palette
    let gradient = Gradient::new(vec![
        get_color(palette.get(&0).unwrap()),
        get_color(palette.get(&1).unwrap()),
        get_color(palette.get(&2).unwrap()),
        get_color(palette.get(&3).unwrap()),
        get_color(palette.get(&4).unwrap()),
        get_color(palette.get(&5).unwrap()),
        get_color(palette.get(&6).unwrap()),
        get_color(palette.get(&7).unwrap()),
        get_color(palette.get(&8).unwrap()),
        get_color(palette.get(&9).unwrap()),
        get_color(palette.get(&10).unwrap()),
        get_color(palette.get(&11).unwrap()),
        get_color(palette.get(&12).unwrap()),
        get_color(palette.get(&13).unwrap()),
        get_color(palette.get(&14).unwrap()),
        get_color(palette.get(&15).unwrap()),
    ]);

    let mut color_number = -1;
    let mut taken_colors: HashMap<i32, String> = gradient.take(256).map(|color| {
        let (r, g, b) = Srgb::from_color(color).into_components();
        let hex_code = encode_hex(
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
        );
        color_number = color_number + 1;
        return (color_number, hex_code);
    }).collect();
    taken_colors.insert(999, String::from(""));
    return taken_colors;
}

fn get_color(hex_code: &str) -> Lch {
    let decoded_hex = decode_hex(hex_code).unwrap_or(vec![0,0,0]);
    let color = Srgb::new(
            decoded_hex[0] as f32 / 255.0,
            decoded_hex[1] as f32 / 255.0,
            decoded_hex[2] as f32 / 255.0,
        );

    return Lch::from_color(color.into_linear());
}

fn encode_hex(r: u8, g: u8, b: u8) -> String {
    return String::from(format!("#{:02X}{:02X}{:02X}", r, g, b));
}

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    let hex_code = if s.starts_with('#') {
        crop_letters(s, 1)
    } else {
        s
    };
    (0..hex_code.len())
        .step_by(2)
        .map(|i| {
            return u8::from_str_radix(&hex_code[i..i + 2], 16);
        })
        .collect()
}

fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().nth(pos) {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}
