use std::{path::PathBuf, env};







fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1);
    match path {
       Some(n) => {
           runfile(&n);
       },
       None => {
           repl();
       },
    }
}

fn runfile(file: &String) {
    let source = std::fs::read_to_string(file).expect(format!("{}", file).as_str());
    run(&source);
}

fn repl() {
    println!();
    loop {
        let mut input = String::new();
        print!(">");
        std::io::stdin().read_line(&mut input).unwrap();
        run(&input);
    }
}

fn run(source: &String) {
    println!("*{}*", source);
}
