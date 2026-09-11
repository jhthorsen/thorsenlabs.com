use crate::template::basename_from_path;
use pulldown_cmark::{Event, Tag, TagEnd};
use serde::Serialize;
use std::collections::HashSet;
use std::{fs, path::Path, path::PathBuf};

#[derive(Hash, Eq, PartialEq)]
enum Section {
    Metadata,
    Ingress,
    Skip,
}

#[derive(Default, Serialize)]
pub struct Markdown {
    pub content: String,
    pub date: String,
    pub description: String,
    pub footer: String,
    pub header: String,
    pub id: String,
    pub ingress: String,
    pub path: PathBuf,
    pub scoped_css: String,
    pub status: String,
    pub title: String,
}

impl Markdown {
    pub fn new_from_path(path: &Path) -> Markdown {
        let basename = basename_from_path(Some(path));
        let basename = match basename.as_str() {
            "index.md" => basename_from_path(path.parent()),
            _ => basename,
        };

        Markdown {
            content: String::from(""),
            date: String::from(""),
            description: String::from(""),
            footer: String::from(""),
            header: String::from(""),
            id: basename.trim_end_matches(".md").to_owned(),
            ingress: String::from(""),
            path: path.to_path_buf(),
            scoped_css: String::from(""),
            status: String::from("draft"),
            title: basename.replace("-", " "),
        }
    }

    pub fn parse(&mut self, content: String) {
        let mut section = Section::Skip;
        let events = markdown_parser(&content.as_str()).collect::<Vec<_>>();
        let parser = heading_ids(events).into_iter();

        let parser = parser.map(|event| {
            match event {
                Event::Start(Tag::MetadataBlock(_)) => {
                    section = Section::Metadata;
                }
                Event::Start(Tag::Paragraph) => {
                    section = Section::Ingress;
                }
                Event::End(TagEnd::MetadataBlock(_)) => {
                    section = Section::Skip;
                }
                Event::End(TagEnd::Paragraph) => {
                    section = Section::Skip;
                }
                Event::SoftBreak => {
                    if section == Section::Ingress && self.ingress.len() < 256 {
                        self.ingress.push_str(" ");
                    }
                }
                Event::Text(ref text) => {
                    if section == Section::Metadata {
                        for line in text.lines() {
                            let kv = line.trim().splitn(2, ':').collect::<Vec<&str>>();
                            if kv.len() != 2 {
                                continue;
                            }

                            match kv[0] {
                                "date" => self.date = kv[1].trim().to_owned(),
                                "description" => self.description = kv[1].trim().to_owned(),
                                "footer" => self.footer = kv[1].trim().to_owned(),
                                "header" => self.header = kv[1].trim().to_owned(),
                                "id" => self.id = kv[1].trim().to_owned(),
                                "scoped_css" => self.scoped_css = kv[1].trim().to_owned(),
                                "status" => self.status = kv[1].trim().to_owned(),
                                "title" => self.title = kv[1].trim().to_owned(),
                                _ => {}
                            }
                        }
                    } else if section == Section::Ingress {
                        if self.ingress.len() < 256 {
                            self.ingress.push_str(&text);
                        } else if !self.ingress.ends_with(".") && !self.ingress.ends_with("?") {
                            self.ingress.push_str("...");
                        }
                    }
                }
                _ => {}
            };

            return event;
        });

        pulldown_cmark::html::push_html(&mut self.content, parser);
    }

    pub fn read(&mut self) -> bool {
        let file_content = fs::read_to_string(&self.path);
        if let Ok(file_content) = file_content {
            self.parse(file_content);
            return true;
        }

        return false;
    }
}

fn markdown_parser(text: &str) -> pulldown_cmark::Parser<'_> {
    let mut options = pulldown_cmark::Options::empty();
    options.insert(pulldown_cmark::Options::ENABLE_GFM);
    options.insert(pulldown_cmark::Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(pulldown_cmark::Options::ENABLE_MATH);
    options.insert(pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION);
    options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    options.insert(pulldown_cmark::Options::ENABLE_TABLES);
    options.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
    options.insert(pulldown_cmark::Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);

    pulldown_cmark::Parser::new_ext(&text, options)
}

fn heading_ids<'a>(events: Vec<Event<'a>>) -> Vec<Event<'a>> {
    let mut used_ids = events
        .iter()
        .filter_map(|event| match event {
            Event::Start(Tag::Heading { id: Some(id), .. }) => Some(id.to_string()),
            _ => None,
        })
        .collect::<HashSet<_>>();
    let mut generated_heading = 0;
    let mut output = Vec::with_capacity(events.len());

    for (index, event) in events.iter().enumerate() {
        let event = match event {
            Event::Start(Tag::Heading {
                level,
                id: None,
                classes,
                attrs,
            }) => {
                generated_heading += 1;
                let heading = heading_text(&events, index);
                let base = slugify_heading(&heading);
                let base = if base.is_empty() {
                    format!("heading-{generated_heading}")
                } else {
                    base
                };
                let mut id = base.clone();
                let mut suffix = 2;
                while used_ids.contains(&id) {
                    id = format!("{base}-{suffix}");
                    suffix += 1;
                }
                used_ids.insert(id.clone());

                Event::Start(Tag::Heading {
                    level: *level,
                    id: Some(id.into()),
                    classes: classes.clone(),
                    attrs: attrs.clone(),
                })
            }
            _ => event.clone(),
        };
        output.push(event);
    }

    output
}

fn heading_text<'a>(events: &[Event<'a>], start: usize) -> String {
    events[start + 1..]
        .iter()
        .take_while(|event| !matches!(event, Event::End(TagEnd::Heading(_))))
        .filter_map(|event| match event {
            Event::Text(text) | Event::Code(text) => Some(text.as_ref()),
            _ => None,
        })
        .collect::<String>()
}

fn slugify_heading(text: &str) -> String {
    let mut slug = String::new();
    let mut needs_separator = false;

    for character in text.chars() {
        if character.is_alphanumeric() {
            if needs_separator && !slug.is_empty() {
                slug.push('-');
            }
            slug.extend(character.to_lowercase());
            needs_separator = false;
        } else if !slug.is_empty() {
            needs_separator = true;
        }
    }

    slug
}

#[cfg(test)]
mod tests {
    use super::Markdown;
    use std::path::Path;

    #[test]
    fn adds_ids_to_markdown_headings() {
        let mut markdown = Markdown::new_from_path(Path::new("/dev/null"));
        markdown.parse("# Hello, world!\n\n## Hello, world!\n\n### Explicit {#custom}".into());

        assert!(markdown.content.contains("<h1 id=\"hello-world\">"));
        assert!(markdown.content.contains("<h2 id=\"hello-world-2\">"));
        assert!(markdown.content.contains("<h3 id=\"custom\">"));
    }
}
