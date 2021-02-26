use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::{is_not, take_while1, take_while_m_n};
use nom::character::complete::alpha1;
use nom::character::complete::{alphanumeric1, char, none_of};
use nom::character::{is_alphanumeric, is_hex_digit, is_oct_digit, is_space};
use nom::combinator::{map_opt, map_res, recognize, value, verify};
use nom::error::{Error, ParseError};
use nom::multi::{fold_many0, many0};
use nom::named;
use nom::number::complete::be_u16;
use nom::sequence::{delimited, pair, preceded, precededc};
use nom::{IResult, InputLength, InputTake};
use regex::internal::Input;
use std::str::FromStr;

// Nom 5's default IO types are &[u8], or dynamic-sized slices.
// Use b"STRING" for [u8; N] static slice types,
// use b"STRING"[..] for [u8] dynamic slice types,
// and use &b"STRING"[..] to get &[u8] immutable referecne to dynamic slice type

// Returns OK(remainder, result) or Err(something???)

// <identifier>     ::= ([A-z_]),(A-z0-9_)*
// <string>         ::= '“‘,<string-inner>*,'’”
// <string-inner>   ::=
// <name>           ::= <symbol>|<string>
// <integer>        ::= [0-9]+
// <float>          ::= ([0-9]*),’.’,([0-9]+)
// <literal>        ::= <symbol>|<string>|<integer>|<float>
// <predicate>      ::= (<symbol>|<string>),’:’,(<ws>*),<operator>,(<ws>*), (<literal>)
// <conjunction>    ::= ‘,’|’:’|’=’|’>’|’<’|’>=’|’<=’|’~=’|’:’|’|’|’\n’
// <expression>     ::= <predicate>,((<ws>*),<conjunction>,<predicate>)*
pub fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)
}

fn parse_unicode_4(input: &str) -> IResult<&str, char> {
    let parse_delimited_hex = preceded(
        char('u'),
        take_while_m_n(4, 4, |c: char| c.is_ascii_hexdigit()),
    );
    let parse_u32 = map_res(parse_delimited_hex, move |s: &str| {
        u32::from_str_radix(s, 16)
    });
    map_opt(parse_u32, |value| std::char::from_u32(value))(input)
}

fn char_numerical_escape(
    prefix: &'static str,
    min_digits: usize,
    max_digits: usize,
    escape_digits_radix: u32,
) -> impl Fn(&str) -> IResult<&str, char> {
    move |input| {
        let parse_delimited_hex = preceded(
            tag(prefix),
            take_while_m_n(min_digits, max_digits, |c: char| c.is_ascii_hexdigit()),
        );
        let parse_u32 = map_res(parse_delimited_hex, move |s: &str| {
            u32::from_str_radix(s, escape_digits_radix)
        });
        map_opt(parse_u32, |value| std::char::from_u32(value))(input)
    }
}

fn parse_unicode_hex_4(input: &str) -> IResult<&str, char> {
    char_numerical_escape("u", 4, 4, 16)(input)
}

fn parse_unicode_hex_8(input: &str) -> IResult<&str, char> {
    char_numerical_escape("U", 8, 8, 16)(input)
}

fn parse_char_hex_2(input: &str) -> IResult<&str, char> {
    char_numerical_escape("x", 1, 2, 16)(input)
}

fn parse_char_octal_3(input: &str) -> IResult<&str, char> {
    char_numerical_escape("", 1, 3, 8)(input)
}

fn parse_escaped_char(input: &str) -> IResult<&str, char> {
    preceded(
        char('\\'),
        alt((
            parse_unicode_hex_8,
            parse_unicode_hex_4,
            parse_char_hex_2,
            parse_char_octal_3,
            // The `value` parser returns a fixed value (the first argument) if its
            // parser (the second argument) succeeds. In these cases, it looks for
            // the marker characters (n, r, t, etc) and returns the matching
            // character (\n, \r, \t, etc).
            value('\u{0A}', char('n')),
            value('\u{09}', char('v')),
            value('\u{0C}', char('f')),
            value('\r', char('r')),
            value('\u{1B}', char('e')),
            value('\u{3F}', char('?')),
            value('\\', char('\\')),
            value('\u{07}', char('a')),
            value('\u{08}', char('b')),
            value('\t', char('t')),
            value('"', char('"')),
            value('\'', char('\'')),
            value('/', char('/')),
        )),
    )(input)
}

fn parse_single_char(input: &str) -> IResult<&str, char> {
    alt((parse_escaped_char, none_of("\\\"")))(input)
}

fn string(input: &str) -> IResult<&str, String> {
    let build_string = fold_many0(
        // Consumes the next logical character
        parse_single_char,
        String::new(),
        // Our folding function. For each fragment, append the fragment to the
        // string.
        |mut string, c| {
            string.push(c);
            string
        },
    );

    // Finally, parse the string. Note that, if `build_string` could accept a raw
    // " character, the closing delimiter " would never match. When using
    // `delimited` with a looping parser (like fold_many0), be sure that the
    // loop won't accidentally match your closing delimiter!
    delimited(char('"'), build_string, char('"'))(input)
}

#[cfg(test)]
mod tests {
    use crate::search::parser::rules::*;
    use nom::combinator::{complete, eof};
    use nom::error::ErrorKind;

    #[test]
    fn test2() {
        let input = "hello";
        let result = identifier(input);
        let expected = "hello";

        println!("{:?}", identifier("customer_name"));
    }

    #[test]
    fn test_identifier() {
        let input = "customer_name";
        assert_eq!(Ok(("", input)), identifier(input));

        let input = "_UnderSc013VariableName__123_";
        assert_eq!(Ok(("", input)), identifier(input));

        let input = "1variable1";
        assert!(identifier(input).is_err());
    }

    #[test]
    fn test_string() {
        let input = "\"\"";
        assert_eq!(Ok(("", input.trim_matches('\"').to_owned())), string(input));
    }
}
