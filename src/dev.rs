// use crate::traits::display_foreign_type;

#[derive(Debug, thiserror::Error)] //error_occurence::ImplErrorOccurence
pub enum TestError<'a> {
    Something {
        // #[error_occurence]
        error: TestEnumError<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error)] //error_occurence::ImplErrorOccurence
pub enum TestEnumError<'a> {
    // #[display_foreign_type]
    Something(crate::dev::KekwLifetime<'a>),
}

impl < 'error_occurence_proc_macro_reserved_lifetime_name, 'a, ConfigGeneric,
> crate :: traits :: error_logs_logic :: to_string_with_config ::
ToStringWithConfigForSourceToStringWithConfig <
'error_occurence_proc_macro_reserved_lifetime_name, ConfigGeneric, > for
TestEnumError < 'a, > where ConfigGeneric : crate :: traits :: fields ::
GetSourcePlaceType + crate :: traits :: fields :: GetTimezone + crate ::
traits :: get_server_address :: GetServerAddress,
{
    fn
    to_string_with_config_for_source_to_string_with_config(& self, config : &
    ConfigGeneric) -> String
    {
        match self
        {
            TestEnumError :: Something(i) =>
            {
                use crate :: traits :: display_foreign_type ::
                DisplayForeignType ; i.display_foreign_type().to_string()
            }
        }
    }
}
impl<'error_occurence_proc_macro_reserved_lifetime_name, 'a>
    crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig<
        'error_occurence_proc_macro_reserved_lifetime_name,
    > for TestEnumError<'a>
{
    fn to_string_without_config(&self) -> String {
        match self {
            TestEnumError::Something(i) => {
                use crate::traits::display_foreign_type::DisplayForeignType;
                i.display_foreign_type().to_string()
            }
        }
    }
}
#[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
pub enum TestEnumErrorWithDeserialize {
    Something(String),
}
impl<'error_occurence_proc_macro_reserved_lifetime_name>
    crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize<
        'error_occurence_proc_macro_reserved_lifetime_name,
    > for TestEnumErrorWithDeserialize
{
    fn to_string_without_config_with_deserialize(&self) -> String {
        match self {
            TestEnumErrorWithDeserialize::Something(i) => i.to_string(),
        }
    }
}
impl<'a> TestEnumError<'a> {
    pub fn into_serialize_deserialize_version(self) -> TestEnumErrorWithDeserialize {
        match self {
            TestEnumError::Something(i) => TestEnumErrorWithDeserialize::Something({
                use crate::traits::display_foreign_type::DisplayForeignType;
                i.display_foreign_type().to_string()
            }),
        }
    }
}
impl<'a> std::fmt::Display for TestEnumError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
        write!(f, "{}", self.to_string_without_config())
    }
}
impl std::fmt::Display for TestEnumErrorWithDeserialize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
        write!(f, "{}", self.to_string_without_config_with_deserialize())
    }
}

pub fn test_fn<'a>() -> Result<(), Box<TestError<'a>>> {
    return Err(Box::new(TestError::Something {
        error: TestEnumError::Something(crate::dev::KekwLifetime {
            s: "kekwlifetimeinners",
        }),
        code_occurence: crate::code_occurence_tufa_common!(),
    }));
}

