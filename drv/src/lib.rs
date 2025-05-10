use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, FieldsNamed, Ident, Type, TypePath, parse_macro_input};

fn is_app_env_type(ty: &Type) -> bool {
	match ty {
		Type::Path(TypePath { path, .. }) => path.is_ident("AppEnv"),
		_ => false,
	}
}

fn handle_named_field(name: &Ident, field: &FieldsNamed) -> TokenStream {
	let extract_ast = field.named.iter().map(|f| {
		let field_name = f.ident.as_ref().unwrap();
		let field_type = &f.ty;

		if is_app_env_type(&field_type) {
			quote! {
				let #field_name = req.app_state::<#field_type>().ok_or(Self::Error::NotConfigured)?;
			}
		} else {
			quote! {
				let #field_name = <#field_type as ntex::web::FromRequest<E>>::from_request(req, _payload).await?;
			}
		}
	});

	let def_ast = field.named.iter().map(|f| {
		let field_name = f.ident.as_ref().unwrap();

		quote! {
			#field_name: #field_name.clone()
		}
	});

	let ast = quote! {
		#(#extract_ast)*

		let state = #name {
			#(#def_ast),*
		};

		Ok(state)
	};

	dbg!(ast.to_string());

	ast
}

fn handle_struct_field(name: &Ident, field: &Data) -> TokenStream {
	match field {
		Data::Struct(f) => match &f.fields {
			Fields::Named(name_field) => handle_named_field(name, name_field),
			_ => panic!(""),
		},
		_ => panic!("AppEnv仅支持struct定义的数据结构！"),
	}
}

#[proc_macro_derive(AppEnv)]
pub fn appenv_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(input as DeriveInput);

	let data_name = input.ident;
	let def = handle_struct_field(&data_name, &input.data);

	let ast = quote! {
		impl<E: ntex::web::ErrorRenderer> ntex::web::FromRequest<E> for #data_name {
			type Error = ntex::web::error::StateExtractorError;

			async fn from_request(req: &ntex::web::HttpRequest, _payload: &mut ntex::http::Payload) -> Result<Self, Self::Error> {
				#def
			}
		}
	};

	println!("{ast}");

	proc_macro::TokenStream::from(ast)
}
