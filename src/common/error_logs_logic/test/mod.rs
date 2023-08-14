// #[test]
// pub fn test_code_occurence() {
//     let git_info = crate::common::git::git_info::GitInfo::default();
//     let code_occurence = crate::common::code_occurence::CodeOccurence::new(
//         git_info.to_git_info_without_lifetime(),
//         file!().to_string(),
//         line!(),
//         column!(),
//         {
//             crate::common::config::config_fields::GetServerPort::get_server_port(
//                 *crate::config_mods::config_struct::ConfigUnchecked::default(),
//             )
//         },
//     );
//     let e = ErrorNamed::Something {
//         eo_display: crate::common::error_logs_logic::test::DisplayStruct {
//             display_struct: std::string::String::from("String")
//         },
//         eo_display_lifetime: crate::common::error_logs_logic::test::DisplayStructLifetime {
//             display_struct_lifetime: std::string::String::from("str")
//         },

//         eo_str_display_with_serialize_deserialize: std::string::String::from("str"),
//         eo_string_display_with_serialize_deserialize: std::string::String::from("String"),
//         eo_display_with_serialize_deserialize: crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct {},
//         eo_display_with_serialize_deserialize_lifetime: crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
//             display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str")
//         },

//         eo_display_foreign_type: crate::common::error_logs_logic::test::DisplayForeignTypeStruct {
//             display_foreign_type_struct: std::string::String::from("String")
//         },
//         eo_display_foreign_type_lifetime: crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime {
//             display_foreign_type_struct: std::string::String::from("str")
//         },

//         eo_display_foreign_type_with_serialize_deserialize: crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct {
//             display_foreign_type_serialize_deserialize_struct: std::string::String::from("String")
//         },
//         eo_display_foreign_type_with_serialize_deserialize_lifetime: crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime {
//             display_foreign_type_serialize_deserialize_struct: std::string::String::from("str")
//         },

//         eo_error_occurence: crate::common::error_logs_logic::test::InnerErrorNamed::Something {
//             string: std::string::String::from("String"),
//             display_with_serialize_deserialize_struct_lifetime: crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
//                 display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//             },
//             code_occurence: code_occurence.clone(),
//         },

//         eo_vec_str_display_with_serialize_deserialize: vec![std::string::String::from("str")],
//         eo_vec_string_display_with_serialize_deserialize:  vec![std::string::String::from("String")],
//         eo_vec_display: vec![crate::common::error_logs_logic::test::DisplayStruct { display_struct: std::string::String::from("String") }],
//         eo_vec_display_lifetime: vec![crate::common::error_logs_logic::test::DisplayStructLifetime { display_struct_lifetime: std::string::String::from("str") }],

//         eo_vec_display_with_serialize_deserialize: vec![crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct {}],
//         eo_vec_display_with_serialize_deserialize_lifetime: vec![crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime { display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str") }],

//         eo_vec_display_foreign_type: vec![crate::common::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") }],
//         eo_vec_display_foreign_type_lifetime: vec![crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") }],

//         eo_vec_display_foreign_type_with_serialize_deserialize: vec![crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") }],
//         eo_vec_display_foreign_type_with_serialize_deserialize_lifetime: vec![crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime{ display_foreign_type_serialize_deserialize_struct: std::string::String::from("str") }],

//         eo_vec_error_occurence: vec![crate::common::error_logs_logic::test::ErrorUnnamed::Something(crate::common::error_logs_logic::test::InnerErrorNamed::Something {
//             string: std::string::String::from("String"),
//             display_with_serialize_deserialize_struct_lifetime: crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
//                 display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//             },
//             code_occurence: code_occurence.clone(),
//         })],

//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display: std::collections::HashMap::from([(
//             "str",
//             crate::common::error_logs_logic::test::DisplayStruct { display_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             crate::common::error_logs_logic::test::DisplayStruct { display_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display_lifetime: std::collections::HashMap::from([(
//             "str",
//             crate::common::error_logs_logic::test::DisplayStructLifetime { display_struct_lifetime: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display_lifetime: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             crate::common::error_logs_logic::test::DisplayStructLifetime { display_struct_lifetime: std::string::String::from("str") },
//         )]),

