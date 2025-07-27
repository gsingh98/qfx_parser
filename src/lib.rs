mod credit_card;
mod sign_on;

use std::fs::File;
use std::io::Read;

use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use chrono::format::ParseError;
use sign_on::SignOnMsgSrsV1;
use thiserror::Error;

use crate::credit_card::CCMsgSrsV1;

#[derive(Error, Debug)]
pub enum QFXParsingError {
    #[error("Incorrect format")]
    NotFoundError(), // TODO: REMOVE THIS LATER
    #[error("Unexpected token found")]
    UnexpectedToken(String),
    #[error("Found unexpected end of file. Expecting more tokens")]
    UnexpectedEOF(String),
    #[error("Missing a required value in the QFX file")]
    MissingRequiredValue(String),
    #[error("Missing a required value in the QFX file")]
    UnexpectedDateFormat(),
    #[error("Missing a required value in the QFX file")]
    InvalidTransactionAmount(),
}

pub(crate) trait Parseable<'a> {
    // Parsing function that takes in an iterator on some tokens. Consumes some tokens
    // and increments the Iterator.
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError>
    where
        Self: Sized;
}

#[derive(Clone)]
/// TODO: More types will need to be supported later
pub struct OFX {
    pub sign_on_msg_srs_v1: Option<SignOnMsgSrsV1>,
    pub credit_card_msg_srs_v1: Option<CCMsgSrsV1>,
}

#[derive(Clone)]
pub struct Status {
    pub code: Option<String>,
    pub severity: Option<String>,
    pub message: Option<String>,
}

impl OFX {
    // Generate
    pub fn new_from_file(file_path: &str) -> Result<Self, QFXParsingError> {
        // TODO: Add some validation for the file along with some logging to know what is going on
        let mut file = File::open(file_path).expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read the file");

        // Find the starting <OFX> tag and remove everything before it.
        // TODO: The information before that tag might actually be useful!
        if let Some(start_index) = contents.find("<OFX>") {
            contents = contents[start_index..].to_string();
        } else {
            return Err(QFXParsingError::UnexpectedToken(
                "Could not find the <OFX> tag in the file".to_string(),
            ));
        }

        // Tabs, newline, whitespace etc can be removed
        contents.retain(|c| !c.is_whitespace());
        // Tokenize on the <> tags and iterate over the vector that is produced.
        // TODO: VULNERABLE TO CODE INJECTION OR SOMETHING LIKE THAT? LOOK IN TO A BETTER APPROACH!
        let output_tokens = contents.split(['<', '>'].as_ref());
        let output = output_tokens.filter(|x| *x != "");
        let mut tokens = output.into_iter();
        while let Some(contents) = tokens.next() {
            match contents {
                "OFX" => {
                    return Ok(OFX::parse(&mut tokens)?);
                }
                _ => {
                    // Error case, unknown token seen
                    return Err(QFXParsingError::UnexpectedToken(format!(
                        "Found an unexpected token. Expecting: OFX, Found {}",
                        contents.to_string()
                    )));
                }
            }
        }
        return Err(QFXParsingError::UnexpectedEOF(
            "Found unexpected EOF. Was still expecting the OFX token to start parsing the file"
                .to_string(),
        ));
    }
}

impl<'a> Parseable<'a> for OFX {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut qfx = OFX {
            sign_on_msg_srs_v1: None,
            credit_card_msg_srs_v1: None,
        };
        while let Some(contents) = tokens.next() {
            match contents {
                // TODO: There are a few more types left to account for!
                "SIGNONMSGSRSV1" => {
                    if let Some(_) = qfx.sign_on_msg_srs_v1 {
                        return Err(QFXParsingError::UnexpectedToken(
                            "The value for sign on message srs v1 is already set".to_string(),
                        ));
                    }
                    qfx.sign_on_msg_srs_v1 = Some(SignOnMsgSrsV1::parse(tokens)?);
                }
                "CREDITCARDMSGSRSV1" => {
                    qfx.credit_card_msg_srs_v1 = Some(CCMsgSrsV1::parse(tokens)?);
                }
                "/OFX" => {
                    return Ok(qfx);
                }
                _ => {
                    return Err(QFXParsingError::UnexpectedToken(format!(
                        "Found unexpected input. Expected Token: /OFX, Found Token: {}",
                        contents.to_string()
                    )));
                }
            }
        }
        return Err(QFXParsingError::UnexpectedEOF(
            "Found unexpected EOF. Was still expecting the '/OFX' token to end parsing".to_string(),
        ));
    }
}

