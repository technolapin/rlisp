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
            let c = self.pop();
            let d = self.chars.peek();
            println!("THIS ITERATION: {:?}", (c , d));
            match (c, d)
            {
                // if the first char is a parenthesis
                (Some('('), _) => return Some(Ok((start, Token::LDel, self.head_pos))),
                (Some(')'), _) => return Some(Ok((start, Token::RDel, self.head_pos))),

                // if escaping a char
                (Some('\\'), None) => return Some(Err(LexicalError::EscapeEOF)),
                (Some('\\'), Some(c)) =>
                {
                    self.pop(); continue
                },
 
                // if end of the word
                (None, _)| // when we escaped the last char
                (Some(_), None) | // end of the string
                (Some(_), Some('('))|
                (Some(_), Some(')'))|
                (Some(_), Some(' '))|
                (Some(_), Some('\n')) =>
                {
                    match &self.input.get(start..self.head_pos)
                    {
                        Some(slice) =>
                            return Some(Ok((start,
                                            Token::Atom(Type::from_str(slice)),
                                            self.head_pos))),
                        None =>
                        {
                            panic!("WEIRD SLICING:\n{}\n({} {})", self.input, start, self.head_pos);
                        }
                    }
                },
                // any other char
                (Some(c), _) =>
                {
                    continue
                }
            }
        }
    }
}


