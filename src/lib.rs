use std::fs::read_to_string;
use std::io::ErrorKind;
use std::process::Command;

pub mod intcode;

const YEAR: u32 = 2019;

#[macro_export]
macro_rules! gen_test {
    ($test: ident, $func: ident, $input: expr, $res: expr) => {
        #[test]
        fn $test() {
            let input = include_str!(concat!("../../tests/", $input));
            assert_eq!(super::$func(input), $res);
        }
    };
}

pub fn get_input(day: i32) -> String {
    let day = format!("{}", day);
    let path = format!("./in/{}", day);
    match read_to_string(&path) {
        Ok(res) => res,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            load_input(&day, &path);
            read_to_string(path).unwrap()
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}

fn load_input(day: &str, path: &str){
    let cookies = match read_to_string("./.cookies"){
        Ok(s) => s,
        Err(e) => {
            panic!(
                "Could not read `.cookies` file {}.\n\
                In order to use autodownload feature create `.cookies` file\n\
                with content 'session=[YOUR_AOC_SESSION_VAR]'", e
            );
        }
    };
    let url = format!("https://adventofcode.com/{}/day/{}/input", YEAR, day);

    Command::new("curl")
        .args(&["--cookie", &cookies.trim(), &url, "-o", path])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
