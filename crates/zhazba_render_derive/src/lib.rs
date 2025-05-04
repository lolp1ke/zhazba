use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, ItemEnum, parse_macro_input};


#[proc_macro_attribute]
pub fn ui_nodes(_args: TokenStream, item: TokenStream) -> TokenStream {
  let item_enum = parse_macro_input!(item as ItemEnum);
  let enum_ident = &item_enum.ident;


  let mut make_methods = Vec::new();
  for var in item_enum.variants.iter() {
    let var_ident = &var.ident;
    let method_name =
      format_ident!("make_{}", var_ident.to_string().to_lowercase());

    match &var.fields {
      Fields::Unnamed(fields) => {
        let defaults = fields.unnamed.iter().map(|f| {
          let ty = &f.ty;
          return quote! {
            <#ty as Default>::default()
          };
        });

        make_methods.push(quote! {
          // #[zhazba_lua::lua_method]
          pub fn #method_name() -> #enum_ident {
            return #enum_ident::#var_ident( #(#defaults),* );
          }
        });
      }
      _ => panic!("Not supported"),
    };
  }


  return TokenStream::from(quote! {
    #item_enum


    // #[zhazba_lua::lua_userdata]
    impl #enum_ident {
      #( #make_methods )*
    }
  });
}
