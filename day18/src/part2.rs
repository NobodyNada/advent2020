use std::io::{self, BufRead};

// Token definitions

/// A token in the input stream.
#[derive(Clone, Copy, Debug)]
enum Token {
    Operator(Operator),
    OpenParen,
    CloseParen,
    Value(i64)
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl Operator {
    /// Converts a character to an operator.
    fn parse(c: char) -> Option<Operator> {
        match c {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Subtract),
            '*' => Some(Operator::Multiply),
            '/' => Some(Operator::Divide),
            _ => None
        }
    }

    /// Computes 'lhs op rhs'.
    fn apply(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Subtract => lhs - rhs,
            Operator::Multiply => lhs * rhs,
            Operator::Divide => lhs / rhs,
        }
    }

    /// Returns an integer representing the precedence of this operator.
    /// Operators with higher `precedence()` should be evaluated first.
    fn precedence(&self) -> u32 {
        match self {
            Operator::Add | Operator::Subtract => 1,
            Operator::Multiply | Operator::Divide => 0
        }
    }
}


/// An error that can occur during lexing or parsing.
#[derive(Debug)]
enum ParseError {
    InvalidValue(<i64 as std::str::FromStr>::Err),
    UnexpectedEOF,
    ExpectedValue,
    ExpectedOperator,
    UnbalancedParens
}

// The lexer

/// An iterator which converts a stream of characters into a stream of tokens.
struct TokenStream<S: Iterator<Item=char>> {
    stream: std::iter::Peekable<S>
}

impl<S: Iterator<Item=char>> Iterator for TokenStream<S> {
    type Item = Result<Token, ParseError>;

    fn next(&mut self) -> Option<Result<Token, ParseError>> {
        // Skip whitespace characters
        while self.stream.peek().map(|c| c.is_whitespace()).unwrap_or(false) {
            self.stream.next();
        }

        match self.stream.peek() {
            None => None,
            Some(&c) => {
                // Is this a special character?
                if let Some(special) = Self::is_special(c) { 
                    self.stream.next();
                    Some(Ok(special)) 
                }
                // Is this a valid value?
                else { Some(self.read_value().map(Token::Value)) }
            }
        }
    }

}

impl<S: Iterator<Item=char>> TokenStream<S> {
    /// Creates a new TokenStream.
    fn new(stream: S) -> Self {
        Self { stream: stream.peekable() }
    }

    /// Reads an integer into a token.
    fn read_value(&mut self) -> Result<i64, ParseError> {
        let value = self.stream.next().ok_or(ParseError::UnexpectedEOF)?;
        let mut value: String = value.to_string();

        // while we have more input...
        while let Some(&c) = self.stream.peek() {
            if c.is_whitespace() || Self::is_special(c).is_some() {
                // We've hit a delimiter; stop
                break;
            } else {
                // Add it to the string.
                self.stream.next();
                value.push(c);
            }
        }

        // Convert to integer
        value.parse().map_err(ParseError::InvalidValue)
    }

    /// Is this character an operator or parenthesis?
    fn is_special(c: char) -> Option<Token> {
        if let Some(op) = Operator::parse(c) { Some(Token::Operator(op)) }
        else if c == '(' { Some(Token::OpenParen) }
        else if c == ')' { Some(Token::CloseParen) }
        else { None }
    }
}

// The parser

/// The token stream should consist of a sequence of (value, operator) pairs.
/// The parser context keeps track of our position in the sequence.
enum ParseContext {
    ExpectingValue,
    ExpectingOperator(i64)
}

/// Parses an expression.
fn parse(stream: &mut impl Iterator<Item=Result<Token, ParseError>>) -> Result<i64, ParseError> {
    parse_(stream, false)
}

/// Parses an expression. paranthetical is false if this is a root expression, or true if this is
/// nested inside another expression using parantheses.
fn parse_(stream: &mut impl Iterator<Item=Result<Token, ParseError>>, parenthetical: bool) 
    -> Result<i64, ParseError> {

    // A chain of 'value operator' sequences of strictly increasing precedence.
    // When we see an inversion in the sequence such as A*B + C (or A*B*C),
    // we immediately evaluate A*B.to preserve the strictly-increasing order.
    let mut operator_stack = Vec::<(i64, Operator)>::new();

    let mut context = ParseContext::ExpectingValue;

    let rhs = loop {
        // Parse a value.
        // If we find the end of the expression instead, return an error.
        let value_tok = stream.next().unwrap_or(Err(ParseError::UnexpectedEOF))?;
        let value = match value_tok {
            Token::Value(value) => value,
            Token::OpenParen => parse_(stream, true)?,

            _ => return Err(ParseError::ExpectedValue),
        }

        // (Try to) parse an operator token.
        // If we find the end the expression instead, return the last value.
        let operator = match stream.next() {
            Some(Ok(Token::Operator(op))) => op,

            Some(Ok(Token::CloseParen)) => // This is the end of a parenthetical expression.
                // Was it *supposed* to be a parenthetical expression?
                // If so, stop parsing. and return the last value.
                if parenthetical { break value; } 
                else { Err(ParseError::UnbalancedParens)? },

            None =>     // This is the end of the input. Was it supposed to end here?
                if parenthetical { Err(ParseError::UnexpectedEOF)? }
                else { break value; },

            Some(Ok(_)) => return Err(ParseError::ExpectedOperator),
            Some(Err(e)) => return Err(e)
        };
    
        // We have a (value, operator) pair; add it to the stack.
        // If we have a pattern of the form '... A*B +', evaluate A*B.
        let mut value = value;
        while let Some(&(prev_value, prev_op)) = operator_stack.last() {
            if prev_op.precedence() >= operator.precedence() {
                operator_stack.pop();
                value = prev_op.apply(prev_value, value);
            } else {
                break;
            }
        }
        
        operator_stack.push((value, operator));
    };

    // We've successfully reached the end of the expression.
    // The operators left on the stack are in order of
    // strictly increasing precedence, 
    for (lhs, op) in operator_stack.into_iter().rev() {
        rhs = op.apply(lhs, rhs);
    }
    Ok(rhs)
}

pub fn run() {
    let mut result = 0;

    for line in io::stdin().lock().lines() {
        let line = line.expect("read error");
        let mut tokens = TokenStream::new(line.chars());

        result += parse(&mut tokens).unwrap();
    }

    println!("{}", result);
}
