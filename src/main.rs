use koopa::back::KoopaGenerator;
use lalrpop_util::lalrpop_mod;
use std::env;
use std::fs::read_to_string;
use std::io::Result;

lalrpop_mod!(sysy);

mod ast;
mod tokoopa;
mod whilecontext;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    if args.len() != 5 || args[3] != "-o" {
        println!("usage: compiler 阶段 输入文件 -o 输出文件");
        return Ok(());
    }
    let input = read_to_string(args[2].clone())?;
    let ast = sysy::CompUnitParser::new().parse(&input).unwrap();
    println!("{:#?}", ast);
    let program = ast.gen_ir();
    if args[1] == "-koopa" {
        let mut gen = KoopaGenerator::new(Vec::new());
        gen.generate_on(&program).unwrap();
        let text_from_ir = std::str::from_utf8(&gen.writer()).unwrap().to_string();
        std::fs::write(args[4].clone(), text_from_ir)?;
    }
    Ok(())
}
