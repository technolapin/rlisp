use lisp::types::*;
use lisp::ast::*;
use lisp::lexer::*;
use lisp::parser;
fn main() -> Result<(), String>
{
    {
        let code = r#"?\𤄀"#;
        for c in code.chars()
        {
            println!("CHAR: {}    LEN: {}", c, c.len_utf8());
        }
        
        use lisp::randomizable::Randomizable;
        use lisp::parser;
        use lisp::ast::Sexpr;
        use lisp::lexer::Lexer;
        {
            let ast = Sexpr::Atom(Type::Sym(Sym::new(code)));
            let pretty_print = format!("{}", ast);
            println!("pretty: {}", pretty_print);
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
    /*
    
    {

        let s = "äbĉéé";

        println!("{:?}", s.chars().collect::<Vec<_>>());
        println!("{:?}", s.chars().collect::<Vec<_>>());
        return Ok(());
    }
    */
    

    let mut context = Context::new();

    context.push_prim(Sym::new("+"), Prim::Add);
    context.push_prim(Sym::new("*"), Prim::Mult);
    context.push_prim(Sym::new("lambda"), Prim::Lambda);
    context.push_prim(Sym::new("let"), Prim::Let);
    {
        let sexpr = Sexpr::List(vec![
            Sexpr::Atom(Type::Sym(Sym::new("+"))),
            Sexpr::Atom(Type::Num(Num::U64(1))),
            Sexpr::List(vec![
                Sexpr::Atom(Type::Sym(Sym::new("*"))),
                Sexpr::Atom(Type::Num(Num::U64(4))),
                Sexpr::Atom(Type::Num(Num::U64(5))),
            ]),
            Sexpr::Atom(Type::Num(Num::U64(2))),
        ]);


        println!("{}", sexpr);
        let res = sexpr.eval(&mut context)?;
        println!("{}", res);
    }

    {
        let sexpr = Sexpr::List(vec![
            Sexpr::List(vec![
                Sexpr::Atom(Type::Sym(Sym::new("lambda"))),
                Sexpr::List(vec![
                    Sexpr::Atom(Type::Sym(Sym::new("a"))),
                    Sexpr::Atom(Type::Sym(Sym::new("b"))),
                ]),
                Sexpr::List(vec![
                    Sexpr::Atom(Type::Sym(Sym::new("*"))),
                    Sexpr::Atom(Type::Sym(Sym::new("a"))),
                    Sexpr::Atom(Type::Sym(Sym::new("b"))),
                ]),

            ]),
            Sexpr::Atom(Type::Num(Num::U64(4))),
            Sexpr::Atom(Type::Num(Num::U64(5))),
        ]);

        println!("{}", sexpr);
        let res = sexpr.eval(&mut context)?;
        println!("{}", res);
    }
    {
        let sexpr = Sexpr::List(vec![
            Sexpr::List(vec![
                Sexpr::Atom(Type::Sym(Sym::new("lambda"))),
                Sexpr::List(vec![
                ]),
                Sexpr::Atom(Type::Char(Char('a'))),
                
            ])
        ]);

        println!("{}", sexpr);
        let res = sexpr.eval(&mut context)?;
        println!("{}", res);
    }

    {
        let sexpr = Sexpr::List(vec![
            Sexpr::List(vec![
                Sexpr::Atom(Type::Sym(Sym::new("lambda"))),
                Sexpr::List(vec![
                    Sexpr::Atom(Type::Sym(Sym::new("a"))),
                    Sexpr::Atom(Type::Sym(Sym::new("b"))),
                ]),
                Sexpr::List(vec![
                    Sexpr::Atom(Type::Sym(Sym::new("*"))),
                    Sexpr::Atom(Type::Sym(Sym::new("a"))),
                    Sexpr::Atom(Type::Sym(Sym::new("b"))),
                ]),

            ]),
            Sexpr::Atom(Type::Num(Num::U64(4))),
            Sexpr::Atom(Type::Num(Num::U64(5))),
        ]);
        println!("{}", sexpr);
        let res = sexpr.eval(&mut context)?;
        println!("{}", res);
    }

    {
        let sexpr = Sexpr::List(vec![
            Sexpr::Atom(Type::Sym(Sym::new("let"))),
            Sexpr::List(vec![
                Sexpr::List(vec![
                    Sexpr::Atom(Type::Sym(Sym::new("a"))),
                    Sexpr::Atom(Type::Num(Num::U64(4))),
                ]),
                Sexpr::List(vec![
                    Sexpr::Atom(Type::Sym(Sym::new("b"))),
                    Sexpr::Atom(Type::Num(Num::U64(5))),
                ]),
            ]),
            Sexpr::List(vec![
                Sexpr::Atom(Type::Sym(Sym::new("+"))),
                Sexpr::Atom(Type::Sym(Sym::new("a"))),
                Sexpr::Atom(Type::Sym(Sym::new("b"))),
            ]),

        ]);
        println!("{}", sexpr);
        let res = sexpr.eval(&mut context)?;
        println!("{}", res);
    } 

    
    {
        let code = "\n  (+ 1 2 3)";

        let lexer = Lexer::new(code);

        for span in lexer
        {
            if let Ok((start, tok, end)) = span
            {
                println!("({}, {}) {:?}: {}", start, end, tok, code.get(start..end).unwrap());
            }
            else
            {
                println!("TOK ERROR:  {:?}", span);
            }
        }
        
    }


    
    {
        
        let input = "(+ 1 2 (- 4 1) 4)";
        let lexer = Lexer::new(input);
        let expr = parser::SexParser::new()
            .parse(input,lexer)
            .unwrap();
        println!("{}", input);
        println!("{:?}", expr);
        println!("{}", expr);
    }

    
    Ok(())
}
