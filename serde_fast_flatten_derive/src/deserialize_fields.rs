use proc_macro2::TokenStream;
use quote::quote;
use serde_derive_internals::{ast as serde_ast, Ctxt, Derive};
use syn::DeriveInput;

pub(crate) fn deserialize_fields(input: DeriveInput) -> TokenStream {
    let cx = Ctxt::new();
    let serde_container = serde_ast::Container::from_ast(&cx, &input, Derive::Deserialize);
    cx.check().unwrap();
    let container = serde_container.expect("failed to get serde attrs for container");

    match &container.data {
        serde_ast::Data::Struct(style, vec) => match style {
            serde_ast::Style::Struct => {
                let struct_name = &container.ident;
                let struct_name_str = container.attrs.name().deserialize_name();

                fn get_field_name<'a>(field: &'a serde_ast::Field<'_>) -> &'a syn::Ident {
                    match &field.member {
                        syn::Member::Named(ident) => ident,
                        syn::Member::Unnamed(_) => panic!("unnamed field in named field struct"),
                    }
                }

                let field_names = vec.iter().fold(quote!(), |fields, field| {
                    let field_name = get_field_name(field);
                    quote!(#fields #field_name,)
                });

                let field_names_str = vec.iter().fold(quote!(), |fields, field| {
                    let field_name_str = field.attrs.name().deserialize_name();
                    quote!(#fields #field_name_str,)
                });

                let fields = vec.iter().fold(quote!(), |fields, field| {
                    let field_name = get_field_name(field);
                    let ty = &field.ty;
                    let field_type = if field.attrs.flatten() {
                        quote!(<#ty as DeserializeFields<'de>>::FieldDeserializer)
                    } else {
                        quote!(Option<#ty>)
                    };
                    quote!(#fields #field_name: #field_type,)
                });

                let deserialize_seq_fields =
                    vec.iter().fold(quote!(), |deserialize_fields, field| {
                        let field_name = get_field_name(field);
                        let field_name_str = field.attrs.name().deserialize_name();
                        let error = format!("missing field `{}`", field_name_str);
                        quote!(
                            #deserialize_fields
                            let #field_name = seq.next_element()?.ok_or_else(|| {
                                <S::Error as serde::de::Error>::custom(#error)
                            })?;
                        )
                    });

                let (deserialize_unflattened_fields, flattened_fields, _, _) = vec.iter().fold(
                    (quote!(), Vec::new(), 0u64, quote!()),
                    |(deserialize_fields, mut flattened_fields, field_id, add_field_ids), field| {
                        if field.attrs.flatten() {
                            flattened_fields.push((field, quote!(#field_id #add_field_ids)));
                            let field_type = &field.ty;
                            let add_field_ids =
                                quote!(#add_field_ids + #field_type::NUM_FIELDS as u64);
                            (
                                deserialize_fields,
                                flattened_fields,
                                field_id,
                                add_field_ids,
                            )
                        } else {
                            let field_name = get_field_name(field);
                            let field_name_str = field.attrs.name().deserialize_name();
                            let deserialize_fields = quote!(
                                #deserialize_fields
                                BorrowedFieldId::String(#field_name_str)
                                //| BorrowedFieldId::Bytes(#field_name_str)
                                | BorrowedFieldId::U64(#field_id #add_field_ids) => {
                                    self.#field_name = Some(map.next_value()?);
                                    Ok(Ok(()))
                                },
                            );
                            (
                                deserialize_fields,
                                flattened_fields,
                                field_id + 1,
                                add_field_ids,
                            )
                        }
                    },
                );

                let deserialize_flattened_fields = flattened_fields
                    .iter()
                    .rev()
                    .map(|(field, offset)| {
                        let field_name = get_field_name(field);
                        quote!(self.#field_name.deserialize_field(field.offset(#offset), map))
                    })
                    .reduce(|fields, field| {
                        quote!(match #field? {
                            Ok(()) => Ok(Ok(())),
                            Err(field) => #fields
                        })
                    })
                    .unwrap_or_else(|| quote!(Ok(Err(field))));

                let deserialize_fields = quote!(
                    #deserialize_unflattened_fields
                    _ => #deserialize_flattened_fields
                );

                let finish_fields = vec.iter().fold(quote!(), |finish_fields, field| {
                    let field_name = get_field_name(field);
                    let field_name_str = field.attrs.name().deserialize_name();
                    let finish_field = if field.attrs.flatten() {
                        quote!(self.#field_name.finish()?)
                    } else {
                        let error = format!("missing field `{field_name_str}`");
                        quote!(self.#field_name.ok_or_else(|| E::custom(#error))?)
                    };
                    quote!(#finish_fields #field_name: #finish_field,)
                });

                quote!(const _: () = {

                    use serde::{
                        Deserialize,
                        de::{Deserializer, MapAccess, SeqAccess, IgnoredAny},
                    };
                    use serde_fast_flatten::{
                        BorrowedFieldId, DeserializeFields, FieldDeserializer,
                        FieldId, SerializeFields
                    };

                    #[derive(Default)]
                    struct Fields<'de> {
                        #fields
                        _marker: std::marker::PhantomData<&'de ()>
                    }

                    struct Visitor;

                    impl<'de> serde::de::Visitor<'de> for Visitor {
                        type Value = #struct_name;

                        fn expecting(
                            &self,
                            formatter: &mut std::fmt::Formatter,
                        ) -> std::fmt::Result {
                            write!(formatter, "struct {}", #struct_name_str)
                        }

                        #[inline]
                        fn visit_map<M>(
                            self,
                            mut map: M,
                        ) -> Result<Self::Value, M::Error>
                        where
                            M: MapAccess<'de>,
                        {
                            let mut fields = Fields::default();
                            while let Some(field) = map.next_key::<FieldId<'de>>()? {
                                if let Err(_field) =
                                    fields.deserialize_field(field, &mut map)?
                                {
                                    map.next_value::<IgnoredAny>()?;
                                }
                            }
                            fields.finish()
                        }

                        #[inline]
                        fn visit_seq<S>(
                            self,
                            mut seq: S,
                        ) -> Result<Self::Value, S::Error>
                        where
                            S: SeqAccess<'de>,
                        {
                            #deserialize_seq_fields
                            Ok(#struct_name { #field_names })
                        }
                    }

                    impl<'de> Deserialize<'de> for #struct_name {
                        #[inline]
                        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                        where
                            D: Deserializer<'de>,
                        {
                            const FIELDS: &[&str] = &[#field_names_str];
                            deserializer.deserialize_struct(#struct_name_str, FIELDS, Visitor)
                        }
                    }

                    impl<'de> DeserializeFields<'de> for #struct_name {
                        type FieldDeserializer = Fields<'de>;
                    }

                    impl<'de> FieldDeserializer<'de> for Fields<'de> {
                        type Value = #struct_name;

                        #[inline]
                        fn deserialize_field<A: MapAccess<'de>>(
                            &mut self,
                            field: FieldId<'de>,
                            map: &mut A,
                        ) -> Result<Result<(), FieldId<'de>>, A::Error>
                        {
                            match field.borrow() {
                                #deserialize_fields
                            }
                        }

                        #[inline]
                        fn finish<E: serde::de::Error>(self) -> Result<Self::Value, E> {
                            Ok(#struct_name { #finish_fields })
                        }
                    }
                };)
            }
            _ => panic!("DeserializeFields cannot be auto-derived for struct without named fields"),
        },
        serde_ast::Data::Enum(_) => panic!("DeserializeFields cannot be auto-derived for enum"),
    }
}
