use proc_macro::TokenStream;
use quote::quote;
use syn::{
  Attribute, ImplItem, ItemImpl, Meta, MetaNameValue, parse_macro_input,
};


#[proc_macro_attribute]
pub fn lua_userdata(args: TokenStream, item: TokenStream) -> TokenStream {
  let impl_item = parse_macro_input!(item as ItemImpl);

  let mut impl_methods = Vec::new();
  for method in impl_item.items.iter() {
    if let ImplItem::Fn(f) = method {
      for attr in f.attrs.iter() {
        if attr.path().is_ident("lua_method") {
          let method_name = f.sig.ident.to_string();
          let rust_ident = &f.sig.ident;

          impl_methods.push(quote! {
            methods.add_method(#method_name, |_, this, args| {
              let res = this.#rust_ident(args)?;
              return Ok(res);
          });
          });
        };
      }
    };
  }

  let self_ty = impl_item.self_ty.as_ref();
  let (impl_generics, ty_generics, where_clause) =
    impl_item.generics.split_for_impl();


  return TokenStream::from(quote! {
    #impl_item

    impl #impl_generics mlua::UserData for #self_ty #ty_generics #where_clause {
      fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
          #(#impl_methods)*
      }
  }

  });
}

// struct LuaWrapper<T>(Arc<RefCell<T>>);
// impl<T> Deref for LuaWrapper<T> {
//   type Target = Arc<RefCell<T>>;

//   fn deref(&self) -> &Self::Target {
//     return &self.0;
//   }
// }

// macro_rules! impl_lua_userdata {
//   ($ident:ident, $add_methods_block:block) => {
//     impl LuaUserData for LuaWrapper<$ident> {
//       fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) $add_methods_block
//     }
//   };
// }
// impl_lua_userdata!(Config, {
//   // methods.add_method();
// });
