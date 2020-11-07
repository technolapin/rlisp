use crate::types::*;
use crate::prims::Prim;
use std::collections::HashMap;


#[derive(Debug, Clone, PartialEq)]
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

    pub fn push(&mut self, name: Sym, value: Value)
    {
        match self.syms_map.get_mut(&name)
        {
            None => {self.syms_map.insert(name, vec![value]);},
            Some(v) => {v.push(value);}
        }
    }
    
    pub fn push_sexpr(&mut self, name: Sym, exp: Sexpr)
    {
        self.push(name, Value::Sexpr(exp))
    }

    pub fn push_prim(&mut self, name: Sym, prim: Prim)
    {
        self.push(name, Value::Prim(prim))
    }

    pub fn push_lambda(&mut self, name: Sym, lmbd: LambdaValue)
    {
        self.push(name, Value::Lambda(lmbd))
    }

    // assuming we pushed correctly
    pub fn pop(&mut self, name: &Sym)
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
        println!("eval: {}", self);
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
                    Ok(Value::Sexpr(Self::Atom(Type::Nil)))
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

#[derive(Debug, Clone, PartialEq)]
pub struct LambdaValue
{
    pub params: Vec<Sym>,
    pub expr: Sexpr,
}


impl LambdaValue
{

    fn execute(&self, given_params: &[Sexpr], context: &mut Context) -> Result<Value, String>
    {
        println!("Execute: {:?}", self);
        println!("Execute BIS: {}", self.expr);
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
                if let Value::Sexpr(sex) = expr.eval(context)?
                {
                    context.push_sexpr(sym.clone(), sex);
                }
                else
                {
                    return Err(format!("LAMBDA EXECUTION ERROR: NON-SEXPR PARAM"));
                }
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


impl Sexpr
{
    /*
    fn desugar(&self) -> Self
    {
        match self
        {
            Self::List(lst) => Self::List(lst.iter().map(|sex| sex.desugar()).collect()),
            Self::Sexpr(sex) =>
        }
            
    }*/
}
