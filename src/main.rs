use regex::Regex;

fn main() {
    let re = Regex::new("h*e+l*o").unwrap();
    assert!(re.is_match("hello"));
}
