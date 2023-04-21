// use valuable::Valuable;

// #[derive(Clone, Debug, Valuable)]
// struct User {
//     name: String,
//     age: u32,
//     something: Vec<bool>,
//     address: Address,
// }

// #[derive(Clone, Debug, Valuable)]
// struct Address {
//     country: String,
//     city: String,
//     street: String,
// }

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SerializeDeserializeStruct {
    one: String
}
impl crate::traits::display_foreign_type::DisplayForeignType
    for SerializeDeserializeStruct
{
    fn display_foreign_type(&self) -> String {
        String::from("SerializeDeserializeStruct")
    }
}
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct DisplayForeignTypeStruct {
    two: String
}
impl crate::traits::display_foreign_type::DisplayForeignType
    for DisplayForeignTypeStruct
{
    fn display_foreign_type(&self) -> String {
        String::from("DisplayForeignTypeStruct")
    }
}

#[derive(Debug)]
pub struct DisplayWithoutSerializeDeserializeStruct {
    three: String
}

impl std::fmt::Display for DisplayWithoutSerializeDeserializeStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.three)
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]//
pub enum SerializeDeserializeDisplayError<'a> {
    Something {
        // #[eo_display]
        // display_without_ser_deser: DisplayWithoutSerializeDeserializeStruct,
        #[eo_display_foreign_type_with_serialize_deserialize]
        serialize_deserialize_struct: SerializeDeserializeStruct,
        #[eo_vec_display_foreign_type_with_serialize_deserialize]
        vec_serialize_deserialize_struct: Vec<SerializeDeserializeStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize]
        hashmap_serialize_deserialize_struct: std::collections::HashMap<String, SerializeDeserializeStruct>,
        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
        hashmap_second_serialize_deserialize_struct: std::collections::HashMap<DisplayForeignTypeStruct, SerializeDeserializeStruct>,
        #[eo_display_foreign_type]
        display_foreign_type_struct: DisplayForeignTypeStruct,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

