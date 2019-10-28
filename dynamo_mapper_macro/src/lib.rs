extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(DynamoMapper)]
pub fn dynamo_mapper_macro_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    return implem_dynamo_mapper_macro(&ast);
}

fn implem_dynamo_mapper_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    if let syn::Data::Struct(data) = &ast.data {
        let mut to_dynamo_lines = Vec::new();
        let mut from_dynamo_lines = Vec::new();

        for field in data.fields.iter() {
            if let Some(field_name) = &field.ident {
                let to_line = quote! {
                    if let Some(value) = self.#field_name.to_dynamo() {
                        item.insert(String::from(stringify!(#field_name)), value);
                    }
                };

                to_dynamo_lines.push(to_line);

                if let syn::Type::Path(type_path) = &field.ty {
                    from_dynamo_lines.push(quote! {
                        if let Some(field_value) = <#type_path>::from_dynamo(&item, String::from(stringify!(#field_name))) {
                            result.#field_name = field_value;
                        }
                    });
                }
                else {
                    panic!("Unknown type");
                };

            }
        }

        let gen = quote! {
            impl DynamoMapper for #name {
                fn to_dynamo(&self) -> DynamoItem {
                    let mut item = DynamoItem::new();
                    #(#to_dynamo_lines)*
                    return item;
                }

                fn from_dynamo(item: &DynamoItem) -> #name {
                    let mut result = #name {..Default::default()};
                    #(#from_dynamo_lines)*
                    return result;
                }
            }
        };
        gen.into()
    } else {
        panic!("Should only be used on struct");
    }
}
