use crate::types::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Prim
{
    Lambda,
    Add,
    Mult,
    Let
}

impl Prim
{
    fn execute(&self, params: &[Sexpr], context: &mut Context) -> Result<Value, String>
    {
        use Prim::*;
        match self
        {
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
                        Sexpr::Atom(_) => Err(format!("Error on lambda construction: first argument must be a symbol list, found atom")),
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
                        }
                    }
                }
            },
            Add => // for now only with u64
            {
                let sum = params.iter()
                    .try_fold(0u64, |sum, p|
                              {
                                  let reduced = p.eval(context)?;
                                  if let Value::Sexpr(Sexpr::Atom(Type::Num(Num::U64(n)))) = reduced
                                  {
                                      Ok(n + sum)
                                  }
                                  else
                                  {
                                      Err(format!("TYPE ERROR BECAUSE OF SHITTY IMPLEMENTATION (found {:?}, expected a u64)", p))
                                  }
                              })?;
                Ok(Value::Sexpr(Sexpr::Atom(Type::Num(Num::U64(sum)))))
            },
            Mult => // for now only with u64
            {
                let mul = params.iter().try_fold(1u64, |mul, p|
                                {
                                    let reduced = p.eval(context)?;
                                    if let Value::Sexpr(Sexpr::Atom(Type::Num(Num::U64(n)))) = reduced
                                    {
                                        Ok(n * mul)
                                    }
                                    else
                                    {
                                        Err(String::from("TYPE ERROR BECAUSE OF SHITTY IMPLEMENTATION"))
                                    }
                                })?;
                Ok(Value::Sexpr(Sexpr::Atom(Type::Num(Num::U64(mul)))))
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
                            return Err(format!("Error in let: first parameter has to be a list (found {})", e))
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


#[derive(Debug, Clone, PartialEq)]
pub struct LambdaValue
{
    params: Vec<Sym>,
    expr: Sexpr,
}


impl LambdaValue
{

    fn execute(&self, given_params: &[Sexpr], context: &mut Context) -> Result<Value, String>
    {
        if given_params.len() != self.params.len()
        {
            Err(format!("Error on lambda call: {} parameters required ({} given)",
                        self.params.len(),
                        given_params.len()))
        }
        else
        {
            for (sym, expr) in self.params.iter().zip(given_params.iter())
            {
                context.push_sexpr(sym.clone(), expr.clone())
            }

            let ret = self.expr.eval(context);

            for sym in self.params.iter()
            {
                context.pop(sym)
            }
            
            ret
        }
    }
}


#[derive(Debug, Clone)]
pub enum Value
{
    Sexpr(Sexpr),
    Prim(Prim),
    Lambda(LambdaValue)
}



pub struct Context
{
    syms_map: HashMap<Sym, Vec<Value>>,
}

impl Context
{
    pub fn new() -> Self
    {
        Self{syms_map: HashMap::new()}
    }

    fn get(&self, name: &Sym) -> Result<Value, String>
    {
        match self.syms_map.get(name)
        {
            None => Err(format!("Symbol {:?} has no known value!", name)),
            Some(e) => Ok(e.last().unwrap().clone()) // TODO: remove unwrap (maybe)
        }
    }

    fn push(&mut self, name: Sym, value: Value)
    {
        match self.syms_map.get_mut(&name)
        {
            None => {self.syms_map.insert(name, vec![value]);},
            Some(v) => {v.push(value);}
        }
    }
    
    fn push_sexpr(&mut self, name: Sym, exp: Sexpr)
    {
        self.push(name, Value::Sexpr(exp))
    }

    pub fn push_prim(&mut self, name: Sym, prim: Prim)
    {
        self.push(name, Value::Prim(prim))
    }

    fn push_lambda(&mut self, name: Sym, lmbd: LambdaValue)
    {
        self.push(name, Value::Lambda(lmbd))
    }

    // assuming we pushed correctly
    fn pop(&mut self, name: &Sym)
    {
        if let Some(v) = self.syms_map.get_mut(name)
        {
            if v.len() == 1
            {
                self.syms_map.remove(name);
            }
            else
            {
                v.pop();
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sexpr
{
    Atom(Type),
    List(Vec<Sexpr>)
}

impl Sexpr
{
    pub fn eval(&self, context: &mut Context) -> Result<Value, String>
    {
        match self
        {
            Self::Atom(atom) => match atom
            {
                Type::Sym(sym) => context.get(sym),
                _ => Ok(Value::Sexpr(self.clone()))
            },
            Self::List(lst) =>
            {
                if lst.len() == 0
                {
                    Ok(Value::Sexpr(Self::Atom(Type::Sym(Sym::nil()))))
                }
                else
                {
                    let params = lst.get(1..lst.len()).unwrap_or(&[]);
                    let op = lst[0].eval(context)?;
                    
                    match op
                    {
                        Value::Sexpr(expr) => Err(format!("{:?} is not a function", expr)),
                        Value::Prim(prim) => prim.execute(params, context),
                        Value::Lambda(lamb) => lamb.execute(params, context)
                    }
                }
            }
        }
    }
}

use std::fmt;
impl fmt::Display for Sexpr
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            Self::Atom(atom) => write!(f, "{}", atom),
            Self::List(lst) =>
            {
                write!(f, "(")?;
                let mut it = lst.iter();
                if let Some(e) = it.next()
                {
                    write!(f, "{}", e)?;
                }
                for e in it
                {
                    write!(f, " {}", e)?;
                }
                write!(f, ")")
            }
        }
    }
}
impl fmt::Display for Value
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            Self::Sexpr(e) => write!(f, "{}", e),
            Self::Lambda(_lam) => write!(f, "[closure]"),
            Self::Prim(_prim) => write!(f, "<PRIM>"),
        }
    }
}