// impl < 'error_occurence_proc_macro_reserved_lifetime_name, 'a, ConfigGeneric >
// crate :: traits :: error_logs_logic :: source_to_string_with_config ::
// SourceToStringWithConfig < 'error_occurence_proc_macro_reserved_lifetime_name,
// ConfigGeneric > for SerializeDeserializeDisplayError < 'a > where
// ConfigGeneric : crate :: traits :: fields :: GetSourcePlaceType + crate ::
// traits :: fields :: GetTimezone + crate :: traits :: fields :: GetServerPort
// {
//     fn source_to_string_with_config(& self, config : & ConfigGeneric) ->
//     String
//     {
//         match self
//         {
//             SerializeDeserializeDisplayError :: Something
//             { display_without_ser_deser, code_occurence : _unused_argument_1 }
//             =>
//             {
//                 format!
//                 ("{{
// {}}}",
//                 {
//                     use crate :: traits :: error_logs_logic ::
//                     lines_space_backslash :: LinesSpaceBackslash ; format!
//                     ("display_without_ser_deser: {}",
//                     display_without_ser_deser).lines_space_backslash()
//                 })
//             }
//         }
//     }
// } impl < 'error_occurence_proc_macro_reserved_lifetime_name, 'a > crate ::
// traits :: error_logs_logic :: source_to_string_without_config ::
// SourceToStringWithoutConfig <
// 'error_occurence_proc_macro_reserved_lifetime_name > for
// SerializeDeserializeDisplayError < 'a >
// {
//     fn source_to_string_without_config(& self) -> String
//     {
//         match self
//         {
//             SerializeDeserializeDisplayError :: Something
//             { display_without_ser_deser, code_occurence : _unused_argument_1 }
//             =>
//             {
//                 format!
//                 ("{{
// {}}}",
//                 {
//                     use crate :: traits :: error_logs_logic ::
//                     lines_space_backslash :: LinesSpaceBackslash ; format!
//                     ("display_without_ser_deser: {}",
//                     display_without_ser_deser).lines_space_backslash()
//                 })
//             }
//         }
//     }
// } impl < 'error_occurence_proc_macro_reserved_lifetime_name, 'a > crate ::
// traits :: error_logs_logic :: get_code_occurence :: GetCodeOccurence <
// 'error_occurence_proc_macro_reserved_lifetime_name > for
// SerializeDeserializeDisplayError < 'a >
// {
//     fn get_code_occurence(& self) -> & crate :: common :: code_occurence ::
//     CodeOccurence
//     {
//         match self
//         {
//             SerializeDeserializeDisplayError :: Something
//             { display_without_ser_deser : _unused_argument_0, code_occurence }
//             => { code_occurence }
//         }
//     }
// }
// #[derive(Debug, thiserror :: Error, serde :: Serialize, serde :: Deserialize)]
// pub enum SerializeDeserializeDisplayErrorWithSerializeDeserialize < 'a >
// {
//     Something
//     {
//         display_without_ser_deser : DisplayWithoutSerializeDeserializeStruct <
//         >, #[serde(borrow)] code_occurence : crate :: common :: code_occurence
//         :: CodeOccurenceWithSerializeDeserialize < 'a >
//     }
// } impl < 'error_occurence_proc_macro_reserved_lifetime_name, 'a > crate ::
// traits :: error_logs_logic :: source_to_string_without_config ::
// SourceToStringWithoutConfig <
// 'error_occurence_proc_macro_reserved_lifetime_name > for
// SerializeDeserializeDisplayErrorWithSerializeDeserialize < 'a >
// {
//     fn source_to_string_without_config(& self) -> String
//     {
//         match self
//         {
//             SerializeDeserializeDisplayErrorWithSerializeDeserialize ::
//             Something
//             { display_without_ser_deser, code_occurence : _unused_argument_1 }
//             =>
//             {
//                 format!
//                 ("{{
// {}}}",
//                 {
//                     use crate :: traits :: error_logs_logic ::
//                     lines_space_backslash :: LinesSpaceBackslash ; format!
//                     ("display_without_ser_deser: {}",
//                     display_without_ser_deser).lines_space_backslash()
//                 })
//             }
//         }
//     }
// } impl < 'error_occurence_proc_macro_reserved_lifetime_name, 'a > crate ::
// traits :: error_logs_logic :: get_code_occurence ::
// GetCodeOccurenceWithSerializeDeserialize <
// 'error_occurence_proc_macro_reserved_lifetime_name > for
// SerializeDeserializeDisplayErrorWithSerializeDeserialize < 'a >
// {
//     fn get_code_occurence_with_serialize_deserialize(& self) -> & crate ::
//     common :: code_occurence :: CodeOccurenceWithSerializeDeserialize
//     {
//         match self
//         {
//             SerializeDeserializeDisplayErrorWithSerializeDeserialize ::
//             Something
//             { display_without_ser_deser : _unused_argument_0, code_occurence }
//             => { code_occurence }
//         }
//     }
// } impl < 'a > SerializeDeserializeDisplayError < 'a >
// {
//     pub fn into_serialize_deserialize_version(self) ->
//     SerializeDeserializeDisplayErrorWithSerializeDeserialize < 'a >
//     {
//         match self
//         {
//             SerializeDeserializeDisplayError :: Something
//             { display_without_ser_deser, code_occurence } =>
//             {
//                 SerializeDeserializeDisplayErrorWithSerializeDeserialize ::
//                 Something
//                 {
//                     display_without_ser_deser : { display_without_ser_deser },
//                     code_occurence :
//                     code_occurence.into_serialize_deserialize_version()
//                 }
//             }
//         }
//     }
// } impl < 'a > std :: fmt :: Display for SerializeDeserializeDisplayError < 'a
// >
// {
//     fn fmt(& self, f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result
//     {
//         use crate :: traits :: error_logs_logic :: to_string_without_config ::
//         ToStringWithoutConfig ; write!
//         (f, "{}", self.to_string_without_config())
//     }
// } impl < 'a > std :: fmt :: Display for
// SerializeDeserializeDisplayErrorWithSerializeDeserialize < 'a >
// {
//     fn fmt(& self, f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result
//     {
//         use crate :: traits :: error_logs_logic :: to_string_without_config ::
//         ToStringWithoutConfigWithSerializeDeserialize ; write!
//         (f, "{}", self.to_string_without_config_with_serialize_deserialize())
//     }
// } impl < 'a > crate :: traits :: error_logs_logic :: error_occurence_named ::
// ErrorOccurenceNamed for SerializeDeserializeDisplayError < 'a >
// { fn error_occurence_named(& self) -> () { () } } impl < 'a >
// SerializeDeserializeDisplayError < 'a >
// {
//     fn compile_time_check_error_occurence_members(& self)
//     {
//         match self
//         {
//             SerializeDeserializeDisplayError :: Something
//             {
//                 display_without_ser_deser : _unused_argument_0, code_occurence
//                 : _unused_argument_1
//             } => {}
//         }
//     }
// }

