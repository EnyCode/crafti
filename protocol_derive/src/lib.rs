use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2, TokenTree};
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, Ident, Index};

#[allow(dead_code)]
struct PacketData {
    id: i32,
}

#[proc_macro_derive(MinecraftPacket, attributes(var, packet))]
pub fn derive_minecraftpacket(input: TokenStream) -> TokenStream {
    let cloned_input = input.clone();
    let stream = parse_macro_input!(cloned_input as DeriveInput);
    let mut data: Option<PacketData> = None;
    let name = stream.ident;

    for attr in stream.attrs {
        if attr.path().is_ident("packet") {
            data = Some(parse_packet_data(attr));
        }
    }

    if data.is_some() {
        let data = data.unwrap();
        let read: TokenStream2 = derive_minecraftreadable(input.clone()).into();
        let write: TokenStream2 = derive_minecraftwriteable(input).into();

        let id = data.id;

        let data = quote! {
            #read
            #write
            impl crate::protocol::stream::MinecraftPacket for #name {
                fn get_id() -> i32 {
                    #id
                }
            }
        };

        return data.into();
    }

    panic!("#[packet] attribute not defined");
}

#[proc_macro_derive(MinecraftReadable, attributes(var))]
pub fn derive_minecraftreadable(input: TokenStream) -> TokenStream {
    let stream = parse_macro_input!(input as DeriveInput);

    let struct_name = stream.ident;

    // TODO: enum support
    let field_data = match stream.data {
        Data::Struct(data) => data.fields,
        _ => panic!("Deriving from MinecraftReadable is only supported for structs"),
    };

    let token = match field_data {
        Fields::Named(named) => {
            let fields = named.named;
            let mut code: Vec<TokenStream2> = Vec::new();

            for field in fields {
                let name = field.ident.unwrap();
                let attrs = field.attrs;
                let ty = field.ty;
                let mut is_var = false;

                for attr in attrs {
                    let path = attr.meta.require_path_only();
                    if path.is_err() {
                        continue;
                    }
                    if path.unwrap().is_ident("var") {
                        is_var = true;
                        code.push(quote! {
                            #name: <#ty as crate::protocol::read::MinecraftReadableVar<R>>::read_var_from(buffer).await?,
                        });
                        continue;
                    }
                }

                if !is_var {
                    code.push(quote! {
                        #name: <#ty as crate::protocol::read::MinecraftReadable<R>>::read_from(buffer).await?,
                    });
                }
            }

            let output = quote! {
                #[async_trait::async_trait] impl<R: async_std::io::Read + Unpin + Send + Sync> crate::protocol::read::MinecraftReadable<R> for #struct_name {
                    async fn read_from(buffer: &mut R) -> Result<Self, anyhow::Error> {
                        Ok(Self { #(#code)* })
                    }
                }
            };

            output.into()
        }
        Fields::Unnamed(unnamed) => {
            let fields = unnamed.unnamed;
            let mut code: Vec<TokenStream2> = Vec::new();

            for (index, field) in fields.iter().enumerate() {
                let name = Index::from(index);
                let attrs = &field.attrs;
                let ty = &field.ty;
                let mut is_var = false;

                for attr in attrs {
                    let path = attr.meta.require_path_only();
                    if path.is_err() {
                        continue;
                    }
                    if path.unwrap().is_ident("var") {
                        is_var = true;
                        code.push(quote! {
                            #name: <#ty as crate::protocol::read::MinecraftReadableVar<R>>::read_var_from(buffer).await?,
                        });
                        continue;
                    }
                }

                if !is_var {
                    code.push(quote! {
                        #name: <#ty as crate::protocol::read::MinecraftReadable<R>>::read_from(buffer).await?,
                    });
                }
            }

            let output = quote! {
                #[async_trait::async_trait] impl<R: async_std::io::Read + Unpin + Send + Sync> crate::protocol::read::MinecraftReadable<R> for #struct_name {
                    async fn read_from(buffer: &mut R) -> Result<Self, anyhow::Error> {
                        Ok(Self { #(#code)* })
                    }
                }
            };

            output.into()
        },
        _ => panic!(
            "Deriving from MinecraftReadable/Writeable is only supported for structs with named or unnamed fields"
        ),
    };

    return token;
}

#[proc_macro_derive(MinecraftWriteable, attributes(var))]
pub fn derive_minecraftwriteable(input: TokenStream) -> TokenStream {
    let stream = parse_macro_input!(input as DeriveInput);

    let struct_name = stream.ident;

    // TODO: enums
    let field_data = match stream.data {
        Data::Struct(data) => data.fields,
        _ => panic!("Deriving from MinecraftWriteable is only supported for structs"),
    };

    let token = match field_data {
        Fields::Named(named) => {
            let fields = named.named;
            let mut code: Vec<TokenStream2> = Vec::new();

            for field in fields {
                let name = field.ident.unwrap();
                let ty = field.ty;
                let attrs = field.attrs;
                let mut is_var = false;

                for attr in attrs {
                    let path = attr.meta.require_path_only();
                    if path.is_err() {
                        continue;
                    }
                    if path.unwrap().is_ident("var") {
                        is_var = true;
                        code.push(quote! {
                            <#ty as crate::protocol::write::MinecraftWriteableVar<W>>::write_var_to(&self.#name, buffer).await?;
                        });
                        continue;
                    }
                }
                if !is_var {
                    code.push(quote! {
                        <#ty as crate::protocol::write::MinecraftWriteable<W>>::write_to(&self.#name, buffer).await?;
                    });
                }
            }

            let output = quote! {
                #[async_trait::async_trait] impl<W: async_std::io::Write + Unpin + Send + Sync> crate::protocol::write::MinecraftWriteable<W> for #struct_name {
                    async fn write_to(&self, buffer: &mut W) -> Result<(), anyhow::Error> {
                        #(#code)*

                        Ok(())
                    }
                }
            };

            output.into()
        },
        Fields::Unnamed(unnamed) => {
            let fields = unnamed.unnamed;
            let mut code: Vec<TokenStream2> = Vec::new();

            for (index, field) in fields.iter().enumerate() {
                let name = Index::from(index);
                let ty = &field.ty;
                let attrs = &field.attrs;
                let mut is_var = false;

                for attr in attrs {
                    let path = attr.meta.require_path_only();
                    if path.is_err() {
                        continue;
                    }
                    if path.unwrap().is_ident("var") {
                        is_var = true;
                        code.push(quote! {
                            <#ty as crate::protocol::write::MinecraftWriteableVar<W>>::write_var_to(&self.#name, buffer).await?;
                        });
                        continue;
                    }
                }
                if !is_var {
                    code.push(quote! {
                        <#ty as crate::protocol::write::MinecraftWriteable<W>>::write_to(&self.#name, buffer).await?;
                    });
                }
            }

            let output = quote! {
                #[async_trait::async_trait] impl<W: async_std::io::Write + Unpin + Send + Sync> crate::protocol::write::MinecraftWriteable<W> for #struct_name {
                    async fn write_to(&self, buffer: &mut W) -> Result<(), anyhow::Error> {
                        #(#code)*

                        Ok(())
                    }
                }
            };

            output.into()
        }
        _ => panic!(
            "Deriving from MinecraftReadable/Writeable is only supported for structs with named fields"
        ),
    };

    return token;
}

