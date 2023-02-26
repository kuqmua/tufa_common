pub fn three<'a>() -> Result<
    (),
    Box<crate::repositories_types::three_four_five_six_seven_eight::ThreeWrapperError<'a>>,
> {
    if let Err(e) = four() {
        let f = crate::repositories_types::three_four_five_six_seven_eight::ThreeWrapperError::Something {
            inner_error: *e,
            code_occurence: crate::code_occurence_tufa_common!(),
        };
        return Err(Box::new(f));
    };
    Ok(())
}

pub fn three_with_deserialize<'a>() -> Result<(), Box<crate::repositories_types::three_four_five_six_seven_eight::ThreeWrapperErrorWithDeserialize<'a>>>{
    if let Err(e) = four_with_deserialize() {
        let f = crate::repositories_types::three_four_five_six_seven_eight::ThreeWrapperErrorWithDeserialize::Something {
            inner_error: *e,
            code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize::new(
                &crate::global_variables::compile_time::git_info::GIT_INFO,
                file!(),
                line!(),
                column!(),
            ),
        };
        return Err(Box::new(f));
    };
    Ok(())
}

pub fn four<'a>(
) -> Result<(), Box<crate::repositories_types::three_four_five_six_seven_eight::FourWrapperError<'a>>>
{
    match (five(), six()) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(f), Err(s)) => {
            let f = crate::repositories_types::three_four_five_six_seven_eight::FourWrapperError::Something {
                inner_errors: std::collections::HashMap::from([
                    (
                        String::from("five_hashmap_key"),
                        crate::repositories_types::three_four_five_six_seven_eight::FourWrapperErrorEnum::Five(*f),
                    ),
                    (
                        String::from("six_hashmap_key"),
                        crate::repositories_types::three_four_five_six_seven_eight::FourWrapperErrorEnum::Six(*s),
                    ),
                ]),
                code_occurence: crate::code_occurence_tufa_common!(),
            };
            return Err(Box::new(f));
        }
    }
}

pub fn four_with_deserialize<'a>() -> Result<
    (),
    Box<
        crate::repositories_types::three_four_five_six_seven_eight::FourWrapperErrorWithDeserialize<
            'a,
        >,
    >,
> {
    match (five_with_deserialize(), six_with_deserialize()) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(f), Err(s)) => {
            let f = crate::repositories_types::three_four_five_six_seven_eight::FourWrapperErrorWithDeserialize::Something {
                inner_errors: std::collections::HashMap::from([
                    (
                        String::from("five_hashmap_key"),
                        crate::repositories_types::three_four_five_six_seven_eight::FourWrapperErrorEnumWithDeserialize::Five(*f),
                    ),
                    (
                        String::from("six_hashmap_key"),
                        crate::repositories_types::three_four_five_six_seven_eight::FourWrapperErrorEnumWithDeserialize::Six(*s),
                    ),
                ]),
                code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize::new(
                    &crate::global_variables::compile_time::git_info::GIT_INFO,
                    file!(),
                    line!(),
                    column!(),
                ),
            };
            return Err(Box::new(f));
        }
    }
}

pub fn five<'a>(
) -> Result<(), Box<crate::repositories_types::three_four_five_six_seven_eight::FiveWrapperError<'a>>>
{
    if let Err(e) = five_one() {
        let f = crate::repositories_types::three_four_five_six_seven_eight::FiveWrapperError::Something {
            inner_errors: std::collections::HashMap::from([(
                String::from("five_one_hashmap_key"),
                crate::repositories_types::three_four_five_six_seven_eight::FiveWrapperErrorEnum::FiveOne(*e),
            )]),
            code_occurence: crate::code_occurence_tufa_common!(),
        };
        return Err(Box::new(f));
    }
    Ok(())
}

pub fn five_with_deserialize<'a>() -> Result<
    (),
    Box<
        crate::repositories_types::three_four_five_six_seven_eight::FiveWrapperErrorWithDeserialize<
            'a,
        >,
    >,
> {
    if let Err(e) = five_one_with_deserialize() {
        let f = crate::repositories_types::three_four_five_six_seven_eight::FiveWrapperErrorWithDeserialize::Something {
            inner_errors: std::collections::HashMap::from([(
                String::from("five_one_hashmap_key"),
                crate::repositories_types::three_four_five_six_seven_eight::FiveWrapperErrorEnumWithDeserialize::FiveOne(*e),
            )]),
            code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize::new(
                &crate::global_variables::compile_time::git_info::GIT_INFO,
                file!(),
                line!(),
                column!(),
            ),
        };
        return Err(Box::new(f));
    }
    Ok(())
}

pub fn five_one<'a>() -> Result<
    (),
    Box<crate::repositories_types::three_four_five_six_seven_eight::FiveOneOriginError<'a>>,
