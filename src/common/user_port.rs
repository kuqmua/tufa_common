#[derive(
    Debug, 
    Clone, 
    serde::Serialize,
    serde::Deserialize,
)]
pub struct UserPort {
    port: u16,
}

impl std::fmt::Display for UserPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.port)
    }
}

#[derive(Debug, strum_macros::Display, thiserror::Error)]
pub enum UserPortTryFromStringErrorNamed {
    SystemPort(u16),//used for system services e.g. HTTP, FTP, SSH, DHCP ... 
    EphemeralPort(u16)//https://www.ncftp.com/ncftpd/doc/misc/ephemeral_ports.html
}

impl TryFrom<u16> for UserPort {
    type Error = UserPortTryFromStringErrorNamed;
    fn try_from(possible_port: u16) -> Result<UserPort, UserPortTryFromStringErrorNamed> {
        if possible_port < 1024 {
            Err(UserPortTryFromStringErrorNamed::SystemPort(possible_port))
        }
        else if possible_port < 49152 {
            Ok(Self {
                port: possible_port
            })
        }
        else {
            Err(UserPortTryFromStringErrorNamed::EphemeralPort(possible_port))
        }
    }
}

impl UserPort {
    fn port(&self) -> &u16 {
        &self.port
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
