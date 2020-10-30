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
    EscapeEOF,
    //NotASCII,
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

    fn extract_atom(&self, start: usize, end: usize) -> Option<Spanned<Token, usize, LexicalError>>
    {
        match &self.input.get(start..end)
        {
            Some(slice) =>
                return Some(Ok((start,
                                Token::Atom(Type::from_str(slice)),
                                end))),
            None =>
            {
                panic!("WEIRD SLICING:\n{}\n({} {})", self.input, start, end);
            }
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
                Some(' ') | Some('\n') =>
                {
                    self.pop();
                },
                None => return None,
                _ => break
            }
        }
        let start = self.head_pos;
        //println!("FETCHING WORD");
        let mut escaping = false;
        loop
        {
            let maybe_c = self.pop();
            /*
                
            if let Some(c) = maybe_c
            {
                if !c.is_ascii()
                {
                    return Some(Err(LexicalError::NotASCII));
                }
            }
             */
            
            let d = self.chars.peek();
            //println!("THIS ITERATION: {:?}", (maybe_c , d, escaping));
            match (maybe_c, d, escaping)
            {
                (Some(_), Some(' '), true)|
                (Some(_), Some('\n'), true)|
                (Some(_), Some('('), true)|
                (Some(_), Some(')'), true)|
                (Some(_), None, true) => // if we escaped the last char
                {
                    return self.extract_atom(start, self.head_pos);
                },
               
                (Some(_), _, true) => // if we escaped (non-end case)
                {
                    // we just take the char without taking it into account
                },

                
                (None, _, true) => // if we escaped the end of the file
                {
                    return Some(Err(LexicalError::EscapeEOF));
                },
                // if the first char is a parenthesis
                (Some('('), _, false) => return Some(Ok((start, Token::LDel, self.head_pos))),
                (Some(')'), _, false) => return Some(Ok((start, Token::RDel, self.head_pos))),

                // if escaping a char
                (Some('\\'), _, false) =>
                {
                    escaping = true;
                    continue
                },
 
                // if end of the word
                (None, _, false)| // when we escaped the last char (not anymore)
                (Some(_), None, false) | // end of the string
                (Some(_), Some('('), false)|
                (Some(_), Some(')'), false)|
                (Some(_), Some(' '), false)|
                (Some(_), Some('\n'), false) =>
                {
                    return self.extract_atom(start, self.head_pos)
                },
                // any other char
                (Some(c), _, false) =>
                {
                }
            }
            escaping = false;
        }
    }
}


