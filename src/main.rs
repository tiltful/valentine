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
#[deny(unreachable_pub)]
#[allow(clippy::type_complexity)]
#[cfg_attr(test, allow(clippy::float_cmp))]
#[cfg_attr(not(test), warn(clippy::dbg_macro))]
use clap::Parser;
mod utils;

/// Command-line argument parser and main function.
///
/// This module defines the `Args` struct for parsing command-line arguments
/// and the `main` function which serves as the entry point of the application.

#[derive(Parser, Debug)]
struct Args {
	/// Set the name
	///
	/// # Examples
	///
	/// ```
	/// --name John
	/// -n John
	/// ```
	#[arg(short, long)]
	name: String,

	/// Set the color
	///
	/// # Examples
	///
	/// ```
	/// --color red
	/// -c red
	/// ```
	#[arg(short, long)]
	color: Option<String>,
}

/// Main function.
///
/// Parses command-line arguments, constructs a heart shape with the given name
/// and optional color, and prints it to the console.
fn main() {
	// Parse command-line arguments into `Args` struct
	let opts: Args = Args::parse();

	// Extract the name and color from the parsed arguments
	let name = opts.name;
	let color = opts.color;

	// Build the heart shape using the provided name and color
	let heart = utils::heart_builder(&name, color.as_deref());

	// Print the heart shape to the console
	println!("{}", heart);
}
