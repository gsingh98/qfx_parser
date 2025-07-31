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
    pub sonrs: Sonrs,
}

#[derive(Clone)]
pub struct Sonrs {
    pub status: Option<Status>,
    pub fi: FinancialInstitution,
    pub bid: Option<String>,
    pub dt_server: DateTime<Utc>, // TODO: Should be using the DateTime parser for this part
    pub dt_acctup: Option<String>,
    pub language: Option<String>,
    pub cookie: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Clone)]
pub struct FinancialInstitution {
    pub org: String,
    pub fid: String, // TODO: Needs to be numeric
}

impl<'a> Parseable<'a> for SignOnMsgSrsV1 {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut s_sonrs = None;
        while let Some(contents) = tokens.next() {
            match contents {
                "SONRS" => {
                    s_sonrs = Some(Sonrs::parse(tokens)?);
                }
                "/SIGNONMSGSRSV1" => {
                    return Ok(Self {
                        sonrs: s_sonrs.ok_or(QFXParsingError::MissingRequiredValue(
                            "Missing value SONRS in SIGNONMSGSRSV1".to_string(),
                        ))?,
                    });
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
        let mut s_org = None;
        let mut s_fid = None;
        while let Some(contents) = tokens.next() {
            match contents {
                "ORG" => {
                    if let Some(severity) = tokens.next() {
                        s_org = Some(severity.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the ORG token".to_string(),
                        ));
                    }
                }
                "FID" => {
                    if let Some(fid) = tokens.next() {
                        s_fid = Some(fid.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the FIT token".to_string(),
                        ));
                    }
                }
                "/FI" => {
                    return Ok(Self {
                        org: s_org.ok_or(QFXParsingError::MissingRequiredValue(
                            "Missing ORG in FI".to_string(),
                        ))?,
                        fid: s_fid.ok_or(QFXParsingError::MissingRequiredValue(
                            "Missing FID in FI".to_string(),
                        ))?,
                    });
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
