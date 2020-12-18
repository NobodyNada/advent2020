use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl Operator {
    fn parse(c: char) -> Option<Operator> {
        match c {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Subtract),
            '*' => Some(Operator::Multiply),
            '/' => Some(Operator::Divide),
            _ => None
        }
    }

    fn apply(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Operator::Add => lhs + rhs,
            Operator::Subtract => lhs - rhs,
            Operator::Multiply => lhs * rhs,
            Operator::Divide => lhs / rhs,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Token {
    Operator(Operator),
    OpenParen,
    CloseParen,
    Value(i64)
}

struct TokenStream<S: Iterator<Item=char>> {
    stream: std::iter::Peekable<S>
}

#[derive(Debug)]
enum ParseError {
    UnexpectedEOF,
    ExpectedValue,
    ExpectedOperator,
    UnbalancedParens,
    InvalidValue(<i64 as std::str::FromStr>::Err)
}

impl<S: Iterator<Item=char>> Iterator for TokenStream<S> {
    type Item = Result<Token, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(true) = self.stream.peek().map(|c| c.is_whitespace()) {
            self.stream.next();
        }
        match self.stream.peek() {
            None => None,
            Some(&c) => {
                if let Some(control) = Self::is_control(c) { 
                    self.stream.next();
                    Some(Ok(control)) 
                }
                else { Some(self.read_value().map(Token::Value)) }
            }
        }
    }

}

impl<S: Iterator<Item=char>> TokenStream<S> {
    fn new(stream: S) -> Self {
        Self { stream: stream.peekable() }
    }

    fn read_value(&mut self) -> Result<i64, ParseError> {
        let value = self.stream.peek();
        let mut value: String = [value.ok_or(ParseError::UnexpectedEOF)?]
            .iter().cloned().collect();

        // consume value
        self.stream.next();

        // while we have more input and it's not a control character
        while let Some(&c) = self.stream.peek() {
            if c.is_whitespace() {
                // consume 
                self.stream.next();
            }
            else if Self::is_control(c).is_none() {
                self.stream.next();
                value.push(c);
            } else {
                break;
            }
        }
        value.parse().map_err(ParseError::InvalidValue)
    }

    fn is_control(c: char) -> Option<Token> {
        if let Some(op) = Operator::parse(c) { Some(Token::Operator(op)) }
        else if c == '(' { Some(Token::OpenParen) }
        else if c == ')' { Some(Token::CloseParen) }
        else { None }
    }
}

#[derive(Debug)]
enum ParseContext {
    Start,
    ExpectingValue(i64, Operator),
    ExpectingOperator(i64),
}

enum TerminationReason {
    EndOfInput,
    CloseParen
}

fn parse_expr(stream: &mut impl Iterator<Item=Result<Token, ParseError>>) 
    -> Result<(i64, TerminationReason), ParseError> {

    let mut context = ParseContext::Start;
    let mut termination_reason = TerminationReason::EndOfInput;

    while let Some(token) = stream.next() {
        let token = token?;
        if let Token::CloseParen = token { 
            termination_reason = TerminationReason::CloseParen;
            break;
        }

        fn parse_value(token: Token, stream: &mut impl Iterator<Item=Result<Token, ParseError>>) 
            -> Result<i64, ParseError> {

            match token {
                Token::Value(value) => Ok(value),
                Token::OpenParen => match parse_expr(stream) {
                    Ok((value, TerminationReason::CloseParen)) => Ok(value),
                    Ok((_, TerminationReason::EndOfInput)) => Err(ParseError::UnexpectedEOF),
                    Err(e) => Err(e)
                }
                _ => Err(ParseError::ExpectedValue)
            }
        }

        context = match context {
            ParseContext::Start => 
                ParseContext::ExpectingOperator(parse_value(token, stream)?),
            ParseContext::ExpectingValue(lhs, op) => 
                ParseContext::ExpectingOperator(op.apply(lhs, parse_value(token, stream)?)),
            ParseContext::ExpectingOperator(value) => match token {
                Token::Operator(op) => ParseContext::ExpectingValue(value, op),
                _ => return Err(ParseError::ExpectedOperator)
            }
        }
    }

    match context {
        ParseContext::Start => Err(ParseError::ExpectedValue),
        ParseContext::ExpectingValue(_, _) => Err(ParseError::ExpectedValue),
        ParseContext::ExpectingOperator(result) => Ok((result, termination_reason))
    }
}

fn parse(stream: &mut impl Iterator<Item=Result<Token, ParseError>>) -> Result<i64, ParseError> {
    match parse_expr(stream) {
        Ok((value, TerminationReason::EndOfInput)) => Ok(value),
        Ok((_, TerminationReason::CloseParen)) => Err(ParseError::UnbalancedParens),
        Err(e) => Err(e)
    }
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