> {
    return Err(Box::new(
        crate::repositories_types::three_four_five_six_seven_eight::FiveOneOriginError::Something {
            error: String::from("five_one error"),
            code_occurence: crate::code_occurence_tufa_common!(),
        },
    ));
}

pub fn five_one_with_deserialize<'a>() -> Result<(), Box<crate::repositories_types::three_four_five_six_seven_eight::FiveOneOriginErrorWithDeserialize<'a>>>{
    return Err(Box::new(crate::repositories_types::three_four_five_six_seven_eight::FiveOneOriginErrorWithDeserialize::Something {
        error: String::from("five_one error"),
        code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize::new(
            &crate::global_variables::compile_time::git_info::GIT_INFO,
            file!(),
            line!(),
            column!(),
        ),
    }));
}

pub fn six<'a>(
) -> Result<(), Box<crate::repositories_types::three_four_five_six_seven_eight::SixWrapperError<'a>>>
{
    let thread_join_handle = std::thread::spawn(move || eight());
    let res = thread_join_handle.join().unwrap();
    match (seven(), res) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(seven_error), Err(eight_error)) => {
            let f = crate::repositories_types::three_four_five_six_seven_eight::SixWrapperError::Something {
                inner_errors: vec![
                    crate::repositories_types::three_four_five_six_seven_eight::SixWrapperErrorEnum::Seven(*seven_error),
                    crate::repositories_types::three_four_five_six_seven_eight::SixWrapperErrorEnum::Eight(*eight_error),
                ],
                code_occurence: crate::code_occurence_tufa_common!(),
            };
            return Err(Box::new(f));
        }
    }
}

pub fn six_with_deserialize<'a>() -> Result<
    (),
    Box<
        crate::repositories_types::three_four_five_six_seven_eight::SixWrapperErrorWithDeserialize<
            'a,
        >,
    >,
> {
    let thread_join_handle = std::thread::spawn(move || eight_with_deserialize());
    let res = thread_join_handle.join().unwrap();
    match (seven_with_deserialize(), res) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(seven_error), Err(eight_error)) => {
            let f = crate::repositories_types::three_four_five_six_seven_eight::SixWrapperErrorWithDeserialize::Something {
                inner_errors: vec![
                    crate::repositories_types::three_four_five_six_seven_eight::SixWrapperErrorEnumWithDeserialize::Seven(*seven_error),
                    crate::repositories_types::three_four_five_six_seven_eight::SixWrapperErrorEnumWithDeserialize::Eight(*eight_error),
                ],
                code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize::new(
                    &crate::global_variables::compile_time::git_info::GIT_INFO,
                    file!(),
                    line!(),
                    column!(),
                ),
            };
            return Err(Box::new(f));
        }
    }
}

pub fn seven<'a>(
) -> Result<(), Box<crate::repositories_types::three_four_five_six_seven_eight::SevenOriginError<'a>>>
{
    return Err(Box::new(
        crate::repositories_types::three_four_five_six_seven_eight::SevenOriginError::Something {
            error: String::from("error_eight"),
            code_occurence: crate::code_occurence_tufa_common!(),
        },
    ));
}

pub fn seven_with_deserialize<'a>() -> Result<
    (),
    Box<
        crate::repositories_types::three_four_five_six_seven_eight::SevenOriginErrorWithDeserialize<
            'a,
        >,
    >,
> {
    return Err(Box::new(crate::repositories_types::three_four_five_six_seven_eight::SevenOriginErrorWithDeserialize::Something {
        error: String::from("error_eight"),
        code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize::new(
            &crate::global_variables::compile_time::git_info::GIT_INFO,
            file!(),
            line!(),
            column!(),
        ),
    }));
}

pub fn eight<'a>(
) -> Result<(), Box<crate::repositories_types::three_four_five_six_seven_eight::EightOriginError<'a>>>
{
    let f =
        crate::repositories_types::three_four_five_six_seven_eight::EightOriginError::Something {
            error: String::from("error_eight"),
            code_occurence: crate::code_occurence_tufa_common!(),
        };
    // use crate::traits::error_logs_logic::error_log::ErrorLog;
    // f.error_log(once_cell::sync::Lazy::force(
    //     &crate::global_variables::runtime::config::CONFIG,
    // ));
    return Err(Box::new(f));
}

pub fn eight_with_deserialize<'a>() -> Result<
    (),
    Box<
        crate::repositories_types::three_four_five_six_seven_eight::EightOriginErrorWithDeserialize<
            'a,
        >,
    >,
> {
    return Err(Box::new(crate::repositories_types::three_four_five_six_seven_eight::EightOriginErrorWithDeserialize::Something {
        error: String::from("error_eight"),
        code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize::new(
            &crate::global_variables::compile_time::git_info::GIT_INFO,
            file!(),
            line!(),
            column!(),
        ),
    }));
}
