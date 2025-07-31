use crate::Parseable;
use crate::QFXParsingError;
use crate::Status;
use crate::parse_ofx_datetime;
use chrono::DateTime;
use chrono::Utc;

// TODO: Require doc comments
// TODO: Require clippy formatting

#[derive(Clone)]
pub struct SignOnMsgSrsV1 {
    sonrs: Option<Sonrs>,
}

#[derive(Clone)]
pub struct Sonrs {
    status: Option<Status>,
    fi: FinancialInstitution,
    bid: Option<String>,
    dt_server: DateTime<Utc>, // TODO: Should be using the DateTime parser for this part
    dt_acctup: Option<String>,
    language: Option<String>,
    cookie: Option<String>,
    user_id: Option<String>,
}

#[derive(Clone)]
pub(crate) struct FinancialInstitution {
    pub(crate) org: Option<String>,
    pub(crate) fid: Option<String>, // TODO: Needs to be numeric
}

impl<'a> Parseable<'a> for SignOnMsgSrsV1 {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut sign_on_msg_srs_v1 = Self { sonrs: None };
        while let Some(contents) = tokens.next() {
            match contents {
                "SONRS" => {
                    sign_on_msg_srs_v1.sonrs = Some(Sonrs::parse(tokens)?);
                }
                "/SIGNONMSGSRSV1" => {
                    return Ok(sign_on_msg_srs_v1);
                }
                _ => {
                    // Error case, unknown token seen
                    return Err(QFXParsingError::UnexpectedToken(format!(
                        "Found unexpected token {} in the SIGNONMSGSRSV1 type",
                        contents.to_string()
                    )));
                }
            }
        }
        return Err(QFXParsingError::UnexpectedEOF(
            "Found unexpected EOF. Was still expecting the '/SIGNONMSGSRSV1' token".to_string(),
        ));
    }
}

impl<'a> Parseable<'a> for Sonrs {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut s_status = None;
        let mut s_fi = None;
        let mut s_bid = None;
        let mut s_dt_server = None;
        let mut s_dt_acctup = None;
        let mut s_language = None;
        let mut s_user_id = None;
        let mut s_cookie = None;
        while let Some(contents) = tokens.next() {
            match contents {
                "STATUS" => {
                    s_status = Some(Status::parse(tokens)?);
                }
                "DTSERVER" => {
                    if let Some(dt_server) = tokens.next() {
                        s_dt_server = Some(parse_ofx_datetime(dt_server).map_err(|e| {
                            QFXParsingError::UnexpectedDateFormat(format!(
                                "Failed to parse DTSERVER date time: {}",
                                e
                            ))
                        })?);
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the DTSERVER token".to_string(),
                        ));
                    }
                }
                "DTACCTUP" => {
                    if let Some(dt_acctup) = tokens.next() {
                        s_dt_acctup = Some(dt_acctup.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the DTACCTUP token".to_string(),
                        ));
                    }
                }
                "LANGUAGE" => {
                    if let Some(language) = tokens.next() {
                        s_language = Some(language.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the LANGUAGE token".to_string(),
                        ));
                    }
                }
                "FI" => {
                    s_fi = Some(FinancialInstitution::parse(tokens)?);
                }
                "INTU.BID" => {
                    if let Some(bid) = tokens.next() {
                        s_bid = Some(bid.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the INTU.BID token".to_string(),
                        ));
                    }
                }
                "INTU.USERID" => {
                    if let Some(user_id) = tokens.next() {
                        s_user_id = Some(user_id.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the INTU.BID token".to_string(),
                        ));
                    }
                }
                "SESSCOOKIE" => {
                    if let Some(cookie) = tokens.next() {
                        s_cookie = Some(cookie.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the SESSCOOKIE token".to_string(),
                        ));
                    }
                }
                "/SONRS" => {
                    return Ok(Self {
                        status: s_status,
                        fi: s_fi.ok_or(QFXParsingError::MissingRequiredValue(
                            "Financial Institution (FI) is required in SONRS".to_string(),
                        ))?,
                        bid: s_bid,
                        dt_server: s_dt_server.ok_or(QFXParsingError::MissingRequiredValue(
                            "DTSERVER is required in SONRS".to_string(),
                        ))?,
                        dt_acctup: s_dt_acctup,
                        cookie: s_cookie,
                        language: s_language,
                        user_id: s_user_id,
                    });
                }
                _ => {
                    // Unknown token
                    // TODO: Need better error parsing
                    return Err(QFXParsingError::UnexpectedToken(format!(
                        "Found unexpected token {} in the SONRS type",
                        contents.to_string()
                    )));
                }
            }
        }
        return Err(QFXParsingError::UnexpectedEOF(
            "Found unexpected EOF. Was still expecting the '/SONRS' token".to_string(),
        ));
    }
}

impl<'a> Parseable<'a> for FinancialInstitution {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut fi = Self {
            org: None,
            fid: None,
        };
        while let Some(contents) = tokens.next() {
            match contents {
                "ORG" => {
                    if let Some(severity) = tokens.next() {
                        fi.org = Some(severity.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the ORG token".to_string(),
                        ));
                    }
                }
                "FID" => {
                    if let Some(fid) = tokens.next() {
                        fi.fid = Some(fid.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the FIT token".to_string(),
                        ));
                    }
                }
                "/FI" => {
                    return Ok(fi);
                }
                _ => {
                    return Err(QFXParsingError::UnexpectedToken(format!(
                        "Found unexpected token {} in the FI type",
                        contents.to_string()
                    )));
                }
            }
        }
        return Err(QFXParsingError::UnexpectedEOF(
            "Found unexpected EOF. Was still expecting the '/FI' token".to_string(),
        ));
    }
}