fn parse_packet_data(attr: Attribute) -> PacketData {
    let meta = attr
        .meta
        .require_list()
        .expect("packet attribute must be a list");

    let values = meta.tokens.clone().into_iter().collect::<Vec<TokenTree>>();
    if values.len() != 3 {
        panic!("Invalid packet attribute length");
    }

    let mut skip = 0;
    let mut processed: Vec<TokenTree> = vec![];

    for i in 0..values.len() {
        if skip > 0 {
            skip -= 1;
            continue;
        }

        if values[i].to_string() == "PacketDirection" || values[i].to_string() == "NetworkStatus" {
            processed.push(TokenTree::Ident(Ident::new(
                &(values[i].to_string().as_str().to_owned() + values[i + 3].to_string().as_str()),
                Span::call_site(),
            )));
            skip = 3;
            continue;
        }

        processed.push(values[i].clone());
    }

    let mut kv: HashMap<String, String> = HashMap::new();

    let grouped: Vec<Vec<TokenTree>> = processed.chunks(4).map(|chunk| chunk.to_vec()).collect();

    for group in grouped {
        if group[1].to_string() != "=" {
            panic!("Invalid packet attribute");
        }

        let key = group[0].to_string();
        let value = group[2].to_string();

        kv.insert(key, value);
    }

    if !kv.contains_key("id") {
        panic!("packet attribute must contain an id");
    }

    return PacketData {
        id: kv.get("id").unwrap().parse::<i32>().unwrap(),
    };
}
