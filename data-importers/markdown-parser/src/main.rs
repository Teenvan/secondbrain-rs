use walkdir::WalkDir;
use pulldown_cmark::{Event, Parser, Options, Tag};
use std::fs;
use std::collections::HashMap;

pub struct MarkdownHeading {
    pub depth: usize,
    pub content: String,
}

fn try_parsing_md_head(input_line: &str) -> Option<MarkdownHeading> {
    let mut depth = 0;

    for character in input_line.chars() {
        match character {
            '#' => depth += 1,
            _ => break,
        }
    }

    // No heading found
    if depth == 0 {
       return  None;
    }

    Some(MarkdownHeading {
        depth, 
        content: input_line[depth..].trim().to_owned() // Get the heading after the ###...
    })
}
 

fn import_md_files(directory: &str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        if let Some(path) = entry.path().to_str(){
            if let Some(extension) = entry.path().extension(){
                if extension == "md" {
                    println!("{}", entry.path().display());
                    files.push(String::from(path));
                }
            }
            
        }
    } 
    files
}

fn convert_markdown_to_text(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    // Create the parser
    let parser = Parser::new_ext(&markdown, options);
    let mut tags_stack = Vec::new();
    let mut buffer = String::new();

    // For each event we push into buffer to produce the plain text version
    for event in parser {
        match event {
            Event::Text(content) => {
                if !tags_stack.iter().any(is_strikethrough) {
                    buffer.push_str(&content)
                }
            }
            Event::Start(tag) => {
                start_tag(&tag, &mut buffer, &mut tags_stack);
                tags_stack.push(tag);
            }

            Event::End(tag) => {
                tags_stack.pop();
                end_tag(&tag, &mut buffer, &tags_stack);
            }

            Event::Code(content) => buffer.push_str(&content),
            Event::SoftBreak => buffer.push(' '),
            _ => (),
        }
    }

    buffer.trim().to_string()
}

fn start_tag(tag: &Tag, buffer: &mut String, tags_stack: &mut Vec<Tag>) {
    match tag {
        Tag::Link(_, _, title) | Tag::Image(_, _, title) => buffer.push_str(&title),
        Tag::Item => {
            buffer.push('\n');
            let mut lists_stack = tags_stack
                .iter_mut()
                .filter_map(|tag| match tag {
                    Tag::List(nb) => Some(nb),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let prefix_tabs_count = lists_stack.len() - 1;
            for _ in 0..prefix_tabs_count {
                buffer.push('\t')
            }
            if let Some(Some(nb)) = lists_stack.last_mut() {
                buffer.push_str(&nb.to_string());
                buffer.push_str(". ");
                *nb += 1;
            } else {
                buffer.push_str("• ");
            }
        }
        Tag::Paragraph | Tag::CodeBlock(_) | Tag::Heading(_, _, _) => buffer.push('\n'),
        _ => (),
    }
}

fn end_tag(tag: &Tag, buffer: &mut String, tags_stack: &[Tag]) {
    match tag {
        Tag::Paragraph | Tag::Heading(_, _, _) => buffer.push('\n'),
        Tag::CodeBlock(_) => {
            if buffer.chars().last() != Some('\n') {
                buffer.push('\n');
            }
        }
        Tag::List(_) => {
            let is_sublist = tags_stack.iter().any(|tag| match tag {
                Tag::List(_) => true,
                _ => false,
            });
            if !is_sublist {
                buffer.push('\n')
            }
        }
        _ => (),
    }
}

fn is_strikethrough(tag: &Tag) -> bool {
    match tag {
        Tag::Strikethrough => return true,
        _ => return false,
    }
}

fn read_markdown(filename: &str) -> String {
    let markdown = fs::read_to_string(filename).expect("Unable to read file");
    let converted = convert_markdown_to_text(&markdown);
    return converted
}

// Extract all the text from a directory for input to the LLM
fn extract_text_from_markdown_directory(directory: &str) -> HashMap<String, String> {
    let map = HashMap::new();
    let files = import_md_files(directory);
    map
}

#[test]
fn test_import_md_files() {
    let directory = "/Users/nmadhukumar/Documents/Projects/second-brain/data-importers/markdown-parser/test-directory";
    let files = import_md_files(directory);
    assert_eq!(files.len(), 96);
    assert!(files[0].ends_with(".md"));       
}

#[test]
fn should_parse_headings() {
    // When
    let value = try_parsing_md_head("### Hello world");
    // Then
    let heading = value.unwrap();
    assert_eq!(heading.depth, 3);
    assert_eq!(heading.content, "Hello world");
}


fn main() {

}
