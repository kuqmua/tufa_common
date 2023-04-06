// #[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)] //, error_occurence::ImplErrorOccurence
// pub enum TestError<'a> {
//     Something {
//         //todo - add here 'a str and 'static str
//         // #[display]
//         #[display_foreign_type]
//         lft_str: crate::dev::KekwLifetime<'a>,
//         #[error_occurence_no_sd_lifetime]
//         error: TestEnumError<'a>,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
// }

// #[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)] //, error_occurence::ImplErrorOccurence
// pub enum TestEnumError<'a> {
//     #[display_foreign_type]
//     Something(crate::dev::KekwLifetime<'a>),
// }

////

////

// pub fn test_fn<'a>() -> Result<(), Box<TestError<'a>>> {
//     return Err(Box::new(TestError::Something {
//         lft_str: crate::dev::KekwLifetime {
//             s: "kekwlifetimeinners",
//         },
//         error: TestEnumError::Something(crate::dev::KekwLifetime {
//             s: "kekwlifetimeinners",
//         }),
//         code_occurence: crate::code_occurence_tufa_common!(),
//     }));
// }

//todo reorder and standart serde_borrow_attribute_handle and rename it in proc_macro
//todo rename into traits - instead of static_str -> String
//todo support 'a str and 'static str, bool u32 and other types in error occurence fields and variants
//todo different lifetimes support for named case(unnamed done - see how)
//todo reserved lifetime name - do somthing with it
//todo deal with &'static str in case of Deserialize -> String
pub fn dev() {
    // if let Err(e) = test_fn() {
    //     println!("{}", *e);
    //     use crate::traits::error_logs_logic::error_log::ErrorLog;
    //     e.error_log(once_cell::sync::Lazy::force(
    //         //todo - this must be call once on start of the program
    //         &crate::global_variables::runtime::config::CONFIG,
    //     ));
    //     let ed = e.into_serialize_deserialize_version();
    //     println!("{ed}");
    //     let xs = serde_json::to_string(&ed).unwrap();
    //     println!("serializes into string {}", xs);
    //     let xd: TestErrorWithDeserialize = serde_json::from_str(&xs).unwrap();
    //     println!("after deserialize \n{xd}");
    // }
    // if let Err(e) = named() {

    //     // println!("{}", *e);
    //     // use crate::traits::error_logs_logic::error_log::ErrorLog;
    //     // e.error_log(once_cell::sync::Lazy::force(
    //     //     //todo - this must be call once on start of the program
    //     //     &crate::global_variables::runtime::config::CONFIG,
    //     // ));
    //     // let ed = e.into_serialize_deserialize_version();
    //     // println!("{ed}");
    //     // // let xs = .unwrap();
    //     // match serde_json::to_string(&ed) {
    //     //     Ok(_) => println!("ok"),
    //     //     Err(e) => println!("{e:#?}"),
    //     // }
    //     // println!("serializes into string {}", xs);
    //     // let xs_clone = xs.clone();
    //     // let xd: NamedErrorWithDeserialize = serde_json::from_str(&xs_clone).unwrap();
    //     // println!("after deserialize \n{xd}");
    // }
}

//////////////////////////////////////////

