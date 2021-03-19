use lspower::lsp::Position;
use std::convert::TryFrom;
use tree_sitter::Point;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Column {
    offset: u32,
    char: char,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Index {
    lines: im::Vector<im::OrdMap<usize, Column>>,
}

// preserves possible trailing newline compared to stdlib lines method on str
fn get_lines(string: &str) -> impl Iterator<Item = &str> {
    string.split('\n').map(|line| {
        let mut chars = line.chars();
        match chars.next_back() {
            Some('\r') => chars.as_str(),
            _ => line,
        }
    })
}

fn calc_offset(prev_utf8_end: usize, prev_utf16_end: u32, byte: usize) -> u32 {
    prev_utf16_end
        .saturating_add(u32::try_from(byte.checked_sub(prev_utf8_end).unwrap()).unwrap_or(u32::MAX))
}

impl Index {
    pub fn new(source: &str) -> Self {
        let mut lines = im::Vector::new();
        // always loops at least once
        for line in get_lines(source) {
            let mut columns = im::OrdMap::new();
            let mut prev_utf8_end: usize = 0;
            let mut prev_utf16_end: u32 = 0;
            for (byte, char) in line.char_indices() {
                let len_utf8 = char.len_utf8();
                let len_utf16 = char.len_utf16();
                if len_utf8 != len_utf16 {
                    let offset = calc_offset(prev_utf8_end, prev_utf16_end, byte);
                    columns.insert(byte, Column { offset, char });
                    prev_utf8_end = byte.saturating_add(len_utf8);
                    prev_utf16_end = offset.saturating_add(u32::try_from(len_utf16).unwrap());
                }
            }
            let len = line.len();
            columns.insert(
                len,
                Column {
                    offset: calc_offset(prev_utf8_end, prev_utf16_end, len),
                    char: '\n',
                },
            );
            lines.push_back(columns);
            // guarantee that every index fits into u32
            if let Err(_) = u32::try_from(lines.len()) {
                break;
            }
        }
        Self { lines }
    }

    pub fn to_lsp(&self, point: Point) -> Position {
        let Point { row, column } = point;
        match self.lines.get(row) {
            Some(columns) => {
                Position {
                    // every index in lines is guaranteed to be at most u32::MAX
                    line: u32::try_from(row).unwrap(),
                    character: match columns.get_prev(&column) {
                        Some((byte, Column { offset, char })) => {
                            let end = byte.saturating_add(char.len_utf8());
                            offset.saturating_add(if column < end || *char == '\n' {
                                0
                            } else {
                                u32::try_from(
                                    char.len_utf16()
                                        .saturating_add(column.checked_sub(end).unwrap()),
                                )
                                .unwrap_or(u32::MAX)
                            })
                        }
                        None => u32::try_from(column).unwrap_or(u32::MAX),
                    },
                }
            }
            // lines is guaranteed to be nonempty
            None => Position {
                // every index in lines is guaranteed to be at most u32::MAX
                line: u32::try_from(self.lines.len().checked_sub(1).unwrap()).unwrap(),
                character: {
                    // guaranteed to have at least one column, max, holding the line length
                    let (_, Column { offset, .. }) = self.lines.back().unwrap().get_max().unwrap();
                    *offset
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_lines_trailing() {
        let text = "foo\r\nbar\n\nbaz\n";
        let mut lines = get_lines(text);

        assert_eq!(Some("foo"), lines.next());
        assert_eq!(Some("bar"), lines.next());
        assert_eq!(Some(""), lines.next());
        assert_eq!(Some("baz"), lines.next());
        assert_eq!(Some(""), lines.next()); // different from stdlib lines method on str

        assert_eq!(None, lines.next());
    }

    #[test]
    fn test_lines_no_trailing() {
        let text = "foo\nbar\n\r\nbaz";
        let mut lines = get_lines(text);

        assert_eq!(Some("foo"), lines.next());
        assert_eq!(Some("bar"), lines.next());
        assert_eq!(Some(""), lines.next());
        assert_eq!(Some("baz"), lines.next());

        assert_eq!(None, lines.next());
    }

    #[test]
    fn test_empty_string() {
        let source = "";
        let index = Index::new(source);
        assert_eq!(index.lines.len(), 1);

        assert_eq!(index.to_lsp(Point::new(0, 0)), Position::new(0, 0));
        assert_eq!(index.to_lsp(Point::new(0, 1)), Position::new(0, 0));
        assert_eq!(index.to_lsp(Point::new(0, 2)), Position::new(0, 0));
        assert_eq!(index.to_lsp(Point::new(0, usize::MAX)), Position::new(0, 0));

        assert_eq!(index.to_lsp(Point::new(1, 0)), Position::new(0, 0));
        assert_eq!(index.to_lsp(Point::new(1, 1)), Position::new(0, 0));
        assert_eq!(index.to_lsp(Point::new(1, 2)), Position::new(0, 0));
        assert_eq!(index.to_lsp(Point::new(1, usize::MAX)), Position::new(0, 0));
    }

    #[test]
    fn test_lsp_spec_example() {
        let source = "a𐐀b";
        let index = Index::new(source);
        assert_eq!(index.lines.len(), 1);

        assert_eq!(index.to_lsp(Point::new(0, 0)), Position::new(0, 0));
        assert_eq!(index.to_lsp(Point::new(0, 1)), Position::new(0, 1));
        assert_eq!(index.to_lsp(Point::new(0, 2)), Position::new(0, 1));
        assert_eq!(index.to_lsp(Point::new(0, 3)), Position::new(0, 1));
        assert_eq!(index.to_lsp(Point::new(0, 4)), Position::new(0, 1));
        assert_eq!(index.to_lsp(Point::new(0, 5)), Position::new(0, 3));
        assert_eq!(index.to_lsp(Point::new(0, 6)), Position::new(0, 4));
        assert_eq!(index.to_lsp(Point::new(0, 7)), Position::new(0, 4));
        assert_eq!(index.to_lsp(Point::new(0, 8)), Position::new(0, 4));
        assert_eq!(index.to_lsp(Point::new(0, usize::MAX)), Position::new(0, 4));

        assert_eq!(index.to_lsp(Point::new(1, 0)), Position::new(0, 4));
        assert_eq!(index.to_lsp(Point::new(1, 1)), Position::new(0, 4));
        assert_eq!(index.to_lsp(Point::new(1, 2)), Position::new(0, 4));
        assert_eq!(index.to_lsp(Point::new(1, usize::MAX)), Position::new(0, 4));

        assert_eq!(index.to_lsp(Point::new(2, 0)), Position::new(0, 4));
        assert_eq!(index.to_lsp(Point::new(2, 1)), Position::new(0, 4));
        assert_eq!(index.to_lsp(Point::new(2, 2)), Position::new(0, 4));
        assert_eq!(index.to_lsp(Point::new(2, usize::MAX)), Position::new(0, 4));
    }

    #[test]
    fn test_hello_world_multiline() {
        let source = "hello\nworld\n";
        let index = Index::new(source);
        assert_eq!(index.lines.len(), 3);

        assert_eq!(index.to_lsp(Point::new(0, 0)), Position::new(0, 0));
        assert_eq!(index.to_lsp(Point::new(0, 1)), Position::new(0, 1));
        assert_eq!(index.to_lsp(Point::new(0, 2)), Position::new(0, 2));
        assert_eq!(index.to_lsp(Point::new(0, 3)), Position::new(0, 3));
        assert_eq!(index.to_lsp(Point::new(0, 4)), Position::new(0, 4));
        assert_eq!(index.to_lsp(Point::new(0, 5)), Position::new(0, 5));
        assert_eq!(index.to_lsp(Point::new(0, 6)), Position::new(0, 5));
        assert_eq!(index.to_lsp(Point::new(0, 7)), Position::new(0, 5));
        assert_eq!(index.to_lsp(Point::new(0, usize::MAX)), Position::new(0, 5));

        assert_eq!(index.to_lsp(Point::new(1, 0)), Position::new(1, 0));
        assert_eq!(index.to_lsp(Point::new(1, 1)), Position::new(1, 1));
        assert_eq!(index.to_lsp(Point::new(1, 2)), Position::new(1, 2));
        assert_eq!(index.to_lsp(Point::new(1, 3)), Position::new(1, 3));
        assert_eq!(index.to_lsp(Point::new(1, 4)), Position::new(1, 4));
        assert_eq!(index.to_lsp(Point::new(1, 5)), Position::new(1, 5));
        assert_eq!(index.to_lsp(Point::new(1, 6)), Position::new(1, 5));
        assert_eq!(index.to_lsp(Point::new(1, 7)), Position::new(1, 5));
        assert_eq!(index.to_lsp(Point::new(1, usize::MAX)), Position::new(1, 5));

        assert_eq!(index.to_lsp(Point::new(2, 0)), Position::new(2, 0));
        assert_eq!(index.to_lsp(Point::new(2, 1)), Position::new(2, 0));
        assert_eq!(index.to_lsp(Point::new(2, 2)), Position::new(2, 0));
        assert_eq!(index.to_lsp(Point::new(2, usize::MAX)), Position::new(2, 0));

        assert_eq!(index.to_lsp(Point::new(3, 0)), Position::new(2, 0));
        assert_eq!(index.to_lsp(Point::new(3, 1)), Position::new(2, 0));
        assert_eq!(index.to_lsp(Point::new(3, 2)), Position::new(2, 0));
        assert_eq!(index.to_lsp(Point::new(3, usize::MAX)), Position::new(2, 0));
    }
}
