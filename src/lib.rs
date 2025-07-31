mod bank_msg;
mod credit_card;
mod sign_on;

use bank_msg::BankMsgSrsV1;
use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use chrono::format::ParseError;
use credit_card::CCMsgSrsV1;
use sign_on::SignOnMsgSrsV1;
use std::fs::File;
use std::io::Read;
use thiserror::Error;

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
    #[error("Found an unexpected datetime format in the qfx file")]
    UnexpectedDateFormat(String),
    #[error("Missing a required value in the QFX file")]
    InvalidTransactionAmount(String),
    #[error("File not found")]
    FileNotFound(String),
    #[error("File read error")]
    FileReadError(String),
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
pub struct QFX {
    pub sign_on_msg_srs_v1: Option<SignOnMsgSrsV1>,
    pub credit_card_msg_srs_v1: Option<CCMsgSrsV1>,
    pub bank_msg_srs_v1: Option<BankMsgSrsV1>,
}

#[derive(Clone)]
pub struct Status {
    pub code: String,
    pub severity: String,
    pub message: Option<String>,
}

#[derive(Clone)]
pub struct LedgerBal {
    pub balance_amount: String,
    pub dt_as_of: DateTime<Utc>,
}

#[derive(Clone)]
pub struct AvailableBalance {
    pub balance_amount: String,
    pub dt_as_of: DateTime<Utc>,
}

#[derive(Clone)]
pub struct BankTranList {
    pub dt_start: DateTime<Utc>,
    pub dt_end: DateTime<Utc>,
    pub transactions: Vec<Stmttrn>,
}

#[derive(Clone)]
pub struct Stmttrn {
    pub trans_type: String,
    pub dt_posted: DateTime<Utc>,
    pub trans_amount: f64,
    pub fit_id: String,
    pub correct_fit_id: Option<String>,
    pub correct_action: Option<String>,
    pub name: String,
    pub memo: Option<String>,
    pub check_num: Option<String>, // Should only be used with CHECK or DEBIT transactions
}

impl QFX {
    // Generate
    pub fn new_from_file(file_path: &str) -> Result<Self, QFXParsingError> {
        // TODO: Add some validation for the file along with some logging to know what is going on
        let mut file =
            File::open(file_path).map_err(|e| QFXParsingError::FileNotFound(e.to_string()))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| QFXParsingError::FileReadError(e.to_string()))?;

        // Find the starting <OFX> tag and remove everything before it.
        // TODO: The information before that tag might actually be useful!
        if let Some(start_index) = contents.find("<OFX>") {
            contents = contents[start_index..].to_string();
        } else {
            return Err(QFXParsingError::UnexpectedToken(
                "Could not find the <OFX> tag in the file".to_string(),
            ));
        }

