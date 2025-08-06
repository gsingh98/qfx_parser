use crate::AvailableBalance;
use crate::BankTranList;
use crate::LedgerBal;
use crate::Parseable;
use crate::QFXParsingError;
use crate::Status;

#[derive(Clone)]
pub struct CCMsgSrsV1 {
    pub ccstmttrns: Ccstmttrnrs, // TODO: This should be a vector, there can be more than 1 of these, but there should be at least 1
}

#[derive(Clone)]
pub struct Ccstmttrnrs {
    pub trnuid: Option<String>,
    pub status: Option<Status>,
    pub ccstmtrs: Ccstmtrs, // TODO: Should not be a pub(crate) visibility
}

#[derive(Clone)]
pub struct Ccstmtrs {
    pub currency: Option<String>,
    pub ccacctfrom: Ccacctfrom,
    pub banktranslist: BankTranList,
    pub ledgerbal: Option<LedgerBal>,
    pub availbal: Option<AvailableBalance>,
}

#[derive(Clone)]
pub struct Ccacctfrom {
    pub acct_id: String,
}

impl<'a> Parseable<'a> for CCMsgSrsV1 {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut s_ccstmttrns = None;
        while let Some(contents) = tokens.next() {
            match contents {
                "CCSTMTTRNRS" => {
                    s_ccstmttrns = Some(Ccstmttrnrs::parse(tokens)?);
                }
                "/CREDITCARDMSGSRSV1" => {
                    return Ok(Self {
                        ccstmttrns: s_ccstmttrns.ok_or(QFXParsingError::MissingRequiredValue(
                            "CCSTMTTRNRS is a requied value in CREDITCARDMSGSRSV1".to_string(),
                        ))?,
                    });
                }
                _ => {
                    // Error case, unknown token seen
                    return Err(QFXParsingError::UnexpectedToken(format!(
                        "Found unexpected token {} in the CREDITCARDMSGSRSV1 type",
                        contents.to_string()
                    )));
                }
            }
        }
        return Err(QFXParsingError::UnexpectedEOF(
            "Found unexpected EOF. Was still expecting the '/CREDITCARDMSGSRSV1' token".to_string(),
        ));
    }
}

impl<'a> Parseable<'a> for Ccstmttrnrs {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut s_trnuid = None;
        let mut s_status = None;
        let mut s_ccstmtrs = None;

        while let Some(contents) = tokens.next() {
            match contents {
                "TRNUID" => {
                    if let Some(trnuid) = tokens.next() {
                        s_trnuid = Some(trnuid.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the TRNUID token".to_string(),
                        ));
                    }
                }
                "STATUS" => {
                    s_status = Some(Status::parse(tokens)?);
                }
                "CCSTMTRS" => s_ccstmtrs = Some(Ccstmtrs::parse(tokens)?),
                "/CCSTMTTRNRS" => {
                    return Ok(Self {
                        trnuid: s_trnuid,
                        status: s_status,
                        ccstmtrs: s_ccstmtrs.ok_or(QFXParsingError::MissingRequiredValue(
                            "CCSTMTRS section is required in CCSTMTTRNRS".to_string(),
                        ))?,
                    });
                }
                _ => {
                    // Error case, unknown token seen
                    return Err(QFXParsingError::UnexpectedToken(format!(
                        "Found unexpected token {} in the CCSTMTTRNRS type",
                        contents.to_string()
                    )));
                }
            }
        }
        return Err(QFXParsingError::UnexpectedEOF(
            "Found unexpected EOF. Was still expecting the '/CCSTMTTRNRS' token".to_string(),
        ));
    }
}

