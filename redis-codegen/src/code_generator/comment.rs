pub(crate) struct Comment(pub Vec<String>);

pub enum CommentKind {
    Outer,
    InnerLine
}
impl Default for CommentKind {
    fn default() -> Self {
        Self::Outer
    }
}
impl Comment {
    pub fn append_with_indent(&self, indent_level: u8, buf: &mut String, kind: CommentKind) {
        for line in &self.0 {
            for _ in 0..indent_level {
                buf.push_str("    ");
            }
            match kind {
                CommentKind::Outer => buf.push_str("///"),
                CommentKind::InnerLine => buf.push_str("//!"),
            }

            if !line.is_empty() {
                buf.push(' ');
            }

            // TODO prost sanitizes comments first. Should we do this here as well?
            buf.push_str(line);
            buf.push('\n');
        }
    }
}