        let mut tokens = tokenize(&contents);
        while let Some(contents) = tokens.next() {
            match contents {
                "OFX" => {
                    return Ok(QFX::parse(&mut tokens)?);
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

impl<'a> Parseable<'a> for QFX {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut qfx = QFX {
            sign_on_msg_srs_v1: None,
            credit_card_msg_srs_v1: None,
            bank_msg_srs_v1: None,
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
                "BANKMSGSRSV1" => {
                    if let Some(_) = qfx.bank_msg_srs_v1 {
                        return Err(QFXParsingError::UnexpectedToken(
                            "The value for bank message srs v1 is already set".to_string(),
                        ));
                    }
                    qfx.bank_msg_srs_v1 = Some(BankMsgSrsV1::parse(tokens)?);
                }
                "/OFX" => {
                    return Ok(qfx);
                }
                _ => {
                    return Err(QFXParsingError::UnexpectedToken(format!(
                        "Found unexpected input in the OFX tag, Found Token: {}",
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
        let mut s_code = None;
        let mut s_severity = None;
        let mut s_message = None;
        while let Some(contents) = tokens.next() {
            match contents {
                "CODE" => {
                    if let Some(code) = tokens.next() {
                        // TODO: What if there is no other stuff? Should error out.
                        s_code = Some(code.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF("Expected token following the CODE token in the STATUS type of CREDITCARDMSGSRSV1".to_string()));
                    }
                }
                "SEVERITY" => {
                    if let Some(severity) = tokens.next() {
                        s_severity = Some(severity.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the SEVERITY token in the STATUS type"
                                .to_string(),
                        ));
                    }
                }
                "MESSAGE" => {
                    if let Some(message) = tokens.next() {
                        s_message = Some(message.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the MESSAGE token in the STATUS type"
                                .to_string(),
                        ));
                    }
                }
                "/STATUS" => {
                    return Ok(Self {
                        code: s_code.ok_or(QFXParsingError::MissingRequiredValue(
                            "Missing CODE in STATUS".to_string(),
                        ))?,
                        severity: s_severity.ok_or(QFXParsingError::MissingRequiredValue(
                            "Missing SEVERITY in STATUS".to_string(),
                        ))?,
                        message: s_message,
                    });
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

impl<'a> Parseable<'a> for LedgerBal {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut s_balance_amount = None;
        let mut s_dt_as_of = None;
        while let Some(contents) = tokens.next() {
            match contents {
                "BALAMT" => {
                    if let Some(balance_amount) = tokens.next() {
                        s_balance_amount = Some(balance_amount.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected value following BALAMT in LEDGERBAL".to_string(),
                        ));
                    }
                }
                "DTASOF" => {
                    if let Some(dt_as_of) = tokens.next() {
                        s_dt_as_of = Some(parse_ofx_datetime(dt_as_of).map_err(|e| {
                            QFXParsingError::UnexpectedDateFormat(format!(
                                "Failed to parse datetime for DTASOF with {}",
                                e
                            ))
                        })?);
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the DTASOF token".to_string(),
                        ));
                    }
                }
                "/LEDGERBAL" => {
                    return Ok(Self {
                        balance_amount: s_balance_amount.ok_or(
                            QFXParsingError::MissingRequiredValue(
                                "Missing BALAMT in LEDGERBAL".to_string(),
                            ),
                        )?,
                        dt_as_of: s_dt_as_of.ok_or(QFXParsingError::MissingRequiredValue(
                            "Missing DTASOF in LEDGERBAL".to_string(),
                        ))?,
                    });
                }
                _ => {
                    // Error case, unknown token seen
                    return Err(QFXParsingError::UnexpectedToken(format!(
                        "Found unexpected token {} in the LEDGERBAL type",
                        contents.to_string()
                    )));
                }
            }
        }
        return Err(QFXParsingError::UnexpectedEOF(
            "Found unexpected EOF. Was still expecting the '/AVAILBAL' token".to_string(),
        ));
    }
}

impl<'a> Parseable<'a> for AvailableBalance {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut s_balance_amount = None;
        let mut s_dt_as_of = None;
        while let Some(contents) = tokens.next() {
            match contents {
                "BALAMT" => {
                    if let Some(balance_amount) = tokens.next() {
                        s_balance_amount = Some(balance_amount.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the BALAMT token".to_string(),
                        ));
                    }
                }
                "DTASOF" => {
                    if let Some(dt_as_of) = tokens.next() {
                        s_dt_as_of = Some(parse_ofx_datetime(dt_as_of).map_err(|e| {
                            QFXParsingError::UnexpectedDateFormat(format!(
                                "Failed to parse datetime for DTASOF with {}",
                                e
                            ))
                        })?);
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the DTASOF token".to_string(),
                        ));
                    }
                }
                "/AVAILBAL" => {
                    return Ok(Self {
                        balance_amount: s_balance_amount.ok_or(
                            QFXParsingError::MissingRequiredValue(
                                "Missing BALAMT in AVAILBAL".to_string(),
                            ),
                        )?,
                        dt_as_of: s_dt_as_of.ok_or(QFXParsingError::MissingRequiredValue(
                            "Missing DTASOF in AVAILBAL".to_string(),
                        ))?,
                    });
                }
                _ => {
                    // Error case, unknown token seen
                    return Err(QFXParsingError::UnexpectedToken(format!(
                        "Found unexpected token {} in the AVAILBAL type",
                        contents.to_string()
                    )));
                }
            }
        }
        return Err(QFXParsingError::UnexpectedEOF(
            "Found unexpected EOF. Was still expecting the '/AVAILBAL' token".to_string(),
        ));
    }
}

impl<'a> Parseable<'a> for BankTranList {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut s_dt_start: Option<DateTime<Utc>> = None;
        let mut s_dt_end: Option<DateTime<Utc>> = None;
        let mut s_transactions: Vec<Stmttrn> = vec![];
        while let Some(contents) = tokens.next() {
            match contents {
                "DTSTART" => {
                    if let Some(dt_start) = tokens.next() {
                        s_dt_start = Some(parse_ofx_datetime(dt_start).map_err(|e| {
                            QFXParsingError::UnexpectedDateFormat(format!(
                                "Failed to parse datetime for DTSTART with {}",
                                e
                            ))
                        })?);
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the DTSTART token".to_string(),
                        ));
                    }
                }
                "DTEND" => {
                    if let Some(dt_end) = tokens.next() {
                        s_dt_end = Some(parse_ofx_datetime(dt_end).map_err(|e| {
                            QFXParsingError::UnexpectedDateFormat(format!(
                                "Failed to parse datetime for DTEND with {}",
                                e
                            ))
                        })?);
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the DTEND token".to_string(),
                        ));
                    }
                }
                "STMTTRN" => {
                    s_transactions.push(Stmttrn::parse(tokens)?);
                }
                "/BANKTRANLIST" => {
                    return Ok(Self {
                        dt_start: s_dt_start.ok_or(QFXParsingError::MissingRequiredValue(
                            "Missing DTSTART in BANKTRANLIST".to_string(),
                        ))?,
                        dt_end: s_dt_end.ok_or(QFXParsingError::MissingRequiredValue(
                            "Missing DTEND in BANKTRANLIST".to_string(),
                        ))?,
                        transactions: s_transactions,
                    });
                }
                _ => {
                    // Error case, unknown token seen
                    return Err(QFXParsingError::UnexpectedToken(format!(
                        "Found unexpected token {} in the BANKTRANLIST type",
                        contents.to_string()
                    )));
                }
            }
        }
        return Err(QFXParsingError::UnexpectedEOF(
            "Found unexpected EOF. Was still expecting the '/BANKTRANLIST' token".to_string(),
        ));
    }
}

