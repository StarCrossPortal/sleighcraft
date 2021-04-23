pub(crate) fn valid_table_name(s: &str) -> bool {
    if s.len() == 0 {
        false
    } else {
        s.chars().nth(0).unwrap().is_alphabetic() && s.chars().all(valid_table_name_ch)
    }
}

fn valid_table_name_ch(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}