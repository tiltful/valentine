use colored::*;
use std::str::FromStr;
use unicode_width::UnicodeWidthChar;

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

/// Builds an ASCII/Unicode heart shape rendered with the provided `name` characters and optional color.
/// Automatically handles wide characters (e.g., CJK ideographs, Hangul) to maintain correct aspect ratio and symmetry.
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

	// Determine if the input contains full-width characters (e.g., CJK, Hangul)
	let has_wide_chars = name_chars
		.iter()
		.any(|&c| UnicodeWidthChar::width(c).unwrap_or(1) > 1);

	// Adjust grid spacing depending on character width
	let space_bg = if has_wide_chars { "  " } else { " " };
	let x_step = if has_wide_chars { 2 } else { 1 };

	let name_len = name_chars.len();
	let mut name_idx = 0;
	let mut output = String::with_capacity(4096);

	for y in (-13..13).rev() {
		let y_f = y as f64 * 0.1;
		let mut x = -25;
		while x < 25 {
			let x_f = x as f64 * 0.05;

			// Heart implicit equation: (x^2 + y^2 - 1)^3 - x^2 * y^3 <= 0
			let formula = x_f.mul_add(x_f, y_f.powi(2)) - 1.0;
			let inside_heart = x_f.powi(2).mul_add(-y_f.powi(3), formula.powi(3)) <= 0.0;

			if inside_heart {
				let ch = name_chars[name_idx % name_len];
				let ch_width = UnicodeWidthChar::width(ch).unwrap_or(1);
				let ch_str = ch.to_string();

				if let Some(color) = parsed_color {
					output.push_str(&ch_str.color(color).to_string());
				} else {
					output.push_str(&ch_str);
				}

				// Pad narrow characters with a space if the input mixes narrow and wide characters
				if has_wide_chars && ch_width == 1 {
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
	fn parse_rgb_with_parentheses() {
		assert_eq!(parse_rgb("(255, 128, 0)"), Some((255, 128, 0)));
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
	fn heart_builder_empty_name() {
		assert_eq!(heart_builder("", None), "");
	}

	#[test]
	fn heart_builder_latin_and_cyrillic() {
		let result_en = heart_builder("Love", None);
		let result_ru = heart_builder("Любовь", None);

		assert!(!result_en.is_empty());
		assert!(!result_ru.is_empty());
		assert!(result_en.contains('L'));
		assert!(result_ru.contains('Л'));
	}

	#[test]
	fn heart_builder_cjk_and_hangul() {
		let result_cn = heart_builder("爱", None);
		let result_kr = heart_builder("사랑", None);

		assert!(!result_cn.is_empty());
		assert!(!result_kr.is_empty());
		assert!(result_cn.contains('爱'));
		assert!(result_kr.contains('사'));
		// Wide characters should trigger double-space background padding
		assert!(result_cn.contains("  "));
	}
}
