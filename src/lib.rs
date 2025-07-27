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
    pub bank_msg_srs_v1: Option<BankMsgSrsV1>,
}

#[derive(Clone)]
pub struct Status {
    pub code: Option<String>,
    pub severity: Option<String>,
    pub message: Option<String>,
}

#[derive(Clone)]
pub struct LedgerBal {
    pub balance_amount: Option<String>,
    pub dt_as_of: Option<String>,
}

#[derive(Clone)]
pub struct AvailableBalance {
    pub balance_amount: Option<String>,
    pub dt_as_of: Option<String>,
}

#[derive(Clone)]
pub struct BankTranList {
    pub dt_start: Option<String>,
    pub dt_end: Option<String>,
    pub transactions: Vec<Stmttrn>, // TODO: Should not have pub(crate) visibility
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

impl<'a> Parseable<'a> for LedgerBal {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut ledger_balance = Self {
            balance_amount: None,
            dt_as_of: None,
        };
        while let Some(contents) = tokens.next() {
            match contents {
                "BALAMT" => {
                    if let Some(balance_amount) = tokens.next() {
                        ledger_balance.balance_amount = Some(balance_amount.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the BALAMT token".to_string(),
                        ));
                    }
                }
                "DTASOF" => {
                    if let Some(dt_as_of) = tokens.next() {
                        ledger_balance.dt_as_of = Some(dt_as_of.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the DTASOF token".to_string(),
                        ));
                    }
                }
                "/LEDGERBAL" => {
                    return Ok(ledger_balance);
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
            "Found unexpected EOF. Was still expecting the '/LEDGERBAL' token".to_string(),
        ));
    }
}

impl<'a> Parseable<'a> for AvailableBalance {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut available_balance = Self {
            balance_amount: None,
            dt_as_of: None,
        };
        while let Some(contents) = tokens.next() {
            match contents {
                "BALAMT" => {
                    if let Some(balance_amount) = tokens.next() {
                        available_balance.balance_amount = Some(balance_amount.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the BALAMT token".to_string(),
                        ));
                    }
                }
                "DTASOF" => {
                    if let Some(dt_as_of) = tokens.next() {
                        available_balance.dt_as_of = Some(dt_as_of.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the DTASOF token".to_string(),
                        ));
                    }
                }
                "/AVAILBAL" => {
                    return Ok(available_balance);
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
        let mut bank_transactions_list = Self {
            dt_start: None,
            dt_end: None,
            transactions: vec![],
        };
        while let Some(contents) = tokens.next() {
            match contents {
                "DTSTART" => {
                    if let Some(dt_start) = tokens.next() {
                        bank_transactions_list.dt_start = Some(dt_start.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the DTSTART token".to_string(),
                        ));
                    }
                }
                "DTEND" => {
                    if let Some(dt_end) = tokens.next() {
                        bank_transactions_list.dt_end = Some(dt_end.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the DTEND token".to_string(),
                        ));
                    }
                }
                "STMTTRN" => {
                    bank_transactions_list
                        .transactions
                        .push(Stmttrn::parse(tokens)?);
                }
                "/BANKTRANLIST" => {
                    return Ok(bank_transactions_list);
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
                        s_dt_posted = Some(
                            parse_ofx_datetime(dt_posted)
                                .map_err(|_| QFXParsingError::UnexpectedDateFormat())?,
                        );
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the DTPOSTED token in STMTTRN".to_string(),
                        ));
                    }
                }
                "TRNAMT" => {
                    if let Some(trans_amount) = tokens.next() {
                        s_trans_amount = Some(
                            trans_amount
                                .parse::<f64>()
                                .map_err(|_| QFXParsingError::InvalidTransactionAmount())?,
                        );
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

// TODO: TESTING: My bank gives a correct fit-id for some transactions even though it points to itself. Handle this gracefully.

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Timelike};

    use super::*;

    fn parse_stmttrn_from_tokens(tokens: Vec<&str>) -> Result<Stmttrn, QFXParsingError> {
        let mut iter = tokens.into_iter();
        Stmttrn::parse(&mut iter)
    }

    #[test]
    fn test_stmttrn_invalidvalid_datetime() {
        let tokens = vec![
            "TRNTYPE",
            "DEBIT",
            "DTPOSTED",
            "202440101T123456Z",
            "TRNAMT",
            "-50.25",
            "FITID",
            "12345",
            "NAME",
            "Test Transaction",
            "/STMTTRN",
        ];
        let stmttrn = parse_stmttrn_from_tokens(tokens);
        assert!(matches!(
            stmttrn,
            Err(QFXParsingError::UnexpectedDateFormat())
        ));
    }

    #[test]
    fn test_stmttrn_invalid_datetime_short() {
        let tokens = vec![
            "TRNTYPE",
            "CREDIT",
            "DTPOSTED",
            "20240101",
            "TRNAMT",
            "100.00",
            "FITID",
            "54321",
            "NAME",
            "Short Date",
            "/STMTTRN",
        ];
        let stmttrn = parse_stmttrn_from_tokens(tokens);
        assert!(
            stmttrn.is_err(),
            "Parsing datetime of format 20240101 succeeded unexpectedly"
        );
    }

    #[test]
    fn test_stmttrn_valid_datetime_with_offset() {
        // TODO: This should parse correctly but just interpret the time in UTC
        let tokens = vec![
            "TRNTYPE",
            "CREDIT",
            "DTPOSTED",
            "20240101T123456[-5:EST]",
            "TRNAMT",
            "200.00",
            "FITID",
            "67890",
            "NAME",
            "Offset Date",
            "/STMTTRN",
        ];
        let stmttrn = parse_stmttrn_from_tokens(tokens);
        assert!(
            stmttrn.is_ok(),
            "Parsing datetime of format 20240101T123456[-5:EST] failed"
        );
        let stmttrn = stmttrn.unwrap();
        // The parser should convert to UTC
        assert_eq!(
            stmttrn.dt_posted,
            Utc.with_ymd_and_hms(2024, 1, 1, 12, 34, 56).unwrap()
        );
    }

    #[test]
    fn test_stmttrn_valid_datetime_with_fractional_seconds() {
        let tokens = vec![
            "TRNTYPE",
            "CREDIT",
            "DTPOSTED",
            "20240101T123456.789Z",
            "TRNAMT",
            "300.00",
            "FITID",
            "98765",
            "NAME",
            "Fractional Seconds",
            "/STMTTRN",
        ];
        let stmttrn = parse_stmttrn_from_tokens(tokens);
        assert!(
            stmttrn.is_ok(),
            "Parsing datetime of format 20240101T123456.789Z failed {:?}",
            stmttrn.err()
        );
        let stmttrn = stmttrn.unwrap();
        assert_eq!(
            stmttrn.dt_posted,
            Utc.with_ymd_and_hms(2024, 01, 01, 12, 34, 56)
                .unwrap()
                .with_nanosecond(789000000)
                .unwrap()
        );
    }
}
