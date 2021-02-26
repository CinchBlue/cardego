use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while_m_n;
use nom::character::complete::{alpha1, multispace0, one_of, space1};
use nom::character::complete::{alphanumeric1, char, none_of, space0};

use nom::combinator::{eof, map_opt, map_res, not, opt, recognize, value};

use nom::multi::{fold_many0, many0, many1, separated_list1};

use nom::sequence::{delimited, pair, preceded, tuple};
use nom::IResult;

pub fn identifier(input: &str) -> IResult<&str, String> {
    map_opt(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |value: &str| Some(value.to_owned()),
    )(input)
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
    alt((parse_escaped_char, none_of("\"")))(input)
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

fn name(input: &str) -> IResult<&str, String> {
    alt((string, identifier))(input)
}

fn integer_base10(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        opt(one_of("-+")),
        alt((
            recognize(pair(tag("0"), not(one_of("123456789")))),
            recognize(pair(one_of("123456789"), many0(one_of("0123456789")))),
        )),
    ))(input)
}

fn decimal_digits(input: &str) -> IResult<&str, &str> {
    recognize(many1(one_of("0123456789")))(input)
}

/// Shamelessly copied from nom's nom_recipes.md, and then modified to fit my
/// use case.
fn float(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        // Recongize the leading + or -.
        opt(one_of("-+")),
        alt((
            // Case one: .42
            recognize(tuple((
                char('.'),
                decimal_digits,
                opt(tuple((one_of("eE"), opt(one_of("+-")), decimal_digits))),
            ))),
            // Case two: 42e42 and 42.42e42
            recognize(tuple((
                decimal_digits,
                opt(preceded(char('.'), decimal_digits)),
                one_of("eE"),
                opt(one_of("+-")),
                decimal_digits,
            ))),
            // Case three: 42. and 42.42
            recognize(tuple((decimal_digits, char('.'), opt(decimal_digits)))),
        )),
    ))(input)
}

fn literal(input: &str) -> IResult<&str, super::ast::Literal> {
    use super::ast::Literal;

    // TODO: need to replace with real error type to catch unwrap.
    alt((
        map_opt(alt((identifier, string)), |value| {
            Some(Literal::String(value))
        }),
        map_opt(integer_base10, |value| {
            Some(Literal::Integer(value.parse::<i64>().unwrap()))
        }),
        map_opt(float, |value| {
            Some(Literal::Float(value.parse::<f64>().unwrap()))
        }),
    ))(input)
}

fn operator(input: &str) -> IResult<&str, super::ast::Operator> {
    use super::ast::Operator;

    alt((
        value(Operator::GreaterOrEqual, tag(">=")),
        value(Operator::LessOrEqual, tag("<=")),
        value(Operator::LikeMatch, tag(":")),
        value(Operator::Equal, tag("=")),
        value(Operator::GreaterThan, tag(">")),
        value(Operator::LessThan, tag("<")),
    ))(input)
}

fn predicate(input: &str) -> IResult<&str, super::ast::Predicate> {
    use super::ast::Predicate;

    let (i, name) = name(input)?;
    let (i, op) = operator(i)?;
    let (i, literal) = literal(i)?;

    Ok((i, Predicate { name, op, literal }))
}

fn and_expression_group(input: &str) -> IResult<&str, super::ast::AndExpressionGroup> {
    preceded(
        opt(space0),
        separated_list1(many1(one_of(" \t,")), predicate),
    )(input)
}

