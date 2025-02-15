use proc_macro2::TokenStream;
use quote::quote;
use serde_derive_internals::{ast as serde_ast, Ctxt, Derive};
use syn::{parse_quote, DeriveInput};

pub(crate) fn serialize_fields(input: DeriveInput) -> TokenStream {
    let cx = Ctxt::new();
    let serde_container = serde_ast::Container::from_ast(&cx, &input, Derive::Serialize);
    cx.check().unwrap();
    let container = serde_container.expect("failed to get serde attrs for container");

    match &container.data {
        serde_ast::Data::Struct(style, vec) => match style {
            serde_ast::Style::Struct => {
                let struct_name = &container.ident;
                let struct_name_str = container.attrs.name().serialize_name();

                let mut generics = container.generics.clone();

                if !vec.is_empty() {
                    let where_clause = generics.make_where_clause();
                    vec.iter().for_each(|field| {
                        let ty = field.ty;
                        let traits = if field.attrs.flatten() {
                            quote!(SerializeFields)
                        } else {
                            quote!(Serialize)
                        };
                        where_clause.predicates.push(parse_quote!(#ty: #traits));
                    });
                }

                let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

                let num_unflattened_fields =
                    vec.iter().filter(|field| !field.attrs.flatten()).count();
                let num_fields = vec.iter().filter(|field| field.attrs.flatten()).fold(
                    quote! { #num_unflattened_fields },
                    |num_fields, field| {
                        let ty = field.ty;
                        quote! { #num_fields + <#ty as serde_fast_flatten::SerializeFields>::NUM_FIELDS }
                    },
                );

                let serialize_fields = vec.iter().fold(quote!(), |serialize_fields, field| {
                    let field_name = match &field.member {
                        syn::Member::Named(ident) => ident,
                        syn::Member::Unnamed(_) => {
                            panic!("unnamed field in named field struct")
                        }
                    };
                    let serialize_field = if field.attrs.flatten() {
                        quote!(self.#field_name.serialize_fields(s)?;)
                    } else {
                        let field_name_str = field.attrs.name().serialize_name();
                        quote!(s.serialize_field(#field_name_str, &self.#field_name)?;)
                    };
                    quote!(#serialize_fields #serialize_field)
                });

                quote!(const _ : () = {
                    use serde::{Serialize, Serializer, ser::SerializeStruct};
                    use serde_fast_flatten::SerializeFields;

                    #[automatically_derived]
                    impl #impl_generics Serialize for #struct_name #ty_generics #where_clause {
                        #[inline]
                        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
                        where
                            S: serde::Serializer,
                        {
                            let mut s = serializer.serialize_struct(#struct_name_str, Self::NUM_FIELDS)?;
                            self.serialize_fields(&mut s)?;
                            s.end()
                        }
                    }

                    #[automatically_derived]
                    impl #impl_generics SerializeFields for #struct_name #ty_generics #where_clause {
                        const NUM_FIELDS : usize = #num_fields;
                        #[inline]
                        fn serialize_fields<S: SerializeStruct>(
                            &self,
                            s: &mut S
                        ) -> std::result::Result<(), S::Error> {
                            #serialize_fields
                            Ok(())
                        }
                    }
                };)
            }
            _ => panic!("SerializeFields cannot be auto-derived for struct without named fields"),
        },
        serde_ast::Data::Enum(_) => panic!("SerializeFields cannot be auto-derived for enum"),
    }
}