//         eo_hashmap_key_str_display_with_serialize_deserialize_value_str_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             "str",
//             "str",
//         )]),
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_str_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             "str",
//         )]),
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_string_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             "str",
//             std::string::String::from("String")
//         )]),
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_string_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             std::string::String::from("String")
//         )]),

//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             "str",
//             crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct {},
//         )]),
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct {},
//         )]),
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             "str",
//             crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
//                 display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//             },
//         )]),
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
//                 display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//             },
//         )]),

//         eo_hashmap_key_str_value_display_foreign_type: std::collections::HashMap::from([(
//             "str",
//             crate::common::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_string_value_display_foreign_type: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             crate::common::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_str_value_display_foreign_type_lifetime: std::collections::HashMap::from([(
//             "str",
//             crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_string_value_display_foreign_type_lifetime: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//         )]),

//         eo_hashmap_key_str_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap::from([(
//             "str",
//             crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_string_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_str_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             "str",
//             crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime { display_foreign_type_serialize_deserialize_struct: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_string_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime { display_foreign_type_serialize_deserialize_struct: std::string::String::from("str") },
//         )]),

//         eo_hashmap_key_str_display_with_serialize_deserialize_value_error_occurence: std::collections::HashMap::from([(
//             "str",
//             crate::common::error_logs_logic::test::ErrorUnnamed::Something(crate::common::error_logs_logic::test::InnerErrorNamed::Something {
//                 string: std::string::String::from("String"),
//                 display_with_serialize_deserialize_struct_lifetime: crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
//                     display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//                 },
//                 code_occurence: code_occurence.clone(),
//             }),
//         )]),
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_error_occurence: std::collections::HashMap::from([(
//             std::string::String::from("String"),
//             crate::common::error_logs_logic::test::ErrorUnnamed::Something(crate::common::error_logs_logic::test::InnerErrorNamed::Something {
//                 string: std::string::String::from("String"),
//                 display_with_serialize_deserialize_struct_lifetime: crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
//                     display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//                 },
//                 code_occurence: code_occurence.clone(),
//             }),
//         )]),

//         eo_hashmap_key_display_foreign_type_value_display: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             crate::common::error_logs_logic::test::DisplayStruct { display_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_display: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             crate::common::error_logs_logic::test::DisplayStruct { display_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_display_foreign_type_value_display_lifetime: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             crate::common::error_logs_logic::test::DisplayStructLifetime { display_struct_lifetime: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_lifetime: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             crate::common::error_logs_logic::test::DisplayStructLifetime { display_struct_lifetime: std::string::String::from("str") },
//         )]),

//         eo_hashmap_key_display_foreign_type_value_str_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             "str",
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_str_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             "str",
//         )]),
//         eo_hashmap_key_display_foreign_type_value_string_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             std::string::String::from("String"),
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_string_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             std::string::String::from("String"),
//         )]),
//         eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct{},
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct{},
//         )]),
//         eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime{ display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime{ display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str") },
//         )]),

//         eo_hashmap_key_display_foreign_type_value_display_foreign_type: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             crate::common::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             crate::common::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_display_foreign_type_value_display_foreign_type_lifetime: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_lifetime: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//         )]),

//         eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") },
//         )]),
//         eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime { display_foreign_type_serialize_deserialize_struct: std::string::String::from("str") },
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") },
//         )]),

//         eo_hashmap_key_display_foreign_type_value_error_occurence: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
//             crate::common::error_logs_logic::test::ErrorUnnamed::Something(crate::common::error_logs_logic::test::InnerErrorNamed::Something {
//                 string: std::string::String::from("String"),
//                 display_with_serialize_deserialize_struct_lifetime: crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
//                     display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//                 },
//                 code_occurence: code_occurence.clone(),
//             }),
//         )]),
//         eo_hashmap_key_display_foreign_type_lifetime_value_error_occurence: std::collections::HashMap::from([(
//             crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: std::string::String::from("str") },
//             crate::common::error_logs_logic::test::ErrorUnnamed::Something(crate::common::error_logs_logic::test::InnerErrorNamed::Something {
//                 string: std::string::String::from("String"),
//                 display_with_serialize_deserialize_struct_lifetime: crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
//                     display_with_serialize_deserialize_struct_lifetime: std::string::String::from("str"),
//                 },
//                 code_occurence: code_occurence.clone(),
//             }),
//         )]),

//         code_occurence,
//     };
//     println!("{e}");
//     let e_serialize_deserialize_version = e.into_serialize_deserialize_version();
//     println!("{e_serialize_deserialize_version}");
//     let e_json = serde_json::to_string(&e_serialize_deserialize_version).unwrap();
//     println!("{e_json}");
//     let e_deserialized: ErrorNamedWithSerializeDeserialize = serde_json::from_str(&e_json).unwrap();
//     println!("{e_deserialized}");
// }

// #[derive(Debug)]
// pub struct DisplayStruct {
//     display_struct: String,
// }
// impl std::fmt::Display for DisplayStruct {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", self.display_struct)
//     }
// }

// #[derive(Debug)]
// pub struct DisplayStructLifetime {
//     display_struct_lifetime: std::string::String,
// }
// impl std::fmt::Display for DisplayStructLifetime {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", self.display_struct_lifetime)
//     }
// }

// #[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
// pub struct DisplayWithSerializeDeserializeStruct {}
// impl std::fmt::Display for DisplayWithSerializeDeserializeStruct {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "DisplayWithSerializeDeserializeStruct")
//     }
// }

// #[derive(Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
// pub struct DisplayWithSerializeDeserializeStructLifetime {
//     display_with_serialize_deserialize_struct_lifetime: String,
// }
// impl std::fmt::Display for DisplayWithSerializeDeserializeStructLifetime {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "DisplayWithSerializeDeserializeStructLifetime")
//     }
// }

// #[derive(Debug, serde::Serialize, serde::Deserialize)]
// pub struct DisplayForeignTypeSerializeDeserializeStruct {
//     display_foreign_type_serialize_deserialize_struct: String,
// }
// impl crate::common::display_foreign_type::DisplayForeignType
//     for DisplayForeignTypeSerializeDeserializeStruct
// {
//     fn display_foreign_type(&self) -> String {
//         std::string::String::from("DisplayForeignTypeSerializeDeserializeStruct")
//     }
// }
// #[derive(Debug, serde::Serialize, serde::Deserialize)]
// pub struct DisplayForeignTypeSerializeDeserializeStructLifetime {
//     display_foreign_type_serialize_deserialize_struct: std::string::String,
// }
// impl crate::common::display_foreign_type::DisplayForeignType
//     for DisplayForeignTypeSerializeDeserializeStructLifetime
// {
//     fn display_foreign_type(&self) -> String {
//         std::string::String::from("DisplayForeignTypeSerializeDeserializeStructLifetime")
//     }
// }

// #[derive(Debug, Hash, Eq, PartialEq)]
// pub struct DisplayForeignTypeStruct {
//     display_foreign_type_struct: String,
// }
// impl crate::common::display_foreign_type::DisplayForeignType for DisplayForeignTypeStruct {
//     fn display_foreign_type(&self) -> String {
//         std::string::String::from("DisplayForeignTypeStruct")
//     }
// }
// #[derive(Debug, Hash, Eq, PartialEq)]
// pub struct DisplayForeignTypeStructLifetime {
//     display_foreign_type_struct: std::string::String,
// }
// impl crate::common::display_foreign_type::DisplayForeignType for DisplayForeignTypeStructLifetime {
//     fn display_foreign_type(&self) -> String {
//         std::string::String::from("DisplayForeignTypeStruct")
//     }
// }

// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum InnerErrorNamed {
//     Something {
//         #[eo_display_with_serialize_deserialize]
//         string: String,
//         #[eo_display_with_serialize_deserialize]
//         display_with_serialize_deserialize_struct_lifetime:
//             DisplayWithSerializeDeserializeStructLifetime,
//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }

// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum ErrorUnnamed {
//     #[eo_error_occurence]
//     Something(crate::common::error_logs_logic::test::InnerErrorNamed),
// }

// #[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
// pub enum ErrorNamed {
//     Something {
//         #[eo_display]
//         eo_display: crate::common::error_logs_logic::test::DisplayStruct,
//         #[eo_display]
//         eo_display_lifetime: crate::common::error_logs_logic::test::DisplayStructLifetime,

//         #[eo_display_with_serialize_deserialize]
//         eo_str_display_with_serialize_deserialize: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         eo_string_display_with_serialize_deserialize: std::string::String,
//         #[eo_display_with_serialize_deserialize]
//         eo_display_with_serialize_deserialize: crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct,
//         #[eo_display_with_serialize_deserialize]
//         eo_display_with_serialize_deserialize_lifetime: crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime,

//         #[eo_display_foreign_type]
//         eo_display_foreign_type: crate::common::error_logs_logic::test::DisplayForeignTypeStruct,
//         #[eo_display_foreign_type]
//         eo_display_foreign_type_lifetime: crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime,

//         #[eo_display_foreign_type_with_serialize_deserialize]
//         eo_display_foreign_type_with_serialize_deserialize: crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct,
//         #[eo_display_foreign_type_with_serialize_deserialize]
//         eo_display_foreign_type_with_serialize_deserialize_lifetime: crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime,

//         #[eo_error_occurence]
//         eo_error_occurence: crate::common::error_logs_logic::test::InnerErrorNamed,

//         #[eo_vec_display]
//         eo_vec_display: std::vec::Vec<crate::common::error_logs_logic::test::DisplayStruct>,
//         #[eo_vec_display]
//         eo_vec_display_lifetime: std::vec::Vec<crate::common::error_logs_logic::test::DisplayStructLifetime>,

//         #[eo_vec_display_with_serialize_deserialize]
//         eo_vec_str_display_with_serialize_deserialize: std::vec::Vec<std::string::String>,
//         #[eo_vec_display_with_serialize_deserialize]
//         eo_vec_string_display_with_serialize_deserialize: std::vec::Vec<std::string::String>,
//         #[eo_vec_display_with_serialize_deserialize]
//         eo_vec_display_with_serialize_deserialize: std::vec::Vec<crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct>,
//         #[eo_vec_display_with_serialize_deserialize]
//         eo_vec_display_with_serialize_deserialize_lifetime: std::vec::Vec<crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime>,

//         #[eo_vec_display_foreign_type]
//         eo_vec_display_foreign_type: std::vec::Vec<crate::common::error_logs_logic::test::DisplayForeignTypeStruct>,
//         #[eo_vec_display_foreign_type]
//         eo_vec_display_foreign_type_lifetime: std::vec::Vec<crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime>,

//         #[eo_vec_display_foreign_type_with_serialize_deserialize]
//         eo_vec_display_foreign_type_with_serialize_deserialize: std::vec::Vec<crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct>,
//         #[eo_vec_display_foreign_type_with_serialize_deserialize]
//         eo_vec_display_foreign_type_with_serialize_deserialize_lifetime: std::vec::Vec<crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime>,

//         #[eo_vec_error_occurence]
//         eo_vec_error_occurence: std::vec::Vec<crate::common::error_logs_logic::test::ErrorUnnamed>,

//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::DisplayStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::DisplayStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display_lifetime: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::DisplayStructLifetime>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display_lifetime: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::DisplayStructLifetime>,

//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_str_display_with_serialize_deserialize: std::collections::HashMap<String, String>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_str_display_with_serialize_deserialize: std::collections::HashMap<std::string::String, String>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_string_display_with_serialize_deserialize: std::collections::HashMap<String, std::string::String>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_string_display_with_serialize_deserialize: std::collections::HashMap<std::string::String, std::string::String>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display_with_serialize_deserialize: std::collections::HashMap<String, crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display_with_serialize_deserialize: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<String, crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime>,

//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type]
//         eo_hashmap_key_str_value_display_foreign_type: std::collections::HashMap<String, crate::common::error_logs_logic::test::DisplayForeignTypeStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type]
//         eo_hashmap_key_string_value_display_foreign_type: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::DisplayForeignTypeStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type]
//         eo_hashmap_key_str_value_display_foreign_type_lifetime: std::collections::HashMap<String, crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type]
//         eo_hashmap_key_string_value_display_foreign_type_lifetime: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime>,

//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize]
//         eo_hashmap_key_str_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap<String, crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize]
//         eo_hashmap_key_string_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize]
//         eo_hashmap_key_str_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap<String, crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize]
//         eo_hashmap_key_string_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime>,

//         #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
//         eo_hashmap_key_str_display_with_serialize_deserialize_value_error_occurence: std::collections::HashMap<String, crate::common::error_logs_logic::test::ErrorUnnamed>,
//         #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
//         eo_hashmap_key_string_display_with_serialize_deserialize_value_error_occurence: std::collections::HashMap<std::string::String, crate::common::error_logs_logic::test::ErrorUnnamed>,

//         #[eo_hashmap_key_display_foreign_type_value_display]
//         eo_hashmap_key_display_foreign_type_value_display: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStruct, crate::common::error_logs_logic::test::DisplayStruct>,
//         #[eo_hashmap_key_display_foreign_type_value_display]
//         eo_hashmap_key_display_foreign_type_lifetime_value_display: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime, crate::common::error_logs_logic::test::DisplayStruct>,
//         #[eo_hashmap_key_display_foreign_type_value_display]
//         eo_hashmap_key_display_foreign_type_value_display_lifetime: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStruct, crate::common::error_logs_logic::test::DisplayStructLifetime>,
//         #[eo_hashmap_key_display_foreign_type_value_display]
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_lifetime: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime, crate::common::error_logs_logic::test::DisplayStructLifetime>,

//         #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_value_str_display_with_serialize_deserialize: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStruct, String>,
//         #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_lifetime_value_str_display_with_serialize_deserialize: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime, String>,
//         #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_value_string_display_with_serialize_deserialize: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStruct, std::string::String>,
//         #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_lifetime_value_string_display_with_serialize_deserialize: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime, std::string::String>,
//         #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStruct, crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct>,
//         #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_with_serialize_deserialize: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime, crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStruct>,
//         #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStruct, crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime>,
//         #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<
//             crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime,
//             crate::common::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime,
//         >,

//         #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
//         eo_hashmap_key_display_foreign_type_value_display_foreign_type: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStruct, crate::common::error_logs_logic::test::DisplayForeignTypeStruct>,
//         #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime, crate::common::error_logs_logic::test::DisplayForeignTypeStruct>,
//         #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
//         eo_hashmap_key_display_foreign_type_value_display_foreign_type_lifetime: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStruct, crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime>,
//         #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_lifetime: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime, crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime>,

//         #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStruct, crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct>,
//         #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime, crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct>,
//         #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStruct, crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime>,
//         #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
//         eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime, crate::common::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime>,

//         #[eo_hashmap_key_display_foreign_type_value_error_occurence]
//         eo_hashmap_key_display_foreign_type_value_error_occurence: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStruct, crate::common::error_logs_logic::test::ErrorUnnamed>,
//         #[eo_hashmap_key_display_foreign_type_value_error_occurence]
//         eo_hashmap_key_display_foreign_type_lifetime_value_error_occurence: std::collections::HashMap<crate::common::error_logs_logic::test::DisplayForeignTypeStructLifetime, crate::common::error_logs_logic::test::ErrorUnnamed>,

//         code_occurence: crate::common::code_occurence::CodeOccurence,
//     },
// }
