use lisp::types::*;
use lisp::ast::*;
use lisp::lexer::*;
use lisp::prims::*;
use lisp::parser;
fn main() -> Result<(), String>
{
    {
    let mut s = String::new();
        s.push('\u{1b}');
        s.push('𣽊');
        s.push('\u{a6f55}');
        s.push('\u{7ad03}');
        println!("{}", s);
//        let ast = Sexpr::List(vec![Sexpr::Atom(Type::Sym(Sym::new("\u{9cc3e}\u{1054c1}𩒴\u{14063}\u{76a6c}"))), Sexpr::Atom(Type::Char(Char(')')))]);
        //let ast = Sexpr::Atom(Type::Char(Char(')')));
    let ast = Sexpr::List(vec![Sexpr::Atom(Type::Char(Char('䓂'))), Sexpr::Atom(Type::Num(Num::Z(155046837041004162)))]);
    let code = &s;
    
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
 

    let mut context = Context::new();

    context.push_prim(Sym::new("quote"), Prim::Quote);
    context.push_prim(Sym::new("atom"), Prim::Atom);
    context.push_prim(Sym::new("eq"), Prim::Eq);
    context.push_prim(Sym::new("car"), Prim::Car);
    context.push_prim(Sym::new("cdr"), Prim::Cdr);
    context.push_prim(Sym::new("cons"), Prim::Cons);
    context.push_prim(Sym::new("cond"), Prim::Cond);

    context.push_prim(Sym::new("+"), Prim::Add);
    context.push_prim(Sym::new("*"), Prim::Mult);
    context.push_prim(Sym::new("lambda"), Prim::Lambda);
    context.push_prim(Sym::new("let"), Prim::Let);
    context.push_sexpr(Sym::new("nil"), Sexpr::Atom(Type::Nil));
    {
        let sexpr = Sexpr::List(vec![
            Sexpr::Atom(Type::Sym(Sym::new("+"))),
            Sexpr::Atom(Type::Num(Num::Z(1))),
            Sexpr::List(vec![
                Sexpr::Atom(Type::Sym(Sym::new("*"))),
                Sexpr::Atom(Type::Num(Num::Z(4))),
                Sexpr::Atom(Type::Num(Num::Z(5))),
            ]),
            Sexpr::Atom(Type::Num(Num::Z(2))),
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
            Sexpr::Atom(Type::Num(Num::Z(4))),
            Sexpr::Atom(Type::Num(Num::Z(5))),
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
            Sexpr::Atom(Type::Num(Num::Z(4))),
            Sexpr::Atom(Type::Num(Num::Z(5))),
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
                    Sexpr::Atom(Type::Num(Num::Z(4))),
                ]),
                Sexpr::List(vec![
                    Sexpr::Atom(Type::Sym(Sym::new("b"))),
                    Sexpr::Atom(Type::Num(Num::Z(5))),
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

    {
        println!("========================================");
        // input + expected result
        let inputs = vec![
            ("(quote (+ 1 1))", "(+ 1 1)"),
            ("(quote (quote 1))", "(quote 1)"),

            ("(atom (quote a))", "t"),
            ("(atom (quote (1 2 3)))", "()"),
            ("(atom (quote ()))", "t"),

            ("(eq 2 (+ 1 1))", "t"),
            ("(eq () nil)", "t"),
            ("(eq () ())", "t"),
            ("(eq () 1)", "()"),

            ("(car ())", "()"),
            ("(car (quote ()))", "()"),
            ("(car (quote (1 2)))", "1"),
            ("(car (quote (() 2)))", "()"),

            ("(cdr ())", "()"),
            ("(cdr (quote ()))", "()"),
            ("(cdr (quote (1)))", "()"),
            ("(cdr (quote (1)))", "()"),
            ("(cdr (quote (1 2)))", "(2)"),
            ("(cdr (quote (() 2)))", "(2)"),
            ("(cdr (quote (1 ())))", "(())"),

            ("(cons () (quote ()))", "(())"),
            ("(cons 1 (quote ()))", "(1)"),
            ("(cons 1 (quote (2 3 4)))", "(1 2 3 4)"),

            ("(cond ())", "()"),
            ("(cond () () () ())", "()"),
            ("(cond () () (6) ())", "6"),
            ("(cond () () () (6))", "6"),
            ("(cond (6) () () ())", "6"),
            ("(cond (1 2))", "2"),
            ("(cond ((eq (quote a) (quote b)) (quote first))
((atom (quote a)) (quote second)))", "second"),

            
        ];
        for (input, expect) in inputs.iter()
        {
            let lexer = Lexer::new(input);
            let expr = parser::SexParser::new()
                .parse(input,lexer)
                .unwrap();
            let out = expr.eval(&mut context).unwrap();
            println!();
            println!("INPUT | {}", input);
            println!("PRETTY| {}", expr);
            println!("AST|{:?}", expr);
            println!("RESULT  = {}", out);
            println!("EXPECTED= {}", expect);
            if &format!("{}", out) != expect
            {
                return Err(format!("TEST FAILED"));
            }
        }
    }

    Ok(())
}