pub fn dev() {
    // let e = SerializeDeserializeDisplayError::Something {
    //     // display_without_ser_deser: DisplayWithoutSerializeDeserializeStruct{ three: String::from("DisplayWithoutSerializeDeserializeStruct") },
    //     serialize_deserialize_struct: SerializeDeserializeStruct{
    //         one: String::from("one")
    //     },
    //     vec_serialize_deserialize_struct: vec![SerializeDeserializeStruct{
    //         one: String::from("one")
    //     }],
    //     hashmap_serialize_deserialize_struct: std::collections::HashMap::from([(
    //         String::from("key"),
    //         SerializeDeserializeStruct{
    //             one: String::from("one")
    //         }
    //     )]),
    //     hashmap_second_serialize_deserialize_struct: std::collections::HashMap::from([(
    //         DisplayForeignTypeStruct{
    //             two: String::from("two")
    //         },
    //         SerializeDeserializeStruct{
    //             one: String::from("one")
    //         }
    //     )]),
    //     display_foreign_type_struct: DisplayForeignTypeStruct{
    //         two: String::from("two")
    //     },
    //     code_occurence: crate::code_occurence_tufa_common!(),
    // };
    // //
    // println!("{}", e);
    // use crate::traits::error_logs_logic::error_log::ErrorLog;
    // e.error_log(once_cell::sync::Lazy::force(
    //     &crate::global_variables::runtime::config::CONFIG,
    // ));
    // // println!("---------------------------------------");
    // let ed = e.into_serialize_deserialize_version();
    // println!("{ed}");
    // // println!("---------------------------------------");
    // let xs = serde_json::to_string(&ed).unwrap();
    // println!("serializes into string {}", xs);
    // println!("---------------------------------------");
    // let xd: SerializeDeserializeDisplayErrorWithSerializeDeserialize = serde_json::from_str(&xs).unwrap();
    // println!("after deserialize \n{xd}");
    // println!("---------------------------------------");
    //
    // let user = User {
    //     name: "Arwen Undomiel".to_string(),
    //     age: 3000,
    //     something: vec![true, false],
    //     address: Address {
    //         country: "Middle Earth".to_string(),
    //         city: "Rivendell".to_string(),
    //         street: "leafy lane".to_string(),
    //     },
    // };
    // tracing::error!(valuable = false, user = ?user);
    // // if let Err(e) = test_fn() {
    // //     println!("{}", *e);
    // //     use crate::traits::error_logs_logic::error_log::ErrorLog;
    // //     e.error_log(once_cell::sync::Lazy::force(
    // //         &crate::global_variables::runtime::config::CONFIG,
    // //     ));
    // //     let ed = e.into_serialize_deserialize_version();
    // //     println!("{ed}");
    // //     let xs = serde_json::to_string(&ed).unwrap();
    // //     println!("serializes into string {}", xs);
    // //     let xd: TestErrorWithSerializeDeserialize = serde_json::from_str(&xs).unwrap();
    // //     println!("after deserialize \n{xd}");
    // // }
    if let Err(e) = named() {
        // println!("---------------------------------------");
        println!("{}", *e);
        use crate::traits::error_logs_logic::error_log::ErrorLog;
        e.error_log(once_cell::sync::Lazy::force(
            &crate::global_variables::runtime::config::CONFIG,
        ));
        // println!("---------------------------------------");
        let ed = e.into_serialize_deserialize_version();
        println!("{ed}");
        // println!("---------------------------------------");
        let xs = serde_json::to_string(&ed).unwrap();
        println!("serializes into string {}", xs);
        println!("---------------------------------------");
        let xd: NamedErrorWithSerializeDeserialize = serde_json::from_str(&xs).unwrap();
        println!("after deserialize \n{xd}");
        println!("---------------------------------------");
    }
}

