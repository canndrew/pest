#[macro_use]
extern crate quote;
extern crate pest_generator;

use pest_generator::derive_parser;
use std::{fs::File, io::prelude::*, path::Path};

fn main() {
    let fuel_pest = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../meta/src/grammar.pest"
    ));
    let rs = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../meta/src/grammar.rs"
    ));

    let derived = {
        let path = fuel_pest.to_string_lossy();
        let fuel_pest = quote! {
            #[grammar = #path]
            pub struct PestParser;
        };
        derive_parser(fuel_pest, false)
    };

    let mut file = File::create(rs).unwrap();

    writeln!(file, "pub struct PestParser;\n{}", derived,).unwrap();
}
