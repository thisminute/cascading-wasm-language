use {
	crate::{
		meta::{Context, Meta},
		tokens::*,
	},
	syn::{
		export::quote::{quote, quote_spanned},
		spanned::Spanned,
	},
};

pub trait Lex {
	fn lex(&self, meta: &mut Meta, context: Option<&Context>);
}

impl Lex for Document {
	fn lex(&self, meta: &mut Meta, context: Option<&Context>) {
		self.root.lex(meta, context);

		match &meta.title {
			Some(_) => {}
			None => meta.warnings.push(quote! {
				compile_error!("you must set a title for the page");
			}),
		};
	}
}

impl Lex for Block {
	fn lex(&self, meta: &mut Meta, context: Option<&Context>) {
		match self.prefix {
			Prefix::Instance => {
				for rule in &self.rules {
					rule.lex(meta, context);
				}
			}
			Prefix::Class => {}
			Prefix::Action => {}
		};
	}
}

impl Lex for Rule {
	fn lex(&self, meta: &mut Meta, _context: Option<&Context>) {
		let property = self.property.to_string();
		let value = &self.value;
		let at_root = true; // context.path.is_none();

		match &property.to_string()[..] {
			// meta information for the page and/or project must be defined at the top level
			"title" if at_root => match &meta.title {
				Some(_title) => meta.errors.push(quote_spanned! {
					value.span()=> compile_error!("title property cannot be set more than once");
				}),
				None => meta.title = Some(quote! { #value }),
			},

			"text" => {}
			"link" => {}
			"tip" => {}
			_ => {}
		}
	}
}