impl<'a> Parseable<'a> for Stmttrn {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut s_trans_type: Option<String> = None;
        let mut s_dt_posted: Option<DateTime<Utc>> = None;
        let mut s_trans_amount: Option<f64> = None;
        let mut s_fit_id: Option<String> = None;
        let mut s_correct_fit_id: Option<String> = None;
        let mut s_name: Option<String> = None;
        let mut s_memo: Option<String> = None;
        let mut s_correct_action: Option<String> = None;
        let mut s_check_num: Option<String> = None;
        while let Some(contents) = tokens.next() {
            match contents {
                "TRNTYPE" => {
                    if let Some(trans_type) = tokens.next() {
                        s_trans_type = Some(trans_type.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the TRNTYPE token in STMTTRN".to_string(),
                        ));
                    }
                }
                "DTPOSTED" => {
                    if let Some(dt_posted) = tokens.next() {
                        s_dt_posted = Some(parse_ofx_datetime(dt_posted).map_err(|e| {
                            QFXParsingError::UnexpectedDateFormat(format!(
                                "Failed to parse datetime for DTPOSTED with {}",
                                e
                            ))
                        })?);
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the DTPOSTED token in STMTTRN".to_string(),
                        ));
                    }
                }
                "TRNAMT" => {
                    if let Some(trans_amount) = tokens.next() {
                        // TODO: This seems to parse unrepresentable values too. Figure out a way to properly parse the values.
                        s_trans_amount = Some(trans_amount.parse::<f64>().map_err(|_| {
                            QFXParsingError::InvalidTransactionAmount(format!(
                                "Invalid transaction amount {}",
                                trans_amount
                            ))
                        })?);
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the TRNAMT token in STMTTRN".to_string(),
                        ));
                    }
                }
                "FITID" => {
                    if let Some(fit_id) = tokens.next() {
                        s_fit_id = Some(fit_id.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the FITID token in STMTTRN".to_string(),
                        ));
                    }
                }
                "CORRECTFITID" => {
                    if let Some(correct_fit_id) = tokens.next() {
                        s_correct_fit_id = Some(correct_fit_id.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the CORRECTFITID token in STMTTRN"
                                .to_string(),
                        ));
                    }
                }
                "CORRECTACTION" => {
                    if let Some(action) = tokens.next() {
                        s_correct_action = Some(action.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the CORRECTACTION token in STMTTRN"
                                .to_string(),
                        ));
                    }
                }
                "NAME" => {
                    if let Some(name) = tokens.next() {
                        s_name = Some(name.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the NAME token in STMTTRN".to_string(),
                        ));
                    }
                }
                "MEMO" => {
                    if let Some(memo) = tokens.next() {
                        s_memo = Some(memo.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the MEMO token in STMTTRN".to_string(),
                        ));
                    }
                }
                "CHECKNUM" => {
                    if let Some(check_num) = tokens.next() {
                        s_check_num = Some(check_num.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the CHECKNUM token in STMTTRN".to_string(),
                        ));
                    }
                }
                "/STMTTRN" => {
                    return Ok(Self {
                        trans_type: s_trans_type.ok_or(QFXParsingError::MissingRequiredValue(
                            "TRANSTYPE value is required in STMTRN".to_string(),
                        ))?, // TODO: Need to get rid of these panics
                        dt_posted: s_dt_posted.ok_or(QFXParsingError::MissingRequiredValue(
                            "DTPOSTED value is required in STMTRN".to_string(),
                        ))?,
                        trans_amount: s_trans_amount.ok_or(
                            QFXParsingError::MissingRequiredValue(
                                "TRNSAMT value is required in STMTRN".to_string(),
                            ),
                        )?,
                        fit_id: s_fit_id.ok_or(QFXParsingError::MissingRequiredValue(
                            "FITID value is required in STMTRN".to_string(),
                        ))?,
                        correct_fit_id: s_correct_fit_id,
                        correct_action: s_correct_action,
                        name: s_name.ok_or(QFXParsingError::MissingRequiredValue(
                            "NAME value is required in STMTRN".to_string(),
                        ))?,
                        memo: s_memo,
                        check_num: s_check_num,
                    });
                }
                _ => {
                    // Error case, unknown token seen
                    return Err(QFXParsingError::UnexpectedToken(format!(
                        "Found unexpected token {} in the STMTTRN type",
                        contents.to_string()
                    )));
                }
            }
        }
        return Err(QFXParsingError::MissingRequiredValue(
            "Found unexpected EOF. Was still expecting the '/STMTTRN' token".to_string(),
        ));
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

