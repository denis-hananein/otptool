mod internal {
    include!(concat!(env!("OUT_DIR"), "/migration.rs"));

    pub fn secret_to_string(secret: &[u8]) -> String {
        return data_encoding::BASE32_NOPAD.encode(secret);
    }
}

#[derive(Debug, Clone)]
pub struct OtpParameters {
    pub name: String,
    pub secret: String,
    pub issuer: String,
    pub algorithm: Algorithm,
    pub digits: DigitCount,
    pub counter: u64,
}

impl OtpParameters {
    pub fn from_base64(data: &[u8]) -> anyhow::Result<Vec<Self>> {
        let decoded_data = match data_encoding::BASE64.decode(data) {
            Ok(data) => data,
            Err(e) => return Err(anyhow::anyhow!(e)),
        };

        let migration_data: internal::Payload =
            match prost::Message::decode(decoded_data.as_slice()) {
                Ok(data) => data,
                Err(e) => return Err(anyhow::anyhow!(e)),
            };

        let mut otp_parameters = Vec::new();
        for op in migration_data.otp_parameters {
            otp_parameters.push(Self::from_proto(&op)?);
        }

        return Ok(otp_parameters);
    }

    fn from_proto(op: &internal::payload::OtpParameters) -> anyhow::Result<Self> {
        return Ok(OtpParameters {
            name: op.name.clone(),
            secret: internal::secret_to_string(&op.secret),
            issuer: op.issuer.clone(),
            algorithm: Algorithm::try_from(op.algorithm)?,
            digits: DigitCount::try_from(op.digits)?,
            counter: op.counter,
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum Algorithm {
    Unspecified = 0,
    Sha1 = 1,
    Sha256 = 2,
    Sha512 = 3,
    Md5 = 4,
}

impl TryFrom<i32> for Algorithm {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Algorithm::Unspecified),
            1 => Ok(Algorithm::Sha1),
            2 => Ok(Algorithm::Sha256),
            3 => Ok(Algorithm::Sha512),
            4 => Ok(Algorithm::Md5),
            _ => Err(anyhow::anyhow!("unknown algorithm")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum DigitCount {
    Unspecified = 0,
    Six = 1,
    Eight = 2,
}

impl TryFrom<i32> for DigitCount {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DigitCount::Unspecified),
            1 => Ok(DigitCount::Six),
            2 => Ok(DigitCount::Eight),
            _ => Err(anyhow::anyhow!("unknown digit count")),
        }
    }
}
