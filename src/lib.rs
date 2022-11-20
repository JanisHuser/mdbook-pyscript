use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Result;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CodeBlockKind::*, Event, Options, Parser, Tag};

pub struct Pyscript;

impl Preprocessor for Pyscript {
    fn name(&self) -> &str {
        "Python"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let mut res = None;
        book.for_each_mut(|item: &mut BookItem| {
            if let Some(Err(_)) = res {
                return;
            }

            if let BookItem::Chapter(ref mut chapter) = *item {
                res = Some(Pyscript::add_pyscript(chapter).map(|md| {
                    chapter.content = md;
                }));
            }
        });

        res.unwrap_or(Ok(())).map(|_| book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

fn escape_html(s: &str) -> String {
    let mut output = String::new();
    for c in s.chars() {
        match c {
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '&' => output.push_str("&amp;"),
            _ => output.push(c),
        }
    }
    output
}

fn add_pyscript(content: &str) -> Result<String> {
    let mut mermaid_content = String::new();
    let mut in_mermaid_block = false;

    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);

    let mut code_span = 0..0;

    let mut mermaid_blocks = vec![];

    let events = Parser::new_ext(content, opts);
    for (e, span) in events.into_offset_iter() {
        log::debug!("e={:?}, span={:?}", e, span);
        if let Event::Start(Tag::CodeBlock(Fenced(code))) = e.clone() {
            if &*code == "python" {
                in_mermaid_block = true;
                mermaid_content.clear();
            }
            continue;
        }

        if !in_mermaid_block {
            continue;
        }

        // We're in the code block. The text is what we want.
        if let Event::Text(_) = e {
            code_span = span;
            continue;
        }

        if let Event::End(Tag::CodeBlock(Fenced(code))) = e {
            assert_eq!(
                "python", &*code,
                "After an opening mermaid code block we expect it to close again"
            );
            in_mermaid_block = false;

            let python_content = &content[code_span.clone()];
            let python_content = escape_html(python_content);
            let python_code = format!(" <py-repl>{}</py-repl>\n\n", python_content);
            mermaid_blocks.push((span, python_code));
        }
    }

    let mut content = content.to_string();
    for (span, block) in mermaid_blocks.iter().rev() {
        let pre_content = &content[0..span.start];
        let post_content = &content[span.end..];
        content = format!("{}\n{}{}", pre_content, block, post_content);
    }
    Ok(content)
}

impl Mermaid {
    fn add_mermaid(chapter: &mut Chapter) -> Result<String> {
        add_mermaid(&chapter.content)
    }
}
