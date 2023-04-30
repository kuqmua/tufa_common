pub(crate) fn stringified_lines_error_vec(stringified_vec: impl std::fmt::Display) -> String {
    format!("[\n{}]", stringified_vec)
}

pub(crate) fn stringified_lines_error_hashmap_element(
    key: impl std::fmt::Display, //todo - it can possibly contains more than one line
    value: impl std::fmt::Display,
) -> String {
    format!("{}: [\n{}]\n", key, lines_space_backslash_addition(value),)
}

pub(crate) fn lines_space_backslash_addition(value: impl std::fmt::Display) -> String {
    value
        .to_string()
        .lines()
        .fold(String::from(""), |mut acc, line| {
            acc.push_str(&format!(" {}\n", line));
            acc
        })
}

pub(crate) fn source_and_code_occurence_formatter(
    stringified_source: impl std::fmt::Display,
    stringified_code_occurence: impl std::fmt::Display,
) -> String {
    format!("{}\n{}", stringified_source, stringified_code_occurence)
}

pub(crate) fn error_occurence_hashmap_formatter(
    inner: impl std::fmt::Display,
) -> String {
    format!("{{\n{}}}", lines_space_backslash_addition(inner))
}