extern crate proc_macro;
extern crate serde;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, LitInt, Meta};

#[proc_macro_derive(TbRow, attributes(order))]
pub fn derive_to_row(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (th_name, serde_body) = get_fields(&input.data);

    let expanded = quote! {
        impl serde::Serialize for #name  {
            fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
                where
                    S: serde::Serializer
                {
                   #serde_body
                }
        }

        impl tb_row::TbRow for #name  {
            fn th_name() -> Vec<String>{
                #th_name
            }
        }

    };

    proc_macro::TokenStream::from(expanded)
}

fn get_fields(data: &Data) -> (TokenStream, TokenStream) {
    let mut serde_body = quote!();
    match data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => {
                    let size = fields.named.len();

                    let mut my_vec: Vec<(Option<proc_macro2::Ident>, usize)> =
                        Vec::with_capacity(size);
                    for i in 0..size {
                        my_vec.push((None, i));
                    }

                    for field in fields.named.iter() {
                        for attr in field.attrs.iter() {
                            // #[order(N)]
                            if attr.path().is_ident("order") {
                                match &attr.meta {
                                    Meta::Path(_) => {}
                                    Meta::List(metalist) => {
                                        let lit: LitInt = metalist.parse_args().unwrap();
                                        let n: usize = lit.base10_parse().unwrap();
                                        my_vec.remove(n);
                                        my_vec.insert(n, (field.ident.clone(), n));
                                    }
                                    Meta::NameValue(_) => {}
                                }
                            }
                        }
                    }

                    let size: usize = my_vec.len();
                    serde_body.extend(quote!(
                        let mut ts: <S as Serializer>::SerializeTupleStruct = serializer.serialize_tuple_struct("row", #size)?;
                    ));

                    let mut fields_vec_innards = quote!();
                    for i in my_vec {
                        let field_ident: proc_macro2::Ident = i.0.unwrap();

                        serde_body.extend(quote!(
                                ts.serialize_field(&self.#field_ident)?;
                        ));

                        fields_vec_innards.extend(quote!(stringify!(#field_ident).to_string(),));
                    }
                    serde_body.extend(quote!(ts.end()));

                    let fields_vec = quote!(vec![#fields_vec_innards]);
                    (fields_vec, serde_body)
                }
                Fields::Unnamed(_) | Fields::Unit => unimplemented!(),
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
