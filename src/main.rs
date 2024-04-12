use std::env;
use tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    tokenizer::tokenizer_json(&args[1])
}



#[test]
fn test() {
    let html = "<h1 style=\"1\" indent=\"0\">ni<h2>你好</h2>Hao</h1>Ni\nHao<h2>Ni Hao</h2>";
    tokenizer::tokenizer_json(html)
}