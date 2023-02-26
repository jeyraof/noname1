/// This code was borrowed from https://github.com/Xe/site/blob/main/src/post/frontmatter.rs
mod kind;

enum State {
    SearchForStart,
    ReadingMarker { count: usize, end: bool },
    ReadingFrontMatter { buf: String, line_start: bool },
    SkipNewline { end: bool },
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("EOF while parsing frontmatter")]
    EOF,
    #[error("Error parsing yaml: {0:?}")]
    Yaml(#[from] serde_yaml::Error),
}

pub fn get_frontmatter(input: &str) -> Result<kind::Frontmatter, Error> {
    let mut state = State::SearchForStart;

    let mut payload = None;
    // let offset;

    let mut chars = input.char_indices();
    'parse: loop {
        let (_, ch) = match chars.next() {
            Some(x) => x,
            None => return Err(Error::EOF)?,
        };
        match &mut state {
            State::SearchForStart => match ch {
                '-' => {
                    state = State::ReadingMarker {
                        count: 1,
                        end: false,
                    };
                }
                '\n' | '\t' | ' ' => {
                    // ignore whitespace
                }
                _ => {
                    panic!("Start of frontmatter not found");
                }
            },
            State::ReadingMarker { count, end } => match ch {
                '-' => {
                    *count += 1;
                    if *count == 3 {
                        state = State::SkipNewline { end: *end };
                    }
                }
                _ => {
                    panic!("Malformed frontmatter marker");
                }
            },
            State::SkipNewline { end } => match ch {
                '\n' => {
                    if *end {
                        // offset = idx + 1;
                        break 'parse;
                    } else {
                        state = State::ReadingFrontMatter {
                            buf: String::new(),
                            line_start: true,
                        };
                    }
                }
                _ => panic!("Expected newline, got {:?}", ch),
            },
            State::ReadingFrontMatter { buf, line_start } => match ch {
                '-' if *line_start => {
                    let mut state_temp = State::ReadingMarker {
                        count: 1,
                        end: true,
                    };
                    std::mem::swap(&mut state, &mut state_temp);
                    if let State::ReadingFrontMatter { buf, .. } = state_temp {
                        payload = Some(buf);
                    } else {
                        unreachable!();
                    }
                }
                ch => {
                    buf.push(ch);
                    *line_start = ch == '\n';
                }
            },
        }
    }

    // unwrap justification: option set in state machine, Rust can't statically analyze it
    let payload = payload.unwrap();
    let fm = serde_yaml::from_str(&payload)?;

    Ok(fm)
}