// pub fn named<'a>() -> Result<(), Box<NamedError<'a>>> {
//     return Err(Box::new(NamedError::Something {
//         a: crate::dev::Omegalul {},
//         b: crate::dev::OmegalulLifetime {
//             s: "omegalullifetime",
//         },
//         c: crate::dev::Kekw {},
//         d: crate::dev::KekwLifetime { s: "kekwlifetime" },
//         e: crate::dev::SevenError::Something {
//             error: String::from("seven_error"),
//             code_occurence: crate::code_occurence_tufa_common!(),
//         },
//         f: vec![crate::dev::Omegalul {}],
//         g: vec![crate::dev::OmegalulLifetime {
//             s: "omegalullifetime",
//         }],
//         h: vec![crate::dev::Kekw {}],
//         j: vec![crate::dev::KekwLifetime { s: "kekwlifetime" }],
//         k: vec![crate::dev::SevenError::Something {
//             error: String::from("seven_error"),
//             code_occurence: crate::code_occurence_tufa_common!(),
//         }],
//         l: std::collections::HashMap::from([(crate::dev::Omegalul {}, crate::dev::Omegalul {})]),
//         m: std::collections::HashMap::from([(
//             crate::dev::Omegalul {},
//             crate::dev::OmegalulLifetime {
//                 s: "omegalullifetime",
//             },
//         )]),
//         n: std::collections::HashMap::from([(
//             crate::dev::OmegalulLifetime {
//                 s: "omegalullifetime",
//             },
//             crate::dev::Omegalul {},
//         )]),
//         o: std::collections::HashMap::from([(
//             crate::dev::OmegalulLifetime {
//                 s: "omegalullifetime",
//             },
//             crate::dev::OmegalulLifetime {
//                 s: "omegalullifetime",
//             },
//         )]),
//         p: std::collections::HashMap::from([(crate::dev::Omegalul {}, crate::dev::Kekw {})]),
//         q: std::collections::HashMap::from([(
//             crate::dev::Omegalul {},
//             crate::dev::KekwLifetime { s: "kekwlifetime" },
//         )]),
//         r: std::collections::HashMap::from([(
//             crate::dev::OmegalulLifetime {
//                 s: "omegalullifetime",
//             },
//             crate::dev::Kekw {},
//         )]),
//         s: std::collections::HashMap::from([(
//             crate::dev::OmegalulLifetime {
//                 s: "omegalullifetime",
//             },
//             crate::dev::KekwLifetime { s: "kekwlifetime" },
//         )]),
//         t: std::collections::HashMap::from([(
//             crate::dev::Omegalul {},
//             crate::dev::SevenError::Something {
//                 error: String::from("seven_error"),
//                 code_occurence: crate::code_occurence_tufa_common!(),
//             },
//         )]),
//         u: std::collections::HashMap::from([(
//             crate::dev::OmegalulLifetime {
//                 s: "omegalullifetime",
//             },
//             crate::dev::SevenError::Something {
//                 error: String::from("seven_error"),
//                 code_occurence: crate::code_occurence_tufa_common!(),
//             },
//         )]),
//         v: std::collections::HashMap::from([(crate::dev::Kekw {}, crate::dev::Omegalul {})]),
//         w: std::collections::HashMap::from([(
//             crate::dev::Kekw {},
//             crate::dev::OmegalulLifetime {
//                 s: "omegalullifetime",
//             },
//         )]),
//         x: std::collections::HashMap::from([(
//             crate::dev::KekwLifetime { s: "kekwlifetime" },
//             crate::dev::Omegalul {},
//         )]),
//         y: std::collections::HashMap::from([(
//             crate::dev::KekwLifetime { s: "kekwlifetime" },
//             crate::dev::OmegalulLifetime {
//                 s: "omegalullifetime",
//             },
//         )]),
//         z: std::collections::HashMap::from([(crate::dev::Kekw {}, crate::dev::Kekw {})]),
//         aa: std::collections::HashMap::from([(
//             crate::dev::Kekw {},
//             crate::dev::KekwLifetime { s: "kekwlifetime" },
//         )]),
//         ab: std::collections::HashMap::from([(
//             crate::dev::KekwLifetime { s: "kekwlifetime" },
//             crate::dev::Kekw {},
//         )]),
//         ac: std::collections::HashMap::from([(
//             crate::dev::KekwLifetime { s: "kekwlifetime" },
//             crate::dev::KekwLifetime { s: "kekwlifetime" },
//         )]),
//         ad: std::collections::HashMap::from([(
//             crate::dev::Kekw {},
//             crate::dev::SevenError::Something {
//                 error: String::from("seven_error"),
//                 code_occurence: crate::code_occurence_tufa_common!(),
//             },
//         )]),
//         af: std::collections::HashMap::from([(
//             crate::dev::KekwLifetime { s: "kekwlifetime" },
//             crate::dev::SevenError::Something {
//                 error: String::from("seven_error"),
//                 code_occurence: crate::code_occurence_tufa_common!(),
//             },
//         )]),
//         ag: vec![OneErrorEnum::HashMapKeyToStringValueToString(
//             std::collections::HashMap::from([(crate::dev::Omegalul {}, crate::dev::Omegalul {})]),
//         )],
//         code_occurence: crate::code_occurence_tufa_common!(),
//     }));
// }