pub fn dev() {
    if let Err(e) = test_fn() {
        println!("{}", *e);
        use crate::traits::error_logs_logic::error_log::ErrorLog;
        e.error_log(once_cell::sync::Lazy::force(
            //todo - this must be call once on start of the program
            &crate::global_variables::runtime::config::CONFIG,
        ));
        let ed = e.into_serialize_deserialize_version();
        println!("{ed}");
        let xs = serde_json::to_string(&ed).unwrap();
        println!("serializes into string {}", xs);
        let xd: TestErrorWithDeserialize = serde_json::from_str(&xs).unwrap();
        println!("after deserialize \n{xd}");
    }
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

impl<'a, ConfigGeneric>
    crate::traits::error_logs_logic::source_to_string_with_config::SourceToStringWithConfig<
        'a,
        ConfigGeneric,
    > for TestError<'a>
where
    ConfigGeneric: crate::traits::fields::GetSourcePlaceType
        + crate::traits::fields::GetTimezone
        + crate::traits::get_server_address::GetServerAddress,
{
    fn source_to_string_with_config(&self, config: &ConfigGeneric) -> String {
        match self {
            TestError::Something {
                error: _unused_argument_0,
                code_occurence: _unused_argument_1,
            } => {
                use crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig;
                self.source_to_string_without_config()
            }
        }
    }
}
impl<'a>
    crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
        'a,
    > for TestError<'a>
{
    fn source_to_string_without_config(&self) -> String {
        match self {
            TestError::Something {
                error,
                code_occurence: _unused_argument_1,
            } => {
                format!(
                    "{{
{}}}",
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
                        error.to_string_without_config().lines_space_backslash()
                    }
                )
            }
        }
    }
}
impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurence<'a>
    for TestError<'a>
{
    fn get_code_occurence(&self) -> &crate::common::code_occurence::CodeOccurence<'a> {
        match self {
            TestError::Something {
                error: _unused_argument_0,
                code_occurence,
            } => code_occurence,
        }
    }
}
#[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
pub enum TestErrorWithDeserialize<'a> {
    Something {
        // #[serde(borrow)]
        error: TestEnumErrorWithDeserialize,
        #[serde(borrow)]
        code_occurence: crate::common::code_occurence::CodeOccurenceWithDeserialize<'a>,
    },
}
impl<'a>
    crate::traits::error_logs_logic::source_to_string_without_config::SourceToStringWithoutConfig<
        'a,
    > for TestErrorWithDeserialize<'a>
{
    fn source_to_string_without_config(&self) -> String {
        match self {
            TestErrorWithDeserialize::Something {
                error,
                code_occurence: _unused_argument_1,
            } => {
                format!(
                    "{{
{}}}",
                    {
                        use crate::traits::error_logs_logic::lines_space_backslash::LinesSpaceBackslash;
                        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
                        error
                            .to_string_without_config_with_deserialize()
                            .lines_space_backslash()
                    }
                )
            }
        }
    }
}
impl<'a> crate::traits::error_logs_logic::get_code_occurence::GetCodeOccurenceWithDeserialize<'a>
    for TestErrorWithDeserialize<'a>
{
    fn get_code_occurence_with_deserialize(
        &self,
    ) -> &crate::common::code_occurence::CodeOccurenceWithDeserialize<'a> {
        match self {
            TestErrorWithDeserialize::Something {
                error: _unused_argument_0,
                code_occurence,
            } => code_occurence,
        }
    }
}
impl<'a> TestError<'a> {
    pub fn into_serialize_deserialize_version(self) -> TestErrorWithDeserialize<'a> {
        match self {
            TestError::Something {
                error,
                code_occurence,
            } => TestErrorWithDeserialize::Something {
                error: { error.into_serialize_deserialize_version() },
                code_occurence: code_occurence.into_serialize_deserialize_version(),
            },
        }
    }
}
impl<'a> std::fmt::Display for TestError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfig;
        write!(f, "{}", self.to_string_without_config())
    }
}
impl<'a> std::fmt::Display for TestErrorWithDeserialize<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::traits::error_logs_logic::to_string_without_config::ToStringWithoutConfigWithDeserialize;
        write!(f, "{}", self.to_string_without_config_with_deserialize())
    }
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

// #[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
// pub enum SevenError<'a> {
//     Something {
//         #[display]
//         error: String,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
// }

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

// #[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
// pub struct Omegalul {}

// impl std::fmt::Display for Omegalul {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "omegalul")
//     }
// }

// #[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
// pub struct OmegalulLifetime<'a> {
//     s: &'a str,
// }

// impl<'a> std::fmt::Display for OmegalulLifetime<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "omegalullifetime")
//     }
// }

// #[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
// pub enum OneErrorEnum<'a> {
//     #[display]
//     ToString(crate::dev::Omegalul),
//     #[display]
//     ToStringLifetime(crate::dev::OmegalulLifetime<'a>),
//     #[display_foreign_type]
//     DisplayForeignType(crate::dev::Kekw),
//     #[display_foreign_type]
//     DisplayForeignTypeLifeTime(crate::dev::KekwLifetime<'a>),
//     #[error_occurence]
//     ErrorOccurence(crate::dev::SevenError<'a>),
//     #[vec_display]
//     VecToString(std::vec::Vec<crate::dev::Omegalul>),
//     #[vec_display_foreign_type]
//     VecDisplayForeignType(std::vec::Vec<crate::dev::Kekw>),
//     #[vec_error_occurence]
//     VecErrorOccurence(std::vec::Vec<crate::dev::SevenError<'a>>),
//     #[hashmap_key_display_value_display]
//     HashMapKeyToStringValueToString(
//         std::collections::HashMap<crate::dev::Omegalul, crate::dev::Omegalul>,
//     ),
//     #[hashmap_key_display_value_display_foreign_type]
//     HashMapKeyToStringValueDisplayForeignType(
//         std::collections::HashMap<crate::dev::Omegalul, crate::dev::Kekw>,
//     ),
//     #[hashmap_key_display_value_error_occurence]
//     HashMapKeyToStringValueErrorOccurence(
//         std::collections::HashMap<crate::dev::Omegalul, crate::dev::SevenError<'a>>,
//     ),
//     #[hashmap_key_display_foreign_type_value_display]
//     HashMapKeyDisplayForeignTypeValueToString(
//         std::collections::HashMap<crate::dev::Kekw, crate::dev::Omegalul>,
//     ),
//     #[hashmap_key_display_foreign_type_value_display_foreign_type]
//     HashMapKeyDisplayForeignTypeValueDisplayForeignType(
//         std::collections::HashMap<crate::dev::Kekw, crate::dev::Kekw>,
//     ),
//     #[hashmap_key_display_foreign_type_value_error_occurence]
//     HashMapKeyDisplayForeignTypeValueErrorOccurence(
//         std::collections::HashMap<crate::dev::Kekw, crate::dev::SevenError<'a>>,
//     ),
// }

