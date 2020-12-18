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

    fn precedence(&self) -> u32 {
        match self {
            Operator::Add | Operator::Subtract => 1,
            Operator::Multiply | Operator::Divide => 0
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

enum ParseContext {
    ExpectingValue,
    ExpectingOperator(i64)
}

enum TerminationReason {
    EndOfInput,
    CloseParen
}

fn parse_expr(stream: &mut impl Iterator<Item=Result<Token, ParseError>>) 
    -> Result<(i64, TerminationReason), ParseError> {

    let mut termination_reason = TerminationReason::EndOfInput;

    // A chain of 'value operator' sequences of strictly increasing precedence.
    // When we see an inversion (A*B + C), we evaluate A*B 
    let mut operator_stack = Vec::<(i64, Operator)>::new();
    let mut context = ParseContext::ExpectingValue;

    while let Some(token) = stream.next() {
        let token = token?;
        if let Token::CloseParen = token { 
            termination_reason = TerminationReason::CloseParen;
            break;
        }

        context = match context {
            ParseContext::ExpectingValue => match token {
                // If this is a value, good -- that's what we're looking for.
                Token::Value(value) => ParseContext::ExpectingOperator(value),

                // If this is a parenthesis, parse a sub-expression.
                Token::OpenParen => match parse_expr(stream)? {
                    // The expression should have terminated with a closing parenthesis.
                    (value, TerminationReason::CloseParen) => ParseContext::ExpectingOperator(value),
                    (_, TerminationReason::EndOfInput) => return Err(ParseError::UnexpectedEOF)
                }
                _ => return Err(ParseError::ExpectedValue)
            },
            ParseContext::ExpectingOperator(mut value) => {
                let next_op = 
                    if let Token::Operator(op) = token { op }
                    else { return Err(ParseError::ExpectedOperator) };

                // If we have a pattern of the form '... A*B +', evaluate A*B.
                while let Some(&(prev_value, prev_op)) = operator_stack.last() {
                    if prev_op.precedence() >= next_op.precedence() {
                        operator_stack.pop();
                        value = prev_op.apply(prev_value, value);
                    } else {
                        break;
                    }
                }
                
                operator_stack.push((value, next_op));
                ParseContext::ExpectingValue
            }
        }
    }

    // We've reached the end of the expression.
    match context {
        ParseContext::ExpectingValue => Err(ParseError::UnexpectedEOF),
        ParseContext::ExpectingOperator(mut rhs) => {
            for (lhs, op) in operator_stack.into_iter().rev() {
                rhs = op.apply(lhs, rhs);
            }
            Ok((rhs, termination_reason))
        }
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
