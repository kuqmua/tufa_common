pub(crate) fn stringified_lines_error_vec(stringified_vec: String) -> String {
    format!("[\n{}]", stringified_vec)
}

pub(crate) fn stringified_lines_error_hashmap_element(
    key: impl std::fmt::Display,
    value: String,
) -> String {
    format!("{} [\n{}]\n", key, lines_space_backslash_addition(value),)
}

pub(crate) fn lines_space_backslash_addition(value: String) -> String {
    value.lines().fold(String::from(""), |mut acc, line| {
        acc.push_str(&format!(" {}\n", line));
        acc
    })
}

pub(crate) fn source_and_code_occurence_formatter(
    stringified_source: String,
    stringified_code_occurence: String,
) -> String {
    format!("{}\n{}", stringified_source, stringified_code_occurence)
}