// #[derive(Debug, thiserror::Error, error_occurence::ImplErrorOccurence)]
// pub enum NamedError<'a> {
//     Something {
//         #[display]
//         a: crate::dev::Omegalul,
//         #[display]
//         b: crate::dev::OmegalulLifetime<'a>,
//         #[display_foreign_type]
//         c: crate::dev::Kekw,
//         #[display_foreign_type]
//         d: crate::dev::KekwLifetime<'a>,
//         #[error_occurence]
//         e: crate::dev::SevenError<'a>,
//         #[vec_display]
//         f: std::vec::Vec<crate::dev::Omegalul>,
//         #[vec_display]
//         g: std::vec::Vec<crate::dev::OmegalulLifetime<'a>>,
//         #[vec_display_foreign_type]
//         h: std::vec::Vec<crate::dev::Kekw>,
//         #[vec_display_foreign_type]
//         j: std::vec::Vec<crate::dev::KekwLifetime<'a>>,
//         #[vec_error_occurence]
//         k: std::vec::Vec<crate::dev::SevenError<'a>>,
//         #[hashmap_key_display_value_display]
//         l: std::collections::HashMap<crate::dev::Omegalul, crate::dev::Omegalul>,
//         #[hashmap_key_display_value_display]
//         m: std::collections::HashMap<crate::dev::Omegalul, crate::dev::OmegalulLifetime<'a>>,
//         #[hashmap_key_display_value_display]
//         n: std::collections::HashMap<crate::dev::OmegalulLifetime<'a>, crate::dev::Omegalul>,
//         #[hashmap_key_display_value_display]
//         o: std::collections::HashMap<
//             crate::dev::OmegalulLifetime<'a>,
//             crate::dev::OmegalulLifetime<'a>,
//         >,
//         #[hashmap_key_display_value_display_foreign_type]
//         p: std::collections::HashMap<crate::dev::Omegalul, crate::dev::Kekw>,
//         #[hashmap_key_display_value_display_foreign_type]
//         q: std::collections::HashMap<crate::dev::Omegalul, crate::dev::KekwLifetime<'a>>,
//         #[hashmap_key_display_value_display_foreign_type]
//         r: std::collections::HashMap<crate::dev::OmegalulLifetime<'a>, crate::dev::Kekw>,
//         #[hashmap_key_display_value_display_foreign_type]
//         s: std::collections::HashMap<
//             crate::dev::OmegalulLifetime<'a>,
//             crate::dev::KekwLifetime<'a>,
//         >,
//         #[hashmap_key_display_value_error_occurence]
//         t: std::collections::HashMap<crate::dev::Omegalul, crate::dev::SevenError<'a>>,
//         #[hashmap_key_display_value_error_occurence]
//         u: std::collections::HashMap<crate::dev::OmegalulLifetime<'a>, crate::dev::SevenError<'a>>,
//         #[hashmap_key_display_foreign_type_value_display]
//         v: std::collections::HashMap<crate::dev::Kekw, crate::dev::Omegalul>,
//         #[hashmap_key_display_foreign_type_value_display]
//         w: std::collections::HashMap<crate::dev::Kekw, crate::dev::OmegalulLifetime<'a>>,
//         #[hashmap_key_display_foreign_type_value_display]
//         x: std::collections::HashMap<crate::dev::KekwLifetime<'a>, crate::dev::Omegalul>,
//         #[hashmap_key_display_foreign_type_value_display]
//         y: std::collections::HashMap<
//             crate::dev::KekwLifetime<'a>,
//             crate::dev::OmegalulLifetime<'a>,
//         >,
//         #[hashmap_key_display_foreign_type_value_display_foreign_type]
//         z: std::collections::HashMap<crate::dev::Kekw, crate::dev::Kekw>,
//         #[hashmap_key_display_foreign_type_value_display_foreign_type]
//         aa: std::collections::HashMap<crate::dev::Kekw, crate::dev::KekwLifetime<'a>>,
//         #[hashmap_key_display_foreign_type_value_display_foreign_type]
//         ab: std::collections::HashMap<crate::dev::KekwLifetime<'a>, crate::dev::Kekw>,
//         #[hashmap_key_display_foreign_type_value_display_foreign_type]
//         ac: std::collections::HashMap<crate::dev::KekwLifetime<'a>, crate::dev::KekwLifetime<'a>>,
//         #[hashmap_key_display_foreign_type_value_error_occurence]
//         ad: std::collections::HashMap<crate::dev::Kekw, crate::dev::SevenError<'a>>,
//         #[hashmap_key_display_foreign_type_value_error_occurence]
//         af: std::collections::HashMap<crate::dev::KekwLifetime<'a>, crate::dev::SevenError<'a>>,
//         #[vec_error_occurence]
//         ag: std::vec::Vec<OneErrorEnum<'a>>,
//         code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
//     },
// }