// Helper tokenizer for input strings
fn tokenize(input: &str) -> impl Iterator<Item = &str> {
    // Tokenize on the <> tags and iterate over the vector that is produced.
    // TODO: VULNERABLE TO CODE INJECTION OR SOMETHING LIKE THAT? LOOK IN TO A BETTER APPROACH!
    input
        .split(['<', '>'].as_ref())
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
}

// TODO: TESTING: My bank gives a correct fit-id for some transactions even though it points to itself. Handle this gracefully.
#[cfg(test)]
mod datetime_tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_datetime_parser_valid_basic() {
        // Basic OFX datetime without timezone or T
        let dt = parse_ofx_datetime("20250725143000").unwrap();
        let correct_dt = Utc.with_ymd_and_hms(2025, 7, 25, 14, 30, 0).unwrap();
        assert_eq!(dt, correct_dt);
    }

    #[test]
    fn test_datetime_parser_valid_with_t() {
        // OFX datetime with T separator
        let dt = parse_ofx_datetime("20250725T143000").unwrap();
        let correct_dt = Utc.with_ymd_and_hms(2025, 7, 25, 14, 30, 0).unwrap();
        assert_eq!(dt, correct_dt);
    }

    #[test]
    fn test_datetime_parser_valid_with_z() {
        // OFX datetime with Z suffix
        let dt = parse_ofx_datetime("20250725T143000Z").unwrap();
        let correct_dt = Utc.with_ymd_and_hms(2025, 7, 25, 14, 30, 0).unwrap();
        assert_eq!(dt, correct_dt);
    }

    #[test]
    fn test_datetime_parser_valid_with_bracketed() {
        // OFX datetime with bracketed timezone info
        let dt = parse_ofx_datetime("20250725143000[+7:PDT]").unwrap();
        let correct_dt = Utc.with_ymd_and_hms(2025, 7, 25, 14, 30, 0).unwrap();
        assert_eq!(dt, correct_dt);
    }

    #[test]
    fn test_datetime_parser_valid_with_t_and_bracketed() {
        // OFX datetime with T and bracketed timezone info
        let dt = parse_ofx_datetime("20250725T143000[+7:PDT]").unwrap();
        let correct_dt = Utc.with_ymd_and_hms(2025, 7, 25, 14, 30, 0).unwrap();
        assert_eq!(dt, correct_dt);
    }

    #[test]
    fn test_datetime_parser_invalid() {
        // Invalid format should return error
        let dt = parse_ofx_datetime("invalid-date-string");
        assert!(dt.is_err());
    }
}

