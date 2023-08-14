#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, getset::Getters)]
pub struct UserPort {
    #[getset(get = "pub")]
    port: u16,
}

impl std::fmt::Display for UserPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.port)
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum UserPortTryFromU16ErrorNamed {
    SystemPort {
        #[eo_display_with_serialize_deserialize]
        port: u16,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
    EphemeralPort {
        #[eo_display_with_serialize_deserialize]
        port: u16,
        code_occurence: crate::common::code_occurence::CodeOccurence,
    },
}

impl UserPort {
    //its not TryFrom<u16> coz its not supported lifetimes in Error annotation
    pub fn try_from_u16(possible_port: u16) -> Result<UserPort, UserPortTryFromU16ErrorNamed> {
        if possible_port < 1024 {
            Err(UserPortTryFromU16ErrorNamed::SystemPort {
                port: possible_port,
                code_occurence: crate::code_occurence_tufa_common!(),
            })
        } else if possible_port < 49152 {
            Ok(Self {
                port: possible_port,
            })
        } else {
            Err(UserPortTryFromU16ErrorNamed::EphemeralPort {
                port: possible_port,
                code_occurence: crate::code_occurence_tufa_common!(),
            })
        }
    }
}

// macro_rules! user_port_try_from_u16_with_possible_runtime_panic{
//     ($possible_port:expr) => {
//         if $possible_port < 1024 {
//             panic!("failed to user_port_try_from_u16!(), reason: system port range 0-1023");
//         }
//         else if $possible_port < 49152 {
//             UserPort {
//                 port: $possible_port
//             }
//         }
//         else {
//             panic!("failed to user_port_try_from_u16!(), reason: ephemeral port range 49152-65535");
//         }
//     }
// }
