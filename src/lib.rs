#![allow(elided_lifetimes_in_paths)]
#![warn(rust_2018_idioms)]
#![warn(future_incompatible)]
#[warn(
	clippy::all,
	clippy::dbg_macro,
	clippy::todo,
	clippy::empty_enum,
	clippy::enum_glob_use,
	clippy::mem_forget,
	clippy::unused_self,
	clippy::filter_map_next,
	clippy::needless_continue,
	clippy::needless_borrow,
	clippy::match_wildcard_for_single_variants,
	clippy::if_let_mutex,
	clippy::await_holding_lock,
	clippy::match_on_vec_items,
	clippy::imprecise_flops,
	clippy::suboptimal_flops,
	clippy::lossy_float_literal,
	clippy::rest_pat_in_fully_bound_structs,
	clippy::fn_params_excessive_bools,
	clippy::exit,
	clippy::inefficient_to_string,
	clippy::linkedlist,
	clippy::macro_use_imports,
	clippy::option_option,
	clippy::verbose_file_reads,
	clippy::unnested_or_patterns,
	clippy::str_to_string,
	nonstandard_style,
	missing_debug_implementations,
	missing_docs
)]
#[allow(clippy::type_complexity)]
#[cfg_attr(test, allow(clippy::float_cmp))]
#[cfg_attr(not(test), warn(clippy::dbg_macro))]
mod utils;
use wasm_bindgen::prelude::*;
use web_sys::{console, window};

/// Entry point for the WebAssembly module.
///
/// This function is called when the WebAssembly module is loaded and executed.
/// It prompts the user for a name, constructs a heart shape using the provided
/// name, and logs the heart shape to the console. Additionally, it updates the
/// inner HTML of an element with the id "output" if it exists.
#[wasm_bindgen(start)]
fn start() {
	// Get the global `window` object, or panic if it does not exist.
	let window = window().expect("no global `window` exists");

	// Get the `document` object from the `window`, or panic if it does not exist.
	let document = window.document().expect("should have a document on window");

	// Prompt the user to enter a name and store the result.
	let name = window
		.prompt_with_message("Enter your Valentine's name:")
		.unwrap_or_default()
		.unwrap();

	// Build a heart shape string using the provided name.
	let heart_str = utils::heart_builder(&name, None).to_string();

	// Log the heart shape string to the console.
	console::log_1(&heart_str.into());

	// Update the inner HTML of the element with id "output", if it exists.
	if let Some(output) = document.get_element_by_id("output") {
		output.set_inner_html("Look at the console for the heart!");
	} else {
		// Log a message to the console if the element with id "output" is not found.
		console::log_1(&"Element with id 'output' not found".into());
	}
}