/// Module to test the Stmttrn type
#[cfg(test)]
mod stmttrn_tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_stmttrn_parse_valid_minimal() {
        let input = "\
                <TRNTYPE>DEBIT\
                <DTPOSTED>20250725T143000Z\
                <TRNAMT>-100.51\
                <FITID>12345\
                <NAME>Test Transaction\
                <CHECKNUM>1001\
            </STMTTRN>";
        let mut tokens = tokenize(input);
        let stmttrn = Stmttrn::parse(&mut tokens).unwrap();
        assert_eq!(stmttrn.trans_type, "DEBIT");
        assert_eq!(
            stmttrn.dt_posted,
            Utc.with_ymd_and_hms(2025, 7, 25, 14, 30, 0).unwrap()
        );
        assert_eq!(stmttrn.trans_amount, -100.51);
        assert_eq!(stmttrn.fit_id, "12345");
        assert_eq!(stmttrn.name, "Test Transaction");
        assert!(stmttrn.memo.is_none());
        assert!(stmttrn.correct_fit_id.is_none());
        assert!(stmttrn.correct_action.is_none());
    }

    #[test]
    fn test_stmttrn_parse_invalid_transaction() {
        let input = "\
                <TRNTYPE>DEBIT\
                <DTPOSTED>20250725T143000Z\
                <TRNAMT>100123456-7891\
                <FITID>12345\
                <NAME>Test Transaction\
                <CHECKNUM>1001\
            </STMTTRN>";
        let mut tokens = tokenize(input);
        let stmttrn = Stmttrn::parse(&mut tokens);
        assert!(stmttrn.is_err());
        if let Err(e) = stmttrn {
            assert!(
                matches!(e, QFXParsingError::InvalidTransactionAmount(_)),
                "Expected InvalidTransactionAmount error, got: {:?}",
                e
            );
        }
    }

    #[test]
    fn test_stmttrn_parse_missing_transaction() {
        let input = "\
                <TRNTYPE>DEBIT\
                <DTPOSTED>20250725T143000Z\
                <FITID>12345\
                <NAME>Test Transaction\
            </STMTTRN>";
        let mut tokens = tokenize(input);
        let stmttrn = Stmttrn::parse(&mut tokens);
        assert!(stmttrn.is_err());
        if let Err(e) = stmttrn {
            assert!(
                matches!(e, QFXParsingError::MissingRequiredValue(_)),
                "Expected MissingRequiredValue error, got: {:?}",
                e
            );
        }
    }

    #[test]
    fn test_stmttrn_parse_missing_date_posted() {
        let input = "\
                <TRNTYPE>DEBIT\
                <TRNAMT>1001237891\
                <FITID>12345\
                <NAME>Test Transaction\
            </STMTTRN>";
        let mut tokens = tokenize(input);
        let stmttrn = Stmttrn::parse(&mut tokens);
        assert!(stmttrn.is_err());
        if let Err(e) = stmttrn {
            assert!(
                matches!(e, QFXParsingError::MissingRequiredValue(_)),
                "Expected MissingRequiredValue error, got: {:?}",
                e
            );
        }
    }

    #[test]
    fn test_stmttrn_parse_missing_name() {
        let input = "\
                <TRNTYPE>DEBIT\
                <DTPOSTED>20250725T143000Z\
                <TRNAMT>1001237891\
                <FITID>12345\
            </STMTTRN>";
        let mut tokens = tokenize(input);
        let stmttrn = Stmttrn::parse(&mut tokens);
        assert!(stmttrn.is_err());
        if let Err(e) = stmttrn {
            assert!(
                matches!(e, QFXParsingError::MissingRequiredValue(_)),
                "Expected MissingRequiredValue error, got: {:?}",
                e
            );
        }
    }

    #[test]
    fn test_stmttrn_parse_missing_fitid() {
        let input = "\
                <TRNTYPE>DEBIT\
                <DTPOSTED>20250725T143000Z\
                <TRNAMT>1001237891\
                <NAME>Test Transaction\
            </STMTTRN>";
        let mut tokens = tokenize(input);
        let stmttrn = Stmttrn::parse(&mut tokens);
        assert!(stmttrn.is_err());
        if let Err(e) = stmttrn {
            assert!(
                matches!(e, QFXParsingError::MissingRequiredValue(_)),
                "Expected MissingRequiredValue error, got: {:?}",
                e
            );
        }
    }

    #[test]
    fn test_stmttrn_parse_missing_transaction_type() {
        let input = "\
                <DTPOSTED>20250725T143000Z\
                <TRNAMT>1001237891\
                <FITID>12345\
                <NAME>Test Transaction\
            </STMTTRN>";
        let mut tokens = tokenize(input);
        let stmttrn = Stmttrn::parse(&mut tokens);
        assert!(stmttrn.is_err());
        if let Err(e) = stmttrn {
            assert!(
                matches!(e, QFXParsingError::MissingRequiredValue(_)),
                "Expected MissingRequiredValue error, got: {:?}",
                e
            );
        }
    }
}

