// TODO: Require doc comments
// TODO: Require clippy formatting
use crate::Parseable;
use crate::QFXParsingError;
use crate::Status;

#[derive(Clone)]
pub struct SignOnMsgSrsV1 {
    sonrs: Option<Sonrs>,
}

#[derive(Clone)]
pub struct Sonrs {
    status: Option<Status>,
    fi: Option<FinancialInstitution>,
    bid: Option<String>,
    dt_server: Option<String>,
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
        let mut sonrs = Self {
            status: None,
            fi: None,
            bid: None,
            dt_server: None,
            language: None,
            user_id: None,
            cookie: None,
        };
        while let Some(contents) = tokens.next() {
            match contents {
                "STATUS" => {
                    sonrs.status = Some(Status::parse(tokens)?);
                }
                "DTSERVER" => {
                    if let Some(dt_server) = tokens.next() {
                        sonrs.dt_server = Some(dt_server.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the DTSERVER token".to_string(),
                        ));
                    }
                }
                "LANGUAGE" => {
                    if let Some(language) = tokens.next() {
                        sonrs.language = Some(language.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the LANGUAGE token".to_string(),
                        ));
                    }
                }
                "FI" => {
                    sonrs.fi = Some(FinancialInstitution::parse(tokens)?);
                }
                "INTU.BID" => {
                    if let Some(bid) = tokens.next() {
                        sonrs.bid = Some(bid.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the INTU.BID token".to_string(),
                        ));
                    }
                }
                "INTU.USERID" => {
                    if let Some(user_id) = tokens.next() {
                        sonrs.user_id = Some(user_id.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the INTU.BID token".to_string(),
                        ));
                    }
                }
                "SESSCOOKIE" => {
                    if let Some(cookie) = tokens.next() {
                        sonrs.cookie = Some(cookie.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the INTU.BID token".to_string(),
                        ));
                    }
                }
                "/SONRS" => {
                    return Ok(sonrs);
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
