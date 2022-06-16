pub(crate) struct Comment(pub Vec<String>);

impl Comment {
    pub fn append_with_indent(&self, indent_level: u8, buf: &mut String) {
        for line in &self.0 {
            for _ in 0..indent_level {
                buf.push_str("    ");
            }
            buf.push_str("/// ");
            // TODO prost sanitizes comments first. Should we do this here as well?
            buf.push_str(line);
            buf.push('\n');
        }
    }
}
