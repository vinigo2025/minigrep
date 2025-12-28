use std::{env, fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        exit();
        return;
    }
    let query = &args[1];
    let file_path = &args[2];

    match read(file_path) {
        Ok(cont) => {
            comp(query, cont);
            }
        Err(e) => { eprintln!("{e}"); }
    }
}

fn exit() {
    eprintln!("usage: minigrep PATTERN FILE");
    // std::process::exit(0);
}
fn read(s: &String) -> Result<String, io::Error> {
    //
    let st = fs::read_to_string(s)?;
    Ok(st)
}
fn comp(query: &String, cont: String) {
    if cont.len() == 0 { return; }
    //
    for line in cont.lines() {
        if line.contains(query) {
            println!("{line}");
        }
    }
}
