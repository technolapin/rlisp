use crate::types::*;
use crate::ast::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Prim
{
    Quote,
    Atom,
    Eq,
    Car,
    Cdr,
    Cons,
    Cond,
    
    Lambda,
    Add,  
    Mult,
    Let
}

impl Prim
{
    pub fn execute(&self, params: &[Sexpr], context: &mut Context) -> Result<Value, String>
    {
        use Prim::*;
        match self
        {
            Quote =>
            {
                if params.len() == 1
                {                  
                    Ok(Value::Sexpr(params[0].clone()))
                }
                else
                {
                    Err(format!("Error: quote takes 1 argument ({} given)", params.len()))
                }
            },
            Atom =>
            {
                if params.len() == 1
                {
                    if let Value::Sexpr(Sexpr::Atom(_)) = params[0].eval(context)?
                    {
                        Ok(Value::Sexpr(Sexpr::Atom(Type::Sym(Sym::t()))))
                    }
                    else
                    {
                        Ok(Value::Sexpr(Sexpr::Atom(Type::Nil)))
                    }
                }
                else
                {
                    Err(format!("Error: atom takes 1 argument ({} given)", params.len()))
                }
            },
            Eq =>
            {
                if params.len() == 2
                {
                    let eval1 = params[0].eval(context)?;
                    let eval2 = params[1].eval(context)?;
                    match (eval1, eval2)
                    {
                        (Value::Sexpr(Sexpr::Atom(atom1)),
                         Value::Sexpr(Sexpr::Atom(atom2))) =>
                        {
                            if atom1 == atom2
                            {
                                Ok(Value::Sexpr(Sexpr::Atom(Type::Sym(Sym::t()))))
                            }
                            else
                            {
                                Ok(Value::Sexpr(Sexpr::Atom(Type::Nil)))
                            }
                        },
                        _ =>  Ok(Value::Sexpr(Sexpr::Atom(Type::Nil)))
                    }
                }
                else
                {
                    Err(format!("Error: eq takes 2 argument ({} given)", params.len()))
                }
            },
            Car =>
            {
                if params.len() == 1
                {
                    let val = params[0].eval(context)?;

                    if let Value::Sexpr(Sexpr::List(v)) = val
                    {
                        // len never 0
                        if v.len() == 0
                        {
                            panic!("Found a non-empty list that is actually empty")
                        }
                        else
                        {
                            v[0].eval(context)
                        }
                    }
                    else if val == Value::Sexpr(Sexpr::Atom(Type::Nil))
                    {
                        Ok(Value::Sexpr(Sexpr::Atom(Type::Nil)))
                    }
                    else
                    {
                        Err(format!("Error: car argument is not a list"))
                    }
                    
                }
                else
                {
                    Err(format!("Error: car takes 1 argument ({} given)", params.len()))
                }

            },
            Cdr =>
            { 
                if params.len() == 1
                {
                    let val = params[0].eval(context)?;

                    if let Value::Sexpr(Sexpr::List(v)) = val
                    {
                        // len never 0
                        if v.len() == 0
                        {
                            panic!("Found a non-empty list that is actually empty")
                        }
                        else if v.len() == 1
                        {
                            Ok(Value::Sexpr(Sexpr::Atom(Type::Nil)))
                        }
                        else
                        {
                            Ok(Value::Sexpr(Sexpr::List(v[1..].iter().cloned().collect::<Vec<_>>())))
                        }
                    }
                    else if val == Value::Sexpr(Sexpr::Atom(Type::Nil))
                    {
                        Ok(Value::Sexpr(Sexpr::Atom(Type::Nil)))
                    }
                    else
                    {
                        Err(format!("Error: car argument is not a list"))
                    }
                    
                }
                else
                {
                    Err(format!("Error: car takes 1 argument ({} given)", params.len()))
                }
            },
            Cons =>
            {
                if params.len() == 2
                {
                    let val1 = params[0].eval(context)?;
                    let val2 = params[1].eval(context)?;
                    
                    if let Value::Sexpr(Sexpr::List(v)) = val2
                    {
                        if let Value::Sexpr(e) = val1
                        {
                            let mut v2 = Vec::with_capacity(v.len()+1);
                            v2.push(e);
                            v2.append(&mut v.clone());
                                
                            
                            Ok(
                                Value::Sexpr(Sexpr::List(v2))
                            )
                        }
                        else
                        {
                            Err(format!("Error: cons first argument is not a sexpr (maybe a lambda or primitive)"))
                        }
                    } else if val2 == Value::Sexpr(Sexpr::Atom(Type::Nil))
                    {
                        if let Value::Sexpr(e) = val1
                        {
                            Ok(Value::Sexpr(Sexpr::List(vec![e])))
                        }
                        else
                        {
                            Err(format!("Error: cons first argument is not a sexpr (maybe a lambda or primitive)"))
                        }
                    }
                    else
                    {
                        Err(format!("Error: cons second argument is not a list"))
                    }
                    
                }
                else
                {
                    Err(format!("Error: cons takes 2 argument ({} given)", params.len()))
                }

            },
            Cond =>
            {
                if params.len() == 0
                {
                    Err(format!("Error: cons take at least one argument"))
                }
                else
                {
                    // check if the params are only lists
                    for e in params.iter()
                    {
                        match e
                        {
                            Sexpr::List(_) => continue,
                            Sexpr::Atom(Type::Nil) => continue,
                            _ => return Err(format!("Error: cons arguments must be lists"))

                        }
                    }

                    for e in params.iter()
                    {
                        // we ignore the nils
                        if let Sexpr::List(v) = e
                        {
                            if v.len() == 0
                            {
                                panic!("Found a non-empty list that is actually empty")
                            }
                            else
                            {
                                let cond = v[0].eval(context)?;
                                if Value::Sexpr(Sexpr::Atom(Type::Nil)) != cond
                                {
                                    if v.len() == 1
                                    {
                                        return Ok(cond);
                                    }
                                    else
                                    {
                                        return v[v.len()-1].eval(context);
                                    }
                                }
                            }
                        }
                    }
                    return Ok(Value::Sexpr(Sexpr::Atom(Type::Nil)));
                }
            },
            Lambda =>
            {
                if params.len() != 2
                {
                    Err(format!("Error on lambda construction: 2 parameters required ({} given)",
                                params.len()))
                }
                else
                {
                    match params[0].clone()
                    {
                        Sexpr::Atom(Type::Nil) =>
                        {

                            // e is then a plain list of symbols
                            Ok(Value::Lambda(
                                LambdaValue
                                {
                                    params: vec![],
                                    expr: params[1].clone()
                                }
                            ))

                        },
                        Sexpr::List(lst) =>
                        {
                            let mut syms = Vec::new();
                            for e in lst.iter()
                            {
                                match e
                                {
                                    Sexpr::List(_) =>
                                    {return Err(format!("Error on lambda construction: all parameters must be symbols (found list)"));},
                                    Sexpr::Atom(Type::Sym(s)) => syms.push(s.clone()),
                                    Sexpr::Atom(other_atom) =>
                                    {return Err(format!("Error on lambda construction: all parameters must be symbols (found {:?}", other_atom));}
                                }
                            }

                            // e is then a plain list of symbols
                            Ok(Value::Lambda(
                                LambdaValue
                                {
                                    params: syms,
                                    expr: params[1].clone()
                                }
                            ))
                        },
                        Sexpr::Atom(_) => Err(format!("Error on lambda construction: first argument must be a symbol list, found atom")),

                    }
                }
            },
            Add => // for now only with i64
            {
                let sum = params.iter()
                    .try_fold(0i64, |sum, p|
                              {
                                  let reduced = p.eval(context)?;
                                  if let Value::Sexpr(Sexpr::Atom(Type::Num(Num::Z(n)))) = reduced
                                  {
                                      Ok(n + sum)
                                  }
                                  else
                                  {
                                      Err(format!("TYPE ERROR BECAUSE OF SHITTY IMPLEMENTATION (found {:?}, expected a i64)", p))
                                  }
                              })?;
                Ok(Value::Sexpr(Sexpr::Atom(Type::Num(Num::Z(sum)))))
            },
            Mult => // for now only with i64
            {
                let mul = params.iter().try_fold(1i64, |mul, p|
                                {
                                    let reduced = p.eval(context)?;
                                    if let Value::Sexpr(Sexpr::Atom(Type::Num(Num::Z(n)))) = reduced
                                    {
                                        Ok(n * mul)
                                    }
                                    else
                                    {
                                        Err(String::from("TYPE ERROR BECAUSE OF SHITTY IMPLEMENTATION"))
                                    }
                                })?;
                Ok(Value::Sexpr(Sexpr::Atom(Type::Num(Num::Z(mul)))))
            },
            Let =>
            {
                if params.len() != 2
                {
                    Err(format!("Error on lambda construction: 2 parameters required ({} given)",
                                params.len()))
                }
                else if let Sexpr::List(lst) = params[0].clone()
                {
                    let mut vars = Vec::new();
                    for e in lst.iter()
                    {
                        if let Sexpr::List(lst) = e
                        {
                            if lst.len() == 2
                            {
                                if let Sexpr::Atom(Type::Sym(sym)) = &lst[0]
                                {
                                    let val = lst[1].eval(context)?;
                                    vars.push((sym, val));
                                }
                                else
                                {
                                    return Err(format!("Error in let: require a couple (symbol sexp) (found {})", e))

                                }
                            }
                            else
                            {
                                return Err(format!("Error in let: require a couple (symbol sexp) (found {})", e))
                            }
                            
                        }
                        else
                        {
                            return Err(format!("Error in let: first parameter has to be a non-empty list (found {})", e))
                        }
                    }

                    for (sym, val) in vars.iter()
                    {
                        context.push((*sym).clone(), val.clone());
                    }
                    let ret = params[1].eval(context)?;
                    for (sym, _) in vars.iter()
                    {
                        context.pop(sym);
                    }
                    
                    
                    Ok(ret)
                }
                else
                {
                    Err(format!("Error on lambda construction: first parameter must be a list ({} given)",
                                params[0]))
                    
                }
            },
        }
    }
}


