use chrono::Utc;
use parser::ast::context::ParseContext;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "fns.pest"]
pub struct FNSParser;

pub mod parser;
use std::fs;

use crate::parser::ast::function::Function;
use crate::parser::ast::type_definition::TypeDefinition;
use crate::parser::ast::Parse;

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Utc::now().to_rfc3339(),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn main() {
    setup_logger().unwrap();
    let unparsed_file = fs::read_to_string("./samples/playground.fn").expect("cannot read file");
    ParseContext::init();

    let file = FNSParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails

    for line in file.into_inner() {
        println!("Rule: {:?}", line.as_rule());
        match line.as_rule() {
            Rule::function => {
                println!("{:#?}", line);
                let f = Function::parse(line).unwrap();
                println!("{:#?}", f);
            }
            Rule::type_definition => {
                println!("{:#?}", line);
                let t = TypeDefinition::parse(line).unwrap();
                println!("{:#?}", t);
            }
            _ => {}
        }
    }
}
