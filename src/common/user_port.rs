#[derive(
    Debug, 
    Clone, 
    serde::Serialize,
    serde::Deserialize,
    getset::Getters,
)]
pub struct UserPort {
    #[getset(get = "pub")]
    port: u16,
}

impl std::fmt::Display for UserPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.port)
    }
}

#[derive(Debug, strum_macros::Display, thiserror::Error)]
pub enum UserPortTryFromStringError {
    SystemPort(u16),//used for system services e.g. HTTP, FTP, SSH, DHCP ... 
    EphemeralPort(u16)//https://www.ncftp.com/ncftpd/doc/misc/ephemeral_ports.html
}
//todo maybe instead of proc_macro use builder pattern with phantom data state?
impl TryFrom<u16> for UserPort {
    type Error = UserPortTryFromStringError;
    fn try_from(possible_port: u16) -> Result<UserPort, UserPortTryFromStringError> {
        if possible_port < 1024 {
            Err(UserPortTryFromStringError::SystemPort(possible_port))
        }
        else if possible_port < 49152 {
            Ok(Self {
                port: possible_port
            })
        }
        else {
            Err(UserPortTryFromStringError::EphemeralPort(possible_port))
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