#[cfg(test)]
mod qfx_file_tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_qfx_parse_valid_file() {
        let file_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/data/sample_bank_msg_transactions.qfx"
        );
        println!("{}", file_path);
        let result = QFX::new_from_file(file_path);
        assert!(
            result.is_ok(),
            "Expected QFX::new_from_file to succeed, got error: {:?}",
            result.err()
        );
        let qfx = result.unwrap();
        // Optionally, check that at least one of the main sections is present
        assert!(
            qfx.sign_on_msg_srs_v1.is_some()
                || qfx.credit_card_msg_srs_v1.is_none()
                || qfx.bank_msg_srs_v1.is_some(),
            "Expected at least one section to be present in parsed QFX"
        );

        let bank_transactions = qfx
            .bank_msg_srs_v1
            .unwrap()
            .stmttrns
            .stmtrs
            .banktranslist
            .transactions;
        assert_eq!(bank_transactions.len(), 2);

        // First transaction
        let t0 = &bank_transactions[0];
        assert_eq!(t0.trans_type, "DEBIT");
        assert_eq!(
            t0.dt_posted,
            chrono::Utc.with_ymd_and_hms(2025, 7, 15, 8, 0, 0).unwrap()
        );
        assert_eq!(t0.trans_amount, -55.75);
        assert_eq!(t0.fit_id, "TXN123456");
        assert_eq!(t0.check_num, Some("1005".to_string()));
        assert_eq!(t0.name, "GROCERY STORE");
        assert_eq!(t0.memo, Some("Weekly groceries".to_string()));

        // Second transaction
        let t1 = &bank_transactions[1];
        assert_eq!(t1.trans_type, "CREDIT");
        assert_eq!(
            t1.dt_posted,
            chrono::Utc.with_ymd_and_hms(2025, 7, 16, 9, 0, 0).unwrap()
        );
        assert_eq!(t1.trans_amount, 1000.00);
        assert_eq!(t1.fit_id, "TXN123457");
        assert_eq!(t1.check_num, None);
        assert_eq!(t1.name, "PAYROLL");
        assert_eq!(t1.memo, Some("DIRECT DEPOSIT".to_string()));

        let cc_transactions = qfx
            .credit_card_msg_srs_v1
            .unwrap()
            .ccstmttrns
            .ccstmtrs
            .banktranslist
            .transactions;
        assert_eq!(cc_transactions.len(), 2);

        // First transaction
        let t0 = &cc_transactions[0];
        assert_eq!(t0.trans_type, "DEBIT");
        assert_eq!(
            t0.dt_posted,
            chrono::Utc.with_ymd_and_hms(2025, 7, 15, 8, 0, 0).unwrap()
        );
        assert_eq!(t0.trans_amount, -55.75);
        assert_eq!(t0.fit_id, "TXN123456");
        assert_eq!(t0.check_num, None);
        assert_eq!(t0.name, "CASH BACK");
        assert_eq!(t0.memo, Some("Weekly groceries".to_string()));

        // Second transaction
        let t1 = &bank_transactions[1];
        assert_eq!(t1.trans_type, "CREDIT");
        assert_eq!(
            t1.dt_posted,
            chrono::Utc.with_ymd_and_hms(2025, 7, 16, 9, 0, 0).unwrap()
        );
        assert_eq!(t1.trans_amount, 1000.00);
        assert_eq!(t1.fit_id, "TXN123457");
        assert_eq!(t1.check_num, None);
        assert_eq!(t1.name, "PAYROLL");
        assert_eq!(t1.memo, Some("DIRECT DEPOSIT".to_string()));
    }

    #[test]
    fn test_qfx_parse_valid_file_no_transactions() {
        let file_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/data/sample_bank_msg_no_transactions.qfx"
        );
        let result = QFX::new_from_file(file_path);
        assert!(
            result.is_ok(),
            "Expected QFX::new_from_file to succeed, got error: {:?}",
            result.err()
        );
        let qfx = result.unwrap();
        // Optionally, check that at least one of the main sections is present
        assert!(
            qfx.sign_on_msg_srs_v1.is_some()
                || qfx.credit_card_msg_srs_v1.is_none()
                || qfx.bank_msg_srs_v1.is_some(),
            "Expected at least one section to be present in parsed QFX"
        );

        let bank_transactions = qfx
            .bank_msg_srs_v1
            .unwrap()
            .stmttrns
            .stmtrs
            .banktranslist
            .transactions;
        assert_eq!(bank_transactions.len(), 0);

        let cc_transactions = qfx
            .credit_card_msg_srs_v1
            .unwrap()
            .ccstmttrns
            .ccstmtrs
            .banktranslist
            .transactions;
        assert_eq!(cc_transactions.len(), 0);
    }

    #[test]
    fn test_qfx_parse_missing_file() {
        let result = QFX::new_from_file("tests/data/does_not_exist.qfx");
        assert!(
            result.is_err(),
            "Expected error when parsing non-existent file"
        );
    }
}

