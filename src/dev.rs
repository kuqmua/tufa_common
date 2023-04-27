use valuable::Valuable;

#[derive(Clone, Debug, Valuable)]
struct User {
    name: String,
    age: u32,
    something: Vec<bool>,
    address: Address,
}

#[derive(Clone, Debug, Valuable)]
struct Address {
    country: String,
    city: String,
    street: String,
}

pub fn dev() {
    let user = User {
        name: "Arwen Undomiel".to_string(),
        age: 3000,
        something: vec![true, false],
        address: Address {
            country: "Middle Earth".to_string(),
            city: "Rivendell".to_string(),
            street: "leafy lane".to_string(),
        },
    };
    tracing::error!(valuable = false, user = ?user);
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
        // eo_display: crate::dev::DisplayStruct {
        //     display_struct: String::from("DisplayStruct")
        // },
        // eo_display_lifetime: crate::dev::DisplayStructLifetime {
        //     display_struct_lifetime: "DisplayStructLifetime"
        // },

        // eo_display_with_serialize_deserialize: crate::dev::DisplayWithSerializeDeserializeStruct {},
        // eo_display_with_serialize_deserialize_lifetime: crate::dev::DisplayWithSerializeDeserializeStructLifetime {
        //     display_with_serialize_deserialize_struct_lifetime: "DisplayWithSerializeDeserializeStructLifetime"
        // },

        // eo_display_foreign_type: crate::dev::DisplayForeignTypeStruct {
        //     display_foreign_type_struct: String::from("DisplayForeignTypeStruct")
        // },
        // eo_display_foreign_type_lifetime: crate::dev::DisplayForeignTypeStructLifetime {
        //     display_foreign_type_struct: "DisplayForeignTypeStructLifetime"
        // },

        // eo_display_foreign_type_with_serialize_deserialize: crate::dev::DisplayForeignTypeSerializeDeserializeStruct {
        //     display_foreign_type_serialize_deserialize_struct: String::from("DisplayForeignTypeSerializeDeserializeStruct")
        // },
        // eo_display_foreign_type_with_serialize_deserialize_lifetime: crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime {
        //     display_foreign_type_serialize_deserialize_struct: "DisplayForeignTypeSerializeDeserializeStructLifetime"
        // },

        // eo_error_occurence: crate::dev::ErrorOccurenceError::Something {
        //     string: String::from("String"),
        //     display_with_serialize_deserializeStructLifetime: crate::dev::DisplayWithSerializeDeserializeStructLifetime {
        //         display_with_serialize_deserialize_struct_lifetime: "DisplayWithSerializeDeserializeStructLifetime",
        //     },
        //     code_occurence: crate::code_occurence_tufa_common!(),
        // },

        // eo_vec_display: vec![crate::dev::DisplayStruct { display_struct: String::from("DisplayStruct") }],
        // eo_vec_display_lifetime: vec![crate::dev::DisplayStructLifetime { display_struct_lifetime: "DisplayStructLifetime" }],

        // eo_vec_display_with_serialize_deserialize: vec![crate::dev::DisplayWithSerializeDeserializeStruct {}],
        // eo_vec_display_with_serialize_deserialize_lifetime: vec![crate::dev::DisplayWithSerializeDeserializeStructLifetime { display_with_serialize_deserialize_struct_lifetime: "DisplayWithSerializeDeserializeStructLifetime" }],

        // eo_vec_display_foreign_type: vec![crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: String::from("DisplayForeignTypeStruct") }],
        // eo_vec_display_foreign_type_lifetime: vec![crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "DisplayForeignTypeStructLifetime" }], 

        // eo_vec_display_foreign_type_with_serialize_deserialize: vec![crate::dev::DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: String::from("DisplayForeignTypeSerializeDeserializeStruct") }],
        // eo_vec_display_foreign_type_with_serialize_deserialize_lifetime: vec![crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime{ display_foreign_type_serialize_deserialize_struct: "DisplayForeignTypeSerializeDeserializeStructLifetime" }], 

        // eo_vec_error_occurence: vec![crate::dev::ErrorOccurenceErrorEnum::Something(crate::dev::ErrorOccurenceError::Something {
        //     string: String::from("String"),
        //     display_with_serialize_deserializeStructLifetime: crate::dev::DisplayWithSerializeDeserializeStructLifetime {
        //         display_with_serialize_deserialize_struct_lifetime: "DisplayWithSerializeDeserializeStructLifetime",
        //     },
        //     code_occurence: crate::code_occurence_tufa_common!(),
        // })],

        eo_hashmap_key_str_value_display: std::collections::HashMap::from([(
            "str",
            crate::dev::DisplayStruct { display_struct: String::from("DisplayStruct") },
        )]),
        eo_hashmap_key_string_value_display: std::collections::HashMap::from([(
            std::string::String::from("string"),
            crate::dev::DisplayStruct { display_struct: String::from("DisplayStruct") },
        )]),
        eo_hashmap_key_str_value_display_lifetime: std::collections::HashMap::from([(
            "str",
            crate::dev::DisplayStructLifetime { display_struct_lifetime: "DisplayStructLifetime" },
        )]),
        eo_hashmap_key_string_value_display_lifetime: std::collections::HashMap::from([(
            std::string::String::from("string"),
            crate::dev::DisplayStructLifetime { display_struct_lifetime: "DisplayStructLifetime" },
        )]),

        eo_hashmap_key_str_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
            "str",
            crate::dev::DisplayWithSerializeDeserializeStruct {},
        )]),
        eo_hashmap_key_string_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
            std::string::String::from("string"),
            crate::dev::DisplayWithSerializeDeserializeStruct {},
        )]),
        eo_hashmap_key_str_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
            "str",
            crate::dev::DisplayWithSerializeDeserializeStructLifetime {
                display_with_serialize_deserialize_struct_lifetime: "DisplayWithSerializeDeserializeStructLifetime",
            },
        )]),
        eo_hashmap_key_string_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
            std::string::String::from("string"),
            crate::dev::DisplayWithSerializeDeserializeStructLifetime {
                display_with_serialize_deserialize_struct_lifetime: "DisplayWithSerializeDeserializeStructLifetime",
            },
        )]),

        eo_hashmap_key_str_value_display_foreign_type: std::collections::HashMap::from([(
            "str",
            crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: String::from("DisplayForeignTypeStruct") },
        )]),
        eo_hashmap_key_string_value_display_foreign_type: std::collections::HashMap::from([(
            std::string::String::from("string"),
            crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: String::from("DisplayForeignTypeStruct") },
        )]),
        eo_hashmap_key_str_value_display_foreign_type_lifetime: std::collections::HashMap::from([(
            "str",
            crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "DisplayForeignTypeStructLifetime" },
        )]),
        eo_hashmap_key_string_value_display_foreign_type_lifetime: std::collections::HashMap::from([(
            std::string::String::from("string"),
            crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "DisplayForeignTypeStructLifetime" },
        )]),

        eo_hashmap_key_str_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap::from([(
            "str",
            crate::dev::DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: String::from("DisplayForeignTypeSerializeDeserializeStruct") },
        )]),
        eo_hashmap_key_string_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap::from([(
            std::string::String::from("string"),
            crate::dev::DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: String::from("DisplayForeignTypeSerializeDeserializeStruct") },
        )]),
        eo_hashmap_key_str_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
            "str",
            crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime { display_foreign_type_serialize_deserialize_struct: "DisplayForeignTypeSerializeDeserializeStructLifetime" },
        )]),
        eo_hashmap_key_string_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
            std::string::String::from("string"),
            crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime { display_foreign_type_serialize_deserialize_struct: "DisplayForeignTypeSerializeDeserializeStructLifetime" },
        )]),

        eo_hashmap_key_str_value_error_occurence: std::collections::HashMap::from([(
            "str",
            crate::dev::ErrorOccurenceErrorEnum::Something(crate::dev::ErrorOccurenceError::Something {
                string: String::from("String"),
                display_with_serialize_deserializeStructLifetime: crate::dev::DisplayWithSerializeDeserializeStructLifetime {
                    display_with_serialize_deserialize_struct_lifetime: "DisplayWithSerializeDeserializeStructLifetime",
                },
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        )]),
        eo_hashmap_key_string_value_error_occurence: std::collections::HashMap::from([(
            std::string::String::from("string"),
            crate::dev::ErrorOccurenceErrorEnum::Something(crate::dev::ErrorOccurenceError::Something {
                string: String::from("String"),
                display_with_serialize_deserializeStructLifetime: crate::dev::DisplayWithSerializeDeserializeStructLifetime {
                    display_with_serialize_deserialize_struct_lifetime: "DisplayWithSerializeDeserializeStructLifetime",
                },
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        )]),

        // eo_hashmap_key_display_foreign_type_value_display: std::collections::HashMap::from([(
        //     crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: String::from("DisplayForeignTypeStruct") },
        //     crate::dev::DisplayStruct { display_struct: String::from("DisplayStruct") },
        // )]),
        // eo_hashmap_key_display_foreign_type_lifetime_value_display: std::collections::HashMap::from([(
        //     crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "DisplayForeignTypeStructLifetime" },
        //     crate::dev::DisplayStruct { display_struct: String::from("DisplayStruct") },
        // )]),
        // eo_hashmap_key_display_foreign_type_value_display_lifetime: std::collections::HashMap::from([(
        //     crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: String::from("DisplayForeignTypeStruct") },
        //     crate::dev::DisplayStructLifetime { display_struct_lifetime: "DisplayStructLifetime" },
        // )]),
        // eo_hashmap_key_display_foreign_type_lifetime_value_display_lifetime: std::collections::HashMap::from([(
        //     crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "DisplayForeignTypeStructLifetime" },
        //     crate::dev::DisplayStructLifetime { display_struct_lifetime: "DisplayStructLifetime" },
        // )]),

        // eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
        //     crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: String::from("DisplayForeignTypeStruct") },
        //     crate::dev::DisplayWithSerializeDeserializeStruct{},
        // )]),
        // eo_hashmap_key_display_foreign_type_lifetime_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
        //     crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "DisplayForeignTypeStructLifetime" },
        //     crate::dev::DisplayWithSerializeDeserializeStruct{},
        // )]),
        // eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
        //     crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: String::from("DisplayForeignTypeStruct") },
        //     crate::dev::DisplayWithSerializeDeserializeStructLifetime{ display_with_serialize_deserialize_struct_lifetime: "DisplayWithSerializeDeserializeStructLifetime" },
        // )]),
        // eo_hashmap_key_display_foreign_type_lifetime_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
        //     crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "DisplayForeignTypeStructLifetime" },
        //     crate::dev::DisplayWithSerializeDeserializeStructLifetime{ display_with_serialize_deserialize_struct_lifetime: "DisplayWithSerializeDeserializeStructLifetime" },
        // )]),

        // eo_hashmap_key_display_foreign_type_value_display_foreign_type: std::collections::HashMap::from([(
        //     crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: String::from("DisplayForeignTypeStruct") },
        //     crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: String::from("DisplayForeignTypeStruct") },
        // )]),
        // eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type: std::collections::HashMap::from([(
        //     crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "DisplayForeignTypeStructLifetime" },
        //     crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: String::from("DisplayForeignTypeStruct") },
        // )]),
        // eo_hashmap_key_display_foreign_type_value_display_foreign_type_lifetime: std::collections::HashMap::from([(
        //     crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: String::from("DisplayForeignTypeStruct") },
        //     crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "DisplayForeignTypeStructLifetime" },
        // )]),
        // eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_lifetime: std::collections::HashMap::from([(
        //     crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "DisplayForeignTypeStructLifetime" },
        //     crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "DisplayForeignTypeStructLifetime" },
        // )]),

        // eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap::from([(
        //     crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: String::from("DisplayForeignTypeStruct") },
        //     crate::dev::DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: String::from("DisplayForeignTypeSerializeDeserializeStruct") },
        // )]),
        // eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap::from([(
        //     crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "DisplayForeignTypeStructLifetime" },
        //     crate::dev::DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: String::from("DisplayForeignTypeSerializeDeserializeStruct") },
        // )]),
        // eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
        //     crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: String::from("DisplayForeignTypeStruct") },
        //     crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime { display_foreign_type_serialize_deserialize_struct: "DisplayForeignTypeSerializeDeserializeStructLifetime" },
        // )]),
        // eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
        //     crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "DisplayForeignTypeStructLifetime" },
        //     crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime { display_foreign_type_serialize_deserialize_struct: "DisplayForeignTypeSerializeDeserializeStructLifetime" },
        // )]),

        // eo_hashmap_key_display_foreign_type_value_error_occurence: std::collections::HashMap::from([(
        //     crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: String::from("DisplayForeignTypeStruct") },
        //     crate::dev::ErrorOccurenceErrorEnum::Something(crate::dev::ErrorOccurenceError::Something {
        //         string: String::from("String"),
        //         display_with_serialize_deserializeStructLifetime: crate::dev::DisplayWithSerializeDeserializeStructLifetime {
        //             display_with_serialize_deserialize_struct_lifetime: "DisplayWithSerializeDeserializeStructLifetime",
        //         },
        //         code_occurence: crate::code_occurence_tufa_common!(),
        //     }),
        // )]),
        // eo_hashmap_key_display_foreign_type_lifetime_value_error_occurence: std::collections::HashMap::from([(
        //     crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "DisplayForeignTypeStructLifetime" },
        //     crate::dev::ErrorOccurenceErrorEnum::Something(crate::dev::ErrorOccurenceError::Something {
        //         string: String::from("String"),
        //         display_with_serialize_deserializeStructLifetime: crate::dev::DisplayWithSerializeDeserializeStructLifetime {
        //             display_with_serialize_deserialize_struct_lifetime: "DisplayWithSerializeDeserializeStructLifetime",
        //         },
        //         code_occurence: crate::code_occurence_tufa_common!(),
        //     }),
        // )]),
//////////////////
        // display_with_ser_deser: String::from("displayserde"),
        // something: crate::dev::SevenError::Something {
        //     error: String::from("seven_error"),
        //     omega: OmegalulLifetime {
        //         s: "omegalllil",
        //     },
        //     code_occurence: crate::code_occurence_tufa_common!(),
        // },
        // display_without_ser_deser: DisplayStruct{ three: String::from("DisplayStruct") },
        // serialize_deserialize_struct: SerializeDeserializeStruct{
        //     one: String::from("one")
        // },
        // vec_serialize_deserialize_struct: vec![SerializeDeserializeStruct{
        //     one: String::from("one")
        // }],
        // hashmap_serialize_deserialize_struct: std::collections::HashMap::from([(
        //     String::from("key"),
        //     SerializeDeserializeStruct{
        //         one: String::from("one")
        //     }
        // )]),
        // hashmap_second_serialize_deserialize_struct: std::collections::HashMap::from([(
        //     DisplayForeignTypeStruct{
        //         two: String::from("two")
        //     },
        //     SerializeDeserializeStruct{
        //         one: String::from("one")
        //     }
        // )]),
        // display_foreign_type_struct: DisplayForeignTypeStruct{
        //     two: String::from("two")
        // },
        // display_without_serialize_deserialize_struct: DisplayStruct {
        //     three: String::from("three"),
        // }, 
        // eo_vec_display: vec![DisplayStruct {
        //     three: String::from("three"),
        // }],
        // eo_hashmap_key_display_with_serialize_deserialize_value_display: std::collections::HashMap::from([(
        //     String::from("key"),
        //     DisplayStruct {
        //         three: String::from("three"),
        //     }
        // ),
        // (
        //     String::from("key2"),
        //     DisplayStruct {
        //         three: String::from("three2"),
        //     }
        // )//todo - change how it shows in console
        
        // ]),
        // //
        // eo_hashmap_key_display_foreign_type_value_display: std::collections::HashMap::from([
        // (
        //     crate::dev::KekwLifetime { s: "kekwlifetime1" },
        //     DisplayStruct {
        //         three: String::from("three"),
        //     }
        // ),
        // (
        //     crate::dev::KekwLifetime { s: "kekwlifetime2" },
        //     DisplayStruct {
        //         three: String::from("three2"),
        //     }
        // )//todo - change how it shows in console
        // ]),
        //

        // a: crate::dev::Omegalul {},
        // b: crate::dev::OmegalulLifetime {
        //     s: "omegalullifetime",
        // },
        // c: crate::dev::Kekw {},
        // d: crate::dev::KekwLifetime { s: "kekwlifetime" },
        // e: crate::dev::SevenError::Something {
        //     error: String::from("seven_error"),
        //     omega: OmegalulLifetime {
        //         s: "omegalllil",
        //     },
        //     code_occurence: crate::code_occurence_tufa_common!(),
        // },
        // f: vec![crate::dev::Omegalul {}],
        // g: vec![crate::dev::OmegalulLifetime {
        //     s: "omegalullifetime",
        // }],
        // h: vec![crate::dev::Kekw {}],
        // j: vec![crate::dev::KekwLifetime { s: "kekwlifetime" }],
        // k: vec![
        //     crate::dev::SevenErrorEnum::Something(
        //         crate::dev::SevenError::Something {
        //             error: String::from("seven_error"),
        //             omega: OmegalulLifetime {
        //                 s: "omegalllil",
        //             },
        //             code_occurence: crate::code_occurence_tufa_common!(),
        //         }
        //     )
        // ],
        // //////////////////////////////
        // v: std::collections::HashMap::from([(crate::dev::Kekw {}, crate::dev::Omegalul {})]),
        // w: std::collections::HashMap::from([(
        //     crate::dev::Kekw {},
        //     crate::dev::OmegalulLifetime {
        //         s: "omegalullifetime",
        //     },
        // )]),
        // x: std::collections::HashMap::from([(
        //     crate::dev::KekwLifetime { s: "kekwlifetime" },
        //     crate::dev::Omegalul {},
        // )]),
        // y: std::collections::HashMap::from([(
        //     crate::dev::KekwLifetime { s: "kekwlifetime" },
        //     crate::dev::OmegalulLifetime {
        //         s: "omegalullifetime",
        //     },
        // )]),
        // z: std::collections::HashMap::from([(crate::dev::Kekw {}, crate::dev::Kekw {})]),
        // aa: std::collections::HashMap::from([(
        //     crate::dev::Kekw {},
        //     crate::dev::KekwLifetime { s: "kekwlifetime" },
        // )]),
        // ab: std::collections::HashMap::from([(
        //     crate::dev::KekwLifetime { s: "kekwlifetime" },
        //     crate::dev::Kekw {},
        // )]),
        // ac: std::collections::HashMap::from([(
        //     crate::dev::KekwLifetime { s: "kekwlifetime" },
        //     crate::dev::KekwLifetime { s: "kekwlifetime" },
        // )]),
        // ad: std::collections::HashMap::from([(
        //     crate::dev::Kekw {},
        //     crate::dev::SevenErrorEnum::Something(
        //         crate::dev::SevenError::Something {
        //             error: String::from("seven_error"),
        //             omega: OmegalulLifetime {
        //                 s: "omegalllil",
        //             },
        //             code_occurence: crate::code_occurence_tufa_common!(),
        //         },
        //     )
        // )]),
        // af: std::collections::HashMap::from([(
        //     crate::dev::KekwLifetime { s: "kekwlifetime" },
        //     crate::dev::SevenErrorEnum::Something(
        //         crate::dev::SevenError::Something {
        //             error: String::from("seven_error"),
        //             omega: OmegalulLifetime {
        //                 s: "omegalllil",
        //             },
        //             code_occurence: crate::code_occurence_tufa_common!(),
        //         },
        //     )
        // )]),
        // ag: vec![OneErrorEnum::ErrorOccurence(crate::dev::SevenError::Something{
        //     error: String::from("seven_error"),
        //     omega: OmegalulLifetime {
        //         s: "omegalllil",
        //     },
        //     code_occurence: crate::code_occurence_tufa_common!(),
        // })
        // ],
        // ah: std::collections::HashMap::from([(
        //     crate::dev::KekwLifetime { s: "kekwlifetime" },
        //     crate::dev::Omegalul {},
        // )]),
        // ai: "aiaiaiaiaiaiaai",
        // ak: std::collections::HashMap::from([(
        //     "akakakaakakakakakak",
        //     crate::dev::Omegalul {},
        // )]),
        // al: vec!["first_vec_elem", "second_vec_elem"],
        code_occurence: crate::code_occurence_tufa_common!(),
    }));
}

