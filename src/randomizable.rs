pub trait Randomizable
{
    fn rand(depth: usize) -> Self;
}

use crate::types::*;
use crate::ast::*;
    
impl Randomizable for Type
{
    fn rand(_: usize) -> Self
    {
        match rand::random::<u32>() % 4
        {
            0 => Self::Sym(Randomizable::rand(0)),
            1 => Self::Num(Randomizable::rand(0)),
            2 => Self::Char(Randomizable::rand(0)),
            _ => Self::Nil,
        }
    }
}

//use rand::Rng;

impl Randomizable for Sym
{
    fn rand(_: usize) -> Self
    {
        let len = rand::random::<usize>() % 20+1;
        let mut s = String::new();
        while s.len() < len
        {
            if let Some(c) = std::char::from_u32(rand::random::<u32>())
            {
                match c
                {
                    ' '|'\n'|'('|')' => {s.push('\\');},
                    _ => ()
                }
                s.push(c);
            }
        }

        Self(s)
        
    }
}

impl Randomizable for Num
{
    fn rand(_: usize) -> Self
    {
        match rand::random::<u32>() % 4
        {
            0 => Self::Z(rand::random::<i64>()),
            1 =>
            {
                Self::make_rational(rand::random::<i64>(), rand::random::<i64>())
            },
            2 => Self::R(rand::random::<f64>()),
            _ => Self::C(rand::random::<f64>(), rand::random::<f64>()),
        }
    }
}

impl Randomizable for Char
{
    fn rand(_: usize) -> Self
    {
        let c = rand::random::<char>();
        Self(c)        
    }
}


impl Randomizable for Sexpr
{
    fn rand(depth: usize) -> Self
    {
        if depth == 0 || rand::random::<u32>() % 2 == 0
        {
            Self::Atom(Randomizable::rand(0))
        }
        else
        {
            let len = rand::random::<usize>() % 3+1;
            Self::List((0..len)
                       .map(|_| Self::rand(depth-1))
                       .collect::<Vec<_>>())
        }
    }
}