#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)] //, error_occurence::ImplErrorOccurence
pub enum SevenError<'a> {
    Something {
        #[display]
        error: String,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct KekwLifetime<'a> {
    s: &'a str,
}

impl<'a> crate::traits::display_foreign_type::DisplayForeignType for KekwLifetime<'a> {
    fn display_foreign_type(&self) -> &'static str {
        "kekwlifetime"
    }
}

#[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Kekw {}

impl crate::traits::display_foreign_type::DisplayForeignType for Kekw {
    fn display_foreign_type(&self) -> &'static str {
        "kekw"
    }
}

#[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Omegalul {}

impl std::fmt::Display for Omegalul {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "omegalul")
    }
}

#[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct OmegalulLifetime<'a> {
    s: &'a str,
}

impl<'a> std::fmt::Display for OmegalulLifetime<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "omegalullifetime")
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)] //, error_occurence::ImplErrorOccurence
pub enum OneErrorEnum<'a> {
    #[display]
    ToString(crate::dev::Omegalul),
    #[display]
    ToStringLifetime(crate::dev::OmegalulLifetime<'a>),
    #[display_foreign_type]
    DisplayForeignType(crate::dev::Kekw),
    #[display_foreign_type]
    DisplayForeignTypeLifeTime(crate::dev::KekwLifetime<'a>),
    #[error_occurence_sd_lifetime]
    ErrorOccurence(crate::dev::SevenError<'a>),
    #[vec_display]
    VecToString(std::vec::Vec<crate::dev::Omegalul>),
    #[vec_display_foreign_type]
    VecDisplayForeignType(std::vec::Vec<crate::dev::Kekw>),
    #[vec_error_occurence_sd_lifetime]
    VecErrorOccurence(std::vec::Vec<crate::dev::SevenError<'a>>),
    #[hashmap_key_display_value_display]
    HashMapKeyToStringValueToString(
        std::collections::HashMap<crate::dev::Omegalul, crate::dev::Omegalul>,
    ),
    #[hashmap_key_display_value_display_foreign_type]
    HashMapKeyToStringValueDisplayForeignType(
        std::collections::HashMap<crate::dev::Omegalul, crate::dev::Kekw>,
    ),
    #[hashmap_key_display_value_error_occurence_sd_lifetime]
    HashMapKeyToStringValueErrorOccurence(
        std::collections::HashMap<crate::dev::Omegalul, crate::dev::SevenError<'a>>,
    ),
    #[hashmap_key_display_foreign_type_value_display]
    HashMapKeyDisplayForeignTypeValueToString(
        std::collections::HashMap<crate::dev::Kekw, crate::dev::Omegalul>,
    ),
    #[hashmap_key_display_foreign_type_value_display_foreign_type]
    HashMapKeyDisplayForeignTypeValueDisplayForeignType(
        std::collections::HashMap<crate::dev::Kekw, crate::dev::Kekw>,
    ),
    #[hashmap_key_display_foreign_type_value_error_occurence_sd_lifetime]
    HashMapKeyDisplayForeignTypeValueErrorOccurence(
        std::collections::HashMap<crate::dev::Kekw, crate::dev::SevenError<'a>>,
    ),
}

#[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
pub enum NamedError<'a> {
    Something {
        #[display]
        a: crate::dev::Omegalul,
        #[display]
        b: crate::dev::OmegalulLifetime<'a>,
        #[display_foreign_type]
        c: crate::dev::Kekw,
        #[display_foreign_type]
        d: crate::dev::KekwLifetime<'a>,
        #[error_occurence_sd_lifetime]
        e: crate::dev::SevenError<'a>,
        #[vec_display]
        f: std::vec::Vec<crate::dev::Omegalul>,
        #[vec_display]
        g: std::vec::Vec<crate::dev::OmegalulLifetime<'a>>,
        #[vec_display_foreign_type]
        h: std::vec::Vec<crate::dev::Kekw>,
        #[vec_display_foreign_type]
        j: std::vec::Vec<crate::dev::KekwLifetime<'a>>,
        #[vec_error_occurence_sd_lifetime]
        k: std::vec::Vec<crate::dev::SevenError<'a>>,
        #[hashmap_key_display_value_display]
        l: std::collections::HashMap<crate::dev::Omegalul, crate::dev::Omegalul>,
        #[hashmap_key_display_value_display]
        m: std::collections::HashMap<crate::dev::Omegalul, crate::dev::OmegalulLifetime<'a>>,
        #[hashmap_key_display_value_display]
        n: std::collections::HashMap<crate::dev::OmegalulLifetime<'a>, crate::dev::Omegalul>,
        #[hashmap_key_display_value_display]
        o: std::collections::HashMap<
            crate::dev::OmegalulLifetime<'a>,
            crate::dev::OmegalulLifetime<'a>,
        >,
        #[hashmap_key_display_value_display_foreign_type]
        p: std::collections::HashMap<crate::dev::Omegalul, crate::dev::Kekw>,
        #[hashmap_key_display_value_display_foreign_type]
        q: std::collections::HashMap<crate::dev::Omegalul, crate::dev::KekwLifetime<'a>>,
        #[hashmap_key_display_value_display_foreign_type]
        r: std::collections::HashMap<crate::dev::OmegalulLifetime<'a>, crate::dev::Kekw>,
        #[hashmap_key_display_value_display_foreign_type]
        s: std::collections::HashMap<
            crate::dev::OmegalulLifetime<'a>,
            crate::dev::KekwLifetime<'a>,
        >,
        #[hashmap_key_display_value_error_occurence_sd_lifetime]
        t: std::collections::HashMap<crate::dev::Omegalul, crate::dev::SevenError<'a>>,
        #[hashmap_key_display_value_error_occurence_sd_lifetime]
        u: std::collections::HashMap<crate::dev::OmegalulLifetime<'a>, crate::dev::SevenError<'a>>,
        #[hashmap_key_display_foreign_type_value_display]
        v: std::collections::HashMap<crate::dev::Kekw, crate::dev::Omegalul>,
        #[hashmap_key_display_foreign_type_value_display]
        w: std::collections::HashMap<crate::dev::Kekw, crate::dev::OmegalulLifetime<'a>>,
        #[hashmap_key_display_foreign_type_value_display]
        x: std::collections::HashMap<crate::dev::KekwLifetime<'a>, crate::dev::Omegalul>,
        #[hashmap_key_display_foreign_type_value_display]
        y: std::collections::HashMap<
            crate::dev::KekwLifetime<'a>,
            crate::dev::OmegalulLifetime<'a>,
        >,
        #[hashmap_key_display_foreign_type_value_display_foreign_type]
        z: std::collections::HashMap<crate::dev::Kekw, crate::dev::Kekw>,
        #[hashmap_key_display_foreign_type_value_display_foreign_type]
        aa: std::collections::HashMap<crate::dev::Kekw, crate::dev::KekwLifetime<'a>>,
        #[hashmap_key_display_foreign_type_value_display_foreign_type]
        ab: std::collections::HashMap<crate::dev::KekwLifetime<'a>, crate::dev::Kekw>,
        #[hashmap_key_display_foreign_type_value_display_foreign_type]
        ac: std::collections::HashMap<crate::dev::KekwLifetime<'a>, crate::dev::KekwLifetime<'a>>,
        #[hashmap_key_display_foreign_type_value_error_occurence_sd_lifetime]
        ad: std::collections::HashMap<crate::dev::Kekw, crate::dev::SevenError<'a>>,
        #[hashmap_key_display_foreign_type_value_error_occurence_sd_lifetime]
        af: std::collections::HashMap<crate::dev::KekwLifetime<'a>, crate::dev::SevenError<'a>>,
        #[vec_error_occurence_sd_lifetime]
        ag: std::vec::Vec<OneErrorEnum<'a>>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
