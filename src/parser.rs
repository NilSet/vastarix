use nom::*;

named!(source_character<&str, &str>, take!(1));
named!(white_space<&str, char>, one_of!("\t\u{000B}\u{000C} \u{00A0}\u{FEFF}\u{1680}\u{2000}\u{2001}\u{2002}\u{2003}\u{2004}\u{2005}\u{2006}\u{2007}\u{2008}\u{2009}\u{200A}\u{202F}\u{205F}\u{3000}"));
named!(line_terminator<&str, char>, one_of!("\n\r\u{2028}\u{2029}"));
named!(line_terminator_sequence<&str, &str>, re_find!("\r\n|[\n\r\u{2028}\u{2029}]")); //i dislike this regex, be more mechanical like spec

named!(comment<&str, &str>, alt!(
        multi_line_comment |
        single_line_comment
        ));
named!(multi_line_comment<&str, &str>, recognize!(do_parse!(tag!("/*") >> opt!(multi_line_comment_chars) >> tag!("*/") >> ())));
named!(multi_line_comment_chars<&str, &str>, recognize!(alt!(
        do_parse!(multi_line_not_asterisk_char >> opt!(multi_line_comment_chars) >> ()) |
        do_parse!(tag!("*") >> opt!(post_asterisk_comment_chars) >> ())
        )));
named!(post_asterisk_comment_chars<&str, &str>, recognize!(alt!(
        do_parse!(multi_line_not_forward_slash_or_asterisk_char >> opt!(multi_line_comment_chars) >> ()) |
        do_parse!(tag!("*") >> opt!(post_asterisk_comment_chars) >> ())
        )));
named!(multi_line_not_asterisk_char<&str, &str>, verify!(source_character, |char| char != "*"));
named!(multi_line_not_forward_slash_or_asterisk_char<&str, &str>, verify!(source_character, |char| char != "*" && char != "/"));
named!(single_line_comment<&str, &str>, recognize!(do_parse!(tag!("//") >> opt!(single_line_comment_chars) >> ())));
named!(single_line_comment_chars<&str, &str>, recognize!(do_parse!(single_line_comment_char >> opt!(single_line_comment_chars) >> ())));
named!(single_line_comment_char<&str, &str>, recognize!(do_parse!(not!(line_terminator) >> source_character >> ())));

#[cfg(test)]
mod test {
    #[test]
    fn term() {
        assert_eq!(
            super::line_terminator_sequence("\r\nstuff\n"),
            Ok(("stuff\n", "\r\n"))
        );
        assert_eq!(
            super::line_terminator_sequence("\n\rstuff\n"),
            Ok(("\rstuff\n", "\n"))
        );
    }
    #[test]
    fn comment() {
        assert_eq!(
            super::comment("// dummy dum dum\n//another"),
            Ok(("\n//another", "// dummy dum dum"))
        );
        println!(
            "{:?}",
            super::multi_line_comment_chars(" */ code")
        );
        println!(
            "{:?}",
            super::multi_line_comment_chars(" **/ code")
        );
        assert_eq!(
            super::multi_line_comment("/* */foo"),
            Ok(("foo", "/* */"))
        );
        assert_eq!(
            super::multi_line_comment("/* **/foo"),
            Ok(("foo", "/* **/"))
        );
    }
}
