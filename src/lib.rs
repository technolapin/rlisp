#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub parser);
lalrpop_mod!(pub lisp_parser);


pub mod types;
pub mod ast;
pub mod prims;

pub mod lexer;

pub mod randomizable;



use crate::types::*;
use crate::ast::*;
use crate::prims::*;

pub struct Lisp
{
    context: Context
}

impl Lisp
{
    pub fn new() -> Self
    {
        let mut context = Context::new();

        context.push_prim(Sym::new("quote"), Prim::Quote);
        context.push_prim(Sym::new("atom"), Prim::Atom);
        context.push_prim(Sym::new("eq"), Prim::Eq);
        context.push_prim(Sym::new("car"), Prim::Car);
        context.push_prim(Sym::new("cdr"), Prim::Cdr);
        context.push_prim(Sym::new("cons"), Prim::Cons);
        context.push_prim(Sym::new("cond"), Prim::Cond);
        context.push_prim(Sym::new("rand"), Prim::Rand);
        context.push_prim(Sym::new("floor"), Prim::Floor);

        context.push_prim(Sym::new("+"), Prim::Add);
        context.push_prim(Sym::new("*"), Prim::Mult);
        context.push_prim(Sym::new("lambda"), Prim::Lambda);
        context.push_prim(Sym::new("let"), Prim::Let);
        context.push_sexpr(Sym::new("nil"), Sexpr::Atom(Type::Nil));
        context.push_lambda(Sym::new("randint"),
                            LambdaValue
                            {
                                params: vec![Sym::new("n")],
                                expr: Sexpr::List(vec![
                                    Sexpr::Atom(Type::Sym(Sym::new("floor"))),
                                    Sexpr::List(vec![
                                        Sexpr::Atom(Type::Sym(Sym::new("*"))), 
                                        Sexpr::List(vec![
                                            Sexpr::Atom(Type::Sym(Sym::new("rand"))),
                                            
                                        ]),
                                        Sexpr::Atom(Type::Sym(Sym::new("n"))),
                                        
                                    ])
                                ])
                            }
        );
        context.push_lambda(Sym::new("dice"),
                            LambdaValue
                            {
                                params: vec![Sym::new("n")],
                                expr:
                                Sexpr::List(vec![
                                    Sexpr::Atom(Type::Sym(Sym::new("+"))),
                                    Sexpr::List(vec![
                                        Sexpr::Atom(Type::Sym(Sym::new("randint"))), 
                                        Sexpr::Atom(Type::Sym(Sym::new("n"))), 
                                    ]),
                                    Sexpr::Atom(Type::Num(Num::Z(1))), 
                                ])
                                    
                            }
        );
        
        Self{context}
    }


    pub fn evaluate(&mut self, code: &str) -> Result<Value, String>
    {
        let lexer = lexer::Lexer::new(code);
        match parser::SexParser::new().parse(code, lexer)
        {
            Err(err) => Err(format!("{:?}", err)),
            Ok(expr) => expr.eval(&mut self.context)
        }
    }
}














#[cfg(test)]
mod tests {
    #[test]
    fn test_parsing()
    {
        use crate::randomizable::Randomizable;
        use crate::parser;
        use crate::ast::Sexpr;
        use crate::lexer::Lexer;
        let n_tests = 1000000;
        for test in 0..n_tests
        {
            let ast = Sexpr::rand(3);
            let pretty_print = format!("{}", ast);
            let lexer = Lexer::new(&pretty_print);
            match parser::SexParser::new()
                .parse(&pretty_print,lexer)
            {
                Ok(parsed) =>
                {
                    assert!(
                        ast == parsed,
                        format!("\n{}\nand\n{}\n are not equal\n(trees:)\n{:?}\n{:?}", ast, parsed, ast, parsed)
                    );
                },
                Err(e) =>
                {
                    panic!(
                        format!("ERROR\n{}\n{:?}\n{:?}", ast, ast, e)
                    );
                }
            }
        }

        
    } 
}
