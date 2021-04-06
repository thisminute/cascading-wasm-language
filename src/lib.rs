extern crate html_minifier;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;
mod data;
mod misc;
mod transform;

use {
	data::{ast::Document, semantics::Semantics},
	proc_macro::TokenStream,
	proc_macro2::TokenStream as TokenStream2,
	quote::ToTokens,
	std::{
		collections::HashMap,
		fs::{read_dir, read_to_string, write},
		path::Path,
	},
	syn::parse_macro_input,
};

// type BoxResult<T> = Result<T, Box<dyn std::error::ErrorError>>;

fn pipeline(document: Document) -> (HashMap<String, String>, TokenStream2) {
	let mut semantics = document.analyze();
	semantics.render();
	for (i, group) in semantics.groups.iter().enumerate() {
		eprintln!("{} {:?}", i, group);
	}
	(semantics.html(), semantics.wasm(true))
}

#[proc_macro]
pub fn cwl(input: TokenStream) -> TokenStream {
	let mut input = input.into();

	// if it exists, import .cwl files from the `cwl` directory and attach them to the input
	let path = "./cwl";
	if Path::new(path).exists() {
		for entry in read_dir(path).expect(&*format!("reading from {}", path)) {
			let entry = entry.expect("reading .cwl file");
			let filename = entry.path().display().to_string();
			if filename.ends_with(".cwl") {
				let contents: TokenStream2 = read_to_string(entry.path()).unwrap().parse().unwrap();
				contents.to_tokens(&mut input);
			}
		}
	}

	let input = input.into();
	let (pages, runtime) = pipeline(parse_macro_input!(input as Document));

	for (filename, html) in pages {
		let filename = match &*filename {
			"/" => "index.html".into(),
			_ => filename,
		};
		let destination = format!("target/html/{}", filename);
		write(&destination, html).expect(&*format!("writing output html code to {}", destination));
	}
	write("target/cwl_macro_output.rs", runtime.to_string()).expect("writing output rust code");

	runtime.into()
}

#[proc_macro]
pub fn cwl_document(input: TokenStream) -> TokenStream {
	let document = parse_macro_input!(input as Document);
	let mut semantics = document.analyze();
	semantics.render();
	let thing = semantics.wasm(false).into();
	eprintln!("{}", thing);
	thing
}

#[proc_macro]
pub fn cwl_header(_input: TokenStream) -> TokenStream {
	Semantics::header().into()
}