#[derive(Debug)]
pub struct DisplayStruct {
    display_struct: String
}

impl std::fmt::Display for DisplayStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.display_struct)
    }
}

#[derive(Debug)]
pub struct DisplayStructLifetime<'a> {
    display_struct_lifetime: &'a str
}

impl<'a> std::fmt::Display for DisplayStructLifetime<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.display_struct_lifetime)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct DisplayWithSerializeDeserializeStruct {}

impl std::fmt::Display for DisplayWithSerializeDeserializeStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DisplayWithSerializeDeserializeStruct")
    }
}

#[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct DisplayWithSerializeDeserializeStructLifetime<'a> {
    display_with_serialize_deserialize_struct_lifetime: &'a str,
}

impl<'a> std::fmt::Display for DisplayWithSerializeDeserializeStructLifetime<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DisplayWithSerializeDeserializeStructLifetime")
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DisplayForeignTypeSerializeDeserializeStruct {
    display_foreign_type_serialize_deserialize_struct: String
}
impl crate::traits::display_foreign_type::DisplayForeignType
    for DisplayForeignTypeSerializeDeserializeStruct
{
    fn display_foreign_type(&self) -> String {
        String::from("DisplayForeignTypeSerializeDeserializeStruct")
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct DisplayForeignTypeSerializeDeserializeStructLifetime<'a> {
    display_foreign_type_serialize_deserialize_struct: &'a str
}
impl<'a> crate::traits::display_foreign_type::DisplayForeignType
    for DisplayForeignTypeSerializeDeserializeStructLifetime<'a>
{
    fn display_foreign_type(&self) -> String {
        String::from("DisplayForeignTypeSerializeDeserializeStructLifetime")
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct DisplayForeignTypeStruct {
    display_foreign_type_struct: String
}
impl crate::traits::display_foreign_type::DisplayForeignType
    for DisplayForeignTypeStruct
{
    fn display_foreign_type(&self) -> String {
        String::from("DisplayForeignTypeStruct")
    }
}
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct DisplayForeignTypeStructLifetime<'a> {
    display_foreign_type_struct: &'a str
}
impl<'a> crate::traits::display_foreign_type::DisplayForeignType
    for DisplayForeignTypeStructLifetime<'a>
{
    fn display_foreign_type(&self) -> String {
        String::from("DisplayForeignTypeStruct")
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)] //, error_occurence::ErrorOccurence
pub enum ErrorOccurenceError<'a> {
    Something {
        #[eo_display_with_serialize_deserialize]
        string: String,
        #[eo_display_with_serialize_deserialize]
        display_with_serialize_deserializeStructLifetime: DisplayWithSerializeDeserializeStructLifetime<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]//, error_occurence::ErrorOccurence
pub enum ErrorOccurenceErrorEnum<'a> {
    #[eo_error_occurence]
    Something(crate::dev::ErrorOccurenceError<'a>),
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]//, error_occurence::ErrorOccurence
pub enum NamedError<'a> {//
    Something {
        // #[eo_display_with_serialize_deserialize]
        // lifetime_str: &'a str,

        // #[eo_display]
        // eo_display: crate::dev::DisplayStruct,
        // #[eo_display]
        // eo_display_lifetime: crate::dev::DisplayStructLifetime<'a>,

        // #[eo_display_with_serialize_deserialize]
        // eo_display_with_serialize_deserialize: crate::dev::DisplayWithSerializeDeserializeStruct,
        // #[eo_display_with_serialize_deserialize]
        // eo_display_with_serialize_deserialize_lifetime: crate::dev::DisplayWithSerializeDeserializeStructLifetime<'a>,

        // #[eo_display_foreign_type]
        // eo_display_foreign_type: crate::dev::DisplayForeignTypeStruct,
        // #[eo_display_foreign_type]
        // eo_display_foreign_type_lifetime: crate::dev::DisplayForeignTypeStructLifetime<'a>,

        // #[eo_display_foreign_type_with_serialize_deserialize]
        // eo_display_foreign_type_with_serialize_deserialize: crate::dev::DisplayForeignTypeSerializeDeserializeStruct,
        // #[eo_display_foreign_type_with_serialize_deserialize]
        // eo_display_foreign_type_with_serialize_deserialize_lifetime: crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime<'a>,

        // #[eo_error_occurence]
        // eo_error_occurence: crate::dev::ErrorOccurenceError<'a>,

        // #[eo_vec_display]
        // eo_vec_display: std::vec::Vec<crate::dev::DisplayStruct>,
        // #[eo_vec_display]
        // eo_vec_display_lifetime: std::vec::Vec<crate::dev::DisplayStructLifetime<'a>>,

        // #[eo_vec_display_with_serialize_deserialize]
        // eo_vec_display_with_serialize_deserialize: std::vec::Vec<crate::dev::DisplayWithSerializeDeserializeStruct>,
        // #[eo_vec_display_with_serialize_deserialize]
        // eo_vec_display_with_serialize_deserialize_lifetime: std::vec::Vec<crate::dev::DisplayWithSerializeDeserializeStructLifetime<'a>>,

        // #[eo_vec_display_foreign_type]
        // eo_vec_display_foreign_type: std::vec::Vec<crate::dev::DisplayForeignTypeStruct>,
        // #[eo_vec_display_foreign_type]
        // eo_vec_display_foreign_type_lifetime: std::vec::Vec<crate::dev::DisplayForeignTypeStructLifetime<'a>>,

        // #[eo_vec_display_foreign_type_with_serialize_deserialize]
        // eo_vec_display_foreign_type_with_serialize_deserialize: std::vec::Vec<crate::dev::DisplayForeignTypeSerializeDeserializeStruct>,
        // #[eo_vec_display_foreign_type_with_serialize_deserialize]
        // eo_vec_display_foreign_type_with_serialize_deserialize_lifetime: std::vec::Vec<crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime<'a>>,

        // #[eo_vec_error_occurence]
        // eo_vec_error_occurence: std::vec::Vec<crate::dev::ErrorOccurenceErrorEnum<'a>>,

        #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
        eo_hashmap_key_str_value_display: std::collections::HashMap<&'a str, crate::dev::DisplayStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
        eo_hashmap_key_string_value_display: std::collections::HashMap<std::string::String, crate::dev::DisplayStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
        eo_hashmap_key_str_value_display_lifetime: std::collections::HashMap<&'a str, crate::dev::DisplayStructLifetime<'a>>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
        eo_hashmap_key_string_value_display_lifetime: std::collections::HashMap<std::string::String, crate::dev::DisplayStructLifetime<'a>>,

        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        eo_hashmap_key_str_value_display_with_serialize_deserialize: std::collections::HashMap<&'a str, crate::dev::DisplayWithSerializeDeserializeStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        eo_hashmap_key_string_value_display_with_serialize_deserialize: std::collections::HashMap<std::string::String, crate::dev::DisplayWithSerializeDeserializeStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        eo_hashmap_key_str_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<&'a str, crate::dev::DisplayWithSerializeDeserializeStructLifetime<'a>>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        eo_hashmap_key_string_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<std::string::String, crate::dev::DisplayWithSerializeDeserializeStructLifetime<'a>>,

        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type]
        eo_hashmap_key_str_value_display_foreign_type: std::collections::HashMap<&'a str, crate::dev::DisplayForeignTypeStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type]
        eo_hashmap_key_string_value_display_foreign_type: std::collections::HashMap<std::string::String, crate::dev::DisplayForeignTypeStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type]
        eo_hashmap_key_str_value_display_foreign_type_lifetime: std::collections::HashMap<&'a str, crate::dev::DisplayForeignTypeStructLifetime<'a>>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type]
        eo_hashmap_key_string_value_display_foreign_type_lifetime: std::collections::HashMap<std::string::String, crate::dev::DisplayForeignTypeStructLifetime<'a>>,

        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize]
        eo_hashmap_key_str_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap<&'a str, crate::dev::DisplayForeignTypeSerializeDeserializeStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize]
        eo_hashmap_key_string_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap<std::string::String, crate::dev::DisplayForeignTypeSerializeDeserializeStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize]
        eo_hashmap_key_str_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap<&'a str, crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime<'a>>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize]
        eo_hashmap_key_string_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap<std::string::String, crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime<'a>>,

        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        eo_hashmap_key_str_value_error_occurence: std::collections::HashMap<&'a str, crate::dev::ErrorOccurenceErrorEnum<'a>>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        eo_hashmap_key_string_value_error_occurence: std::collections::HashMap<std::string::String, crate::dev::ErrorOccurenceErrorEnum<'a>>,

        // #[eo_hashmap_key_display_foreign_type_value_display]
        // eo_hashmap_key_display_foreign_type_value_display: std::collections::HashMap<crate::dev::DisplayForeignTypeStruct, crate::dev::DisplayStruct>,
        // #[eo_hashmap_key_display_foreign_type_value_display]
        // eo_hashmap_key_display_foreign_type_lifetime_value_display: std::collections::HashMap<crate::dev::DisplayForeignTypeStructLifetime<'a>, crate::dev::DisplayStruct>,
        // #[eo_hashmap_key_display_foreign_type_value_display]
        // eo_hashmap_key_display_foreign_type_value_display_lifetime: std::collections::HashMap<crate::dev::DisplayForeignTypeStruct, crate::dev::DisplayStructLifetime<'a>>,
        // #[eo_hashmap_key_display_foreign_type_value_display]
        // eo_hashmap_key_display_foreign_type_lifetime_value_display_lifetime: std::collections::HashMap<crate::dev::DisplayForeignTypeStructLifetime<'a>, crate::dev::DisplayStructLifetime<'a>>,

        // #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        // eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize: std::collections::HashMap<crate::dev::DisplayForeignTypeStruct, crate::dev::DisplayWithSerializeDeserializeStruct>,
        // #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        // eo_hashmap_key_display_foreign_type_lifetime_value_display_with_serialize_deserialize: std::collections::HashMap<crate::dev::DisplayForeignTypeStructLifetime<'a>, crate::dev::DisplayWithSerializeDeserializeStruct>,
        // #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        // eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<crate::dev::DisplayForeignTypeStruct, crate::dev::DisplayWithSerializeDeserializeStructLifetime<'a>>,
        // #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        // eo_hashmap_key_display_foreign_type_lifetime_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<
        //     crate::dev::DisplayForeignTypeStructLifetime<'a>,
        //     crate::dev::DisplayWithSerializeDeserializeStructLifetime<'a>,
        // >,

        // #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
        // eo_hashmap_key_display_foreign_type_value_display_foreign_type: std::collections::HashMap<crate::dev::DisplayForeignTypeStruct, crate::dev::DisplayForeignTypeStruct>,
        // #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
        // eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type: std::collections::HashMap<crate::dev::DisplayForeignTypeStructLifetime<'a>, crate::dev::DisplayForeignTypeStruct>,
        // #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
        // eo_hashmap_key_display_foreign_type_value_display_foreign_type_lifetime: std::collections::HashMap<crate::dev::DisplayForeignTypeStruct, crate::dev::DisplayForeignTypeStructLifetime<'a>>,
        // #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
        // eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_lifetime: std::collections::HashMap<crate::dev::DisplayForeignTypeStructLifetime<'a>, crate::dev::DisplayForeignTypeStructLifetime<'a>>,

        // #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
        // eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap<crate::dev::DisplayForeignTypeStruct, crate::dev::DisplayForeignTypeSerializeDeserializeStruct>,
        // #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
        // eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap<crate::dev::DisplayForeignTypeStructLifetime<'a>, crate::dev::DisplayForeignTypeSerializeDeserializeStruct>,
        // #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
        // eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap<crate::dev::DisplayForeignTypeStruct, crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime<'a>>,
        // #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
        // eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap<crate::dev::DisplayForeignTypeStructLifetime<'a>, crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime<'a>>,

        // #[eo_hashmap_key_display_foreign_type_value_error_occurence]
        // eo_hashmap_key_display_foreign_type_value_error_occurence: std::collections::HashMap<crate::dev::DisplayForeignTypeStruct, crate::dev::ErrorOccurenceErrorEnum<'a>>,
        // #[eo_hashmap_key_display_foreign_type_value_error_occurence]
        // eo_hashmap_key_display_foreign_type_lifetime_value_error_occurence: std::collections::HashMap<crate::dev::DisplayForeignTypeStructLifetime<'a>, crate::dev::ErrorOccurenceErrorEnum<'a>>,

