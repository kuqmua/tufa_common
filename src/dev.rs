pub fn three<'a>(
) -> Result<(), Box<crate::repositories_types::three_four_five_six_seven_eight::ThreeError<'a>>> {
    if let Err(e) = four() {
        let f = crate::repositories_types::three_four_five_six_seven_eight::ThreeError::Something {
            inner_error: *e,
            code_occurence: crate::code_occurence_tufa_common!(),
        };
        return Err(Box::new(f));
    };
    Ok(())
}

pub fn three_with_deserialize<'a>() -> Result<
    (),
    Box<crate::repositories_types::three_four_five_six_seven_eight::ThreeErrorWithDeserialize<'a>>,
> {
    if let Err(e) = four_with_deserialize() {
        let f = crate::repositories_types::three_four_five_six_seven_eight::ThreeErrorWithDeserialize::Something {
            inner_error: *e,
            code_occurence: crate::code_occurence_tufa_common_with_deserialize!(),
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
                        crate::repositories_types::three_four_five_six_seven_eight::FourErrorEnum::Five(*f),
                    ),
                    (
                        String::from("six_hashmap_key"),
                        crate::repositories_types::three_four_five_six_seven_eight::FourErrorEnum::Six(*s),
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
                        crate::repositories_types::three_four_five_six_seven_eight::FourErrorEnumWithDeserialize::Five(*f),
                    ),
                    (
                        String::from("six_hashmap_key"),
                        crate::repositories_types::three_four_five_six_seven_eight::FourErrorEnumWithDeserialize::Six(*s),
                    ),
                ]),
                code_occurence: crate::code_occurence_tufa_common_with_deserialize!(),
            };
            return Err(Box::new(f));
        }
    }
}

pub fn five<'a>(
) -> Result<(), Box<crate::repositories_types::three_four_five_six_seven_eight::FiveError<'a>>> {
    if let Err(e) = five_one() {
        let f = crate::repositories_types::three_four_five_six_seven_eight::FiveError::Something {
            inner_errors: std::collections::HashMap::from([(
                String::from("five_one_hashmap_key"),
                crate::repositories_types::three_four_five_six_seven_eight::FiveErrorEnum::FiveOne(
                    *e,
                ),
            )]),
            code_occurence: crate::code_occurence_tufa_common!(),
        };
        return Err(Box::new(f));
    }
    Ok(())
}

pub fn five_with_deserialize<'a>() -> Result<
    (),
    Box<crate::repositories_types::three_four_five_six_seven_eight::FiveErrorWithDeserialize<'a>>,
> {
    if let Err(e) = five_one_with_deserialize() {
        let f = crate::repositories_types::three_four_five_six_seven_eight::FiveErrorWithDeserialize::Something {
            inner_errors: std::collections::HashMap::from([(
                String::from("five_one_hashmap_key"),
                crate::repositories_types::three_four_five_six_seven_eight::FiveErrorEnumWithDeserialize::FiveOne(*e),
            )]),
            code_occurence: crate::code_occurence_tufa_common_with_deserialize!(),
        };
        return Err(Box::new(f));
    }
    Ok(())
}

pub fn five_one<'a>(
) -> Result<(), Box<crate::repositories_types::three_four_five_six_seven_eight::FiveOneError<'a>>> {
    return Err(Box::new(
        crate::repositories_types::three_four_five_six_seven_eight::FiveOneError::Something {
            error: String::from("five_one error"),
            code_occurence: crate::code_occurence_tufa_common!(),
        },
    ));
}

pub fn five_one_with_deserialize<'a>() -> Result<
    (),
    Box<
        crate::repositories_types::three_four_five_six_seven_eight::FiveOneErrorWithDeserialize<'a>,
    >,
> {
    return Err(Box::new(crate::repositories_types::three_four_five_six_seven_eight::FiveOneErrorWithDeserialize::Something {
        error: String::from("five_one error"),
        code_occurence: crate::code_occurence_tufa_common_with_deserialize!(),
    }));
}

pub fn six<'a>(
) -> Result<(), Box<crate::repositories_types::three_four_five_six_seven_eight::SixError<'a>>> {
    let thread_join_handle = std::thread::spawn(move || eight());
    let res = thread_join_handle.join().unwrap();
    match (seven(), res) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(seven_error), Err(eight_error)) => {
            let f = crate::repositories_types::three_four_five_six_seven_eight::SixError::Something {
                inner_errors: vec![
                    crate::repositories_types::three_four_five_six_seven_eight::SixErrorEnum::Seven(*seven_error),
                    crate::repositories_types::three_four_five_six_seven_eight::SixErrorEnum::Eight(*eight_error),
                ],
                code_occurence: crate::code_occurence_tufa_common!(),
            };
            return Err(Box::new(f));
        }
    }
}

pub fn six_with_deserialize<'a>() -> Result<
    (),
    Box<crate::repositories_types::three_four_five_six_seven_eight::SixErrorWithDeserialize<'a>>,
> {
    let thread_join_handle = std::thread::spawn(move || eight_with_deserialize());
    let res = thread_join_handle.join().unwrap();
    match (seven_with_deserialize(), res) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(seven_error), Err(eight_error)) => {
            let f = crate::repositories_types::three_four_five_six_seven_eight::SixErrorWithDeserialize::Something {
                inner_errors: vec![
                    crate::repositories_types::three_four_five_six_seven_eight::SixErrorEnumWithDeserialize::Seven(*seven_error),
                    crate::repositories_types::three_four_five_six_seven_eight::SixErrorEnumWithDeserialize::Eight(*eight_error),
                ],
                code_occurence: crate::code_occurence_tufa_common_with_deserialize!(),
            };
            return Err(Box::new(f));
        }
    }
}

pub fn seven<'a>(
) -> Result<(), Box<crate::repositories_types::three_four_five_six_seven_eight::SevenError<'a>>> {
    return Err(Box::new(
        crate::repositories_types::three_four_five_six_seven_eight::SevenError::Something {
            error: String::from("error_eight"),
            code_occurence: crate::code_occurence_tufa_common!(),
        },
    ));
}

pub fn seven_with_deserialize<'a>() -> Result<
    (),
    Box<crate::repositories_types::three_four_five_six_seven_eight::SevenErrorWithDeserialize<'a>>,
> {
    return Err(Box::new(crate::repositories_types::three_four_five_six_seven_eight::SevenErrorWithDeserialize::Something {
        error: String::from("error_eight"),
        code_occurence: crate::code_occurence_tufa_common_with_deserialize!(),
    }));
}

pub fn eight<'a>(
) -> Result<(), Box<crate::repositories_types::three_four_five_six_seven_eight::EightError<'a>>> {
    let f = crate::repositories_types::three_four_five_six_seven_eight::EightError::Something {
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
    Box<crate::repositories_types::three_four_five_six_seven_eight::EightErrorWithDeserialize<'a>>,
> {
    return Err(Box::new(crate::repositories_types::three_four_five_six_seven_eight::EightErrorWithDeserialize::Something {
        error: String::from("error_eight"),
        code_occurence: crate::code_occurence_tufa_common_with_deserialize!(),
    }));
}
