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


use std::str::CharIndices;
use std::iter::Peekable;

pub struct Lexer<'input> {
    chars: Peekable<CharIndices<'input>>,
    input: &'input str
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self
    {
        Lexer
        {
            chars: input.char_indices().peekable(),
            input
        }
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
                Some((_, ' ')) | Some((_, '\n')) => {self.chars.next();},
                None => return None,
                _ => break
            }
        }
        let start = self.chars.peek().unwrap().0;
        let mut word = String::new();
        loop
        {
            match (self.chars.next(), self.chars.peek())
            {
                /*
                (Some((end, ' ')), _) |
                (Some((end, '\n')), _) => return Some(Ok((start, Token::Atom(Type::from_str(&self.input[start..end])), end))),*/ // impossible at first

                // if the first char is a parenthesis
                (Some((start, '(')), _) => return Some(Ok((start, Token::LDel, start+1))),
                (Some((start, ')')), _) => return Some(Ok((start, Token::RDel, start+1))),

                // if escaping a char
                (Some((_, '\\')), None) => return Some(Err(LexicalError::EscapeEOF)),
                (Some((_, '\\')), Some((_, c))) =>
                {
                    word.push(*c);
                    self.chars.next(); continue
                },
 
                // if end of the word
                (Some((i, c)), None) | // end of the string
                (Some((i, c)), Some((_, '(')))|
                (Some((i, c)), Some((_, ')')))|
                (Some((i, c)), Some((_, ' ')))|
                (Some((i, c)), Some((_, '\n'))) =>
                {
                    word.push(c);
                    return Some(Ok((start, Token::Atom(Type::from_str(&word)), i+1)))
                },
                // any other char
                (Some((i, c)), _) =>
                {
                    word.push(c);
                    continue
                },
                (None, _) => unreachable!()
            }
        }
    }
}


