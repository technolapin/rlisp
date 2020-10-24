#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub parser);


pub mod types;
pub mod ast;

pub mod lexer;

pub mod randomizable;


#[cfg(test)]
mod tests {
    #[test]
    fn test_parsing()
    {
        use crate::randomizable::Randomizable;
        use crate::parser;
        use crate::ast::Sexpr;
        use crate::lexer::Lexer;
        let n_tests = 10000;
        for test in 0..n_tests
        {
            let ast = Sexpr::rand(1);
            let pretty_print = format!("{}", ast);
            let lexer = Lexer::new(&pretty_print);
            let parsed = parser::SexParser::new()
                .parse(&pretty_print,lexer)
                .unwrap();

            assert!(
                ast == parsed,
                format!("\n{}\nand\n{}\n are not equal\n(trees:)\n{:?}\n{:?}", ast, parsed, ast, parsed)
            );
        }

        
    }
}
