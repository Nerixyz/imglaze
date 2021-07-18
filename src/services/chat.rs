/// Always returns the left half. If there is a space, returns `Some` in the right half, else `None` is returned.
pub fn opt_next_space(input: &str) -> (&str, Option<&str>) {
    if let Some(idx) = input.find(' ') {
        let (left, right) = input.split_at(idx);

        (left.trim(), Some(right.trim()))
    } else {
        (input, None)
    }
}
