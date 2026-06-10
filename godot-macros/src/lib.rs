use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ImplItem, ImplItemFn, ItemImpl};

#[proc_macro_attribute]
pub fn godot_damageable(_attr: TokenStream, item: TokenStream) -> TokenStream {
	let mut input = parse_macro_input!(item as ItemImpl);

	if input.trait_.is_some() {
		return quote! {
			compile_error!("#[godot_damageable] must be placed on an inherent impl (typically the same impl that has #[godot_api]), not on impl Damageable for Type");
			#input
		}
		.into();
	}

	let funcs = input.items.iter().filter_map(|item| {
		if let ImplItem::Fn(method) = item && method.attrs.iter().any(|attr| attr.path().is_ident("func")) {
			Some(method)
		} else {
			None
		}
	}).collect::<Vec<_>>();

	let has_apply_damage_impl = funcs.iter().any(|method|
		method.sig.ident == "apply_damage" &&
		method.sig.inputs.len() == 2 &&
		matches!(method.sig.inputs[1].clone(), syn::FnArg::Typed(pat_type) if matches!(&*pat_type.ty, syn::Type::Path(type_path) if type_path.path.is_ident("f32")))
	);
	let has_kill_impl = funcs.iter().any(|method|
		method.sig.ident == "kill" &&
		method.sig.inputs.len() == 1
	);


	if !has_apply_damage_impl {
		let method: ImplItemFn = syn::parse_quote! {
			/// Auto-implemented function to apply damage to this Node, as a Damageable.
			/// Mainly intended for use through Godot.
			#[func]
			pub fn apply_damage(&mut self, amount: f32) {
				< Self as boundless::damage::Damageable >::apply_damage(self, amount);
			}
		};
		input.items.push(ImplItem::Fn(method));
	}

	if !has_kill_impl {
		let method: ImplItemFn = syn::parse_quote! {
			/// Auto-implemented function to kill this Node, as a Damageable.
			/// Mainly intended for use through Godot.
			#[func]
			pub fn kill(&mut self) {
				< Self as boundless::damage::Damageable >::kill(self);
			}
		};
		input.items.push(ImplItem::Fn(method));
	}

	quote!(#input).into()
}