//////////////////////////////////////////

pub fn named<'a>() -> Result<(), Box<NamedError<'a>>> {
    return Err(Box::new(NamedError::Something {
        a: crate::dev::Omegalul {},
        b: crate::dev::OmegalulLifetime {
            s: "omegalullifetime",
        },
        c: crate::dev::Kekw {},
        d: crate::dev::KekwLifetime { s: "kekwlifetime" },
        e: crate::dev::SevenError::Something {
            error: String::from("seven_error"),
            omega: OmegalulLifetime {
                s: "omegalllil",
            },
            code_occurence: crate::code_occurence_tufa_common!(),
        },
        f: vec![crate::dev::Omegalul {}],
        g: vec![crate::dev::OmegalulLifetime {
            s: "omegalullifetime",
        }],
        h: vec![crate::dev::Kekw {}],
        j: vec![crate::dev::KekwLifetime { s: "kekwlifetime" }],
        k: vec![
            crate::dev::SevenErrorEnum::Something(
                crate::dev::SevenError::Something {
                    error: String::from("seven_error"),
                    omega: OmegalulLifetime {
                        s: "omegalllil",
                    },
                    code_occurence: crate::code_occurence_tufa_common!(),
                }
            )
        ],
        //////////////////////////////////``
        // l: std::collections::HashMap::from([(crate::dev::Omegalul {}, crate::dev::Omegalul {})]),
        // m: std::collections::HashMap::from([(
        //     crate::dev::Omegalul {},
        //     crate::dev::OmegalulLifetime {
        //         s: "omegalullifetime",
        //     },
        // )]),
        // n: std::collections::HashMap::from([(
        //     crate::dev::OmegalulLifetime {
        //         s: "omegalullifetime",
        //     },
        //     crate::dev::Omegalul {},
        // )]),
        // o: std::collections::HashMap::from([(
        //     crate::dev::OmegalulLifetime {
        //         s: "omegalullifetime",
        //     },
        //     crate::dev::OmegalulLifetime {
        //         s: "omegalullifetime",
        //     },
        // )]),
        // p: std::collections::HashMap::from([(crate::dev::Omegalul {}, crate::dev::Kekw {})]),
        // q: std::collections::HashMap::from([(
        //     crate::dev::Omegalul {},
        //     crate::dev::KekwLifetime { s: "kekwlifetime" },
        // )]),
        // r: std::collections::HashMap::from([(
        //     crate::dev::OmegalulLifetime {
        //         s: "omegalullifetime",
        //     },
        //     crate::dev::Kekw {},
        // )]),
        // s: std::collections::HashMap::from([(
        //     crate::dev::OmegalulLifetime {
        //         s: "omegalullifetime",
        //     },
        //     crate::dev::KekwLifetime { s: "kekwlifetime" },
        // )]),
        // t: std::collections::HashMap::from([(
        //     crate::dev::Omegalul {},
        //     crate::dev::SevenError::Something {
        //         error: String::from("seven_error"),
        //         omega: OmegalulLifetime {
        //             s: "omegalllil",
        //         },
        //         code_occurence: crate::code_occurence_tufa_common!(),
        //     },
        // )]),
        // u: std::collections::HashMap::from([(
        //     crate::dev::OmegalulLifetime {
        //         s: "omegalullifetime",
        //     },
        //     crate::dev::SevenError::Something {
        //         error: String::from("seven_error"),
        //         omega: OmegalulLifetime {
        //             s: "omegalllil",
        //         },
        //         code_occurence: crate::code_occurence_tufa_common!(),
        //     },
        // )]),
        //////////////////////////////
        v: std::collections::HashMap::from([(crate::dev::Kekw {}, crate::dev::Omegalul {})]),
        w: std::collections::HashMap::from([(
            crate::dev::Kekw {},
            crate::dev::OmegalulLifetime {
                s: "omegalullifetime",
            },
        )]),
        x: std::collections::HashMap::from([(
            crate::dev::KekwLifetime { s: "kekwlifetime" },
            crate::dev::Omegalul {},
        )]),
        y: std::collections::HashMap::from([(
            crate::dev::KekwLifetime { s: "kekwlifetime" },
            crate::dev::OmegalulLifetime {
                s: "omegalullifetime",
            },
        )]),
        z: std::collections::HashMap::from([(crate::dev::Kekw {}, crate::dev::Kekw {})]),
        aa: std::collections::HashMap::from([(
            crate::dev::Kekw {},
            crate::dev::KekwLifetime { s: "kekwlifetime" },
        )]),
        ab: std::collections::HashMap::from([(
            crate::dev::KekwLifetime { s: "kekwlifetime" },
            crate::dev::Kekw {},
        )]),
        ac: std::collections::HashMap::from([(
            crate::dev::KekwLifetime { s: "kekwlifetime" },
            crate::dev::KekwLifetime { s: "kekwlifetime" },
        )]),
        ad: std::collections::HashMap::from([(
            crate::dev::Kekw {},
            crate::dev::SevenErrorEnum::Something(
                crate::dev::SevenError::Something {
                    error: String::from("seven_error"),
                    omega: OmegalulLifetime {
                        s: "omegalllil",
                    },
                    code_occurence: crate::code_occurence_tufa_common!(),
                },
            )
        )]),
        af: std::collections::HashMap::from([(
            crate::dev::KekwLifetime { s: "kekwlifetime" },
            crate::dev::SevenErrorEnum::Something(
                crate::dev::SevenError::Something {
                    error: String::from("seven_error"),
                    omega: OmegalulLifetime {
                        s: "omegalllil",
                    },
                    code_occurence: crate::code_occurence_tufa_common!(),
                },
            )
        )]),
        ag: vec![OneErrorEnum::ErrorOccurence(crate::dev::SevenError::Something{
            error: String::from("seven_error"),
            omega: OmegalulLifetime {
                s: "omegalllil",
            },
            code_occurence: crate::code_occurence_tufa_common!(),
        })
        ],
        ah: std::collections::HashMap::from([(
            crate::dev::KekwLifetime { s: "kekwlifetime" },
            crate::dev::Omegalul {},
        )]),
        ai: "aiaiaiaiaiaiaai",
        ak: std::collections::HashMap::from([(
            "akakakaakakakakakak",
            crate::dev::Omegalul {},
        )]),
        al: vec!["first_vec_elem", "second_vec_elem"],
        code_occurence: crate::code_occurence_tufa_common!(),
    }));
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)] //, error_occurence::ErrorOccurence
pub enum SevenError<'a> {
    Something {
        #[eo_display_with_serialize_deserialize]
        error: String,
        #[eo_display_with_serialize_deserialize]
        omega: OmegalulLifetime<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]//, error_occurence::ErrorOccurence