impl<'a> Parseable<'a> for Ccstmtrs {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut s_currency = None;
        let mut s_ccacctfrom = None;
        let mut s_banktranslist = None;
        let mut s_ledgerbal = None;
        let mut s_availbal = None;
        while let Some(contents) = tokens.next() {
            match contents {
                "CURDEF" => {
                    if let Some(curdef) = tokens.next() {
                        s_currency = Some(curdef.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the CURDEF token".to_string(),
                        ));
                    }
                }
                "CCACCTFROM" => {
                    s_ccacctfrom = Some(Ccacctfrom::parse(tokens)?);
                }
                "BANKTRANLIST" => {
                    s_banktranslist = Some(BankTranList::parse(tokens)?);
                }
                "LEDGERBAL" => {
                    s_ledgerbal = Some(LedgerBal::parse(tokens)?);
                }
                "AVAILBAL" => {
                    s_availbal = Some(AvailableBalance::parse(tokens)?);
                }
                "/CCSTMTRS" => {
                    return Ok(Self {
                        currency: s_currency,
                        availbal: s_availbal,
                        banktranslist: s_banktranslist.ok_or(
                            QFXParsingError::MissingRequiredValue(
                                "BANKTRANSLIST is a required value in CCSTMTRS".to_string(),
                            ),
                        )?,
                        ccacctfrom: s_ccacctfrom.ok_or(QFXParsingError::MissingRequiredValue(
                            "CCACCTFROM is a required value in CCSTMTRS".to_string(),
                        ))?,
                        ledgerbal: s_ledgerbal,
                    });
                }
                _ => {
                    // Error case, unknown token seen
                    return Err(QFXParsingError::UnexpectedToken(format!(
                        "Found unexpected token {} in the CCSTMTRS type",
                        contents.to_string()
                    )));
                }
            }
        }
        return Err(QFXParsingError::UnexpectedEOF(
            "Found unexpected EOF. Was still expecting the '/CCSTMTRS'".to_string(),
        ));
    }
}

impl<'a> Parseable<'a> for Ccacctfrom {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut s_acct_id = None;
        while let Some(contents) = tokens.next() {
            match contents {
                "ACCTID" => {
                    if let Some(curdef) = tokens.next() {
                        s_acct_id = Some(curdef.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the ACCTID token".to_string(),
                        ));
                    }
                }
                "/CCACCTFROM" => {
                    return Ok(Self {
                        acct_id: s_acct_id.ok_or(QFXParsingError::MissingRequiredValue(
                            "ACCTID is a required value in CCSTMTRS".to_string(),
                        ))?,
                    });
                }
                _ => {
                    // Error case, unknown token seen
                    return Err(QFXParsingError::UnexpectedToken(format!(
                        "Found unexpected token {} in the CCACCTFROM type",
                        contents.to_string()
                    )));
                }
            }
        }
        return Err(QFXParsingError::UnexpectedEOF(
            "Found unexpected EOF. Was still expecting the '/CCACCTFROM' token".to_string(),
        ));
    }
}

#[cfg(test)]
mod test_ccacctfrom {
    use super::*;
    use crate::tokenize;

    #[test]
    fn test_ccacctfrom_valid_input() {
        let input = "\
            <ACCTID> 1234567890\
            </CCACCTFROM>";
        let mut tokens = tokenize(input);

        let result = Ccacctfrom::parse(&mut tokens);
        assert!(result.is_ok());
        let ccacctfrom = result.unwrap();
        assert_eq!(ccacctfrom.acct_id, "1234567890");
    }

    #[test]
    fn test_ccacctfrom_missing_acctid() {
        let input = "</CCACCTFROM>";
        let mut tokens = tokenize(input);

        let result = Ccacctfrom::parse(&mut tokens);
        assert!(matches!(
            result,
            Err(QFXParsingError::MissingRequiredValue(msg)) if msg.contains("ACCTID is a required value")
        ));
    }

    #[test]
    fn test_ccacctfrom_unknown_tag() {
        let input = "<UNKNOWNTAG>value</CCACCTFROM>";
        let mut tokens = tokenize(input);

        let result = Ccacctfrom::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedToken(msg)) if msg.contains("UNKNOWNTAG") && msg.contains("CCACCTFROM")
        ));
    }
}

#[cfg(test)]
mod test_ccstmttrnrs {
    use super::*;
    use crate::tokenize;

    #[test]
    fn test_ccstmttrnrs_valid_minimal() {
        let input = "\
            <CCSTMTRS>\
                <CCACCTFROM>\
                    <ACCTID>1234567890\
                </CCACCTFROM>\
                <BANKTRANLIST>\
                    <DTSTART>20250715080000\
                    <DTEND>20250716090000\
                </BANKTRANLIST>\
            </CCSTMTRS>\
            </CCSTMTTRNRS>";
        let mut tokens = tokenize(input);

        let result = Ccstmttrnrs::parse(&mut tokens);
        assert!(result.is_ok());
        let ccstmttrnrs = result.unwrap();
        assert!(ccstmttrnrs.trnuid.is_none());
        assert!(ccstmttrnrs.status.is_none());
        assert_eq!(ccstmttrnrs.ccstmtrs.ccacctfrom.acct_id, "1234567890");
    }

