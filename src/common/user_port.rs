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

impl std::convert::TryFrom<u16> for UserPort {
    type Error = UserPortTryFromU16ErrorNamed;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value < 1024 {
            Err(Self::Error::SystemPort {
                port: value,
                code_occurence: crate::code_occurence_tufa_common!(),
            })
        } else if value < 49152 {
            Ok(Self { port: value })
        } else {
            Err(Self::Error::EphemeralPort {
                port: value,
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