pub enum SevenErrorEnum<'a> {
    #[eo_error_occurence]
    Something(crate::dev::SevenError<'a>),
}

//////////////////////////

#[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct KekwLifetime<'a> {
    s: &'a str,
}

impl<'a> crate::traits::display_foreign_type::DisplayForeignType for KekwLifetime<'a> {
    fn display_foreign_type(&self) -> String {
        String::from("kekwlifetime")
    }
}

#[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Kekw {}

impl crate::traits::display_foreign_type::DisplayForeignType for Kekw {
    fn display_foreign_type(&self) -> String {
        String::from("kekw")
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

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)] //, error_occurence::ErrorOccurence
pub enum OneErrorEnum<'a> {
    // #[eo_display]
    // ToString(crate::dev::Omegalul),
    // #[eo_display]
    // ToStringLifetime(crate::dev::OmegalulLifetime<'a>),
    // #[eo_display_foreign_type]
    // DisplayForeignType(crate::dev::Kekw),
    // #[eo_display_foreign_type]
    // DisplayForeignTypeLifeTime(crate::dev::KekwLifetime<'a>),
    // #[eo_error_occurence]
    ErrorOccurence(crate::dev::SevenError<'a>),
    // #[eo_vec_display]
    // VecToString(std::vec::Vec<crate::dev::Omegalul>),
    // #[eo_vec_display_foreign_type]
    // VecDisplayForeignType(std::vec::Vec<crate::dev::Kekw>),
    // #[eo_vec_error_occurence]
    // VecErrorOccurence(std::vec::Vec<crate::dev::SevenError<'a>>),
    // #[eo_hashmap_key_display_value_display]
    // HashMapKeyToStringValueToString(
    //     std::collections::HashMap<crate::dev::Omegalul, crate::dev::Omegalul>,
    // ),
    // #[eo_hashmap_key_display_value_display_foreign_type]
    // HashMapKeyToStringValueDisplayForeignType(
    //     std::collections::HashMap<crate::dev::Omegalul, crate::dev::Kekw>,
    // ),
    // #[eo_hashmap_key_display_value_error_occurence]
    // HashMapKeyToStringValueErrorOccurence(
    //     std::collections::HashMap<crate::dev::Omegalul, crate::dev::SevenError<'a>>,
    // ),
    // #[eo_hashmap_key_display_foreign_type_value_display]
    // HashMapKeyDisplayForeignTypeValueToString(
    //     std::collections::HashMap<crate::dev::Kekw, crate::dev::Omegalul>,
    // ),
    // #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
    // HashMapKeyDisplayForeignTypeValueDisplayForeignType(
    //     std::collections::HashMap<crate::dev::Kekw, crate::dev::Kekw>,
    // ),
    // #[eo_hashmap_key_display_foreign_type_value_error_occurence]
    // HashMapKeyDisplayForeignTypeValueErrorOccurence(
    //     std::collections::HashMap<crate::dev::Kekw, crate::dev::SevenError<'a>>,
    // ),
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]//, error_occurence::ErrorOccurence
pub enum NamedError<'a> {//
    Something {
        #[eo_display_with_serialize_deserialize]
        a: crate::dev::Omegalul,
        #[eo_display_with_serialize_deserialize]
        b: crate::dev::OmegalulLifetime<'a>,
        #[eo_display_foreign_type]
        c: crate::dev::Kekw,
        #[eo_display_foreign_type]
        d: crate::dev::KekwLifetime<'a>,
        #[eo_error_occurence]
        e: crate::dev::SevenError<'a>,
        #[eo_vec_display_with_serialize_deserialize]
        f: std::vec::Vec<crate::dev::Omegalul>,
        #[eo_vec_display_with_serialize_deserialize]
        g: std::vec::Vec<crate::dev::OmegalulLifetime<'a>>,
        #[eo_vec_display_foreign_type]
        h: std::vec::Vec<crate::dev::Kekw>,
        #[eo_vec_display_foreign_type]
        j: std::vec::Vec<crate::dev::KekwLifetime<'a>>,
        #[eo_vec_error_occurence]
        k: std::vec::Vec<crate::dev::SevenErrorEnum<'a>>,
        //////////////////////////////
        // #[eo_hashmap_key_display_value_display]
        // l: std::collections::HashMap<crate::dev::Omegalul, crate::dev::Omegalul>,
        // #[eo_hashmap_key_display_value_display]
        // m: std::collections::HashMap<crate::dev::Omegalul, crate::dev::OmegalulLifetime<'a>>,
        // #[eo_hashmap_key_display_value_display]
        // n: std::collections::HashMap<crate::dev::OmegalulLifetime<'a>, crate::dev::Omegalul>,
        // #[eo_hashmap_key_display_value_display]
        // o: std::collections::HashMap<
        //     crate::dev::OmegalulLifetime<'a>,
        //     crate::dev::OmegalulLifetime<'a>,
        // >,
        // #[eo_hashmap_key_display_value_display_foreign_type]
        // p: std::collections::HashMap<crate::dev::Omegalul, crate::dev::Kekw>,
        // #[eo_hashmap_key_display_value_display_foreign_type]
        // q: std::collections::HashMap<crate::dev::Omegalul, crate::dev::KekwLifetime<'a>>,
        // #[eo_hashmap_key_display_value_display_foreign_type]
        // r: std::collections::HashMap<crate::dev::OmegalulLifetime<'a>, crate::dev::Kekw>,
        // #[eo_hashmap_key_display_value_display_foreign_type]
        // s: std::collections::HashMap<
        //     crate::dev::OmegalulLifetime<'a>,
        //     crate::dev::KekwLifetime<'a>,
        // >,
        // #[eo_hashmap_key_display_value_error_occurence]
        // t: std::collections::HashMap<crate::dev::Omegalul, crate::dev::SevenError<'a>>,
        // #[eo_hashmap_key_display_value_error_occurence]
        // u: std::collections::HashMap<crate::dev::OmegalulLifetime<'a>, crate::dev::SevenError<'a>>,
        ///////////////////////////////
        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        v: std::collections::HashMap<crate::dev::Kekw, crate::dev::Omegalul>,
        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        w: std::collections::HashMap<crate::dev::Kekw, crate::dev::OmegalulLifetime<'a>>,
        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        x: std::collections::HashMap<crate::dev::KekwLifetime<'a>, crate::dev::Omegalul>,
        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        y: std::collections::HashMap<
            crate::dev::KekwLifetime<'a>,
            crate::dev::OmegalulLifetime<'a>,
        >,
        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
        z: std::collections::HashMap<crate::dev::Kekw, crate::dev::Kekw>,
        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
        aa: std::collections::HashMap<crate::dev::Kekw, crate::dev::KekwLifetime<'a>>,
        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
        ab: std::collections::HashMap<crate::dev::KekwLifetime<'a>, crate::dev::Kekw>,
        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
        ac: std::collections::HashMap<crate::dev::KekwLifetime<'a>, crate::dev::KekwLifetime<'a>>,
        #[eo_hashmap_key_display_foreign_type_value_error_occurence]
        ad: std::collections::HashMap<crate::dev::Kekw, crate::dev::SevenErrorEnum<'a>>,
        #[eo_hashmap_key_display_foreign_type_value_error_occurence]
        af: std::collections::HashMap<crate::dev::KekwLifetime<'a>, crate::dev::SevenErrorEnum<'a>>,
        #[eo_vec_error_occurence]
        ag: std::vec::Vec<OneErrorEnum<'a>>,
        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        ah: std::collections::HashMap<crate::dev::KekwLifetime<'a>, crate::dev::Omegalul>,
        #[eo_display_with_serialize_deserialize]
        ai: &'a str,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        ak: std::collections::HashMap<&'a str, crate::dev::Omegalul>,
        #[eo_vec_display_with_serialize_deserialize]
        al: std::vec::Vec<&'a str>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}
////////////////////////////////////////////