    #[test]
    fn test_ccstmttrnrs_valid_with_trnuid() {
        let input = "\
            <TRNUID>12345-67890\
            <CCSTMTRS>\
                <CCACCTFROM>\
                    <ACCTID>1234567890\
                </CCACCTFROM>\
                <BANKTRANLIST>\
                    <DTSTART>20250715080000\
                    <DTEND>20250716090000\
                </BANKTRANLIST>\
            </CCSTMTRS>\
            </CCSTMTTRNRS>";
        let mut tokens = tokenize(input);

        let result = Ccstmttrnrs::parse(&mut tokens);
        assert!(result.is_ok());
        let ccstmttrnrs = result.unwrap();
        assert_eq!(ccstmttrnrs.trnuid, Some("12345-67890".to_string()));
        assert!(ccstmttrnrs.status.is_none());
        assert_eq!(ccstmttrnrs.ccstmtrs.ccacctfrom.acct_id, "1234567890");
    }

    #[test]
    fn test_ccstmttrnrs_valid_with_status() {
        let input = "\
            <STATUS>\
                <CODE>0\
                <SEVERITY>INFO\
            </STATUS>\
            <CCSTMTRS>\
                <CCACCTFROM>\
                    <ACCTID>1234567890\
                </CCACCTFROM>\
                <BANKTRANLIST>\
                    <DTSTART>20250715080000\
                    <DTEND>20250716090000\
                </BANKTRANLIST>\
            </CCSTMTRS>\
            </CCSTMTTRNRS>";
        let mut tokens = tokenize(input);

        let result = Ccstmttrnrs::parse(&mut tokens);
        assert!(result.is_ok());
        let ccstmttrnrs = result.unwrap();
        assert!(ccstmttrnrs.trnuid.is_none());
        assert!(ccstmttrnrs.status.is_some());
        assert_eq!(ccstmttrnrs.ccstmtrs.ccacctfrom.acct_id, "1234567890");
    }

    #[test]
    fn test_ccstmttrnrs_missing_ccstmtrs() {
        let input = "\
            <TRNUID>12345-67890\
            </CCSTMTTRNRS>";
        let mut tokens = tokenize(input);

        let result = Ccstmttrnrs::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::MissingRequiredValue(msg)) if msg.contains("CCSTMTRS section is required")
        ));
    }

    #[test]
    fn test_ccstmttrnrs_unknown_tag() {
        let input = "\
            <UNKNOWNTAG>value\
            <CCSTMTRS>\
                <CCACCTFROM>\
                    <ACCTID>1234567890\
                </CCACCTFROM>\
                <BANKTRANLIST>\
                    <DTSTART>20250715080000\
                    <DTEND>20250716090000\
                </BANKTRANLIST>\
            </CCSTMTRS>\
            </CCSTMTTRNRS>";
        let mut tokens = tokenize(input);

        let result = Ccstmttrnrs::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedToken(msg)) if msg.contains("UNKNOWNTAG") && msg.contains("CCSTMTTRNRS")
        ));
    }

    #[test]
    fn test_ccstmttrnrs_unexpected_eof_after_trnuid() {
        let input = "<TRNUID>";
        let mut tokens = tokenize(input);

        let result = Ccstmttrnrs::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedEOF(msg)) if msg.contains("Expected token following the TRNUID token")
        ));
    }

    #[test]
    fn test_ccstmttrnrs_unexpected_eof_missing_closing_tag() {
        let input = "\
            <TRNUID>12345-67890\
            <CCSTMTRS>\
                <CCACCTFROM>\
                    <ACCTID>1234567890\
                </CCACCTFROM>\
                <BANKTRANLIST>\
                    <DTSTART>20250715080000\
                    <DTEND>20250716090000\
                </BANKTRANLIST>\
            </CCSTMTRS>";
        let mut tokens = tokenize(input);

        let result = Ccstmttrnrs::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedEOF(msg)) if msg.contains("Was still expecting the '/CCSTMTTRNRS' token")
        ));
    }
}

