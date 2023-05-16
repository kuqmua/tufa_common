#[derive(
    Debug, 
    Clone, 
    serde::Serialize,
    serde::Deserialize,
)]
pub struct UserPort {
    port: u16,
}
//todo impl init macro to compile time check for range 1024 to 49151
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