#[cfg(test)]
mod available_balance_tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_available_balance_parse_missing_amount() {
        // Missing BALAMT and DTASOF
        let input = "<DTASOF>20250725T143000Z</AVAILBAL>";
        let mut tokens = tokenize(input);
        let result = AvailableBalance::parse(&mut tokens);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                matches!(e, QFXParsingError::MissingRequiredValue(_)),
                "Expected MissingRequiredValue error, got: {:?}",
                e
            );
        }
    }

    #[test]
    fn test_available_balance_parse_missing_date() {
        // Missing BALAMT and DTASOF
        let input = "<BALAMT>1234.56</AVAILBAL>";
        let mut tokens = input
            .split(['<', '>'].as_ref())
            .filter(|x| !x.trim().is_empty());
        let result = AvailableBalance::parse(&mut tokens);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                matches!(e, QFXParsingError::MissingRequiredValue(_)),
                "Expected MissingRequiredValue error, got: {:?}",
                e
            );
        }
    }

    #[test]
    fn test_available_balance_parse_valid() {
        let input = "\
            <BALAMT>1234.56\
            <DTASOF>20250725T143000Z\
        </AVAILBAL>";
        let mut tokens = input
            .split(['<', '>'].as_ref())
            .filter(|x| !x.trim().is_empty());
        let result = AvailableBalance::parse(&mut tokens);

        assert!(
            result.is_ok(),
            "Expected parse to succeed, got: {:?}",
            result.err()
        );
        let avail_bal = result.unwrap();
        assert_eq!(avail_bal.balance_amount, "1234.56");
        assert_eq!(
            avail_bal.dt_as_of,
            chrono::Utc
                .with_ymd_and_hms(2025, 7, 25, 14, 30, 0)
                .unwrap()
        );
    }
}

#[cfg(test)]
mod ledger_balance_tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_ledger_balance_parse_missing_amount() {
        // Missing BALAMT and DTASOF
        let input = "<DTASOF>20250725T143000Z</LEDGERBAL>";
        let mut tokens = input
            .split(['<', '>'].as_ref())
            .filter(|x| !x.trim().is_empty());
        let result = LedgerBal::parse(&mut tokens);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                matches!(e, QFXParsingError::MissingRequiredValue(_)),
                "Expected MissingRequiredValue error, got: {:?}",
                e
            );
        }
    }

    #[test]
    fn test_ledger_balance_parse_missing_date() {
        // Missing BALAMT and DTASOF
        let input = "<BALAMT>1234.56</LEDGERBAL>";
        let mut tokens = input
            .split(['<', '>'].as_ref())
            .filter(|x| !x.trim().is_empty());
        let result = LedgerBal::parse(&mut tokens);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                matches!(e, QFXParsingError::MissingRequiredValue(_)),
                "Expected MissingRequiredValue error, got: {:?}",
                e
            );
        }
    }

    #[test]
    fn test_ledger_balance_parse_valid() {
        let input = "\
            <BALAMT>1234.56\
            <DTASOF>20250725T143000Z\
        </LEDGERBAL>";
        let mut tokens = input
            .split(['<', '>'].as_ref())
            .filter(|x| !x.trim().is_empty());
        let result = LedgerBal::parse(&mut tokens);

        assert!(
            result.is_ok(),
            "Expected parse to succeed, got: {:?}",
            result.err()
        );
        let avail_bal = result.unwrap();
        assert_eq!(avail_bal.balance_amount, "1234.56");
        assert_eq!(
            avail_bal.dt_as_of,
            chrono::Utc
                .with_ymd_and_hms(2025, 7, 25, 14, 30, 0)
                .unwrap()
        );
    }
}

