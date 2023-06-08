use time::{Date, Duration, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

use crate::{
    parse::{expect, expect_number, tokens::Token, unexpected},
    EvalError, TokenType,
};

use super::Iter;

pub fn parse_duration<'a>(tokens: &mut Iter<'a>) -> Result<Duration, EvalError> {
    let mut total = Duration::ZERO;
    let mut parsed_some = false;

    loop {
        let Some(Ok((Token::Number(num), pos))) = tokens.peek() else {
            if parsed_some {
                break;
            } else {
                return Err(unexpected(tokens.next().transpose()?));
            }
        };

        let num = *num;
        let pos = *pos;
        tokens.next();

        let unit = match tokens.next().transpose()? {
            Some((Token::Word(word), _)) => Ok(word),
            other => Err(unexpected(other)),
        }?;

        let num = num
            .try_into()
            .map_err(|_| EvalError::LiteralOutOfRange { pos })?;
        let unit = get_unit(unit).ok_or(EvalError::UnknownUnit { unit: unit.into() })?;

        let duration = unit
            .checked_mul(num)
            .ok_or(EvalError::LiteralOutOfRange { pos })?;

        total = total
            .checked_add(duration)
            .ok_or(EvalError::LiteralOutOfRange { pos })?;

        parsed_some = true;
    }

    Ok(total)
}

fn get_unit(name: &str) -> Option<Duration> {
    match name {
        "d" => Some(Duration::DAY),
        "h" => Some(Duration::HOUR),
        "m" => Some(Duration::MINUTE),
        "s" => Some(Duration::SECOND),
        "ms" => Some(Duration::MILLISECOND),
        _ => None,
    }
}

pub fn parse_date<'a>(
    tokens: &mut Iter<'a>,
    first: u64,
    pos: usize,
) -> Result<OffsetDateTime, EvalError> {
    let date = {
        let year = first
            .try_into()
            .map_err(|_| EvalError::LiteralOutOfRange { pos })?;

        expect(tokens, TokenType::Dash)?;
        let month = expect_number(tokens)?;
        expect(tokens, TokenType::Dash)?;
        let day = expect_number(tokens)?;

        let month = u8::try_into(month).map_err(|_| EvalError::LiteralOutOfRange { pos })?;

        Date::from_calendar_date(year, month, day)
            .map_err(|_| EvalError::LiteralOutOfRange { pos })?
    };

    let time = {
        let hour = expect_number(tokens)?;
        expect(tokens, TokenType::Colon)?;
        let minute = expect_number(tokens)?;
        let second = parse_optional_time_component(tokens, 0)?;

        Time::from_hms(hour, minute, second).map_err(|_| EvalError::LiteralOutOfRange { pos })?
    };

    let offset = {
        let negative = match tokens.next().transpose()? {
            Some((Token::Plus, _)) => Ok(false),
            Some((Token::Dash, _)) => Ok(true),
            other => Err(unexpected(other)),
        }?;

        let hours: i8 = expect_number(tokens)?;
        let minutes = parse_optional_time_component(tokens, 0)?;
        let seconds = parse_optional_time_component(tokens, 0)?;

        let hours = if negative { -hours } else { hours };

        UtcOffset::from_hms(hours, minutes, seconds)
            .map_err(|_| EvalError::LiteralOutOfRange { pos })?
    };

    let datetime = PrimitiveDateTime::new(date, time).assume_offset(offset);

    Ok(datetime)
}

fn parse_optional_time_component<'a, I>(tokens: &mut Iter<'a>, default: I) -> Result<I, EvalError>
where
    I: TryFrom<u64>,
{
    if tokens
        .next_if(|t| matches!(t, Ok((Token::Colon, _))))
        .is_some()
    {
        expect_number(tokens)
    } else {
        Ok(default)
    }
}
