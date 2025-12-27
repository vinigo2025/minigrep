use std::{env, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        exit();
        return;
    }

    let query = &args[1];
    let file_path = &args[2];

    let ct = read(file_path);

    if let Ok(cont) = ct {
        // println!("{query}");
        // println!("With text:\r\n{cont}");
        comp(query, cont);
        //
    } else if let Err(e) = ct {
        println!("{e}");
        return;
    }
}

fn exit() {
    println!("usage: minigrep PATTERN FILE");
    // std::process::exit(0);
}
fn read(s: &String) -> Result<String, io::Error> {
    //
    let st = fs::read_to_string(s)?;
    Ok(st)
}
fn comp(query: &String, cont: String) {
    if cont.len() == 0 { return; }
    // let _v: Vec<&str> = Vec::new();
    let lines: Vec<&str> = cont.lines().collect();
    for line in lines {
        if line.contains(query) {
            println!("{line}");
        }
    }
}
