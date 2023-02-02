pub(crate) fn stringified_lines_error_hashmap_element(
    key: impl std::fmt::Display,
    value: String,
) -> String {
    format!(
        "{} [\n{}]\n",
        key,
        value.lines().fold(String::from(""), |mut acc, line| {
            acc.push_str(&format!(" {}\n", line));
            acc
        })
    )
}

pub(crate) fn stringified_lines_error_vec_element(vec_element: String) -> String {
    vec_element
        .lines()
        .fold(String::from(""), |mut acc, vec_element| {
            acc.push_str(&format!(" {}\n", vec_element));
            acc
        })
}

pub(crate) fn stringified_lines_error_vec(stringified_vec: String) -> String {
    format!("[\n{}]", stringified_vec)
}
