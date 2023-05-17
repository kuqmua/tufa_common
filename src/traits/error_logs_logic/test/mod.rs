#[test]
pub fn test_code_occurence() {
    let git_info = crate::common::git::git_info::GitInformation::default();
    let code_occurence = crate::common::code_occurence::CodeOccurence::new(
        &git_info,
        file!(),
        line!(),
        column!(),
        {
            use crate::traits::config_fields::GetServerPort;
            *crate::config_mods::config_struct::ConfigBuilder::default().get_server_port()
        }
    );
    let e = ErrorNamed::Something {
        eo_display: crate::traits::error_logs_logic::test::DisplayStruct {
            display_struct: std::string::String::from("String")
        },
        eo_display_lifetime: crate::traits::error_logs_logic::test::DisplayStructLifetime {
            display_struct_lifetime: "str"
        },

        eo_str_display_with_serialize_deserialize: "str",
        eo_string_display_with_serialize_deserialize: std::string::String::from("String"),
        eo_display_with_serialize_deserialize: crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStruct {},
        eo_display_with_serialize_deserialize_lifetime: crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
            display_with_serialize_deserialize_struct_lifetime: "str"
        },

        eo_display_foreign_type: crate::traits::error_logs_logic::test::DisplayForeignTypeStruct {
            display_foreign_type_struct: std::string::String::from("String")
        },
        eo_display_foreign_type_lifetime: crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime {
            display_foreign_type_struct: "str"
        },

        eo_display_foreign_type_with_serialize_deserialize: crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct {
            display_foreign_type_serialize_deserialize_struct: std::string::String::from("String")
        },
        eo_display_foreign_type_with_serialize_deserialize_lifetime: crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime {
            display_foreign_type_serialize_deserialize_struct: "str"
        },

        eo_error_occurence: crate::traits::error_logs_logic::test::InnerErrorNamed::Something {
            string: std::string::String::from("String"),
            display_with_serialize_deserialize_struct_lifetime: crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
                display_with_serialize_deserialize_struct_lifetime: "str",
            },
            code_occurence: code_occurence.clone(),
        },

        eo_vec_str_display_with_serialize_deserialize: vec!["str"],
        eo_vec_string_display_with_serialize_deserialize:  vec![std::string::String::from("String")],
        eo_vec_display: vec![crate::traits::error_logs_logic::test::DisplayStruct { display_struct: std::string::String::from("String") }],
        eo_vec_display_lifetime: vec![crate::traits::error_logs_logic::test::DisplayStructLifetime { display_struct_lifetime: "str" }],

        eo_vec_display_with_serialize_deserialize: vec![crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStruct {}],
        eo_vec_display_with_serialize_deserialize_lifetime: vec![crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime { display_with_serialize_deserialize_struct_lifetime: "str" }],

        eo_vec_display_foreign_type: vec![crate::traits::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") }],
        eo_vec_display_foreign_type_lifetime: vec![crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" }], 

        eo_vec_display_foreign_type_with_serialize_deserialize: vec![crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") }],
        eo_vec_display_foreign_type_with_serialize_deserialize_lifetime: vec![crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime{ display_foreign_type_serialize_deserialize_struct: "str" }], 

        eo_vec_error_occurence: vec![crate::traits::error_logs_logic::test::ErrorUnnamed::Something(crate::traits::error_logs_logic::test::InnerErrorNamed::Something {
            string: std::string::String::from("String"),
            display_with_serialize_deserialize_struct_lifetime: crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
                display_with_serialize_deserialize_struct_lifetime: "str",
            },
            code_occurence: code_occurence.clone(),
        })],

        eo_hashmap_key_str_display_with_serialize_deserialize_value_display: std::collections::HashMap::from([(
            "str",
            crate::traits::error_logs_logic::test::DisplayStruct { display_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_string_display_with_serialize_deserialize_value_display: std::collections::HashMap::from([(
            std::string::String::from("String"),
            crate::traits::error_logs_logic::test::DisplayStruct { display_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_str_display_with_serialize_deserialize_value_display_lifetime: std::collections::HashMap::from([(
            "str",
            crate::traits::error_logs_logic::test::DisplayStructLifetime { display_struct_lifetime: "str" },
        )]),
        eo_hashmap_key_string_display_with_serialize_deserialize_value_display_lifetime: std::collections::HashMap::from([(
            std::string::String::from("String"),
            crate::traits::error_logs_logic::test::DisplayStructLifetime { display_struct_lifetime: "str" },
        )]),

        eo_hashmap_key_str_display_with_serialize_deserialize_value_str_display_with_serialize_deserialize: std::collections::HashMap::from([(
            "str",
            "str",
        )]),
        eo_hashmap_key_string_display_with_serialize_deserialize_value_str_display_with_serialize_deserialize: std::collections::HashMap::from([(
            std::string::String::from("String"),
            "str",
        )]),
        eo_hashmap_key_str_display_with_serialize_deserialize_value_string_display_with_serialize_deserialize: std::collections::HashMap::from([(
            "str",
            std::string::String::from("String")
        )]),
        eo_hashmap_key_string_display_with_serialize_deserialize_value_string_display_with_serialize_deserialize: std::collections::HashMap::from([(
            std::string::String::from("String"),
            std::string::String::from("String")
        )]),

        eo_hashmap_key_str_display_with_serialize_deserialize_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
            "str",
            crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStruct {},
        )]),
        eo_hashmap_key_string_display_with_serialize_deserialize_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
            std::string::String::from("String"),
            crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStruct {},
        )]),
        eo_hashmap_key_str_display_with_serialize_deserialize_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
            "str",
            crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
                display_with_serialize_deserialize_struct_lifetime: "str",
            },
        )]),
        eo_hashmap_key_string_display_with_serialize_deserialize_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
            std::string::String::from("String"),
            crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
                display_with_serialize_deserialize_struct_lifetime: "str",
            },
        )]),

        eo_hashmap_key_str_value_display_foreign_type: std::collections::HashMap::from([(
            "str",
            crate::traits::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_string_value_display_foreign_type: std::collections::HashMap::from([(
            std::string::String::from("String"),
            crate::traits::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_str_value_display_foreign_type_lifetime: std::collections::HashMap::from([(
            "str",
            crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
        )]),
        eo_hashmap_key_string_value_display_foreign_type_lifetime: std::collections::HashMap::from([(
            std::string::String::from("String"),
            crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
        )]),

        eo_hashmap_key_str_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap::from([(
            "str",
            crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_string_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap::from([(
            std::string::String::from("String"),
            crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_str_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
            "str",
            crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime { display_foreign_type_serialize_deserialize_struct: "str" },
        )]),
        eo_hashmap_key_string_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
            std::string::String::from("String"),
            crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime { display_foreign_type_serialize_deserialize_struct: "str" },
        )]),

        eo_hashmap_key_str_display_with_serialize_deserialize_value_error_occurence: std::collections::HashMap::from([(
            "str",
            crate::traits::error_logs_logic::test::ErrorUnnamed::Something(crate::traits::error_logs_logic::test::InnerErrorNamed::Something {
                string: std::string::String::from("String"),
                display_with_serialize_deserialize_struct_lifetime: crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
                    display_with_serialize_deserialize_struct_lifetime: "str",
                },
                code_occurence: code_occurence.clone(),
            }),
        )]),
        eo_hashmap_key_string_display_with_serialize_deserialize_value_error_occurence: std::collections::HashMap::from([(
            std::string::String::from("String"),
            crate::traits::error_logs_logic::test::ErrorUnnamed::Something(crate::traits::error_logs_logic::test::InnerErrorNamed::Something {
                string: std::string::String::from("String"),
                display_with_serialize_deserialize_struct_lifetime: crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
                    display_with_serialize_deserialize_struct_lifetime: "str",
                },
                code_occurence: code_occurence.clone(),
            }),
        )]),

        eo_hashmap_key_display_foreign_type_value_display: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            crate::traits::error_logs_logic::test::DisplayStruct { display_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_display: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            crate::traits::error_logs_logic::test::DisplayStruct { display_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_display_foreign_type_value_display_lifetime: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            crate::traits::error_logs_logic::test::DisplayStructLifetime { display_struct_lifetime: "str" },
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_display_lifetime: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            crate::traits::error_logs_logic::test::DisplayStructLifetime { display_struct_lifetime: "str" },
        )]),

        eo_hashmap_key_display_foreign_type_value_str_display_with_serialize_deserialize: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            "str",
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_str_display_with_serialize_deserialize: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            "str",
        )]),
        eo_hashmap_key_display_foreign_type_value_string_display_with_serialize_deserialize: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            std::string::String::from("String"),
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_string_display_with_serialize_deserialize: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            std::string::String::from("String"),
        )]),
        eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStruct{},
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStruct{},
        )]),
        eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime{ display_with_serialize_deserialize_struct_lifetime: "str" },
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime{ display_with_serialize_deserialize_struct_lifetime: "str" },
        )]),

        eo_hashmap_key_display_foreign_type_value_display_foreign_type: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            crate::traits::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            crate::traits::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_display_foreign_type_value_display_foreign_type_lifetime: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_lifetime: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
        )]),

        eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime { display_foreign_type_serialize_deserialize_struct: "str" },
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime { display_foreign_type_serialize_deserialize_struct: "String" },
        )]),

        eo_hashmap_key_display_foreign_type_value_error_occurence: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            crate::traits::error_logs_logic::test::ErrorUnnamed::Something(crate::traits::error_logs_logic::test::InnerErrorNamed::Something {
                string: std::string::String::from("String"),
                display_with_serialize_deserialize_struct_lifetime: crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
                    display_with_serialize_deserialize_struct_lifetime: "str",
                },
                code_occurence: code_occurence.clone(),
            }),
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_error_occurence: std::collections::HashMap::from([(
            crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            crate::traits::error_logs_logic::test::ErrorUnnamed::Something(crate::traits::error_logs_logic::test::InnerErrorNamed::Something {
                string: std::string::String::from("String"),
                display_with_serialize_deserialize_struct_lifetime: crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime {
                    display_with_serialize_deserialize_struct_lifetime: "str",
                },
                code_occurence: code_occurence.clone(),
            }),
        )]),

        code_occurence,
    };
    println!("{e}");
    use crate::traits::error_logs_logic::error_log::ErrorLog;
    // e.error_log(&crate::config_mods::config_struct::ConfigBuilder::default());
    let e_serialize_deserialize_version = e.into_serialize_deserialize_version();
    println!("{e_serialize_deserialize_version}");
    let e_json = serde_json::to_string(&e_serialize_deserialize_version).unwrap();
    println!("{e_json}");
    let e_deserialized: ErrorNamedWithSerializeDeserialize = serde_json::from_str(&e_json).unwrap();
    println!("{e_deserialized}");
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
        std::string::String::from("DisplayForeignTypeSerializeDeserializeStruct")
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
        std::string::String::from("DisplayForeignTypeSerializeDeserializeStructLifetime")
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
        std::string::String::from("DisplayForeignTypeStruct")
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
        std::string::String::from("DisplayForeignTypeStruct")
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum InnerErrorNamed<'a> {
    Something {
        #[eo_display_with_serialize_deserialize]
        string: String,
        #[eo_display_with_serialize_deserialize]
        display_with_serialize_deserialize_struct_lifetime: DisplayWithSerializeDeserializeStructLifetime<'a>,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum ErrorUnnamed<'a> {
    #[eo_error_occurence]
    Something(crate::traits::error_logs_logic::test::InnerErrorNamed<'a>),
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum ErrorNamed<'a> {
    Something {
        #[eo_display]
        eo_display: crate::traits::error_logs_logic::test::DisplayStruct,
        #[eo_display]
        eo_display_lifetime: crate::traits::error_logs_logic::test::DisplayStructLifetime<'a>,

        #[eo_display_with_serialize_deserialize]
        eo_str_display_with_serialize_deserialize: &'a str,
        #[eo_display_with_serialize_deserialize]
        eo_string_display_with_serialize_deserialize: std::string::String,
        #[eo_display_with_serialize_deserialize]
        eo_display_with_serialize_deserialize: crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStruct,
        #[eo_display_with_serialize_deserialize]
        eo_display_with_serialize_deserialize_lifetime: crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime<'a>,

        #[eo_display_foreign_type]
        eo_display_foreign_type: crate::traits::error_logs_logic::test::DisplayForeignTypeStruct,
        #[eo_display_foreign_type]
        eo_display_foreign_type_lifetime: crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime<'a>,

        #[eo_display_foreign_type_with_serialize_deserialize]
        eo_display_foreign_type_with_serialize_deserialize: crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct,
        #[eo_display_foreign_type_with_serialize_deserialize]
        eo_display_foreign_type_with_serialize_deserialize_lifetime: crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime<'a>,

        #[eo_error_occurence]
        eo_error_occurence: crate::traits::error_logs_logic::test::InnerErrorNamed<'a>,

        #[eo_vec_display]
        eo_vec_display: std::vec::Vec<crate::traits::error_logs_logic::test::DisplayStruct>,
        #[eo_vec_display]
        eo_vec_display_lifetime: std::vec::Vec<crate::traits::error_logs_logic::test::DisplayStructLifetime<'a>>,

        #[eo_vec_display_with_serialize_deserialize]
        eo_vec_str_display_with_serialize_deserialize: std::vec::Vec<&'a str>,
        #[eo_vec_display_with_serialize_deserialize]
        eo_vec_string_display_with_serialize_deserialize: std::vec::Vec<std::string::String>,
        #[eo_vec_display_with_serialize_deserialize]
        eo_vec_display_with_serialize_deserialize: std::vec::Vec<crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStruct>,
        #[eo_vec_display_with_serialize_deserialize]
        eo_vec_display_with_serialize_deserialize_lifetime: std::vec::Vec<crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime<'a>>,

        #[eo_vec_display_foreign_type]
        eo_vec_display_foreign_type: std::vec::Vec<crate::traits::error_logs_logic::test::DisplayForeignTypeStruct>,
        #[eo_vec_display_foreign_type]
        eo_vec_display_foreign_type_lifetime: std::vec::Vec<crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime<'a>>,

        #[eo_vec_display_foreign_type_with_serialize_deserialize]
        eo_vec_display_foreign_type_with_serialize_deserialize: std::vec::Vec<crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct>,
        #[eo_vec_display_foreign_type_with_serialize_deserialize]
        eo_vec_display_foreign_type_with_serialize_deserialize_lifetime: std::vec::Vec<crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime<'a>>,

        #[eo_vec_error_occurence]
        eo_vec_error_occurence: std::vec::Vec<crate::traits::error_logs_logic::test::ErrorUnnamed<'a>>,

        #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
        eo_hashmap_key_str_display_with_serialize_deserialize_value_display: std::collections::HashMap<&'a str, crate::traits::error_logs_logic::test::DisplayStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
        eo_hashmap_key_string_display_with_serialize_deserialize_value_display: std::collections::HashMap<std::string::String, crate::traits::error_logs_logic::test::DisplayStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
        eo_hashmap_key_str_display_with_serialize_deserialize_value_display_lifetime: std::collections::HashMap<&'a str, crate::traits::error_logs_logic::test::DisplayStructLifetime<'a>>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
        eo_hashmap_key_string_display_with_serialize_deserialize_value_display_lifetime: std::collections::HashMap<std::string::String, crate::traits::error_logs_logic::test::DisplayStructLifetime<'a>>,

        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        eo_hashmap_key_str_display_with_serialize_deserialize_value_str_display_with_serialize_deserialize: std::collections::HashMap<&'a str, &'a str>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        eo_hashmap_key_string_display_with_serialize_deserialize_value_str_display_with_serialize_deserialize: std::collections::HashMap<std::string::String, &'a str>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        eo_hashmap_key_str_display_with_serialize_deserialize_value_string_display_with_serialize_deserialize: std::collections::HashMap<&'a str, std::string::String>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        eo_hashmap_key_string_display_with_serialize_deserialize_value_string_display_with_serialize_deserialize: std::collections::HashMap<std::string::String, std::string::String>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        eo_hashmap_key_str_display_with_serialize_deserialize_value_display_with_serialize_deserialize: std::collections::HashMap<&'a str, crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        eo_hashmap_key_string_display_with_serialize_deserialize_value_display_with_serialize_deserialize: std::collections::HashMap<std::string::String, crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        eo_hashmap_key_str_display_with_serialize_deserialize_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<&'a str, crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime<'a>>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        eo_hashmap_key_string_display_with_serialize_deserialize_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<std::string::String, crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime<'a>>,

        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type]
        eo_hashmap_key_str_value_display_foreign_type: std::collections::HashMap<&'a str, crate::traits::error_logs_logic::test::DisplayForeignTypeStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type]
        eo_hashmap_key_string_value_display_foreign_type: std::collections::HashMap<std::string::String, crate::traits::error_logs_logic::test::DisplayForeignTypeStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type]
        eo_hashmap_key_str_value_display_foreign_type_lifetime: std::collections::HashMap<&'a str, crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime<'a>>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type]
        eo_hashmap_key_string_value_display_foreign_type_lifetime: std::collections::HashMap<std::string::String, crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime<'a>>,

        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize]
        eo_hashmap_key_str_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap<&'a str, crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize]
        eo_hashmap_key_string_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap<std::string::String, crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize]
        eo_hashmap_key_str_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap<&'a str, crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime<'a>>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_foreign_type_with_serialize_deserialize]
        eo_hashmap_key_string_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap<std::string::String, crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime<'a>>,

        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        eo_hashmap_key_str_display_with_serialize_deserialize_value_error_occurence: std::collections::HashMap<&'a str, crate::traits::error_logs_logic::test::ErrorUnnamed<'a>>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        eo_hashmap_key_string_display_with_serialize_deserialize_value_error_occurence: std::collections::HashMap<std::string::String, crate::traits::error_logs_logic::test::ErrorUnnamed<'a>>,

        #[eo_hashmap_key_display_foreign_type_value_display]
        eo_hashmap_key_display_foreign_type_value_display: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStruct, crate::traits::error_logs_logic::test::DisplayStruct>,
        #[eo_hashmap_key_display_foreign_type_value_display]
        eo_hashmap_key_display_foreign_type_lifetime_value_display: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime<'a>, crate::traits::error_logs_logic::test::DisplayStruct>,
        #[eo_hashmap_key_display_foreign_type_value_display]
        eo_hashmap_key_display_foreign_type_value_display_lifetime: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStruct, crate::traits::error_logs_logic::test::DisplayStructLifetime<'a>>,
        #[eo_hashmap_key_display_foreign_type_value_display]
        eo_hashmap_key_display_foreign_type_lifetime_value_display_lifetime: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime<'a>, crate::traits::error_logs_logic::test::DisplayStructLifetime<'a>>,

        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_value_str_display_with_serialize_deserialize: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStruct, &'a str>,
        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_lifetime_value_str_display_with_serialize_deserialize: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime<'a>, &'a str>,
        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_value_string_display_with_serialize_deserialize: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStruct, std::string::String>,
        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_lifetime_value_string_display_with_serialize_deserialize: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime<'a>, std::string::String>,
        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStruct, crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStruct>,
        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_lifetime_value_display_with_serialize_deserialize: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime<'a>, crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStruct>,
        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStruct, crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime<'a>>,
        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_lifetime_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<
            crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime<'a>,
            crate::traits::error_logs_logic::test::DisplayWithSerializeDeserializeStructLifetime<'a>,
        >,

        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
        eo_hashmap_key_display_foreign_type_value_display_foreign_type: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStruct, crate::traits::error_logs_logic::test::DisplayForeignTypeStruct>,
        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
        eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime<'a>, crate::traits::error_logs_logic::test::DisplayForeignTypeStruct>,
        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
        eo_hashmap_key_display_foreign_type_value_display_foreign_type_lifetime: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStruct, crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime<'a>>,
        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
        eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_lifetime: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime<'a>, crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime<'a>>,

        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStruct, crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct>,
        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime<'a>, crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStruct>,
        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStruct, crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime<'a>>,
        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime<'a>, crate::traits::error_logs_logic::test::DisplayForeignTypeSerializeDeserializeStructLifetime<'a>>,

        #[eo_hashmap_key_display_foreign_type_value_error_occurence]
        eo_hashmap_key_display_foreign_type_value_error_occurence: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStruct, crate::traits::error_logs_logic::test::ErrorUnnamed<'a>>,
        #[eo_hashmap_key_display_foreign_type_value_error_occurence]
        eo_hashmap_key_display_foreign_type_lifetime_value_error_occurence: std::collections::HashMap<crate::traits::error_logs_logic::test::DisplayForeignTypeStructLifetime<'a>, crate::traits::error_logs_logic::test::ErrorUnnamed<'a>>,

        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}