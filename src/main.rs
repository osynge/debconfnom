#[macro_use]
extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::{escaped, take_while, take_while_m_n},
    character::complete::not_line_ending,
    character::complete::{alphanumeric1 as alphanumeric, char, one_of},
    combinator::map_res,
    combinator::opt,
    error::{context, convert_error, ErrorKind, ParseError, VerboseError},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

fn get_test_data() -> String {
    String::from(
        "Template: man-db/auto-update
Type: boolean
Default: true
Description: for internal use; can be preseeded
",
    )
}

fn key_val_delimiter_parser(i: &str) -> IResult<&str, &str> {
    tag(": ")(i) // will consume bytes if the input begins with "abcd"
}

fn line_delimiter_parser(i: &str) -> IResult<&str, &str> {
    tag("\n")(i) // will consume bytes if the input begins with "abcd"
}

fn template_parser(i: &str) -> IResult<&str, &str> {
    tag("Template")(i) // will consume bytes if the input begins with "abcd"
}

fn type_parser(i: &str) -> IResult<&str, &str> {
    tag("Type")(i) // will consume bytes if the input begins with "abcd"
}

fn parser_default_key(i: &str) -> IResult<&str, &str> {
    tag("Default")(i) // will consume bytes if the input begins with "abcd"
}

fn value_type_select(i: &str) -> IResult<&str, &str> {
    tag("select")(i) // will consume bytes if the input begins with "abcd"
}

fn value_type_multiselect(i: &str) -> IResult<&str, &str> {
    tag("multiselect")(i) // will consume bytes if the input begins with "abcd"
}

fn value_type_string(i: &str) -> IResult<&str, &str> {
    tag("string")(i) // will consume bytes if the input begins with "abcd"
}

fn value_type_boolean(i: &str) -> IResult<&str, &str> {
    tag("boolean")(i) // will consume bytes if the input begins with "abcd"
}

fn value_type_note(i: &str) -> IResult<&str, &str> {
    tag("note")(i) // will consume bytes if the input begins with "abcd"
}

fn value_type_text(i: &str) -> IResult<&str, &str> {
    tag("text")(i) // will consume bytes if the input begins with "abcd"
}

fn value_type_password(i: &str) -> IResult<&str, &str> {
    tag("password")(i) // will consume bytes if the input begins with "abcd"
}

fn parse_str(i: &str) -> IResult<&str, &str> {
    escaped(alphanumeric, '\\', one_of("\"n\\"))(i)
}

fn keyval_parser(i: &str) -> IResult<&str, &str> {
    not_line_ending(i)
}

// nom::error::ErrorKind

fn template_line_parser(i: &str) -> IResult<&str, (&str, &str)> {
    let (i, (red, _, blue, _)) = tuple((
        template_parser,
        key_val_delimiter_parser,
        keyval_parser,
        line_delimiter_parser,
    ))(i)?;
    Ok((i, (red, blue)))
}

fn line_parser_type(i: &str) -> IResult<&str, (&str, &str)> {
    let (i, (red, _, blue, _)) = tuple((
        type_parser,
        key_val_delimiter_parser,
        alt((
            value_type_select,
            value_type_multiselect,
            value_type_string,
            value_type_boolean,
            value_type_note,
            value_type_text,
            value_type_password,
        )),
        line_delimiter_parser,
    ))(i)?;

    Ok((i, (red, blue)))
}

fn default_line_parser(i: &str) -> IResult<&str, Option<(&str, &str, &str, &str)>> {
    let red = opt(tuple((
        parser_default_key,
        key_val_delimiter_parser,
        keyval_parser,
        line_delimiter_parser,
    )))(i)?;
    Ok(red)
}

fn main() {
    assert_eq!(
        template_line_parser("Template: man-db/auto-update"),
        Ok((": man-db/auto-update", ("Template", "s")))
    );
    assert_eq!(type_parser("Type: boolean"), Ok((": boolean", "Type")));
    assert_eq!(keyval_parser(": boolean\nsss"), Ok(("sss", "boolean")));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_template_parser() {
        assert_eq!(
            template_parser("Template: man-db/auto-update"),
            Ok((": man-db/auto-update", "Template"))
        );
    }

    #[test]
    fn test_keyval_parser() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(keyval_parser("boolean\nsss"), Ok(("\nsss", "boolean")));
    }

    #[test]
    fn test_keyval_parser_with_hyphon() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(
            keyval_parser("man-db/auto-update\nsss"),
            Ok(("\nsss", "man-db/auto-update"))
        );
    }

    #[test]
    fn test_type_parser() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(type_parser("Type: boolean"), Ok((": boolean", "Type")));
    }

    #[test]
    fn test_template_line_parser() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(
            template_line_parser("Template: man-db/auto-update\n"),
            Ok(("", ("Template", "man-db/auto-update")))
        );
    }
    #[test]
    fn test_line_parser_type_select() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(
            line_parser_type("Type: select\n"),
            Ok(("", ("Type", "select")))
        );
    }

    #[test]
    fn test_line_parser_type_multiselect() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(
            line_parser_type("Type: multiselect\n"),
            Ok(("", ("Type", "multiselect")))
        );
    }

    #[test]
    fn test_line_parser_type_string() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(
            line_parser_type("Type: string\n"),
            Ok(("", ("Type", "string")))
        );
    }

    #[test]
    fn test_line_parser_type_boolean() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(
            line_parser_type("Type: boolean\n"),
            Ok(("", ("Type", "boolean")))
        );
    }

    #[test]
    fn test_line_parser_type_note() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(line_parser_type("Type: note\n"), Ok(("", ("Type", "note"))));
    }

    #[test]
    fn test_line_parser_type_text() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(line_parser_type("Type: text\n"), Ok(("", ("Type", "text"))));
    }

    #[test]
    fn test_line_parser_type_password() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(
            line_parser_type("Type: password\n"),
            Ok(("", ("Type", "password")))
        );
    }

    #[test]
    fn test_line_parser_default_line_parser_nome() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(
            default_line_parser("Type: password\n"),
            Ok(("Type: password\n", None))
        );
    }

    #[test]
    fn test_line_parser_default_line_parser_some() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(
            default_line_parser("Default: true\n"),
            Ok(("", Some(("Default", ": ", "true", "\n"))))
        );
    }

}
