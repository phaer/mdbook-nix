mod nix_repl;

use std::collections::HashMap;
use std::fmt::Display;
use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{Event, Parser, Tag, CodeBlockKind};
use pulldown_cmark_to_cmark::cmark;
use anyhow::{Result, anyhow};
use std::fmt;


pub struct Nix;

impl Nix {
    pub fn new() -> Nix {
        Nix
    }
}


#[derive(Debug)]
pub struct PreprocessErrors {
    message: String,
    errors: Vec<anyhow::Error>,
}

impl Display for PreprocessErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Err(res) = write!(f, "{}", self.message) {
            return Err(res)
        }
        for error in self.errors.iter() {
            if let Err(res) = write!(f, "{}", error) {
                return Err(res)
            }
        }
        return Ok(())
    }
}

impl std::error::Error for PreprocessErrors {}


#[derive(Debug, Clone)]
struct CodeBlockInfo {
    language: String,
    attributes: HashMap<String, String>
}

impl CodeBlockInfo {
    fn new(input: &str, count: u32) -> Option<Self> {
        if input.is_empty() {
            None
        } else {
            let mut attributes: HashMap<String, String> = HashMap::new();
            let mut tokens = input.split(' ').into_iter();
            let language = if let Some(language) = tokens.next() {
               language
            } else {
                input
            }.to_string();
            for attr_token in tokens {
                let mut tokens = attr_token.split('=');
                if let Some(name) = tokens.next() {
                    attributes.insert(name.into(), tokens.next().unwrap_or("").into());
                }
            }
            if !attributes.contains_key("name") {
                attributes.insert("name".into(), format!("block-{}", count));
            }
            Some(CodeBlockInfo {language, attributes})
      }
    }
}

impl fmt::Display for CodeBlockInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, "{} {}",
            self.language,
            self.attributes
                .iter()
                .map(|(k,v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

fn eval_nix_blocks(
    input: &str
) -> Result<String> {
    let mut buf = String::with_capacity(input.len());

    let mut repl = nix_repl::Repl::new()?;

    let mut events = vec![];
    let mut block_counter = 0;
    let mut block: Option<CodeBlockInfo> = None;

    for event in Parser::new(input).into_iter() {
        let mut new = match event.clone() {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(s))) if s.starts_with("nix") => {
                block_counter += 1;
                block = CodeBlockInfo::new(&s, block_counter);
                vec![
                    Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(block.clone().unwrap().to_string().into())))
                ]
            }
            Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(s))) if s.starts_with("nix") => {
                block = None;
                vec![event]
            },
            Event::Text(s) => {
                if let Some(block) = block.clone() {
                    eprintln!("{:#?}, {}", block, block.to_string());
                    vec![
                        Event::Text(s.clone()),
                        Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(block.to_string().into()))),
                        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced("result".into()))),
                        Event::Text(repl.eval(&s)?.into()),
                        Event::End(Tag::CodeBlock(CodeBlockKind::Fenced("result".into()))),
                    ]
                } else {
                    vec![event]
                }

            },
            _ => vec![event]
        };
        events.append(&mut new)
    }

    cmark(events.iter(), &mut buf).map(|_| buf).map_err(|err| {
        anyhow!("Markdown serialization failed: {}", err)
    })
}


impl Preprocessor for Nix {
    fn name(&self) -> &str {
        "nix-preprocessor"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        // In testing we want to tell the preprocessor to blow up by setting a
        // particular config value
        if let Some(nix_cfg) = ctx.config.get_preprocessor(self.name()) {
            if nix_cfg.contains_key("blow-up") {
                anyhow::bail!("Boom!!1!");
            }
        }

        let mut errors = vec![];
        book.for_each_mut(|item| {
            match item {
                BookItem::Chapter(chapter) => {
                    match eval_nix_blocks(&chapter.content) {
                        Ok(result) => chapter.content = result,
                        Err(e) => errors.push(e),
                    }
                },
                BookItem::Separator => (),
                BookItem::PartTitle(_) => (),
            }
        });

        if errors.len() == 0 {
            Ok(book)
        } else {
            Err(anyhow!(PreprocessErrors {
                message: format!("While pre-processing {:#?}", ctx.config.get("book.title")),
                errors
            }))
        }
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;
    use std::matches;

    #[test]
    fn nix_preprocessor_run() {
        let input_chapter_1 = r##"
# mdbook-nix

```nix
1 + 2
```
"##;

        let input_json = json!([
            {
                "root": "/path/to/book",
                "config": {
                    "book": {
                        "authors": ["AUTHOR"],
                        "language": "en",
                        "multilingual": false,
                        "src": "src",
                        "title": "TITLE"
                    },
                    "preprocessor": {
                        "nix": {}
                    }
                },
                "renderer": "html",
                "mdbook_version": "0.4.21"
            },
            {
                "sections": [
                    {
                        "Chapter": {
                            "name": "Chapter 1",
                            "content": input_chapter_1,
                            "number": [1],
                            "sub_items": [],
                            "path": "chapter_1.md",
                            "source_path": "chapter_1.md",
                            "parent_names": []
                        }
                    }
                ],
                "__non_exhaustive": null
            }
        ]);
        let input_json = input_json.to_string();

        let (ctx, book) = mdbook::preprocess::CmdPreprocessor::parse_input(input_json.as_bytes()).unwrap();
        let result = Nix::new().run(&ctx, book);
        assert!(result.is_ok());
        let first = &result.unwrap().sections[0];
        assert!(matches!(first, BookItem::Chapter(_)));
        if let BookItem::Chapter(chapter) = first {
            assert!(chapter.content.contains("result\n3"));
        }

    }
}
