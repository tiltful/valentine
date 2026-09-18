use colored::*;
#[allow(unused_imports)]
use std::str::FromStr;

fn parse_rgb(color: &str) -> Option<(u8, u8, u8)> {
	let parts: Vec<&str> = color
		.trim_matches(|c| c == '(' || c == ')' || c == ' ')
		.split([',', ' '])
		.filter(|s| !s.is_empty())
		.collect();

	if parts.len() == 3 {
		if let (Ok(r), Ok(g), Ok(b)) = (
			parts[0].trim().parse(),
			parts[1].trim().parse(),
			parts[2].trim().parse(),
		) {
			return Some((r, g, b));
		}
	}
	None
}

/// Checks if a character is full-width (e.g., CJK ideographs)
fn is_wide_char(c: char) -> bool {
	matches!(c,
		'\u{1100}'..='\u{11FF}' |
		'\u{2E80}'..='\u{A4CF}' |
		'\u{AC00}'..='\u{D7A3}' |
		'\u{F900}'..='\u{FAFF}' |
		'\u{FE30}'..='\u{FE4F}' |
		'\u{FF00}'..='\u{FFEF}' |
		'\u{20000}'..='\u{2FA1F}'
	)
}

/// Builds a ASCII/Unicode heart shape rendered with the provided `name` characters and optional color.
pub fn heart_builder(name: &str, color: Option<&str>) -> String {
	let parsed_color = color.and_then(|c| {
		if let Some(rgb) = parse_rgb(c) {
			Some(Color::TrueColor {
				r: rgb.0,
				g: rgb.1,
				b: rgb.2,
			})
		} else {
			Color::from_str(c).ok()
		}
	});

	let name_chars: Vec<char> = name.chars().collect();
	if name_chars.is_empty() {
		return String::new();
	}

	// Determine if the input contains wide CJK characters to align background spacing
	let has_wide_chars = name_chars.iter().any(|&c| is_wide_char(c));
	let space_bg = if has_wide_chars { "  " } else { " " };

	let name_len = name_chars.len();
	let mut name_idx = 0;
	let mut output = String::with_capacity(4096);

	// Set horizontal step: 2 grid columns for wide CJK characters, 1 for standard characters
	let x_step = if has_wide_chars { 2 } else { 1 };

	// Grid coordinates adjusted for standard terminal character aspect ratios
	for y in (-13..13).rev() {
		let y_f = y as f64 * 0.1;
		let mut x = -25;
		while x < 25 {
			// Keep x_f calculation scaled to maintain consistent geometry
			let x_f = x as f64 * 0.05;

			// Heart implicit equation: (x^2 + y^2 - 1)^3 - x^2 * y^3 <= 0
			let formula = x_f.mul_add(x_f, y_f.powi(2)) - 1.0;
			let inside_heart = x_f.powi(2).mul_add(-y_f.powi(3), formula.powi(3)) <= 0.0;

			if inside_heart {
				let ch = name_chars[name_idx % name_len];
				let ch_str = ch.to_string();

				if let Some(color) = parsed_color {
					output.push_str(&ch_str.color(color).to_string());
				} else {
					output.push_str(&ch_str);
				}

				// If input mixes narrow and wide characters, pad narrow ones to maintain symmetry
				if has_wide_chars && !is_wide_char(ch) {
					output.push(' ');
				}

				name_idx += 1;
			} else {
				output.push_str(space_bg);
			}

			x += x_step;
		}
		output.push('\n');
	}

	output
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_rgb_valid_comma_separated() {
		assert_eq!(parse_rgb("255,255,255"), Some((255, 255, 255)));
	}

	#[test]
	fn parse_rgb_valid_space_separated() {
		assert_eq!(parse_rgb("255 255 255"), Some((255, 255, 255)));
	}

	#[test]
	fn parse_rgb_invalid_format() {
		assert_eq!(parse_rgb("255,255"), None);
	}

	#[test]
	fn parse_rgb_non_numeric() {
		assert_eq!(parse_rgb("abc,255,255"), None);
	}

	#[test]
	fn heart_builder_with_invalid_rgb_color() {
		let result = heart_builder("Love", Some("invalid"));
		assert!(!result.contains("\u{1b}[38;2;"));
	}

	#[test]
	fn heart_builder_without_color() {
		let result = heart_builder("Love", None);
		assert!(!result.contains("\u{1b}[38;2;"));
	}
}
