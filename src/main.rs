extern crate argparse;

use std::io::Read;
use std::io;
use std::collections::HashMap;

use argparse::{ArgumentParser, StoreTrue};
use named_entity_table::get_named_entities;

mod named_entity_table;

fn encode_string(str : &String, dec : bool, hex : bool) {
    let (by_cp, _) = get_named_entities();

    for c in str.chars() {
        let c_u32 = c as u32;
        if c_u32 > 127 {
            if dec {
                print!("&#{c_u32};");
            }
            else if hex {
                print!("&#x{c_u32:04x};");
            }
            else {
                match by_cp.get(&c_u32) {
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

fn decode_entity(by_name : &HashMap<String, u32>, entity : &str) -> Option<u32> {
    if &entity[0..3] == "&#x" {
        match u32::from_str_radix(&entity[3..entity.len() - 1], 16) {
            Ok(cp) => Some(cp),
            _ => None
        }
    }
    else if &entity[0..2] == "&#" {
        match u32::from_str_radix(&entity[2..entity.len() - 1], 10) {
            Ok(cp) => Some(cp),
            _ => None
        }
    }
    else {
        match by_name.get(entity) {
            Some(cp) => Some(*cp),
            None => None
        }
    }
}

fn decode_string(original : &String) {
    let (_, by_name) = get_named_entities();
    let chars : Vec<_> = original.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '&' => {
                let mut semi_index = 0;
                for o in (i+1)..(i+10) {
                    if o < original.len() {
                        if chars[o] == ';' {
                            semi_index = o;
                            break;
                        }
                    }
                    else {
                        break;
                    }
                }

                if semi_index > 0 {
                    let entity_str = &original[i..semi_index + 1];
                    decode_entity(&by_name, &entity_str);
                    match decode_entity(&by_name, entity_str) {
                        Some(cp) => match char::from_u32(cp) {
                            Some(c) => print!("{}", c),
                            _ => print!("{}", '\u{fffd}'),
                        }
                        _ => print!("{}", '\u{fffd}')
                    }
                    i = semi_index;
                }
            },
            c => {
                print!("{}", c);
            }
        }
        i += 1;
    }
}

fn main() {
    let mut hex = false;
    let mut dec = false;
    let mut decode = false;

    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut hex)
            .add_option(&["-h", "--hex"], StoreTrue,
            "Print all entities as hex");
        ap.refer(&mut dec)
            .add_option(&["-d", "--decimal"], StoreTrue,
            "Print all entities as decimal");
        ap.refer(&mut decode)
            .add_option(&["-D", "--decode"], StoreTrue,
            "decodes entities in the given string");
        ap.parse_args_or_exit();
    }

    let mut contents = String::from("");
    if io::stdin().read_to_string(&mut contents).is_ok() {
        if decode {
            decode_string(&contents);
        }
        else {
            encode_string(&contents, dec, hex);
        }
    }
}
