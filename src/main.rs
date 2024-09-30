mod ast;
mod codegen;
mod token;
use std::fs;
use std::io::Write;
fn main() {
    let program = fs::read_to_string("tests/testprogram.sco");
    match program {
        Ok(contents) => {
            let mut lexer = token::Lexer::new();
            lexer.tokenize(&contents);
            let tokens = lexer.return_tok();
            match fs::File::create("tokendump.txt") {
                Ok(mut file) => {
                    if let Err(e) = writeln!(file, "{:?}", tokens) {
                        eprintln!("Failed to write to file: {}", e);
                    }
                }
                Err(e) => eprintln!("Failed to create file: {}", e),
            }
            //println!("{:?}", tokens);
            let mut parser = ast::Parser::new(tokens);

            parser.parse();
            /*
            match parser.parse() {
                Ok(ast) => {
                    // Create the CodeGen instance with the context
                    let mut code_gen = codegen::CodeGenerator::new();

                    //code_gen.generate(ast);
                    code_gen.generate_ir(ast);
                    let res = code_gen.generate_c_file();

                    match res {
                        Ok(a) => {
                            println!("Codegen sucessfull {:?}", a);
                        }

                        Err(e) => {
                            eprintln!("Generation error {}", e);
                        }
                    }
                }

                Err(e) => {
                    eprintln!("Parsing error {}", e);
                }
            }
             */
        }
        Err(e) => eprintln!("Failed to read file: {}", e),
    }

    //let context = Context::create();
}
