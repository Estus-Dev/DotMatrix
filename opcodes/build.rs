use std::{fs::File, io::Write};

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};

fn main() -> Result<()> {
    // Regenerate if `opcodes.json` changes
    println!("cargo:rerun-if-changed=opcodes.json");

    let opcode_json: Vec<Opcode> = serde_json::from_slice(include_bytes!("./opcodes.json"))?;

    assert_eq!(256, opcode_json.len(), "Must have exactly 256 opcodes");

    let syn_file = build_opcodes_file(&opcode_json)?;
    let mut file = File::create("./src/opcodes.rs")?;

    write!(file, "{}", prettyplease::unparse(&syn_file))?;

    Ok(())
}

fn build_opcodes_file(opcodes: &[Opcode]) -> Result<syn::File> {
    Ok(syn::File {
        shebang: None,
        attrs: vec![],
        items: vec![
            syn::parse2(build_enum(opcodes))?,
            syn::parse2(build_from(opcodes))?,
            syn::parse2(build_display(opcodes))?,
        ],
    })
}

fn build_enum(opcodes: &[Opcode]) -> TokenStream {
    let opcodes = opcodes.iter().map(|op| {
        let id = format_ident!("{}", op.id);
        let opcode = op.opcode;

        quote! { #id = #opcode }
    });

    quote! {
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[repr(u8)]
        pub enum Opcode {
            #(#opcodes),*
        }
    }
}

fn build_from(opcodes: &[Opcode]) -> TokenStream {
    let opcodes = opcodes.iter().map(|op| {
        let id = format_ident!("{}", op.id);
        let opcode = op.opcode;

        quote! { #opcode => Self::#id }
    });

    quote! {
        impl From<u8> for Opcode {
            fn from(opcode: u8) -> Self {
                match opcode {
                    #(#opcodes),*
                }
            }
        }
    }
}

fn build_display(opcodes: &[Opcode]) -> TokenStream {
    let opcodes = opcodes.iter().map(|op| {
        let id = format_ident!("{}", op.id);
        println!("{id}");
        let mnemonic = op.mnemonic.first().unwrap();

        quote! { Self::#id => #mnemonic }
    });

    quote! {
        impl std::fmt::Display for Opcode {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", match self {
                    #(#opcodes),*
                })
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Opcode {
    opcode: u8,
    id: String,
    mnemonic: Vec<String>,
    length: usize,
}
