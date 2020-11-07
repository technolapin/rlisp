use lisp::types::*;
use lisp::ast::*;
use lisp::lexer::*;
use lisp::prims::*;
use lisp::parser;
use lisp::lisp_parser;

use lisp::Lisp;

fn eval_str(input: &str, context: &mut Context) -> Result<Value, String>
{
    let lexer = Lexer::new(input);
    match parser::SexParser::new()
        .parse(input,lexer)
    {
        Err(err) => Err(format!("{:?}", err)),
        Ok(expr) => expr.eval(context)
    }
}

    
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
    context.push_lambda(Sym::new("defun"),
                        LambdaValue
                        {
                            params: vec![Sym::new("fun_name"), Sym::new("fun_params"), Sym::new("fun_expr"), Sym::new("fun_out")],
                            expr:
                            Sexpr::List(vec![
                                Sexpr::Atom(Type::Sym(Sym::new("let"))),
                                Sexpr::List(vec![
                                    Sexpr::List(vec![
                                        Sexpr::Atom(Type::Sym(Sym::new("fun_name"))),
                                        Sexpr::List(vec![
                                            Sexpr::Atom(Type::Sym(Sym::new("lambda"))),
                                            Sexpr::Atom(Type::Sym(Sym::new("fun_params"))),
                                            Sexpr::Atom(Type::Sym(Sym::new("fun_expr"))),
                                        ]),                                        
                                        
                                    ]),
                                ]),
                                Sexpr::Atom(Type::Sym(Sym::new("fun_out"))),
                            ])
                                
                        }
    );


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

            ("(+ 1 2)", "3"),
            ("(+ 1/2 2)", "5/2"),
            //("(+ -1/2 2)", "1/2"),
            //("(+ -1/2 -2)", "1/2"),
            ("(+ -1. 2)", "1"),
            ("(+ -1.2 2)", "0.8"),
            ("(+ -6.2 2)", "-4.2"),
            ("(+ -4.0+2.0i 2)", "-2+2i"),
            
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
            println!("RESULT  = {}   ({:?})", out, out);
            println!("EXPECTED= {}", expect);
            if &format!("{}", out) != expect
            {
                return Err(format!("TEST FAILED"));
            }
        }
    }
    if false
    {
        println!("======NEW GRAMMAR==================================");
        // input + expected result
        let inputs = vec![
            ("()", "()"),
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

            ("(+ 1 2)", "3"),
            ("(+ 1/2 2)", "5/2"),
            //("(+ -1/2 2)", "1/2"),
            //("(+ -1/2 -2)", "1/2"),
            ("(+ -1. 2)", "1"),
            ("(+ -1.2 2)", "0.8"),
            ("(+ -6.2 2)", "-4.2"),
            
        ];
        for (input, expect) in inputs.iter()
        {
            let expr = lisp_parser::SexParser::new()
                .parse(input)
                .unwrap();
            let out = expr.eval(&mut context).unwrap();
            println!();
            println!("INPUT | {}", input);
            println!("PRETTY| {}", expr);
            println!("AST|{:?}", expr);
            println!("RESULT  = {}   ({:?})", out, out);
            println!("EXPECTED= {}", expect);
            if &format!("{}", out) != expect
            {
                return Err(format!("TEST FAILED"));
            }
        }
    }
    {
        let v = vec![
            Num::Z(i64::max_value()),
            Num::Q(1, 2),
        ];
        let v = vec![
            Num::Z(64),
            Num::Q(-1, 0),
        ];

        let sum = v.iter().try_fold(Num::Z(0), |sum, n| sum.add(n))?;
        println!("{}", sum);

    }
    {
        let code = r#"
(
lambda foo (a b)
    (+ a b)
)

"#;

        let clean_code = code.chars().filter(|c| !c.is_control()).collect::<String>();

        let visited_token = EscapingLexer::new(&clean_code)
            .filter_map(|el| if let Ok((_, c, _)) = el {Some(c)} else {None})
            .collect::<String>();
        
        println!("{}", code);
        println!("{}", clean_code);
        println!("{}", visited_token);



        
    }
    {
        let mut lisp = Lisp::new();
        let inputs = vec![
            "(rand)",
            "(* (rand) 10)",
            "(floor (* (rand) 10))",
            "(lambda (n) (floor (* (rand) n)))",
            //            "(let ((randint (lambda (n) (floor (* (rand) n)))))(+ (randint 2) 1))",
            "(randint 10)",
            "(dice 2)",
            "(dice (dice 10))",
//            "(defun foo (n) (+ n 1) 4)"
        ];
        for input in inputs.iter()
        {
            let lexer = Lexer::new(input);
            let expr = parser::SexParser::new()
                .parse(input,lexer)
                .unwrap();
            println!();
            println!("INPUT | {}", input);
            println!("PRETTY| {}", expr);
            println!("AST|{:?}", expr);
            let out = expr.eval(&mut context).unwrap();
            println!("RESULT  = {}   ({:?})", out, out);
        }
    }    
    
    {
        let mut lisp = Lisp::new();
        let inputs = vec![
            "(rand)",
            "(* (rand) 10)",
            "(floor (* (rand) 10))",
            "(lambda (n) (floor (* (rand) n)))",
            //            "(let ((randint (lambda (n) (floor (* (rand) n)))))(+ (randint 2) 1))",
            "(randint 10)",
            "(dice 2)",
            "(dice (dice 10))",
//            "(defun foo (n) (+ n 1) 4)"
        ];
        for input in inputs.iter()
        {
            println!();
            println!("INPUT | {}", input);
            let out = lisp.evaluate(input)?;
            println!("RESULT  = {}   ({:?})", out, out);
        }
    }    
    
    Ok(())
}
