mod models;

use crate::models::output::Output;
use regex::Regex;
use crate::models::data::Data;

pub fn render_to_html(output: &String) -> String {
    let re = Regex::new(r"(\n)").unwrap();
    let output = re.replace_all(output, "").to_string();
    let deserialize_result: Output = serde_json::from_str(&output)
        .expect("deserialize error");
    let blocks = deserialize_result.blocks;
    let mut html = String::new();
    for block in blocks {
        let data = block.data;
        match data {
            Data::Header { text, level } => {
                let header_html = format!("<h{l}>{t}</h{l}>",
                                          l = level.to_string(),
                                          t = text
                );
                html.push_str(header_html.as_str());
            }
            Data::Paragraph { text } => {
                let paragraph_html = format!("<p>{}</p>", text);
                html.push_str(paragraph_html.as_str());
            }
        }
    }
    html
}