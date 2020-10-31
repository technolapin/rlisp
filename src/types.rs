use regex::RegexSet;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Type
{
    Nil,
    Sym(Sym),
    Num(Num),
    Char(Char)
}

impl Type
{
    pub fn from_str(s: &str) -> Self
    {
        /*
        println!("parsing {} : ", s);
        for c in s.chars()
        {print!("[{}] ", c);}
         */
        let set = RegexSet::new(&[
            r"^[0-9]+$",
            r"^\?\\.$",
        ]).unwrap();
        let recognized = set.matches(s).into_iter().next();
        // println!("automata returned {:?}", recognized);
        
        match recognized
        {
            None => Type::Sym(Sym::new(s)),
            Some(0) => Type::Num(Num::Z(i64::from_str(s).unwrap())),
            Some(1) => Type::Char(Char(s.chars().nth(2).unwrap())),
            Some(_) => unreachable!()
                
        }

    }
}

use std::fmt;

impl fmt::Display for Type
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            Self::Sym(sym) => write!(f, "{}", sym),
            Self::Num(num) => write!(f, "{}", num),
            Self::Char(cha) => write!(f, "{}", cha),
            Self::Nil => write!(f, "()"),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Char(pub char);

impl fmt::Display for Char
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "?\\{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sym(pub String);

impl fmt::Display for Sym
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.0)
    }
}


impl Sym
{
    pub fn new(name: &str) -> Self
    {
        Self(String::from(name))
    }
    pub fn t() -> Self
    {
        Self::new("t")
    }

}

#[derive(Debug, Clone, PartialEq)]
pub enum Num
{
    Z(i64),
    Q(i64, i64),
    R(f64),
    C(f64, f64)
}

impl Num
{
    fn are_homogenous(nums: &[Self]) -> bool
    {
        use Num::*;
        if nums.len() <= 1
        {
            true
        }
        else
        {
            match (&nums[0], &nums[1])
            {
                (Z(_), Z(_))|
                (Q(_, _), Q(_, _))|
                (R(_), R(_))|
                (C(_, _), C(_, _)) => Num::are_homogenous(&nums[1..]),
                _ => false
            }
        }
        
    }
}

impl fmt::Display for Num
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            Self::Z(x) => write!(f, "{}", x),
            Self::Q(a, b) => write!(f, "{}/{}", a, b),
            Self::R(x) => write!(f, "{}", x),
            Self::C(a, b) => write!(f, "{}+i{}", a, b),
        }
    }
}



impl Type
{
}
