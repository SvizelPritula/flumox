use std::iter::Peekable;

use crate::{expr::Expr, EvalError, Value};

use self::{
    temporal::{parse_date, parse_duration},
    tokens::{Token, Tokens},
};
pub use tokens::TokenType;

mod temporal;
mod tokens;

type Iter<'a> = Peekable<Tokens<'a>>;

pub fn parse<'a>(string: &'a str) -> Result<Expr<'a>, EvalError> {
    let mut tokens = Tokens::new(string).peekable();

    let expr = parse_add(&mut tokens)?;

    expect(&mut tokens, TokenType::Eof)?;
    Ok(expr)
}

fn parse_add<'a>(tokens: &mut Iter<'a>) -> Result<Expr<'a>, EvalError> {
    let mut expr = parse_or(tokens)?;

    loop {
        if tokens
            .next_if(|t| matches!(t, Ok((Token::Plus, _))))
            .is_some()
        {
            let duration = parse_duration(tokens)?;

            expr = Expr::Add {
                value: Box::new(expr),
                duration,
            }
        } else {
            break;
        }
    }

    Ok(expr)
}

fn parse_or<'a>(tokens: &mut Iter<'a>) -> Result<Expr<'a>, EvalError> {
    let mut expr = parse_and(tokens)?;

    loop {
        if tokens
            .next_if(|t| matches!(t, Ok((Token::Or, _))))
            .is_some()
        {
            let right = parse_and(tokens)?;

            expr = Expr::Or {
                left: Box::new(expr),
                right: Box::new(right),
            }
        } else {
            break;
        }
    }

    Ok(expr)
}

fn parse_and<'a>(tokens: &mut Iter<'a>) -> Result<Expr<'a>, EvalError> {
    let mut expr = parse_terminal(tokens)?;

    loop {
        if tokens
            .next_if(|t| matches!(t, Ok((Token::And, _))))
            .is_some()
        {
            let right = parse_terminal(tokens)?;

            expr = Expr::And {
                left: Box::new(expr),
                right: Box::new(right),
            }
        } else {
            break;
        }
    }

    Ok(expr)
}

fn parse_terminal<'a>(tokens: &mut Iter<'a>) -> Result<Expr<'a>, EvalError> {
    match tokens.next().transpose()? {
        Some((Token::Number(num), pos)) => Ok(Expr::Literal {
            value: Value::Since(parse_date(tokens, num, pos)?),
        }),
        Some((Token::Word("always"), _)) => Ok(Expr::Literal {
            value: Value::Always,
        }),
        Some((Token::Word("never"), _)) => Ok(Expr::Literal {
            value: Value::Never,
        }),
        Some((Token::Word(word), _)) => Ok(Expr::Field {
            path: parse_path(tokens, word)?,
        }),
        Some((Token::LeftParen, _)) => {
            let expr = parse_add(tokens)?;
            expect(tokens, TokenType::RightParen)?;
            Ok(expr)
        }
        other => Err(unexpected(other)),
    }
}

fn parse_path<'a>(tokens: &mut Iter<'a>, first: &'a str) -> Result<Vec<&'a str>, EvalError> {
    let mut path = vec![first];

    loop {
        if tokens
            .next_if(|t| matches!(t, Ok((Token::Dot, _))))
            .is_some()
        {
            let word = match tokens.next().transpose()? {
                Some((Token::Word(word), _)) => word,
                other => {
                    return Err(unexpected(other));
                }
            };

            path.push(word);
        } else {
            break;
        }
    }

    Ok(path)
}

fn unexpected<'a>(token: Option<(Token<'a>, usize)>) -> EvalError {
    EvalError::UnexpectedToken {
        token: TokenType::new(token.map(|(t, _)| t)),
        pos: token.map(|(_, p)| p),
    }
}

fn expect<'a>(tokens: &mut Iter<'a>, expected: TokenType) -> Result<(), EvalError> {
    let token = tokens.next().transpose()?;
    let actual = TokenType::new(token.map(|(t, _)| t));

    if actual != expected {
        Err(EvalError::UnexpectedToken {
            token: actual,
            pos: token.map(|(_, p)| p),
        })
    } else {
        Ok(())
    }
}

fn expect_number<'a, I>(tokens: &mut Iter<'a>) -> Result<I, EvalError>
where
    I: TryFrom<u64>,
{
    match tokens.next().transpose()? {
        Some((Token::Number(number), pos)) => number
            .try_into()
            .map_err(|_| EvalError::LiteralOutOfRange { pos }),
        other => Err(unexpected(other)),
    }
}