fn expression(input: &str) -> IResult<&str, super::ast::Expression> {
    separated_list1(
        recognize(tuple((space0, many1(one_of("\n;|\0"))))),
        and_expression_group,
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::search::parser::ast::{Literal, Operator, Predicate};
    use crate::search::parser::rules::*;

    #[test]
    fn test_identifier() {
        let input = "customer_name";
        assert_eq!(Ok(("", input.to_owned())), identifier(input));

        let input = "_UnderSc013VariableName__123_";
        assert_eq!(Ok(("", input.to_owned())), identifier(input));

        let input = "1variable1";
        assert!(identifier(input).is_err());
    }

    #[test]
    fn test_string() {
        let input = "\"\"";
        assert_eq!(Ok(("", input.trim_matches('\"').to_owned())), string(input));
    }

    #[test]
    fn test_integer_base10() {
        let input = "0";
        assert_eq!(Ok(("", input)), integer_base10(input));

        let input = "123985";
        assert_eq!(Ok(("", input)), integer_base10(input));

        let input = "-0";
        assert_eq!(Ok(("", input)), integer_base10(input));

        let input = "-99";
        assert_eq!(Ok(("", input)), integer_base10(input));

        let input = "s";
        assert!(integer_base10(input).is_err());

        let input = "0123";
        assert!(integer_base10(input).is_err());
    }

    #[test]
    fn test_float() {
        let input = "42";
        assert!(float(input).is_err());

        let input = "+42";
        assert!(float(input).is_err());

        let input = ".42";
        assert_eq!(Ok(("", input)), float(input));
        assert!(input.parse::<f64>().is_ok());

        let input = "42e12";
        assert_eq!(Ok(("", input)), float(input));
        assert!(input.parse::<f64>().is_ok());

        let input = "42.42e42";
        assert_eq!(Ok(("", input)), float(input));
        assert!(input.parse::<f64>().is_ok());

        let input = "42.";
        assert_eq!(Ok(("", input)), float(input));
        assert!(input.parse::<f64>().is_ok());

        let input = "42.42";
        assert_eq!(Ok(("", input)), float(input));
        assert!(input.parse::<f64>().is_ok());

        let input = "-.42";
        assert_eq!(Ok(("", input)), float(input));
        assert!(input.parse::<f64>().is_ok());

        let input = "+42e12";
        assert_eq!(Ok(("", input)), float(input));
        assert!(input.parse::<f64>().is_ok());

        let input = "-42.42e42";
        assert_eq!(Ok(("", input)), float(input));
        assert!(input.parse::<f64>().is_ok());

        let input = "+42.";
        assert_eq!(Ok(("", input)), float(input));
        assert!(input.parse::<f64>().is_ok());

        let input = "+42.42";
        assert_eq!(Ok(("", input)), float(input));
        assert!(input.parse::<f64>().is_ok());
    }

    #[test]
    fn test_predicate() {
        let input = "name:hello_there";
        assert_eq!(
            Ok((
                "",
                Predicate {
                    name: "name".to_owned(),
                    op: Operator::LikeMatch,
                    literal: Literal::String("hello_there".to_owned())
                }
            )),
            predicate(input),
        );

        let input = "\"customer_name\"=robot_sam123";
        assert_eq!(
            Ok((
                "",
                Predicate {
                    name: "customer_name".to_owned(),
                    op: Operator::Equal,
                    literal: Literal::String("robot_sam123".to_owned())
                }
            )),
            predicate(input),
        );

        let input = "power>=2";
        assert_eq!(
            Ok((
                "",
                Predicate {
                    name: "power".to_owned(),
                    op: Operator::GreaterOrEqual,
                    literal: Literal::Integer(2),
                }
            )),
            predicate(input),
        );

        let input = "\"dueño\"<\"Maria Carter Jiménez\"";
        assert_eq!(
            Ok((
                "",
                Predicate {
                    name: "dueño".to_owned(),
                    op: Operator::LessThan,
                    literal: Literal::String("Maria Carter Jiménez".to_owned()),
                }
            )),
            predicate(input),
        );

        let input = "\"社長\"=\"小林\"";
        assert_eq!(
            Ok((
                "",
                Predicate {
                    name: "社長".to_owned(),
                    op: Operator::Equal,
                    literal: Literal::String("小林".to_owned())
                }
            )),
            predicate(input),
        );
    }

    #[test]
    fn test_and_expression_group() {
        let input = "a=1 b=2 c=3";
        assert_eq!(
            Ok((
                "",
                vec![
                    Predicate {
                        name: "a".to_owned(),
                        op: Operator::Equal,
                        literal: Literal::Integer(1),
                    },
                    Predicate {
                        name: "b".to_owned(),
                        op: Operator::Equal,
                        literal: Literal::Integer(2),
                    },
                    Predicate {
                        name: "c".to_owned(),
                        op: Operator::Equal,
                        literal: Literal::Integer(3),
                    },
                ]
            )),
            and_expression_group(input)
        );

        let input = "name=Sword  power>=3 ,,,  power<=5 , initiative=0";
        assert_eq!(
            Ok((
                "",
                vec![
                    Predicate {
                        name: "name".to_owned(),
                        op: Operator::Equal,
                        literal: Literal::String("Sword".to_owned()),
                    },
                    Predicate {
                        name: "power".to_owned(),
                        op: Operator::GreaterOrEqual,
                        literal: Literal::Integer(3),
                    },
                    Predicate {
                        name: "power".to_owned(),
                        op: Operator::LessOrEqual,
                        literal: Literal::Integer(5),
                    },
                    Predicate {
                        name: "initiative".to_owned(),
                        op: Operator::Equal,
                        literal: Literal::Integer(0),
                    },
                ]
            )),
            and_expression_group(input)
        );
    }

    #[test]
    fn test_expression() {
        let input = "a=1";
        assert_eq!(
            Ok((
                "",
                vec![vec![Predicate {
                    name: "a".to_owned(),
                    op: Operator::Equal,
                    literal: Literal::Integer(1),
                }]]
            )),
            expression(input)
        );

        let input = "a=1 b=2 c=3 ; name=Sword  power>=3 ,,,  power<=5 , initiative=0";
        assert_eq!(
            Ok((
                "",
                vec![
                    vec![
                        Predicate {
                            name: "a".to_owned(),
                            op: Operator::Equal,
                            literal: Literal::Integer(1),
                        },
                        Predicate {
                            name: "b".to_owned(),
                            op: Operator::Equal,
                            literal: Literal::Integer(2),
                        },
                        Predicate {
                            name: "c".to_owned(),
                            op: Operator::Equal,
                            literal: Literal::Integer(3),
                        },
                    ],
                    vec![
                        Predicate {
                            name: "name".to_owned(),
                            op: Operator::Equal,
                            literal: Literal::String("Sword".to_owned()),
                        },
                        Predicate {
                            name: "power".to_owned(),
                            op: Operator::GreaterOrEqual,
                            literal: Literal::Integer(3),
                        },
                        Predicate {
                            name: "power".to_owned(),
                            op: Operator::LessOrEqual,
                            literal: Literal::Integer(5),
                        },
                        Predicate {
                            name: "initiative".to_owned(),
                            op: Operator::Equal,
                            literal: Literal::Integer(0),
                        },
                    ]
                ]
            )),
            expression(input)
        );

        let input = "a=1 b=2 c=3 \n\n name=Sword  power>=3 ,,,  power<=5 , initiative=0";
        assert_eq!(
            Ok((
                "",
                vec![
                    vec![
                        Predicate {
                            name: "a".to_owned(),
                            op: Operator::Equal,
                            literal: Literal::Integer(1),
                        },
                        Predicate {
                            name: "b".to_owned(),
                            op: Operator::Equal,
                            literal: Literal::Integer(2),
                        },
                        Predicate {
                            name: "c".to_owned(),
                            op: Operator::Equal,
                            literal: Literal::Integer(3),
                        },
                    ],
                    vec![
                        Predicate {
                            name: "name".to_owned(),
                            op: Operator::Equal,
                            literal: Literal::String("Sword".to_owned()),
                        },
                        Predicate {
                            name: "power".to_owned(),
                            op: Operator::GreaterOrEqual,
                            literal: Literal::Integer(3),
                        },
                        Predicate {
                            name: "power".to_owned(),
                            op: Operator::LessOrEqual,
                            literal: Literal::Integer(5),
                        },
                        Predicate {
                            name: "initiative".to_owned(),
                            op: Operator::Equal,
                            literal: Literal::Integer(0),
                        },
                    ]
                ]
            )),
            expression(input)
        );
    }
}