#[cfg(test)]
mod test_ccstmtrs {
    use super::*;
    use crate::tokenize;

    #[test]
    fn test_ccstmtrs_valid_minimal() {
        let input = "\
            <CCACCTFROM>\
                <ACCTID>1234567890\
            </CCACCTFROM>\
            <BANKTRANLIST>\
                <DTSTART>20250715080000\
                <DTEND>20250716090000\
            </BANKTRANLIST>\
            </CCSTMTRS>";
        let mut tokens = tokenize(input);

        let result = Ccstmtrs::parse(&mut tokens);
        assert!(result.is_ok());
        let ccstmtrs = result.unwrap();
        assert!(ccstmtrs.currency.is_none());
        assert_eq!(ccstmtrs.ccacctfrom.acct_id, "1234567890");
        assert!(ccstmtrs.ledgerbal.is_none());
        assert!(ccstmtrs.availbal.is_none());
    }

    #[test]
    fn test_ccstmtrs_valid_with_currency() {
        let input = "\
            <CURDEF>USD\
            <CCACCTFROM>\
                <ACCTID>1234567890\
            </CCACCTFROM>\
            <BANKTRANLIST>\
                <DTSTART>20250715080000\
                <DTEND>20250716090000\
            </BANKTRANLIST>\
            </CCSTMTRS>";
        let mut tokens = tokenize(input);

        let result = Ccstmtrs::parse(&mut tokens);
        assert!(result.is_ok());
        let ccstmtrs = result.unwrap();
        assert_eq!(ccstmtrs.currency, Some("USD".to_string()));
        assert_eq!(ccstmtrs.ccacctfrom.acct_id, "1234567890");
    }

    #[test]
    fn test_ccstmtrs_valid_with_ledgerbal() {
        let input = "\
            <CCACCTFROM>\
                <ACCTID>1234567890\
            </CCACCTFROM>\
            <BANKTRANLIST>\
                <DTSTART>20250715080000\
                <DTEND>20250716090000\
            </BANKTRANLIST>\
            <LEDGERBAL>\
                <BALAMT>1000.00\
                <DTASOF>20250715080000\
            </LEDGERBAL>\
            </CCSTMTRS>";
        let mut tokens = tokenize(input);

        let result = Ccstmtrs::parse(&mut tokens);
        assert!(result.is_ok());
        let ccstmtrs = result.unwrap();
        assert!(ccstmtrs.ledgerbal.is_some());
        assert_eq!(ccstmtrs.ccacctfrom.acct_id, "1234567890");
    }

    #[test]
    fn test_ccstmtrs_valid_with_availbal() {
        let input = "\
            <CCACCTFROM>\
                <ACCTID>1234567890\
            </CCACCTFROM>\
            <BANKTRANLIST>\
                <DTSTART>20250715080000\
                <DTEND>20250716090000\
            </BANKTRANLIST>\
            <AVAILBAL>\
                <BALAMT>500.00\
                <DTASOF>20250715080000\
            </AVAILBAL>\
            </CCSTMTRS>";
        let mut tokens = tokenize(input);

        let result = Ccstmtrs::parse(&mut tokens);
        assert!(result.is_ok());
        let ccstmtrs = result.unwrap();
        assert!(ccstmtrs.availbal.is_some());
        assert_eq!(ccstmtrs.ccacctfrom.acct_id, "1234567890");
    }

