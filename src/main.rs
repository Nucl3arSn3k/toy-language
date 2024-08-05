mod ast;
mod codegen;
mod token;
use std::fs;
fn main() {
    let program = fs::read_to_string("tests/testprogram.txt");
    match program {
        Ok(contents) => {
            let mut lexer = token::Lexer::new();
            lexer.tokenize(&contents);
            let tokens = lexer.return_tok();

            println!("{:?}", tokens);
            let mut parser = ast::Parser::new(tokens);

            match parser.parse() {
                Ok(ast) => {
                    

                    // Create the CodeGen instance with the context
                    let mut code_gen = codegen::CodeGenerator::new();

                    //code_gen.generate(ast);
                    code_gen.generate_ir(ast);
                    let res = code_gen.generate_c_file();
                }

                Err(e) => {
                    eprintln!("Parsing error {}",e);
                }
            }
            
        }
        Err(e) => eprintln!("Failed to read file: {}", e),
    }

    //let context = Context::create();
}
