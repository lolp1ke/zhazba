use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{
  Fields, FnArg, ImplItem, ItemEnum, ItemImpl, Pat, parse_macro_input,
};


#[proc_macro_attribute]
pub fn lua_method(_args: TokenStream, item: TokenStream) -> TokenStream {
  item
}
#[proc_macro_attribute]
pub fn lua_userdata(_args: TokenStream, item: TokenStream) -> TokenStream {
  let item_impl = parse_macro_input!(item as ItemImpl);

  let mut lua_methods = Vec::new();
  for impl_item in item_impl.items.iter() {
    match impl_item {
      ImplItem::Fn(f) => {
        let mut early_exit = true;
        for f_attr in f.attrs.iter() {
          if f_attr.path().is_ident("lua_method") {
            early_exit = false;
            break;
          };
        }
        if early_exit {
          continue;
        };
        let method_name = &f.sig.ident;

        let mut arg_idents = Vec::new();
        let mut is_mut = false;
        for f_args in f.sig.inputs.iter() {
          match &f_args {
            FnArg::Typed(arg_pat) => {
              if let Pat::Ident(arg) = &*arg_pat.pat {
                arg_idents.push(arg.ident.clone());
              };
            }
            FnArg::Receiver(arg_self) => {
              is_mut = arg_self.mutability.is_some();
            }
          };
        }


        let add_method =
          format_ident!("add_method{}", if is_mut { "_mut" } else { "" });
        lua_methods.push(quote! {
          methods.#add_method(
            stringify!(#method_name),
            |_, this, (#(#arg_idents),*)| {
              let res = this.#method_name(#(#arg_idents),*);
              return Ok(res);
            }
          );
        });
      }

      _ => {}
    };
  }


  let self_ty = item_impl.self_ty.as_ref();
  let (impl_generics, ty_generics, where_clause) =
    item_impl.generics.split_for_impl();

  return TokenStream::from(quote! {
    #item_impl

    impl #impl_generics zhazba_lua::UserData for #self_ty #ty_generics #where_clause {
      fn add_methods<M: zhazba_lua::UserDataMethods<Self>>(methods: &mut M) {
          #(#lua_methods)*

          methods.add_meta_method(zhazba_lua::MetaMethod::ToString, |_, this: &Self, ()| {
            return Ok(format!("{:?}", this));
          });
      }
    }
  });
}

#[proc_macro_attribute]
pub fn lua_userdata_enum(_args: TokenStream, item: TokenStream) -> TokenStream {
  let item_enum = parse_macro_input!(item as ItemEnum);
  let enum_ident = &item_enum.ident;

  let mut method_defs = Vec::new();
  for variant in item_enum.variants.iter() {
    let var_ident = &variant.ident;
    match &variant.fields {
      Fields::Unit => {
        method_defs.push(quote! {
          methods.add_method(stringify!(#var_ident), |_, _, ()| {
            return Ok(#enum_ident::#var_ident);
          });
        });
      }
      Fields::Unnamed(fields) => {
        let num_fields = fields.unnamed.len();
        let arg_idents: Vec<_> = (0..num_fields)
          .map(|i| {
            syn::Ident::new(&format!("arg{i}"), proc_macro2::Span::call_site())
          })
          .collect();
        let arg_types = fields.unnamed.iter().map(|f| &f.ty);

        method_defs.push(quote! {
          methods.add_method(stringify!(#var_ident), |_, _, ( #(#arg_idents),* ): ( #(#arg_types),* )| {
            return Ok(#enum_ident::#var_ident( #(#arg_idents),* ));
          });
        });
      }

      _ => {}
    }
  }


  let factory = syn::Ident::new(
    &format!("{}UserDataFactory", enum_ident.to_string()),
    Span::call_site(),
  );
  return TokenStream::from(quote! {
    #item_enum

    pub struct #factory;
    impl zhazba_lua::UserData for #factory {
      fn add_methods<M: zhazba_lua::UserDataMethods<Self>>(methods: &mut M) {
        #(#method_defs)*
      }
    }
    impl zhazba_lua::UserData for #enum_ident {}
    impl zhazba_lua::FromLua for #enum_ident {
      fn from_lua(value: zhazba_lua::Value, _: &zhazba_lua::Lua) -> zhazba_lua::Result<Self> {
        return Ok(match value.as_userdata() {
          Some(ud) => match ud.borrow::<Self>() {
            Ok(ud) => ud.clone(),

            _ => {
              return Err(zhazba_lua::Error::RuntimeError(format!("Expected: {}", stringify!(#enum_ident))));
            }
          }

          _ => {
            return Err(zhazba_lua::Error::RuntimeError(format!("Expected userdata")));
          }
        });
      }
    }
  });
}
