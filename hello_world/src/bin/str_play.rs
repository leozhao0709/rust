use unicode_segmentation::UnicodeSegmentation;

pub fn main() {
    // let s1 = "🥹☹️🥳😒";
    // let s2 = &s1[..];

    // println!("s1 is {s1}");
    // println!("s2 is {s2}");

    // let s3 = "👨‍👩‍👧‍👦the🦀";
    // // for ch in s3.chars() {
    // //     println!("{ch}")
    // // }

    // for ch in s3.graphemes(true) {
    //     println!("{ch}")
    // }

    let mut s = String::from("Hello");
    let s_ref = &s;
    let len = cal_len(s_ref);

    let s_mut = &mut s;
    modify_s(s_mut);
    println!("len = {len}");
    // println!("s_ref = {}", s_ref);
    println!("s = {}", s);

    let s = String::from("你好");
    println!("s length = {}, s[0..3] = {}", s.len(), &s[0..3]);
}

fn cal_len(s: &str) -> usize {
    s.len()
}

fn modify_s(s: &mut String) {
    s.push_str(" world!");
}
