use nom::{
    branch::alt,
    bytes::{complete::take_while1, escaped_transform},
    character::{char, complete::space0, satisfy},
    combinator::{all_consuming, recognize},
    error::Error,
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    Err, Parser,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ForwardedElement {
    pub by: Option<String>,
    pub r#for: Option<String>,
    pub host: Option<String>,
    pub proto: Option<String>,
}

fn token<'a>() -> impl Parser<&'a [u8], Output = &'a str, Error = Error<&'a [u8]>> {
    fn is_token_char(c: u8) -> bool {
        matches!(c,
            b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z'
            | b'!' | b'#' | b'$' | b'%' | b'&' | b'\'' | b'*'| b'+'
            | b'-' | b'.' | b'^' | b'_' | b'`' | b'|' | b'~'
        )
    }

    take_while1(is_token_char).map_res(std::str::from_utf8)
}

fn quoted_string<'a>() -> impl Parser<&'a [u8], Output = String, Error = Error<&'a [u8]>> {
    fn is_unescaped_char(c: u8) -> bool {
        matches!(c, b'\t' | b' ' | b'!' | b'#'..=b'[' | b']'..b'~')
    }

    fn is_escaped_char(c: char) -> bool {
        matches!(c, '\t' | ' ') | c.is_ascii_graphic()
    }

    let inner = escaped_transform(
        take_while1(is_unescaped_char),
        '\\',
        recognize(satisfy(is_escaped_char)),
    )
    .map_res(String::from_utf8);
    delimited(char('"'), inner, char('"'))
}

fn forwarded_pair<'a>() -> impl Parser<&'a [u8], Output = (&'a str, String), Error = Error<&'a [u8]>>
{
    separated_pair(
        token(),
        char('='),
        alt((token().map(ToOwned::to_owned), quoted_string())),
    )
}

fn forwarded_element<'a>(
) -> impl Parser<&'a [u8], Output = ForwardedElement, Error = Error<&'a [u8]>> {
    separated_list1(char(';'), forwarded_pair()).map(|pairs| {
        let mut element = ForwardedElement::default();

        for (name, value) in pairs {
            match name {
                _ if name.eq_ignore_ascii_case("by") => element.by = Some(value),
                _ if name.eq_ignore_ascii_case("for") => element.r#for = Some(value),
                _ if name.eq_ignore_ascii_case("host") => element.host = Some(value),
                _ if name.eq_ignore_ascii_case("proto") => element.proto = Some(value),
                _ => {}
            }
        }

        element
    })
}

fn forwarded_header<'a>(
) -> impl Parser<&'a [u8], Output = Vec<ForwardedElement>, Error = Error<&'a [u8]>> {
    separated_list1(delimited(space0, char(','), space0), forwarded_element())
}

pub fn parse<'a>(
    values: impl Iterator<Item = &'a [u8]>,
) -> Result<Vec<ForwardedElement>, Err<Error<&'a [u8]>>> {
    let mut result = Vec::new();

    for input in values {
        result.extend(all_consuming(forwarded_header()).parse_complete(input)?.1);
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use crate::{parse, ForwardedElement};

    #[test]
    fn single_unquoted_for() {
        let value = b"for=195.113.20.176";
        let result = parse([value.as_ref()].into_iter()).unwrap();
        assert_eq!(
            result,
            vec![ForwardedElement {
                r#for: Some(String::from("195.113.20.176")),
                ..Default::default()
            }]
        )
    }

    #[test]
    fn single_quoted_for() {
        let value = b"for=\"195.113.20.176\"";
        let result = parse([value.as_ref()].into_iter()).unwrap();
        assert_eq!(
            result,
            vec![ForwardedElement {
                r#for: Some(String::from("195.113.20.176")),
                ..Default::default()
            }]
        )
    }

    #[test]
    fn single_escaped_for() {
        let value = b"for=\"\\\"value\\\"\"";
        let result = parse([value.as_ref()].into_iter()).unwrap();
        assert_eq!(
            result,
            vec![ForwardedElement {
                r#for: Some(String::from("\"value\"")),
                ..Default::default()
            }]
        )
    }

    #[test]
    fn single_all() {
        let value = b"for=f;by=b;proto=p;host=h";
        let result = parse([value.as_ref()].into_iter()).unwrap();
        assert_eq!(
            result,
            vec![ForwardedElement {
                by: Some(String::from("b")),
                r#for: Some(String::from("f")),
                proto: Some(String::from("p")),
                host: Some(String::from("h")),
            }]
        )
    }

    #[test]
    fn multiple_for_inline() {
        let value = b"for=a,for=b";
        let result = parse([value.as_ref()].into_iter()).unwrap();
        assert_eq!(
            result,
            vec![
                ForwardedElement {
                    r#for: Some(String::from("a")),
                    ..Default::default()
                },
                ForwardedElement {
                    r#for: Some(String::from("b")),
                    ..Default::default()
                }
            ]
        )
    }

    #[test]
    fn multiple_for_separate() {
        let value = [b"for=a".as_ref(), b"for=b".as_ref()];
        let result = parse(value.into_iter()).unwrap();
        assert_eq!(
            result,
            vec![
                ForwardedElement {
                    r#for: Some(String::from("a")),
                    ..Default::default()
                },
                ForwardedElement {
                    r#for: Some(String::from("b")),
                    ..Default::default()
                }
            ]
        )
    }
}
