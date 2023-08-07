#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, getset::Getters)]
pub struct Bigserial {
    #[getset(get = "pub")]
    // #[serde(deserialize_with = "deserialize_bigserial")]
    bigserial: i64, //todo postgres bigserial max = i64::MAX, but invalid in i64 < 0
}

//
// use serde::de::{Deserialize, Deserializer};

// #[derive(serde::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Trigger {
//     #[serde(deserialize_with = "deserialize_bigserial")]
//     cron: i64,
// }

fn deserialize_bigserial<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use serde::Deserialize;
    let buf = i64::deserialize(deserializer)?;

    // use std::str::FromStr;
    // String::from_str(&buf).map_err(serde::de::Error::custom)
    Ok(buf)
}
//

impl std::fmt::Display for Bigserial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.bigserial)
    }
}

#[derive(Debug, thiserror::Error, error_occurence::ErrorOccurence)]
pub enum BigserialTryFromI64ErrorNamed<'a> {
    BelowZero {
        #[eo_display_with_serialize_deserialize]
        below_zero: i64,
        code_occurence: crate::common::code_occurence::CodeOccurence<'a>,
    },
}

impl Bigserial {
    //its not TryFrom<i64> coz its not supported lifetimes in Error annotation
    pub fn try_from_i64<'a>(
        possible_bigserial: i64,
    ) -> Result<Bigserial, BigserialTryFromI64ErrorNamed<'a>> {
        if possible_bigserial.is_positive() {
            Ok(Bigserial {
                bigserial: possible_bigserial,
            })
        } else {
            Err(BigserialTryFromI64ErrorNamed::BelowZero {
                below_zero: possible_bigserial,
                code_occurence: crate::code_occurence_tufa_common!(),
            })
        }
    }
}

pub trait GetPostgresBigserialId {
    fn get_postgres_bigserial_id(&self) -> &i64; //Bigserial
}
