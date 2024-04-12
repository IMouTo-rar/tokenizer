extern crate markup5ever_rcdom as rcdom;
extern crate xml5ever;

use std::io;

use rcdom::{Handle, NodeData, RcDom};
use xml5ever::driver::parse_document;
use xml5ever::tendril::TendrilSink;
use soup::Soup;

fn walk(prefix: &str, handle: &Handle) {
    let node = handle;

    print!("{}", prefix);
    match &node.data {
        NodeData::Document => {
            println!("#document");
        },

        NodeData::Text { ref contents } => {
            println!("#text {}", contents.borrow().escape_default());
        },

        NodeData::Element { ref name, attrs, .. } => {
            print!("{} ", name.local);
            for attr in attrs.borrow().to_vec(){
                print!("-{}: {} ", attr.name.local, attr.value.escape_default())
            }
            println!("");
        },

        _ => {},
    }

    let new_indent = {
        let mut temp = String::new();
        temp.push_str(prefix);
        temp.push_str("    ");
        temp
    };

    for child in node
        .children
        .borrow()
        .iter()
        .filter(|child| matches!(child.data, NodeData::Text { .. } | NodeData::Element { .. }))
    {
        walk(&new_indent, child);
    }
}

#[allow(dead_code)]
pub fn tokenizer_json(html: &str) {
    let preprocess = "<root>".to_string() + html + "</root>";
    let preprocess = preprocess.as_bytes();
    let mut preprocess = io::BufReader::new(preprocess);

    // To parse XML into a tree form, we need a TreeSink
    // luckily xml5ever comes with a static RC backed tree represetation.
    let dom: RcDom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut preprocess)
        .unwrap();

    // Execute our visualizer on RcDom
    walk("", &dom.document);
}

#[allow(dead_code)]
pub fn tokenizer_raw(html : &str) -> String{
    let soup = Soup::new(html);
    soup.text()
}