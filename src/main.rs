use std::env;
use std::borrow::Cow;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", tokenizer_raw(&args[1]));
}

fn tokenizer_raw(html : & str) -> Cow<str>{
    // 使用正则表达式匹配所有 id 为 info 的 div 元素内容
    let tags_to_match = vec![
        "text",
        "background",
        "size",
        "mid-size",
        "h3-size",
        "b",
        "i",
        "u",
        "delete",
        "bullet",
        "order"
    ];
    let re_str = tags_to_match.iter().map(|&tag| format!(r"<(/|){}[^>]*>", tag)).collect::<Vec<String>>().join("|");
    let re = Regex::new(&re_str).unwrap();

    // 使用循环遍历匹配结果
    re.replace_all(html, "")
}