#[macro_export]
macro_rules! regex_parser {
    ($re:expr; $test:expr => $($match:ident),+) => {
        use lazy_static::lazy_static;
        use regex::Regex;

        lazy_static! {
            static ref RE: Regex = Regex::new($re).unwrap();
        }

        let captures = RE.captures($test).expect(format!(r#"String "{}" does not match regex!"#, $test).as_str());

        let mut iter = captures.iter();
        iter.next();
        $(
            let $match = iter.next().flatten().expect("Could not locate match in regex!").as_str();
        )+
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn digit_regex() {
        regex_parser!(r"(\d+)-(\d+)"; "1-2" => a, b);
        assert_eq!(a, "1");
        assert_eq!(b, "2");
    }

    #[test]
    fn word_regex() {
        regex_parser!(r"(\w+) (\w+)"; "hello world" => a, b);
        assert_eq!(a, "hello");
        assert_eq!(b, "world");
    }

    #[test]
    fn aoc_parse() {
        let lines = r"Position: x=123, y=456: Ends at 12
Position: x=-1, y=24: Ends at -23
Position: x=-1.5, y=24.123123: Ends at -12
Position: x=-1123, y=09123: Ends at 0";
        struct Thing {
            x: f32,
            y: f32,
            end: i32,
        }

        let parsed = lines
            .lines()
            .map(|line| {
                regex_parser!(r"Position: x=(.+), y=(.+): Ends at (.+)"; line => x, y, end);
                Thing {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                    end: end.parse().unwrap(),
                }
            })
            .collect::<Vec<_>>();

        // write all the asserts

        assert_eq!(parsed[0].x, 123.0);
        assert_eq!(parsed[0].y, 456.0);
        assert_eq!(parsed[0].end, 12);

        assert_eq!(parsed[1].x, -1.0);
        assert_eq!(parsed[1].y, 24.0);
        assert_eq!(parsed[1].end, -23);

        assert_eq!(parsed[2].x, -1.5);
        assert_eq!(parsed[2].y, 24.123123);
        assert_eq!(parsed[2].end, -12);

        assert_eq!(parsed[3].x, -1123.0);
        assert_eq!(parsed[3].y, 9123.0);
        assert_eq!(parsed[3].end, 0);
    }

    #[test]
    #[should_panic(expected = "Could not locate match in regex!")]
    fn should_fail() {
        regex_parser!(r"(\d+)-(\d+)"; "1-2" => _a, _b, _c);
    }

    #[test]
    #[should_panic(expected = r#"String "1?-2" does not match regex!"#)]
    fn should_fail2() {
        regex_parser!(r"(\d+)-(\d+)"; "1?-2" => _a);
    }
}
