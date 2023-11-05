use proc_macro::{TokenStream, TokenTree};


#[proc_macro_attribute]
pub fn declare_plugin(attr: TokenStream, mut item: TokenStream) -> TokenStream {
	
	let attribute = parse_atrribute(attr);
	let meta = parse_item(item.clone());

	println!("{:#?}, {:#?}", attribute, meta);

	if let (Some(name), Some(type_name)) = (attribute.name, meta.type_name) {
		let plugin_create = format!("
			#[no_mangle]
			#[link_section = \".plugins\"]
			pub unsafe extern \"C\" fn __create_{0}() -> my_devkit_plugin::ffi::Plugin {{
				{1}::default().into()
			}}
		", name, type_name);

		let stream: TokenStream = plugin_create.parse().unwrap();
		item.extend(stream);
	}

	item
}

#[derive(Debug)]
struct PluginAttribute {
	pub name: Option<String>
}

impl Default for PluginAttribute {
    fn default() -> Self {
        Self { name: None }
    }
}

fn parse_atrribute(stream: TokenStream) -> PluginAttribute {
	let mut attribute = PluginAttribute::default();
	let mut iter = stream.into_iter();
	
	loop {

		if let Some(token) = iter.next() {

			let name = match token {
				TokenTree::Ident(ident) => ident.to_string(),
				_ => break
			};

			let token = iter.next();
			if token.is_none() { break; }

			let _ = match token.unwrap() {
				TokenTree::Punct(p) => p,
				_ => break
			};

			let token = iter.next();
			if token.is_none() { break; }

			let literal = match token.unwrap() {
				TokenTree::Literal(l) => l,
				_ => break
			};

			let value = literal.to_string().replace("\"", "");

			match name.to_string().as_str() {
			 	"name" => attribute.name = Some(value),
				_ => {} 
			}

		}

		break;
	}


	attribute
}

#[derive(Debug)]
struct PluginMeta {
	pub type_name: Option<String>
}

impl Default for PluginMeta {
    fn default() -> Self {
        Self { type_name: None }
    }
}

fn parse_item(stream: TokenStream) -> PluginMeta {

	let mut meta = PluginMeta::default();
	let mut iter = stream.into_iter();
	
	loop {
		if let Some(token) = iter.next() {
			if let TokenTree::Ident(intent) = token {
				if intent.to_string().eq("struct") {
					let type_name = iter.next().and_then(|t| match t {
						TokenTree::Ident(i) => Some(i.to_string()),
						_ => None	
					});

					meta.type_name = type_name;
					break;
				}
			}
		} 
		else {
			break;
		}
	}

	meta
}