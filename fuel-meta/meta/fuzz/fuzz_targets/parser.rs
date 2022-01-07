#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate fuel_pest_meta;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = fuel_pest_meta::parser::parse(fuel_pest_meta::parser::Rule::grammar_rules, s);
    }
});