impl<'a> Parseable<'a> for Status {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut status = Self {
            code: None,
            severity: None,
            message: None,
        };
        while let Some(contents) = tokens.next() {
            match contents {
                "CODE" => {
                    if let Some(code) = tokens.next() {
                        // TODO: What if there is no other stuff? Should error out.
                        status.code = Some(code.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF("Expected token following the CODE token in the STATUS type of CREDITCARDMSGSRSV1".to_string()));
                    }
                }
                "SEVERITY" => {
                    if let Some(severity) = tokens.next() {
                        status.severity = Some(severity.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the SEVERITY token in the STATUS type"
                                .to_string(),
                        ));
                    }
                }
                "MESSAGE" => {
                    if let Some(message) = tokens.next() {
                        status.message = Some(message.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the MESSAGE token in the STATUS type"
                                .to_string(),
                        ));
                    }
                }
                "/STATUS" => {
                    return Ok(status);
                }
                _ => {
                    // Error case, unknown token seen
                    return Err(QFXParsingError::UnexpectedToken(format!(
                        "Found unexpected token {} in the STATUS type of CREDITCARDMSGSRSV1",
                        contents.to_string()
                    )));
                }
            }
        }
        return Err(QFXParsingError::UnexpectedEOF("Found unexpected EOF. Was still expecting the '/STATUS' token in the CREDITCARDMSGSRSV1 subtype".to_string()));
    }
}

// Parses a date time in the OFX standard format. Expects something like: 20250725143000[+7:PDT], 20250725T143000[+7:PDT], 20250725T143000Z, 20250725143000
// TODO: For the time being this will ignore the timezone at the end and treat everything in UTC time. %Z in chrono
// does not have the concept of timezones because it claims ambiguitiy (CST = China Standard Time or Central Standard Time)
fn parse_ofx_datetime(s: &str) -> Result<DateTime<Utc>, ParseError> {
    // Remove the ending [0:PDT] stuff.
    let mut s = remove_last_bracketed(s);
    s = s.strip_suffix("Z").unwrap_or(s);

    // Attempt to parse with milliseconds
    let format_with_tz_ms = "%Y%m%d%H%M%S%.f";
    if let Ok(dt) = NaiveDateTime::parse_from_str(s, format_with_tz_ms) {
        return Ok(dt.and_utc());
    }

    let format_with_tz_ms = "%Y%m%dT%H%M%S%.f";
    if let Ok(dt) = NaiveDateTime::parse_from_str(s, format_with_tz_ms) {
        return Ok(dt.and_utc());
    }

    // Attempt to parse without any time information
    let format_with_tz_ms = "%Y%m%dT%H%M%S";
    if let Ok(dt) = NaiveDateTime::parse_from_str(s, format_with_tz_ms) {
        return Ok(dt.and_utc());
    }

    // Attempt to parse with a seconds granularity
    let format_with_s = "%Y%m%d%H%M%S";
    NaiveDateTime::parse_from_str(s, format_with_s).map(|dt| dt.and_utc())
}

// Removes trailing brackets from a string. Eg. helloworld[xxxx] -> helloworld
fn remove_last_bracketed(s: &str) -> &str {
    if let Some(pos) = s.rfind('[') {
        return &s[..pos];
    }
    s
}