    #[test]
    fn test_ccstmtrs_missing_ccacctfrom() {
        let input = "\
            <BANKTRANLIST>\
                <DTSTART>20250715080000\
                <DTEND>20250716090000\
            </BANKTRANLIST>\
            </CCSTMTRS>";
        let mut tokens = tokenize(input);

        let result = Ccstmtrs::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::MissingRequiredValue(msg)) if msg.contains("CCACCTFROM is a required value")
        ));
    }

    #[test]
    fn test_ccstmtrs_missing_banktranlist() {
        let input = "\
            <CCACCTFROM>\
                <ACCTID>1234567890\
            </CCACCTFROM>\
            </CCSTMTRS>";
        let mut tokens = tokenize(input);

        let result = Ccstmtrs::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::MissingRequiredValue(msg)) if msg.contains("BANKTRANSLIST is a required value")
        ));
    }

    #[test]
    fn test_ccstmtrs_unknown_tag() {
        let input = "\
            <UNKNOWNTAG>value\
            <CCACCTFROM>\
                <ACCTID>1234567890\
            </CCACCTFROM>\
            <BANKTRANLIST>\
                <DTSTART>20250715080000\
                <DTEND>20250716090000\
            </BANKTRANLIST>\
            </CCSTMTRS>";
        let mut tokens = tokenize(input);

        let result = Ccstmtrs::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedToken(msg)) if msg.contains("UNKNOWNTAG") && msg.contains("CCSTMTRS")
        ));
    }

    #[test]
    fn test_ccstmtrs_unexpected_eof_after_curdef() {
        let input = "<CURDEF>";
        let mut tokens = tokenize(input);

        let result = Ccstmtrs::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedEOF(msg)) if msg.contains("Expected token following the CURDEF token")
        ));
    }

    #[test]
    fn test_ccstmtrs_unexpected_eof_missing_closing_tag() {
        let input = "\
            <CURDEF>USD\
            <CCACCTFROM>\
                <ACCTID>1234567890\
            </CCACCTFROM>\
            <BANKTRANLIST>\
                <DTSTART>20250715080000\
                <DTEND>20250716090000\
            </BANKTRANLIST>";
        let mut tokens = tokenize(input);

        let result = Ccstmtrs::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedEOF(msg)) if msg.contains("Was still expecting the '/CCSTMTRS'")
        ));
    }
}

#[cfg(test)]
mod test_ccmsgsrsv1 {
    use super::*;
    use crate::tokenize;

    #[test]
    fn test_ccmsgsrsv1_valid_minimal() {
        let input = "\
            <CCSTMTTRNRS>\
                <CCSTMTRS>\
                    <CCACCTFROM>\
                        <ACCTID>1234567890\
                    </CCACCTFROM>\
                    <BANKTRANLIST>\
                        <DTSTART>20250715080000\
                        <DTEND>20250716090000\
                    </BANKTRANLIST>\
                </CCSTMTRS>\
            </CCSTMTTRNRS>\
            </CREDITCARDMSGSRSV1>";
        let mut tokens = tokenize(input);

        let result = CCMsgSrsV1::parse(&mut tokens);
        assert!(result.is_ok());
        let ccmsgsrsv1 = result.unwrap();
        assert!(ccmsgsrsv1.ccstmttrns.trnuid.is_none());
        assert!(ccmsgsrsv1.ccstmttrns.status.is_none());
        assert_eq!(
            ccmsgsrsv1.ccstmttrns.ccstmtrs.ccacctfrom.acct_id,
            "1234567890"
        );
    }

    #[test]
    fn test_ccmsgsrsv1_missing_ccstmttrnrs() {
        let input = "</CREDITCARDMSGSRSV1>";
        let mut tokens = tokenize(input);

        let result = CCMsgSrsV1::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::MissingRequiredValue(msg)) if msg.contains("CCSTMTTRNRS is a requied value")
        ));
    }

    #[test]
    fn test_ccmsgsrsv1_unknown_tag() {
        let input = "<UNKNOWNTAG>";
        let mut tokens = tokenize(input);

        let result = CCMsgSrsV1::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedToken(msg)) if msg.contains("UNKNOWNTAG") && msg.contains("CREDITCARDMSGSRSV1")
        ));
    }

    #[test]
    fn test_ccmsgsrsv1_unexpected_eof_missing_closing_tag() {
        let input = "\
            <CCSTMTTRNRS>\
                <CCSTMTRS>\
                    <CCACCTFROM>\
                        <ACCTID>1234567890\
                    </CCACCTFROM>\
                    <BANKTRANLIST>\
                        <DTSTART>20250715080000\
                        <DTEND>20250716090000\
                    </BANKTRANLIST>\
                </CCSTMTRS>\
            </CCSTMTTRNRS>";
        let mut tokens = tokenize(input);

        let result = CCMsgSrsV1::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedEOF(msg)) if msg.contains("Was still expecting the '/CREDITCARDMSGSRSV1' token")
        ));
    }
}
