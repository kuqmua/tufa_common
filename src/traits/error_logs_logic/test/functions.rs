// use crate::traits::error_logs_logic::error_log::ErrorLog;

// #[test]
// pub fn test_code_occurence_logic() {
//     if let Err(e) = three() {
//         //todo - this actually must be a config struct function, not an error - config.error_log(e)
//         println!("{}", *e);
//         e.error_log(once_cell::sync::Lazy::force(
//             &crate::global_variables::runtime::config::CONFIG,
//         ));
//     }
//     // println!("kek");
//     assert_eq!(true, true)
// }

pub fn three<'a>() -> Result<(), Box<crate::traits::error_logs_logic::test::types::ThreeError<'a>>>
{
    if let Err(e) = four() {
        let f = crate::traits::error_logs_logic::test::types::ThreeError::Something {
            inner_error: *e,
            code_occurence: crate::code_occurence_tufa_common!(),
        };
        return Err(Box::new(f));
    };
    Ok(())
}

pub fn three_with_deserialize<'a>(
) -> Result<(), Box<crate::traits::error_logs_logic::test::types::ThreeErrorWithDeserialize<'a>>> {
    if let Err(e) = four_with_deserialize() {
        let f =
            crate::traits::error_logs_logic::test::types::ThreeErrorWithDeserialize::Something {
                inner_error: *e,
                code_occurence: crate::code_occurence_tufa_common_with_deserialize!(),
            };
        return Err(Box::new(f));
    };
    Ok(())
}

pub fn four<'a>(
) -> Result<(), Box<crate::traits::error_logs_logic::test::types::FourWrapperError<'a>>> {
    match (five(), six()) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(f), Err(s)) => {
            let f = crate::traits::error_logs_logic::test::types::FourWrapperError::Something {
                inner_errors: std::collections::HashMap::from([
                    (
                        String::from("five_hashmap_key"),
                        crate::traits::error_logs_logic::test::types::FourErrorEnum::Five(*f),
                    ),
                    (
                        String::from("six_hashmap_key"),
                        crate::traits::error_logs_logic::test::types::FourErrorEnum::Six(*s),
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
    Box<crate::traits::error_logs_logic::test::types::FourWrapperErrorWithDeserialize<'a>>,
> {
    match (five_with_deserialize(), six_with_deserialize()) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(f), Err(s)) => {
            let f = crate::traits::error_logs_logic::test::types::FourWrapperErrorWithDeserialize::Something {
                inner_errors: std::collections::HashMap::from([
                    (
                        String::from("five_hashmap_key"),
                        crate::traits::error_logs_logic::test::types::FourErrorEnumWithDeserialize::Five(*f),
                    ),
                    (
                        String::from("six_hashmap_key"),
                        crate::traits::error_logs_logic::test::types::FourErrorEnumWithDeserialize::Six(*s),
                    ),
                ]),
                code_occurence: crate::code_occurence_tufa_common_with_deserialize!(),
            };
            return Err(Box::new(f));
        }
    }
}

pub fn five<'a>() -> Result<(), Box<crate::traits::error_logs_logic::test::types::FiveError<'a>>> {
    if let Err(e) = five_one() {
        let f = crate::traits::error_logs_logic::test::types::FiveError::Something {
            inner_errors: std::collections::HashMap::from([(
                String::from("five_one_hashmap_key"),
                crate::traits::error_logs_logic::test::types::FiveErrorEnum::FiveOne(*e),
            )]),
            code_occurence: crate::code_occurence_tufa_common!(),
        };
        return Err(Box::new(f));
    }
    Ok(())
}

pub fn five_with_deserialize<'a>(
) -> Result<(), Box<crate::traits::error_logs_logic::test::types::FiveErrorWithDeserialize<'a>>> {
    if let Err(e) = five_one_with_deserialize() {
        let f = crate::traits::error_logs_logic::test::types::FiveErrorWithDeserialize::Something {
            inner_errors: std::collections::HashMap::from([(
                String::from("five_one_hashmap_key"),
                crate::traits::error_logs_logic::test::types::FiveErrorEnumWithDeserialize::FiveOne(
                    *e,
                ),
            )]),
            code_occurence: crate::code_occurence_tufa_common_with_deserialize!(),
        };
        return Err(Box::new(f));
    }
    Ok(())
}

pub fn five_one<'a>(
) -> Result<(), Box<crate::traits::error_logs_logic::test::types::FiveOneError<'a>>> {
    return Err(Box::new(
        crate::traits::error_logs_logic::test::types::FiveOneError::Something {
            error: String::from("five_one error"),
            code_occurence: crate::code_occurence_tufa_common!(),
        },
    ));
}

