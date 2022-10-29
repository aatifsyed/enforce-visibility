use std::iter;

use proc_macro::TokenStream;
use proc_macro_error::{emit_error, proc_macro_error};
use syn::{parse_macro_input, DeriveInput, Field, Visibility};

#[proc_macro_attribute]
#[proc_macro_error]
pub fn public(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = proc_macro2::TokenStream::from(attr);
    let orig_item = item.clone();
    let item = parse_macro_input!(item as DeriveInput);

    if !attr.is_empty() {
        emit_error!(attr, "arguments to this macro are not supported");
    }

    if !is_public(&item.vis) {
        emit_error!(
            item,
            "definition must be public: try adding `pub` before the declaration"
        );
    }

    for field in match item.data {
        syn::Data::Struct(data) => {
            Box::new(data.fields.into_iter()) as Box<dyn Iterator<Item = Field>>
        }
        syn::Data::Enum(_) => Box::new(iter::empty()), // enum fields are implied public
        syn::Data::Union(data) => Box::new(data.fields.named.into_iter()),
    } {
        if !is_public(&field.vis) {
            emit_error!(
                field,
                "field must be public: try adding `pub` before the declaration"
            )
        }
    }

    orig_item
}

fn is_public(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let t = trybuild::TestCases::new();
        t.pass("trybuild/pass/**/*.rs");
        t.compile_fail("trybuild/fail/**/*.rs")
    }
}
