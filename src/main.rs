mod ast;
mod codegen;
mod token;
use std::fs;
use std::io::Write;

use ast::ASTNode;
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

            let val = parser.parse(); //Result returned here

            let printnodes = |node: &ASTNode| {
                println!("{:?}", node);
            };
            match val {
                Ok(asts) => {
                    let ast_pretty = asts.clone(); //should be small enough to get away with a clone for now. Could use arc, but obnoxious refactor. Possibly if preformance tanks later.
                    asts.traverse(&printnodes);
                    let mut code_gen = codegen::CodeGenerator::new();
                    
                    code_gen.generate_ir(ast_pretty);
                    match code_gen.generate_c_file() {
                        Ok(o) => println!("C source file generated!"),
                        Err(e) => println!(" C source file gen failed with error {e}"),
                    };
                }

                Err(e) => {
                    println!("Error returning AST {}",e);
                }
            }

            
            /* 
            match parser.parse() {
                Ok(ast) => {
                    // Create the CodeGen instance with the context
                     //still 0 statements?
                    //code_gen.generate_ir(ast);
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
            }*/
             
        }
        Err(e) => eprintln!("Failed to read file: {}", e),
    }

    //let context = Context::create();
}