pub fn five_one_with_deserialize<'a>(
) -> Result<(), Box<crate::traits::error_logs_logic::test::types::FiveOneErrorWithDeserialize<'a>>>
{
    return Err(Box::new(
        crate::traits::error_logs_logic::test::types::FiveOneErrorWithDeserialize::Something {
            error: String::from("five_one error"),
            code_occurence: crate::code_occurence_tufa_common_with_deserialize!(),
        },
    ));
}

pub fn six<'a>() -> Result<(), Box<crate::traits::error_logs_logic::test::types::SixError<'a>>> {
    let thread_join_handle = std::thread::spawn(move || eight());
    let res = thread_join_handle.join().unwrap();
    match (seven(), res) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(seven_error), Err(eight_error)) => {
            let f = crate::traits::error_logs_logic::test::types::SixError::Something {
                inner_errors: vec![
                    crate::traits::error_logs_logic::test::types::SixErrorEnum::Seven(*seven_error),
                    crate::traits::error_logs_logic::test::types::SixErrorEnum::Eight(*eight_error),
                ],
                code_occurence: crate::code_occurence_tufa_common!(),
            };
            return Err(Box::new(f));
        }
    }
}

pub fn six_with_deserialize<'a>(
) -> Result<(), Box<crate::traits::error_logs_logic::test::types::SixErrorWithDeserialize<'a>>> {
    let thread_join_handle = std::thread::spawn(move || eight_with_deserialize());
    let res = thread_join_handle.join().unwrap();
    match (seven_with_deserialize(), res) {
        (Ok(_), Ok(_)) => todo!(),
        (Ok(_), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(seven_error), Err(eight_error)) => {
            let f = crate::traits::error_logs_logic::test::types::SixErrorWithDeserialize::Something {
                inner_errors: vec![
                    crate::traits::error_logs_logic::test::types::SixErrorEnumWithDeserialize::Seven(*seven_error),
                    crate::traits::error_logs_logic::test::types::SixErrorEnumWithDeserialize::Eight(*eight_error),
                ],
                code_occurence: crate::code_occurence_tufa_common_with_deserialize!(),
            };
            return Err(Box::new(f));
        }
    }
}

pub fn seven<'a>() -> Result<(), Box<crate::traits::error_logs_logic::test::types::SevenError<'a>>>
{
    return Err(Box::new(
        crate::traits::error_logs_logic::test::types::SevenError::Something {
            error: String::from("error_eight"),
            code_occurence: crate::code_occurence_tufa_common!(),
        },
    ));
}

pub fn seven_with_deserialize<'a>(
) -> Result<(), Box<crate::traits::error_logs_logic::test::types::SevenErrorWithDeserialize<'a>>> {
    return Err(Box::new(
        crate::traits::error_logs_logic::test::types::SevenErrorWithDeserialize::Something {
            error: String::from("error_eight"),
            code_occurence: crate::code_occurence_tufa_common_with_deserialize!(),
        },
    ));
}

pub fn eight<'a>() -> Result<(), Box<crate::traits::error_logs_logic::test::types::EightError<'a>>>
{
    let f = crate::traits::error_logs_logic::test::types::EightError::Something {
        error: String::from("error_eight"),
        code_occurence: crate::code_occurence_tufa_common!(),
    };
    // use crate::traits::error_logs_logic::error_log::ErrorLog;
    // f.error_log(once_cell::sync::Lazy::force(
    //     &crate::global_variables::runtime::config::CONFIG,
    // ));
    return Err(Box::new(f));
}

pub fn eight_with_deserialize<'a>(
) -> Result<(), Box<crate::traits::error_logs_logic::test::types::EightErrorWithDeserialize<'a>>> {
    return Err(Box::new(
        crate::traits::error_logs_logic::test::types::EightErrorWithDeserialize::Something {
            error: String::from("error_eight"),
            code_occurence: crate::code_occurence_tufa_common_with_deserialize!(),
        },
    ));
}
