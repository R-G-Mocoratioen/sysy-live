use std::env;
use lalrpop_util::lalrpop_mod;
use std::fs::read_to_string;
use std::io::Result;

lalrpop_mod!(sysy);

mod ast;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    if args.len() != 5 || args[3] != "-o" {
        println!("usage: compiler 阶段 输入文件 -o 输出文件");
        return Ok(());
    }
    let input = read_to_string(args[2].clone())?;
    if args[1] == "-koopa" {
        let ast = sysy::CompUnitParser::new().parse(&input).unwrap();
        println!("{:#?}", ast);
    }
    Ok(())
}