use crate::types::Type;
pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Debug, Clone)]
pub enum Token
{
    Atom(Type),
    LDel,
    RDel,
}

#[derive(Debug)]
pub enum LexicalError
{
    EscapeEOF
}


use std::str::Chars;
use std::iter::Peekable;

pub struct Lexer<'input> {
    chars: Peekable<Chars<'input>>,
    head_pos: usize,
    input: &'input str
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self
    {
        Lexer
        {
            chars: input.chars().peekable(),
            head_pos: 0,
            input
        }
    }

    pub fn pop(&mut self) -> Option<char>
    {
        let maybe_c = self.chars.next();

        if let Some(c) = maybe_c
        {
            self.head_pos += c.len_utf8();
        }

        maybe_c

    }
    
}


impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, LexicalError>;
    fn next(&mut self) -> Option<Self::Item> {

        // FIND THE START
        loop
        {
            match self.chars.peek()
            {
                Some(' ') | Some('\n') =>
                {
                    self.pop();
                },
                None => return None,
                _ => break
            }
        }
        let start = self.head_pos;
        loop
        {
            match (self.pop(), self.chars.peek())
            {
                // if the first char is a parenthesis
                (Some('('), _) => return Some(Ok((start, Token::LDel, start+1))),
                (Some(')'), _) => return Some(Ok((start, Token::RDel, start+1))),

                // if escaping a char
                (Some('\\'), None) => return Some(Err(LexicalError::EscapeEOF)),
                (Some('\\'), Some(c)) =>
                {
                    self.chars.next(); continue
                },
 
                // if end of the word
                (Some(c), None) | // end of the string
                (Some(c), Some('('))|
                (Some(c), Some(')'))|
                (Some(c), Some(' '))|
                (Some(c), Some('\n')) =>
                {
                    return Some(Ok((start, Token::Atom(Type::from_str(&self.input[start..self.head_pos])), self.head_pos)))
                },
                // any other char
                (Some(c), _) =>
                {
                    continue
                },
                (None, _) => unreachable!()
            }
        }
    }
}


