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

#[derive(Debug, Clone, Copy, PartialEq)]
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

    fn compare_tier(&self, other: &Self) -> Self
    {
        use Num::*;
        * match (self, other)
        {
            (Z(_), _) => other,
            (Q(_, _), Z(_)) => self,
            (Q(_, _), _) => other,
            (R(_), C(_, _)) => other,
            _ => self
        }
    }
    
    fn higher_tier(nums: &[Self]) -> Self
    {
        if nums.len() == 0
        {
            Self::Z(0)
        }
        else
        {
            nums[0].compare_tier(&Self::higher_tier(&nums[1..]))
        }
    }

    /// does litteraly nothing
    fn to_Z(&self) -> Self
    {
        *self
    }

    /// returns a casting of self into Q or self if it is already more complex
    fn to_Q(&self) -> Self
    {
        use Num::*;
        match self
        {
            Z(a) => Q(*a, 1),
            _ => *self
        }
    }
    
    fn to_R(&self) -> Self
    {
        use Num::*;
        match self
        {
            Z(a) => R(*a as f64),
            Q(a, b) => R((*a as f64) / (*b as f64)),
            _ => *self
        }
    }
    
    fn to_C(&self) -> Self
    {
        use Num::*;
        match self
        {
            Z(a) => C(*a as f64, 0.),
            Q(a, b) => C((*a as f64) / (*b as f64), 0.),
            R(a) => C(*a, 0.),
            _ => *self
        }
    }
    
    pub fn cast_to_model(&self, model: &Self) -> Self
    {
        match model
        {
            Self::Z(_) => self.to_Z(),
            Self::Q(_, _) => self.to_Q(),
            Self::R(_) => self.to_R(),
            Self::C(_, _) => self.to_C(),
        }
    }
 
    pub fn degenerate(nums: &[Self]) -> Vec<Self>
    {
        let worst = Self::higher_tier(nums);
        nums.iter()
            .map(|num| num.cast_to_model(&worst))
            .collect::<Vec<_>>()
    }

    fn double_cast(&self, other: &Self) -> (Self, Self)
    {
        let self_casted = self.cast_to_model(other);
        let other_casted = other.cast_to_model(&self_casted);
        (self_casted, other_casted)

    }
    /// a and be must be non-negative
    fn gcd(a: i64, b: i64) -> i64
    {
        println!("    GCD {} {}", a, b);
        if a == 0
        {
            b
        }
        else
        {
            match (a % 2, b % 2)
            {
                (0, 0) => 2*Self::gcd(a/2, b/2),
                (0, 1) => Self::gcd(a/2, b),
                (1, 0) => Self::gcd(a, b/2),
                _ => Self::gcd((a-b).abs(), a.min(b))
                    
            }
        }
    }

    fn make_rational(u: i64, v: i64) -> Self
    {
        use Num::*;
        match (u, v)
        {
            (a, 0) => R(a as f64*f64::INFINITY),
            (0, _) => Q(0, 1),
            (u, v) =>
            {
                let sign = u.signum()*v.signum();
                let (u, v) = (u.abs(), v.abs());
                let gcd = Self::gcd(u, v);
                
                Q(sign*u/gcd, v/gcd)
            }
        }

    }

    fn iscalar_add(a: i64, b: i64) -> Result<i64, String>
    {
        a.checked_add(b).ok_or(format!("Add overflow"))
    }

    fn iscalar_mul(a: i64, b: i64) -> Result<i64, String>
    {
        a.checked_mul(b).ok_or(format!("Mul overflow"))
    }
    
    pub fn add(&self, other: &Self) -> Result<Self, String>
    {
        use Num::*;
        match self.double_cast(other)
        {
            (Z(a), Z(b)) => Ok(Z(Self::iscalar_add(a, b)?)),
            (Q(a, b), Q(c, d)) =>
            {
                let ad = Self::iscalar_mul(a, d)?; 
                let cb = Self::iscalar_mul(c, b)?; 
                let bd = Self::iscalar_mul(b, d)?; 

                let u = Self::iscalar_add(ad, cb)?;
                let v = bd;
                Ok(Self::make_rational(u, v))
            },
            (R(a), R(b)) => Ok(R(a+b)),
            (C(a, b), C(c, d)) => Ok(C(a+c, b+d)),
            _ => unreachable!()
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
