use lisp::types::*;
use lisp::ast::*;
use lisp::lexer::*;
use lisp::parser;
fn main() -> Result<(), String>
{
    {
//        let ast = Sexpr::List(vec![Sexpr::Atom(Type::Sym(Sym::new("\u{9cc3e}\u{1054c1}𩒴\u{14063}\u{76a6c}"))), Sexpr::Atom(Type::Char(Char(')')))]);
        //let ast = Sexpr::Atom(Type::Char(Char(')')));
        let ast = Sexpr::List(vec![Sexpr::Atom(Type::Char(Char('䓂'))), Sexpr::Atom(Type::Num(Num::U64(155046837041004162)))]);
        let code = &format!("{}", ast);
        for c in code.chars()
        {
            println!("CHAR: {}    LEN: {}", c, c.len_utf8());
        }
        
        use lisp::randomizable::Randomizable;
        use lisp::parser;
        use lisp::ast::Sexpr;
        use lisp::lexer::Lexer;
        {
            let lexer = Lexer::new(&code);

            println!("        CODE: {}", code);
            let parsed = parser::SexParser::new()
                .parse(&code, lexer)
                .unwrap();
            println!("PRETTY PRINT: {}", parsed);
            println!("AST: {:?}", parsed);
        }
  
    }
    return Ok(());
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
