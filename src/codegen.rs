use crate::ast;


use std::collections::HashMap;
use std::env::var;
use std::hash::Hash;
use crate::token::Type;
use std::fs::File;
use std::io::Write;
pub struct CodeGenerator{
    variables: HashMap<String, VariableInfo>,
    c_code: String,
    //tree_properties:
}

struct VariableInfo{
    linenum: u32



}

impl CodeGenerator {
    pub fn new() -> Self {
        

        CodeGenerator {
            variables: HashMap::new(),
            c_code: String::new(),
        }
    }


    


    pub fn generate(&self, mut ast: ast::ASTNode) {
        Self::process_ast(ast)
    }

    fn process_ast(node: ast::ASTNode) {
        match node {
            ast::ASTNode::Program(statements) => {
                println!("Program with {} statements", statements.len());
                for statement in statements {
                    Self::process_ast(statement);
                }
            }
            ast::ASTNode::VariableDeclaration {
                identifier,
                initializer,
                line,
                var_type,
            } => {
                println!(
                    "Variable Declaration: {} on line {} of type {}",
                    identifier,
                    line,
                    var_type.as_ref().map_or("Unknown".to_string(), |t| t.to_string())
                );
                if let Some(init) = initializer {
                    println!("  Initialized with:");
                    Self::process_ast(*init);
                }
            }
            ast::ASTNode::DisplayStatement(identifier, line) => {
                println!("Display Statement: {} on line {}", identifier, line);
            }
            ast::ASTNode::DisplayIntStatement(identifier, line) => {
                println!("Display int Statement: {} on line {}", identifier, line);
            }
            ast::ASTNode::DisplayStringStatement(identifier, line) => {
                println!("Display string Statement: {} on line {}", identifier, line);
            }
            ast::ASTNode::ExpressionStatement {
                expression,
                identifier,
                line,
            } => {
                println!(
                    "Expression Statement: {} = ... on line {}",
                    identifier, line
                );
                Self::process_ast(*expression);
            }
            ast::ASTNode::BinaryOp {
                operator,
                left,
                right,
                line,
            } => {
                println!("Binary Operation: {} on line {}", operator, line);
                println!("  Left:");
                Self::process_ast(*left);
                println!("  Right:");
                Self::process_ast(*right);
            }
            ast::ASTNode::Identifier(name, line) => {
                println!("Identifier: {} on line {}", name, line);
            }
            ast::ASTNode::Number(value, line) => {
                println!("Number: {} on line {}", value, line);
            }
            ast::ASTNode::StringLiteral(value, line) => {
                println!("String Literal: \"{}\" on line {}", value, line);
            }

            ast::ASTNode::IfStatement { condition, then_block, else_if_blocks, else_block, line } =>{


                match else_block{

                    None =>{
                        println!("If Statement:{:?} Then block{:?},elseif block{:?}  line{:?}",condition,then_block,else_if_blocks,line);
                    }

                    Some(_) => {

                        println!("If Statement:{:?} Then block{:?},elseif block{:?} else block{:?} line{:?}",condition,then_block,else_if_blocks,else_block,line);
                    }
                }

                

            }
        }
    }

    pub fn generate_ir(&mut self, ast: ast::ASTNode) -> String {
        self.c_code.clear();
        self.c_code.push_str("#include <stdio.h>\n\n");
        self.c_code.push_str("int main() {\n");
        self.gen_ir_ast(ast);
        self.c_code.push_str("    return 0;\n");
        self.c_code.push_str("}\n");
        self.c_code.clone()
    }

    fn gen_ir_ast(&mut self, node: ast::ASTNode) {
        match node {
            ast::ASTNode::Program(statements) => {
                for statement in statements {
                    self.gen_ir_ast(statement);
                }
            }
            ast::ASTNode::VariableDeclaration {
                identifier,
                initializer,
                line,
                var_type,
            } => {
                let mut fact = true;
                let type_str = var_type.as_ref().map_or("Unknown".to_string(), |t| t.to_string());
                match type_str.as_str() {
                    "Int" => {
                        self.c_code.push_str(&format!("int {};\n", identifier));
                        fact = false;
                    }
                    "Str" => {
                        self.c_code.push_str(&format!("char {}[]=", identifier)); // Assuming a max length of 100
                    }
                    _ => {
                        self.c_code.push_str(&format!("/* Unknown type */ void* {};\n", identifier));
                    }
                }
                self.variables.insert(identifier.clone(), VariableInfo { linenum: line });
                if let Some(init) = initializer {
                    if(!fact){
                        self.c_code.push_str(&format!("{} = ", identifier));
                        self.gen_ir_ast(*init);
                        self.c_code.push_str(";\n");
                    }
                    else{
                        //self.c_code.push_str(&format!("{}=", identifier));
                        self.gen_ir_ast(*init);
                        self.c_code.push_str(";\n");

                    }
                    
                }
            }
            ast::ASTNode::DisplayStatement(identifier, _line) => {
                self.c_code.push_str(&format!("printf(\"%d\\n\", {});\n", identifier));
            }
            ast::ASTNode::DisplayIntStatement(identifier, _line) => {
                self.c_code.push_str(&format!("printf(\"%d\\n\", {});\n", identifier));
            }
            ast::ASTNode::DisplayStringStatement(identifier, _line) => {
                self.c_code.push_str(&format!("printf(\"%s\\n\", {});\n", identifier));
            }
            ast::ASTNode::ExpressionStatement {
                expression,
                identifier,
                line,
            } => {
                self.c_code.push_str(&format!("{} =",identifier));
                self.gen_ir_ast(*expression);
                self.c_code.push_str(";\n");
            }
            ast::ASTNode::BinaryOp {
                operator,
                left,
                right,
                line,
            } => {
                self.c_code.push_str("(");
                self.gen_ir_ast(*left);
                self.c_code.push_str(&format!(" {} ", operator));
                self.gen_ir_ast(*right);
                self.c_code.push_str(")");
            }
            ast::ASTNode::Identifier(name, _line) => {
                self.c_code.push_str(&name);
            }
            ast::ASTNode::Number(value, _line) => {
                self.c_code.push_str(&value.to_string());
            }
            ast::ASTNode::StringLiteral(value, _line) => {
                self.c_code.push_str(&format!("\"{}\"", value));
            }
            ast::ASTNode::IfStatement { condition, then_block, else_if_blocks, else_block, line } =>{

                /* 
                self.c_code.push_str(&format!("if ({:?}) {{\n{:?}}}\n", condition, then_block));
                if else_if_blocks.is_empty(){


                }
                else{


                    self.c_code.push_str(string);
                }

                */

                
            }
        }


    }
    
    pub fn generate_c_file(&self) -> std::io::Result<()> {
        let mut c_file = File::create("output/code.c")?;
        write!(c_file, "{}", self.c_code)?;
        Ok(())
    }

    
}
