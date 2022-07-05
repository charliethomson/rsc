use std::{
    fs::File,
    io::{Read, Write},
    time::SystemTime,
};

mod lexer;
mod parser;

fn main() {
    let mut file_contents = String::new();
    let mut file = std::fs::File::open("./examples/simple.rsc").unwrap();
    file.read_to_string(&mut file_contents).unwrap();
    let mut out = File::create(format!(
        "./logs/{}.log",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    ))
    .unwrap();
    let lex_result = lexer::Lexer::new(&file_contents).collect::<Vec<_>>();
    out.write_all(format!("{:#?}", lex_result).as_bytes())
        .unwrap();
}
