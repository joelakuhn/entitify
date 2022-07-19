extern crate argparse;

use std::io::Read;
use std::io;

use argparse::{ArgumentParser, StoreTrue};
use named_entity_table::get_named_entities;

mod named_entity_table;

fn main() {
    let mut hex = false;
    let mut dec = false;

    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut hex)
            .add_option(&["-h", "--hex"], StoreTrue,
            "Print all entities as hex");
        ap.refer(&mut dec)
            .add_option(&["-d", "--decimal"], StoreTrue,
            "Print all entities as decimal");
        ap.parse_args_or_exit();
    }

    let named = get_named_entities();
    let mut contents = String::from("");
    if io::stdin().read_to_string(&mut contents).is_ok() {
        for c in contents.chars() {
            let c_u32 = c as u32;
            if c_u32 > 127 {
                if dec {
                    print!("&#{c_u32};");
                }
                else if hex {
                    print!("&#x{c_u32:04x};");
                }
                else {
                    match named.get(&c_u32) {
                        Some(ent) => print!("{ent}"),
                        None => print!("&#x{c_u32:04x};"),
                    };
                }
            }
            else {
                print!("{}", c);
            }
        }
    }
}