#[cfg(test)]
mod banktranlist_tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_banktranlist_parse_valid() {
        let input = "\
            <DTSTART>20250725T143000Z\
            <DTEND>20250726T143000Z\
            <STMTTRN>\
                <TRNTYPE>DEBIT\
                <DTPOSTED>20250725T143000Z\
                <TRNAMT>-100.51\
                <FITID>12345\
                <NAME>Test Transaction\
            </STMTTRN>\
            <STMTTRN>\
                <TRNTYPE>CREDIT\
                <DTPOSTED>20250726T143000Z\
                <TRNAMT>200.00\
                <FITID>12346\
                <NAME>Another Transaction\
            </STMTTRN>\
        </BANKTRANLIST>";
        let mut tokens = tokenize(input);
        let result = BankTranList::parse(&mut tokens);
        assert!(
            result.is_ok(),
            "Expected parse to succeed, got: {:?}",
            result.err()
        );
        let banktranlist = result.unwrap();
        assert_eq!(
            banktranlist.dt_start,
            Utc.with_ymd_and_hms(2025, 7, 25, 14, 30, 0).unwrap()
        );
        assert_eq!(
            banktranlist.dt_end,
            Utc.with_ymd_and_hms(2025, 7, 26, 14, 30, 0).unwrap()
        );
        assert_eq!(banktranlist.transactions.len(), 2);
        assert_eq!(banktranlist.transactions[0].trans_type, "DEBIT");
        assert_eq!(banktranlist.transactions[1].trans_type, "CREDIT");
    }

    #[test]
    fn test_banktranlist_parse_missing_dtstart() {
        let input = "\
            <DTEND>20250726T143000Z\
            </BANKTRANLIST>";
        let mut tokens = tokenize(input);
        let result = BankTranList::parse(&mut tokens);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                matches!(e, QFXParsingError::MissingRequiredValue(_)),
                "Expected MissingRequiredValue error for missing DTSTART, got: {:?}",
                e
            );
        }
    }

    #[test]
    fn test_banktranlist_parse_missing_dtend() {
        let input = "\
            <DTSTART>20250725T143000Z\
            </BANKTRANLIST>";
        let mut tokens = tokenize(input);
        let result = BankTranList::parse(&mut tokens);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                matches!(e, QFXParsingError::MissingRequiredValue(_)),
                "Expected MissingRequiredValue error for missing DTEND, got: {:?}",
                e
            );
        }
    }

    #[test]
    fn test_banktranlist_parse_missing_end_tag() {
        let input = "\
            <DTSTART>20250725T143000Z\
            <DTEND>20250726T143000Z\
        ";
        let mut tokens = tokenize(input);
        let result = BankTranList::parse(&mut tokens);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(
                matches!(e, QFXParsingError::UnexpectedEOF(_)),
                "Expected UnexpectedEOF error for missing /BANKTRANLIST, got: {:?}",
                e
            );
        }
    }
}

#[cfg(test)]
mod status_tests {
    use super::*;

    #[test]
    fn test_status_parse_valid_all_fields() {
        let input = "\
            <CODE>200\
            <SEVERITY>INFO\
            <MESSAGE>Everything OK\
        </STATUS>";
        let mut tokens = tokenize(input);
        let result = Status::parse(&mut tokens);
        assert!(result.is_ok());
        let status = result.unwrap();
        assert_eq!(status.code, "200");
        assert_eq!(status.severity, "INFO");
        assert_eq!(status.message, Some("Everything OK".to_string()));
    }

    #[test]
    fn test_status_parse_missing_code() {
        let input = "\
            <SEVERITY>ERROR\
            <MESSAGE>Missing code\
        </STATUS>";
        let mut tokens = tokenize(input);
        let result = Status::parse(&mut tokens);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, QFXParsingError::MissingRequiredValue(_)));
        }
    }

    #[test]
    fn test_status_parse_missing_severity() {
        let input = "\
            <CODE>404\
            <MESSAGE>Missing severity\
        </STATUS>";
        let mut tokens = tokenize(input);
        let result = Status::parse(&mut tokens);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, QFXParsingError::MissingRequiredValue(_)));
        }
    }
}