///////////////////////////////////////////////////////////////////////////

// //
//         // #[eo_display_with_serialize_deserialize]
//         // display_with_ser_deser: String,
//         // #[eo_error_occurence]
//         // something: crate::dev::SevenError<'a>,

//         // #[eo_display_foreign_type_with_serialize_deserialize]
//         // serialize_deserialize_struct: SerializeDeserializeStruct,
//         // #[eo_vec_display_foreign_type_with_serialize_deserialize]
//         // vec_serialize_deserialize_struct: Vec<SerializeDeserializeStruct>,


//         // #[eo_display_foreign_type]
//         // display_foreign_type_struct: DisplayForeignTypeStruct,
//         // #[eo_display]
//         // display_without_serialize_deserialize_struct: DisplayStruct,
//         // #[eo_vec_display]
//         // eo_vec_display: Vec<DisplayStruct>,
//         // #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
//         // eo_hashmap_key_display_with_serialize_deserialize_value_display: std::collections::HashMap<String, DisplayStruct>,
//         // #[eo_hashmap_key_display_foreign_type_value_display]
//         // eo_hashmap_key_display_foreign_type_value_display: std::collections::HashMap<crate::dev::KekwLifetime<'a>, DisplayStruct>,
// //

//         #[eo_display_with_serialize_deserialize]
//         a: crate::dev::Omegalul,
//         #[eo_display_with_serialize_deserialize]
//         b: crate::dev::OmegalulLifetime<'a>,
//         #[eo_display_foreign_type]
//         c: crate::dev::Kekw,
//         #[eo_display_foreign_type]
//         d: crate::dev::KekwLifetime<'a>,
//         #[eo_error_occurence]
//         e: crate::dev::SevenError<'a>,
//         #[eo_vec_display_with_serialize_deserialize]
//         f: std::vec::Vec<crate::dev::Omegalul>,
//         #[eo_vec_display_with_serialize_deserialize]
//         g: std::vec::Vec<crate::dev::OmegalulLifetime<'a>>,
//         #[eo_vec_display_foreign_type]
//         h: std::vec::Vec<crate::dev::Kekw>,
//         #[eo_vec_display_foreign_type]
//         j: std::vec::Vec<crate::dev::KekwLifetime<'a>>,
//         //////////////////////////////

//         #[eo_vec_error_occurence]
//         ag: std::vec::Vec<OneErrorEnum<'a>>,

//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         ak: std::collections::HashMap<&'a str, crate::dev::Omegalul>,
//         #[eo_vec_display_with_serialize_deserialize]
//         al: std::vec::Vec<&'a str>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
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
    ErrorOccurence(crate::dev::ErrorOccurenceError<'a>),
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
