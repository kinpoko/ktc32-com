pub fn strtol(s: String) -> (String, Option<i64>) {
    let mut num = 0;
    let chars = s.chars();
    let mut has_digits = false;
    let mut index = s.len();

    for (i, c) in chars.enumerate() {
        if c.is_ascii_digit() {
            has_digits = true;
            num = num * 10 + (c as i64 - '0' as i64);
        } else {
            index = i;
            break;
        }
    }
    if !has_digits {
        return (s, None);
    }
    let result = num;
    let (_, remainder) = s.split_at(index);
    (remainder.to_string(), Some(result))
}
