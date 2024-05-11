use regex::Regex;

fn main() {
    let name_pattern = r"(?<First>[A-Z]{1}[a-z]{2,8})\|([A-Z]{1}[a-z]{2,8})\|([A-Z]{1}[a-z]{2,8})";
    let input_text = "Taylor|Lei|Sylvia";

    // test match
    let name_regex = Regex::new(name_pattern).expect("Pattern error");
    println!("Name is matching: {}", name_regex.is_match(input_text));

    // find one match
    let match_result = name_regex.find(input_text).expect("cannot find with regex");
    println!("Who went to store? {}", match_result.as_str());

    // find all match
    let all_captures = name_regex.captures(input_text).expect("cannot capture");
    all_captures.iter().enumerate().for_each(|(i, capture)| {
        println!(
            "capture {i}: {:#?}",
            capture.expect("capture error").as_str()
        );
    });

    // capture with name
    let first = all_captures.name("First").expect("capture error").as_str();
    println!("first capture is {}", first);
}
