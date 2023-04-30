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
        println!("{}", *e);
        use crate::traits::error_logs_logic::error_log::ErrorLog;
        e.error_log(once_cell::sync::Lazy::force(
            &crate::global_variables::runtime::config::CONFIG,
        ));
        let ed = e.into_serialize_deserialize_version();
        println!("{ed}");
        let xs = serde_json::to_string(&ed).unwrap();
        println!("serializes into string {}", xs);
        let xd: ErrorNamedWithSerializeDeserialize = serde_json::from_str(&xs).unwrap();
        println!("after deserialize \n{xd}");
    }
}

pub fn named<'a>() -> Result<(), Box<ErrorNamed<'a>>> {
    return Err(Box::new(ErrorNamed::Something {
        eo_display: crate::dev::DisplayStruct {
            display_struct: std::string::String::from("String")
        },
        eo_display_lifetime: crate::dev::DisplayStructLifetime {
            display_struct_lifetime: "str"
        },

        eo_str_display_with_serialize_deserialize: "str",
        eo_string_display_with_serialize_deserialize: std::string::String::from("String"),
        eo_display_with_serialize_deserialize: crate::dev::DisplayWithSerializeDeserializeStruct {},
        eo_display_with_serialize_deserialize_lifetime: crate::dev::DisplayWithSerializeDeserializeStructLifetime {
            display_with_serialize_deserialize_struct_lifetime: "str"
        },

        eo_display_foreign_type: crate::dev::DisplayForeignTypeStruct {
            display_foreign_type_struct: std::string::String::from("String")
        },
        eo_display_foreign_type_lifetime: crate::dev::DisplayForeignTypeStructLifetime {
            display_foreign_type_struct: "str"
        },

        eo_display_foreign_type_with_serialize_deserialize: crate::dev::DisplayForeignTypeSerializeDeserializeStruct {
            display_foreign_type_serialize_deserialize_struct: std::string::String::from("String")
        },
        eo_display_foreign_type_with_serialize_deserialize_lifetime: crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime {
            display_foreign_type_serialize_deserialize_struct: "str"
        },

        eo_error_occurence: crate::dev::ErrorOccurenceErrorNamed::Something {
            string: std::string::String::from("String"),
            display_with_serialize_deserialize_struct_lifetime: crate::dev::DisplayWithSerializeDeserializeStructLifetime {
                display_with_serialize_deserialize_struct_lifetime: "str",
            },
            code_occurence: crate::code_occurence_tufa_common!(),
        },

        eo_vec_str_display_with_serialize_deserialize: vec!["str"],
        eo_vec_string_display_with_serialize_deserialize:  vec![std::string::String::from("String")],
        eo_vec_display: vec![crate::dev::DisplayStruct { display_struct: std::string::String::from("String") }],
        eo_vec_display_lifetime: vec![crate::dev::DisplayStructLifetime { display_struct_lifetime: "str" }],

        eo_vec_display_with_serialize_deserialize: vec![crate::dev::DisplayWithSerializeDeserializeStruct {}],
        eo_vec_display_with_serialize_deserialize_lifetime: vec![crate::dev::DisplayWithSerializeDeserializeStructLifetime { display_with_serialize_deserialize_struct_lifetime: "str" }],

        eo_vec_display_foreign_type: vec![crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") }],
        eo_vec_display_foreign_type_lifetime: vec![crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" }], 

        eo_vec_display_foreign_type_with_serialize_deserialize: vec![crate::dev::DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") }],
        eo_vec_display_foreign_type_with_serialize_deserialize_lifetime: vec![crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime{ display_foreign_type_serialize_deserialize_struct: "str" }], 

        eo_vec_error_occurence: vec![crate::dev::ErrorUnnamed::Something(crate::dev::ErrorOccurenceErrorNamed::Something {
            string: std::string::String::from("String"),
            display_with_serialize_deserialize_struct_lifetime: crate::dev::DisplayWithSerializeDeserializeStructLifetime {
                display_with_serialize_deserialize_struct_lifetime: "str",
            },
            code_occurence: crate::code_occurence_tufa_common!(),
        })],

        eo_hashmap_key_str_display_with_serialize_deserialize_value_display: std::collections::HashMap::from([(
            "str",
            crate::dev::DisplayStruct { display_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_string_display_with_serialize_deserialize_value_display: std::collections::HashMap::from([(
            std::string::String::from("String"),
            crate::dev::DisplayStruct { display_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_str_display_with_serialize_deserialize_value_display_lifetime: std::collections::HashMap::from([(
            "str",
            crate::dev::DisplayStructLifetime { display_struct_lifetime: "str" },
        )]),
        eo_hashmap_key_string_display_with_serialize_deserialize_value_display_lifetime: std::collections::HashMap::from([(
            std::string::String::from("String"),
            crate::dev::DisplayStructLifetime { display_struct_lifetime: "str" },
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
            crate::dev::DisplayWithSerializeDeserializeStruct {},
        )]),
        eo_hashmap_key_string_display_with_serialize_deserialize_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
            std::string::String::from("String"),
            crate::dev::DisplayWithSerializeDeserializeStruct {},
        )]),
        eo_hashmap_key_str_display_with_serialize_deserialize_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
            "str",
            crate::dev::DisplayWithSerializeDeserializeStructLifetime {
                display_with_serialize_deserialize_struct_lifetime: "str",
            },
        )]),
        eo_hashmap_key_string_display_with_serialize_deserialize_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
            std::string::String::from("String"),
            crate::dev::DisplayWithSerializeDeserializeStructLifetime {
                display_with_serialize_deserialize_struct_lifetime: "str",
            },
        )]),

        eo_hashmap_key_str_value_display_foreign_type: std::collections::HashMap::from([(
            "str",
            crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_string_value_display_foreign_type: std::collections::HashMap::from([(
            std::string::String::from("String"),
            crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_str_value_display_foreign_type_lifetime: std::collections::HashMap::from([(
            "str",
            crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
        )]),
        eo_hashmap_key_string_value_display_foreign_type_lifetime: std::collections::HashMap::from([(
            std::string::String::from("String"),
            crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
        )]),

        eo_hashmap_key_str_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap::from([(
            "str",
            crate::dev::DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_string_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap::from([(
            std::string::String::from("String"),
            crate::dev::DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_str_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
            "str",
            crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime { display_foreign_type_serialize_deserialize_struct: "str" },
        )]),
        eo_hashmap_key_string_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
            std::string::String::from("String"),
            crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime { display_foreign_type_serialize_deserialize_struct: "str" },
        )]),

        eo_hashmap_key_str_display_with_serialize_deserialize_value_error_occurence: std::collections::HashMap::from([(
            "str",
            crate::dev::ErrorUnnamed::Something(crate::dev::ErrorOccurenceErrorNamed::Something {
                string: std::string::String::from("String"),
                display_with_serialize_deserialize_struct_lifetime: crate::dev::DisplayWithSerializeDeserializeStructLifetime {
                    display_with_serialize_deserialize_struct_lifetime: "str",
                },
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        )]),
        eo_hashmap_key_string_display_with_serialize_deserialize_value_error_occurence: std::collections::HashMap::from([(
            std::string::String::from("String"),
            crate::dev::ErrorUnnamed::Something(crate::dev::ErrorOccurenceErrorNamed::Something {
                string: std::string::String::from("String"),
                display_with_serialize_deserialize_struct_lifetime: crate::dev::DisplayWithSerializeDeserializeStructLifetime {
                    display_with_serialize_deserialize_struct_lifetime: "str",
                },
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        )]),

        eo_hashmap_key_display_foreign_type_value_display: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            crate::dev::DisplayStruct { display_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_display: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            crate::dev::DisplayStruct { display_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_display_foreign_type_value_display_lifetime: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            crate::dev::DisplayStructLifetime { display_struct_lifetime: "str" },
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_display_lifetime: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            crate::dev::DisplayStructLifetime { display_struct_lifetime: "str" },
        )]),

        eo_hashmap_key_display_foreign_type_value_str_display_with_serialize_deserialize: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            "str",
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_str_display_with_serialize_deserialize: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            "str",
        )]),
        eo_hashmap_key_display_foreign_type_value_string_display_with_serialize_deserialize: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            std::string::String::from("String"),
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_string_display_with_serialize_deserialize: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            std::string::String::from("String"),
        )]),
        eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            crate::dev::DisplayWithSerializeDeserializeStruct{},
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_display_with_serialize_deserialize: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            crate::dev::DisplayWithSerializeDeserializeStruct{},
        )]),
        eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            crate::dev::DisplayWithSerializeDeserializeStructLifetime{ display_with_serialize_deserialize_struct_lifetime: "str" },
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            crate::dev::DisplayWithSerializeDeserializeStructLifetime{ display_with_serialize_deserialize_struct_lifetime: "str" },
        )]),

        eo_hashmap_key_display_foreign_type_value_display_foreign_type: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_display_foreign_type_value_display_foreign_type_lifetime: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_lifetime: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
        )]),

        eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            crate::dev::DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            crate::dev::DisplayForeignTypeSerializeDeserializeStruct { display_foreign_type_serialize_deserialize_struct: std::string::String::from("String") },
        )]),
        eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime { display_foreign_type_serialize_deserialize_struct: "str" },
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime { display_foreign_type_serialize_deserialize_struct: "String" },
        )]),

        eo_hashmap_key_display_foreign_type_value_error_occurence: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStruct { display_foreign_type_struct: std::string::String::from("String") },
            crate::dev::ErrorUnnamed::Something(crate::dev::ErrorOccurenceErrorNamed::Something {
                string: std::string::String::from("String"),
                display_with_serialize_deserialize_struct_lifetime: crate::dev::DisplayWithSerializeDeserializeStructLifetime {
                    display_with_serialize_deserialize_struct_lifetime: "str",
                },
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        )]),
        eo_hashmap_key_display_foreign_type_lifetime_value_error_occurence: std::collections::HashMap::from([(
            crate::dev::DisplayForeignTypeStructLifetime { display_foreign_type_struct: "str" },
            crate::dev::ErrorUnnamed::Something(crate::dev::ErrorOccurenceErrorNamed::Something {
                string: std::string::String::from("String"),
                display_with_serialize_deserialize_struct_lifetime: crate::dev::DisplayWithSerializeDeserializeStructLifetime {
                    display_with_serialize_deserialize_struct_lifetime: "str",
                },
                code_occurence: crate::code_occurence_tufa_common!(),
            }),
        )]),

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
pub enum ErrorOccurenceErrorNamed<'a> {
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
    Something(crate::dev::ErrorOccurenceErrorNamed<'a>),
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum ErrorNamed<'a> {
    Something {
        #[eo_display]
        eo_display: crate::dev::DisplayStruct,
        #[eo_display]
        eo_display_lifetime: crate::dev::DisplayStructLifetime<'a>,

        #[eo_display_with_serialize_deserialize]
        eo_str_display_with_serialize_deserialize: &'a str,
        #[eo_display_with_serialize_deserialize]
        eo_string_display_with_serialize_deserialize: std::string::String,
        #[eo_display_with_serialize_deserialize]
        eo_display_with_serialize_deserialize: crate::dev::DisplayWithSerializeDeserializeStruct,
        #[eo_display_with_serialize_deserialize]
        eo_display_with_serialize_deserialize_lifetime: crate::dev::DisplayWithSerializeDeserializeStructLifetime<'a>,

        #[eo_display_foreign_type]
        eo_display_foreign_type: crate::dev::DisplayForeignTypeStruct,
        #[eo_display_foreign_type]
        eo_display_foreign_type_lifetime: crate::dev::DisplayForeignTypeStructLifetime<'a>,

        #[eo_display_foreign_type_with_serialize_deserialize]
        eo_display_foreign_type_with_serialize_deserialize: crate::dev::DisplayForeignTypeSerializeDeserializeStruct,
        #[eo_display_foreign_type_with_serialize_deserialize]
        eo_display_foreign_type_with_serialize_deserialize_lifetime: crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime<'a>,

        #[eo_error_occurence]
        eo_error_occurence: crate::dev::ErrorOccurenceErrorNamed<'a>,

        #[eo_vec_display]
        eo_vec_display: std::vec::Vec<crate::dev::DisplayStruct>,
        #[eo_vec_display]
        eo_vec_display_lifetime: std::vec::Vec<crate::dev::DisplayStructLifetime<'a>>,

        #[eo_vec_display_with_serialize_deserialize]
        eo_vec_str_display_with_serialize_deserialize: std::vec::Vec<&'a str>,
        #[eo_vec_display_with_serialize_deserialize]
        eo_vec_string_display_with_serialize_deserialize: std::vec::Vec<std::string::String>,
        #[eo_vec_display_with_serialize_deserialize]
        eo_vec_display_with_serialize_deserialize: std::vec::Vec<crate::dev::DisplayWithSerializeDeserializeStruct>,
        #[eo_vec_display_with_serialize_deserialize]
        eo_vec_display_with_serialize_deserialize_lifetime: std::vec::Vec<crate::dev::DisplayWithSerializeDeserializeStructLifetime<'a>>,

        #[eo_vec_display_foreign_type]
        eo_vec_display_foreign_type: std::vec::Vec<crate::dev::DisplayForeignTypeStruct>,
        #[eo_vec_display_foreign_type]
        eo_vec_display_foreign_type_lifetime: std::vec::Vec<crate::dev::DisplayForeignTypeStructLifetime<'a>>,

        #[eo_vec_display_foreign_type_with_serialize_deserialize]
        eo_vec_display_foreign_type_with_serialize_deserialize: std::vec::Vec<crate::dev::DisplayForeignTypeSerializeDeserializeStruct>,
        #[eo_vec_display_foreign_type_with_serialize_deserialize]
        eo_vec_display_foreign_type_with_serialize_deserialize_lifetime: std::vec::Vec<crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime<'a>>,

        #[eo_vec_error_occurence]
        eo_vec_error_occurence: std::vec::Vec<crate::dev::ErrorUnnamed<'a>>,

        #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
        eo_hashmap_key_str_display_with_serialize_deserialize_value_display: std::collections::HashMap<&'a str, crate::dev::DisplayStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
        eo_hashmap_key_string_display_with_serialize_deserialize_value_display: std::collections::HashMap<std::string::String, crate::dev::DisplayStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
        eo_hashmap_key_str_display_with_serialize_deserialize_value_display_lifetime: std::collections::HashMap<&'a str, crate::dev::DisplayStructLifetime<'a>>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display]
        eo_hashmap_key_string_display_with_serialize_deserialize_value_display_lifetime: std::collections::HashMap<std::string::String, crate::dev::DisplayStructLifetime<'a>>,

        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        eo_hashmap_key_str_display_with_serialize_deserialize_value_str_display_with_serialize_deserialize: std::collections::HashMap<&'a str, &'a str>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        eo_hashmap_key_string_display_with_serialize_deserialize_value_str_display_with_serialize_deserialize: std::collections::HashMap<std::string::String, &'a str>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        eo_hashmap_key_str_display_with_serialize_deserialize_value_string_display_with_serialize_deserialize: std::collections::HashMap<&'a str, std::string::String>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        eo_hashmap_key_string_display_with_serialize_deserialize_value_string_display_with_serialize_deserialize: std::collections::HashMap<std::string::String, std::string::String>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        eo_hashmap_key_str_display_with_serialize_deserialize_value_display_with_serialize_deserialize: std::collections::HashMap<&'a str, crate::dev::DisplayWithSerializeDeserializeStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        eo_hashmap_key_string_display_with_serialize_deserialize_value_display_with_serialize_deserialize: std::collections::HashMap<std::string::String, crate::dev::DisplayWithSerializeDeserializeStruct>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        eo_hashmap_key_str_display_with_serialize_deserialize_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<&'a str, crate::dev::DisplayWithSerializeDeserializeStructLifetime<'a>>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_display_with_serialize_deserialize]
        eo_hashmap_key_string_display_with_serialize_deserialize_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<std::string::String, crate::dev::DisplayWithSerializeDeserializeStructLifetime<'a>>,

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
        eo_hashmap_key_str_display_with_serialize_deserialize_value_error_occurence: std::collections::HashMap<&'a str, crate::dev::ErrorUnnamed<'a>>,
        #[eo_hashmap_key_display_with_serialize_deserialize_value_error_occurence]
        eo_hashmap_key_string_display_with_serialize_deserialize_value_error_occurence: std::collections::HashMap<std::string::String, crate::dev::ErrorUnnamed<'a>>,

        #[eo_hashmap_key_display_foreign_type_value_display]
        eo_hashmap_key_display_foreign_type_value_display: std::collections::HashMap<crate::dev::DisplayForeignTypeStruct, crate::dev::DisplayStruct>,
        #[eo_hashmap_key_display_foreign_type_value_display]
        eo_hashmap_key_display_foreign_type_lifetime_value_display: std::collections::HashMap<crate::dev::DisplayForeignTypeStructLifetime<'a>, crate::dev::DisplayStruct>,
        #[eo_hashmap_key_display_foreign_type_value_display]
        eo_hashmap_key_display_foreign_type_value_display_lifetime: std::collections::HashMap<crate::dev::DisplayForeignTypeStruct, crate::dev::DisplayStructLifetime<'a>>,
        #[eo_hashmap_key_display_foreign_type_value_display]
        eo_hashmap_key_display_foreign_type_lifetime_value_display_lifetime: std::collections::HashMap<crate::dev::DisplayForeignTypeStructLifetime<'a>, crate::dev::DisplayStructLifetime<'a>>,

        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_value_str_display_with_serialize_deserialize: std::collections::HashMap<crate::dev::DisplayForeignTypeStruct, &'a str>,
        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_lifetime_value_str_display_with_serialize_deserialize: std::collections::HashMap<crate::dev::DisplayForeignTypeStructLifetime<'a>, &'a str>,
        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_value_string_display_with_serialize_deserialize: std::collections::HashMap<crate::dev::DisplayForeignTypeStruct, std::string::String>,
        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_lifetime_value_string_display_with_serialize_deserialize: std::collections::HashMap<crate::dev::DisplayForeignTypeStructLifetime<'a>, std::string::String>,
        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize: std::collections::HashMap<crate::dev::DisplayForeignTypeStruct, crate::dev::DisplayWithSerializeDeserializeStruct>,
        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_lifetime_value_display_with_serialize_deserialize: std::collections::HashMap<crate::dev::DisplayForeignTypeStructLifetime<'a>, crate::dev::DisplayWithSerializeDeserializeStruct>,
        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<crate::dev::DisplayForeignTypeStruct, crate::dev::DisplayWithSerializeDeserializeStructLifetime<'a>>,
        #[eo_hashmap_key_display_foreign_type_value_display_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_lifetime_value_display_with_serialize_deserialize_lifetime: std::collections::HashMap<
            crate::dev::DisplayForeignTypeStructLifetime<'a>,
            crate::dev::DisplayWithSerializeDeserializeStructLifetime<'a>,
        >,

        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
        eo_hashmap_key_display_foreign_type_value_display_foreign_type: std::collections::HashMap<crate::dev::DisplayForeignTypeStruct, crate::dev::DisplayForeignTypeStruct>,
        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
        eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type: std::collections::HashMap<crate::dev::DisplayForeignTypeStructLifetime<'a>, crate::dev::DisplayForeignTypeStruct>,
        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
        eo_hashmap_key_display_foreign_type_value_display_foreign_type_lifetime: std::collections::HashMap<crate::dev::DisplayForeignTypeStruct, crate::dev::DisplayForeignTypeStructLifetime<'a>>,
        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type]
        eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_lifetime: std::collections::HashMap<crate::dev::DisplayForeignTypeStructLifetime<'a>, crate::dev::DisplayForeignTypeStructLifetime<'a>>,

        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap<crate::dev::DisplayForeignTypeStruct, crate::dev::DisplayForeignTypeSerializeDeserializeStruct>,
        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_with_serialize_deserialize: std::collections::HashMap<crate::dev::DisplayForeignTypeStructLifetime<'a>, crate::dev::DisplayForeignTypeSerializeDeserializeStruct>,
        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap<crate::dev::DisplayForeignTypeStruct, crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime<'a>>,
        #[eo_hashmap_key_display_foreign_type_value_display_foreign_type_with_serialize_deserialize]
        eo_hashmap_key_display_foreign_type_lifetime_value_display_foreign_type_with_serialize_deserialize_lifetime: std::collections::HashMap<crate::dev::DisplayForeignTypeStructLifetime<'a>, crate::dev::DisplayForeignTypeSerializeDeserializeStructLifetime<'a>>,

        #[eo_hashmap_key_display_foreign_type_value_error_occurence]
        eo_hashmap_key_display_foreign_type_value_error_occurence: std::collections::HashMap<crate::dev::DisplayForeignTypeStruct, crate::dev::ErrorUnnamed<'a>>,
        #[eo_hashmap_key_display_foreign_type_value_error_occurence]
        eo_hashmap_key_display_foreign_type_lifetime_value_error_occurence: std::collections::HashMap<crate::dev::DisplayForeignTypeStructLifetime<'a>, crate::dev::ErrorUnnamed<'a>>,

        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}