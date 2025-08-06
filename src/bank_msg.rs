use crate::AvailableBalance;
use crate::BankTranList;
use crate::LedgerBal;
use crate::Parseable;
use crate::QFXParsingError;
use crate::Status;

#[derive(Clone)]
pub struct BankMsgSrsV1 {
    pub stmttrns: Stmttrnrs, // TODO: This should be a vector, there can be more than 1 of these, but there should be at least 1
}

#[derive(Clone)]
pub struct Stmttrnrs {
    pub trnuid: Option<String>,
    pub status: Option<Status>,
    pub stmtrs: Stmtrs,
}

#[derive(Clone)]
pub struct Stmtrs {
    pub currency: Option<String>,
    pub bankacctfrom: Bankacctfrom,
    pub banktranslist: BankTranList,
    pub ledgerbal: Option<LedgerBal>,
    pub availbal: Option<AvailableBalance>,
}

#[derive(Clone)]
pub struct Bankacctfrom {
    pub acct_id: String,
    pub acct_type: String,
    pub bank_id: Option<String>,
}

impl<'a> Parseable<'a> for BankMsgSrsV1 {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut s_stmttrns = None;
        while let Some(contents) = tokens.next() {
            match contents {
                "STMTTRNRS" => {
                    s_stmttrns = Some(Stmttrnrs::parse(tokens)?);
                }
                "/BANKMSGSRSV1" => {
                    return Ok(Self {
                        stmttrns: s_stmttrns.ok_or(QFXParsingError::MissingRequiredValue(
                            "STMTTRNRS is a required value in BANKMSGSRSV1".to_string(),
                        ))?,
                    });
                }
                _ => {
                    // Error case, unknown token seen
                    return Err(QFXParsingError::UnexpectedToken(format!(
                        "Found unexpected token {} in the BANKMSGSRSV1 type",
                        contents.to_string()
                    )));
                }
            }
        }
        return Err(QFXParsingError::UnexpectedEOF(
            "Found unexpected EOF. Was still expecting the '/BANKMSGSRSV1' token".to_string(),
        ));
    }
}

impl<'a> Parseable<'a> for Stmttrnrs {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut s_trnuid = None;
        let mut s_status = None;
        let mut s_stmtrs = None;

        while let Some(contents) = tokens.next() {
            match contents {
                "TRNUID" => {
                    if let Some(trnuid) = tokens.next() {
                        s_trnuid = Some(trnuid.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected a token following the TRNUID token".to_string(),
                        ));
                    }
                }
                "STATUS" => {
                    s_status = Some(Status::parse(tokens)?);
                }
                "STMTRS" => s_stmtrs = Some(Stmtrs::parse(tokens)?),
                "/STMTTRNRS" => {
                    return Ok(Self {
                        trnuid: s_trnuid,
                        status: s_status,
                        stmtrs: s_stmtrs.ok_or(QFXParsingError::MissingRequiredValue(
                            "STMTRS section is required in STMTTRNRS".to_string(),
                        ))?,
                    });
                }
                _ => {
                    // Error case, unknown token seen
                    return Err(QFXParsingError::UnexpectedToken(format!(
                        "Found unexpected token {} in the STMTTRNRS type",
                        contents.to_string()
                    )));
                }
            }
        }
        return Err(QFXParsingError::UnexpectedEOF(
            "Found unexpected EOF. Was still expecting the '/STMTTRNRS' token".to_string(),
        ));
    }
}

impl<'a> Parseable<'a> for Bankacctfrom {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut s_acct_id = None;
        let mut s_acct_type = None;
        let mut s_bank_id = None;
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
                "ACCTTYPE" => {
                    if let Some(acct_type) = tokens.next() {
                        s_acct_type = Some(acct_type.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the ACCTTYPE token".to_string(),
                        ));
                    }
                }
                "BANKID" => {
                    if let Some(bank_id) = tokens.next() {
                        s_bank_id = Some(bank_id.to_string());
                    } else {
                        return Err(QFXParsingError::UnexpectedEOF(
                            "Expected token following the ACCTTYPE token".to_string(),
                        ));
                    }
                }
                "/BANKACCTFROM" => {
                    return Ok(Self {
                        acct_id: s_acct_id.ok_or(QFXParsingError::MissingRequiredValue(
                            "ACCTID is a required value in BANKACCTFROM".to_string(),
                        ))?,
                        acct_type: s_acct_type.ok_or(QFXParsingError::MissingRequiredValue(
                            "ACCTTYPE is a required value in BANKACCTFROM".to_string(),
                        ))?,
                        bank_id: s_bank_id,
                    });
                }
                _ => {
                    // Error case, unknown token seen
                    return Err(QFXParsingError::UnexpectedToken(format!(
                        "Found unexpected token {} in the BANKACCTFROM type",
                        contents.to_string()
                    )));
                }
            }
        }
        return Err(QFXParsingError::UnexpectedEOF(
            "Found unexpected EOF. Was still expecting the '/BANKACCTFROM' token".to_string(),
        ));
    }
}

impl<'a> Parseable<'a> for Stmtrs {
    fn parse(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Self, QFXParsingError> {
        let mut s_currency = None;
        let mut s_bankacctfrom = None;
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
                "BANKACCTFROM" => {
                    s_bankacctfrom = Some(Bankacctfrom::parse(tokens)?);
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
                "/STMTRS" => {
                    return Ok(Self {
                        currency: s_currency,
                        availbal: s_availbal,
                        banktranslist: s_banktranslist.ok_or(
                            QFXParsingError::MissingRequiredValue(
                                "BANKTRANSLIST is a required value in STMTRS".to_string(),
                            ),
                        )?,
                        bankacctfrom: s_bankacctfrom.ok_or(
                            QFXParsingError::MissingRequiredValue(
                                "BANKACCTFROM is a required value in STMTRS".to_string(),
                            ),
                        )?,
                        ledgerbal: s_ledgerbal,
                    });
                }
                _ => {
                    // Error case, unknown token seen
                    return Err(QFXParsingError::UnexpectedToken(format!(
                        "Found unexpected token {} in the STMTRS type",
                        contents.to_string()
                    )));
                }
            }
        }
        return Err(QFXParsingError::UnexpectedEOF(
            "Found unexpected EOF. Was still expecting the '/STMTRS'".to_string(),
        ));
    }
}

#[cfg(test)]
mod test_bankacctfrom {
    use super::*;
    use crate::tokenize;

    #[test]
    fn test_bankacctfrom_valid_minimal() {
        let input = "\
            <ACCTID>1234567890\
            <ACCTTYPE>CHECKING\
            </BANKACCTFROM>";
        let mut tokens = tokenize(input);

        let result = Bankacctfrom::parse(&mut tokens);
        assert!(result.is_ok());
        let bankacctfrom = result.unwrap();
        assert_eq!(bankacctfrom.acct_id, "1234567890");
        assert_eq!(bankacctfrom.acct_type, "CHECKING");
        assert!(bankacctfrom.bank_id.is_none());
    }

    #[test]
    fn test_bankacctfrom_valid_with_bank_id() {
        let input = "\
            <ACCTID>1234567890\
            <ACCTTYPE>SAVINGS\
            <BANKID>123456789\
            </BANKACCTFROM>";
        let mut tokens = tokenize(input);

        let result = Bankacctfrom::parse(&mut tokens);
        assert!(result.is_ok());
        let bankacctfrom = result.unwrap();
        assert_eq!(bankacctfrom.acct_id, "1234567890");
        assert_eq!(bankacctfrom.acct_type, "SAVINGS");
        assert_eq!(bankacctfrom.bank_id, Some("123456789".to_string()));
    }

    #[test]
    fn test_bankacctfrom_missing_acctid() {
        let input = "\
            <ACCTTYPE>CHECKING\
            </BANKACCTFROM>";
        let mut tokens = tokenize(input);

        let result = Bankacctfrom::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::MissingRequiredValue(msg)) if msg.contains("ACCTID is a required value")
        ));
    }

    #[test]
    fn test_bankacctfrom_missing_accttype() {
        let input = "\
            <ACCTID>1234567890\
            </BANKACCTFROM>";
        let mut tokens = tokenize(input);

        let result = Bankacctfrom::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::MissingRequiredValue(msg)) if msg.contains("ACCTTYPE is a required value")
        ));
    }

    #[test]
    fn test_bankacctfrom_unknown_tag() {
        let input = "\
            <UNKNOWNTAG>value\
            <ACCTID>1234567890\
            <ACCTTYPE>CHECKING\
            </BANKACCTFROM>";
        let mut tokens = tokenize(input);

        let result = Bankacctfrom::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedToken(msg)) if msg.contains("UNKNOWNTAG") && msg.contains("BANKACCTFROM")
        ));
    }

    #[test]
    fn test_bankacctfrom_unexpected_eof_after_acctid() {
        let input = "<ACCTID>";
        let mut tokens = tokenize(input);

        let result = Bankacctfrom::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedEOF(msg)) if msg.contains("Expected token following the ACCTID token")
        ));
    }

    #[test]
    fn test_bankacctfrom_unexpected_eof_after_accttype() {
        let input = "\
            <ACCTID>1234567890\
            <ACCTTYPE>";
        let mut tokens = tokenize(input);

        let result = Bankacctfrom::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedEOF(msg)) if msg.contains("Expected token following the ACCTTYPE token")
        ));
    }

    #[test]
    fn test_bankacctfrom_unexpected_eof_after_bankid() {
        let input = "\
            <ACCTID>1234567890\
            <ACCTTYPE>CHECKING\
            <BANKID>";
        let mut tokens = tokenize(input);

        let result = Bankacctfrom::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedEOF(msg)) if msg.contains("Expected token following the ACCTTYPE token")
        ));
    }

    #[test]
    fn test_bankacctfrom_unexpected_eof_missing_closing_tag() {
        let input = "\
            <ACCTID>1234567890\
            <ACCTTYPE>CHECKING\
            <BANKID>123456789";
        let mut tokens = tokenize(input);

        let result = Bankacctfrom::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedEOF(msg)) if msg.contains("Was still expecting the '/BANKACCTFROM' token")
        ));
    }
}

#[cfg(test)]
mod test_stmttrnrs {
    use super::*;
    use crate::tokenize;

    #[test]
    fn test_stmttrnrs_valid_minimal() {
        let input = "\
            <STMTRS>\
                <BANKACCTFROM>\
                    <ACCTID>1234567890\
                    <ACCTTYPE>CHECKING\
                </BANKACCTFROM>\
                <BANKTRANLIST>\
                    <DTSTART>20250715080000\
                    <DTEND>20250716090000\
                </BANKTRANLIST>\
            </STMTRS>\
            </STMTTRNRS>";
        let mut tokens = tokenize(input);

        let result = Stmttrnrs::parse(&mut tokens);
        assert!(result.is_ok());
        let stmttrnrs = result.unwrap();
        assert!(stmttrnrs.trnuid.is_none());
        assert!(stmttrnrs.status.is_none());
        assert_eq!(stmttrnrs.stmtrs.bankacctfrom.acct_id, "1234567890");
        assert_eq!(stmttrnrs.stmtrs.bankacctfrom.acct_type, "CHECKING");
    }

    #[test]
    fn test_stmttrnrs_valid_with_trnuid() {
        let input = "\
            <TRNUID>12345-67890\
            <STMTRS>\
                <BANKACCTFROM>\
                    <ACCTID>1234567890\
                    <ACCTTYPE>SAVINGS\
                </BANKACCTFROM>\
                <BANKTRANLIST>\
                    <DTSTART>20250715080000\
                    <DTEND>20250716090000\
                </BANKTRANLIST>\
            </STMTRS>\
            </STMTTRNRS>";
        let mut tokens = tokenize(input);

        let result = Stmttrnrs::parse(&mut tokens);
        assert!(result.is_ok());
        let stmttrnrs = result.unwrap();
        assert_eq!(stmttrnrs.trnuid, Some("12345-67890".to_string()));
        assert!(stmttrnrs.status.is_none());
        assert_eq!(stmttrnrs.stmtrs.bankacctfrom.acct_id, "1234567890");
        assert_eq!(stmttrnrs.stmtrs.bankacctfrom.acct_type, "SAVINGS");
    }

    #[test]
    fn test_stmttrnrs_valid_with_status() {
        let input = "\
            <STATUS>\
                <CODE>0\
                <SEVERITY>INFO\
            </STATUS>\
            <STMTRS>\
                <BANKACCTFROM>\
                    <ACCTID>1234567890\
                    <ACCTTYPE>CHECKING\
                </BANKACCTFROM>\
                <BANKTRANLIST>\
                    <DTSTART>20250715080000\
                    <DTEND>20250716090000\
                </BANKTRANLIST>\
            </STMTRS>\
            </STMTTRNRS>";
        let mut tokens = tokenize(input);

        let result = Stmttrnrs::parse(&mut tokens);
        assert!(result.is_ok());
        let stmttrnrs = result.unwrap();
        assert!(stmttrnrs.trnuid.is_none());
        assert!(stmttrnrs.status.is_some());
        assert_eq!(stmttrnrs.stmtrs.bankacctfrom.acct_id, "1234567890");
        assert_eq!(stmttrnrs.stmtrs.bankacctfrom.acct_type, "CHECKING");
    }

    #[test]
    fn test_stmttrnrs_missing_stmtrs() {
        let input = "\
            <TRNUID>12345-67890\
            </STMTTRNRS>";
        let mut tokens = tokenize(input);

        let result = Stmttrnrs::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::MissingRequiredValue(msg)) if msg.contains("STMTRS section is required")
        ));
    }

    #[test]
    fn test_stmttrnrs_unknown_tag() {
        let input = "\
            <UNKNOWNTAG>value\
            <STMTRS>\
                <BANKACCTFROM>\
                    <ACCTID>1234567890\
                    <ACCTTYPE>CHECKING\
                </BANKACCTFROM>\
                <BANKTRANLIST>\
                    <DTSTART>20250715080000\
                    <DTEND>20250716090000\
                </BANKTRANLIST>\
            </STMTRS>\
            </STMTTRNRS>";
        let mut tokens = tokenize(input);

        let result = Stmttrnrs::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedToken(msg)) if msg.contains("UNKNOWNTAG") && msg.contains("STMTTRNRS")
        ));
    }

    #[test]
    fn test_stmttrnrs_unexpected_eof_after_trnuid() {
        let input = "<TRNUID>";
        let mut tokens = tokenize(input);

        let result = Stmttrnrs::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedEOF(msg)) if msg.contains("Expected a token following the TRNUID token")
        ));
    }

    #[test]
    fn test_stmttrnrs_unexpected_eof_missing_closing_tag() {
        let input = "\
            <TRNUID>12345-67890\
            <STMTRS>\
                <BANKACCTFROM>\
                    <ACCTID>1234567890\
                    <ACCTTYPE>CHECKING\
                </BANKACCTFROM>\
                <BANKTRANLIST>\
                    <DTSTART>20250715080000\
                    <DTEND>20250716090000\
                </BANKTRANLIST>\
            </STMTRS>";
        let mut tokens = tokenize(input);

        let result = Stmttrnrs::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedEOF(msg)) if msg.contains("Was still expecting the '/STMTTRNRS' token")
        ));
    }
}

#[cfg(test)]
mod test_stmtrs {
    use super::*;
    use crate::tokenize;

    #[test]
    fn test_stmtrs_valid_minimal() {
        let input = "\
            <BANKACCTFROM>\
                <ACCTID>1234567890\
                <ACCTTYPE>CHECKING\
            </BANKACCTFROM>\
            <BANKTRANLIST>\
                <DTSTART>20250715080000\
                <DTEND>20250716090000\
            </BANKTRANLIST>\
            </STMTRS>";
        let mut tokens = tokenize(input);

        let result = Stmtrs::parse(&mut tokens);
        assert!(result.is_ok());
        let stmtrs = result.unwrap();
        assert!(stmtrs.currency.is_none());
        assert_eq!(stmtrs.bankacctfrom.acct_id, "1234567890");
        assert_eq!(stmtrs.bankacctfrom.acct_type, "CHECKING");
        assert!(stmtrs.ledgerbal.is_none());
        assert!(stmtrs.availbal.is_none());
    }

    #[test]
    fn test_stmtrs_valid_with_currency() {
        let input = "\
            <CURDEF>USD\
            <BANKACCTFROM>\
                <ACCTID>1234567890\
                <ACCTTYPE>SAVINGS\
            </BANKACCTFROM>\
            <BANKTRANLIST>\
                <DTSTART>20250715080000\
                <DTEND>20250716090000\
            </BANKTRANLIST>\
            </STMTRS>";
        let mut tokens = tokenize(input);

        let result = Stmtrs::parse(&mut tokens);
        assert!(result.is_ok());
        let stmtrs = result.unwrap();
        assert_eq!(stmtrs.currency, Some("USD".to_string()));
        assert_eq!(stmtrs.bankacctfrom.acct_id, "1234567890");
        assert_eq!(stmtrs.bankacctfrom.acct_type, "SAVINGS");
    }

    #[test]
    fn test_stmtrs_valid_with_ledgerbal() {
        let input = "\
            <BANKACCTFROM>\
                <ACCTID>1234567890\
                <ACCTTYPE>CHECKING\
            </BANKACCTFROM>\
            <BANKTRANLIST>\
                <DTSTART>20250715080000\
                <DTEND>20250716090000\
            </BANKTRANLIST>\
            <LEDGERBAL>\
                <BALAMT>1000.00\
                <DTASOF>20250715080000\
            </LEDGERBAL>\
            </STMTRS>";
        let mut tokens = tokenize(input);

        let result = Stmtrs::parse(&mut tokens);
        assert!(result.is_ok());
        let stmtrs = result.unwrap();
        assert!(stmtrs.ledgerbal.is_some());
        assert_eq!(stmtrs.bankacctfrom.acct_id, "1234567890");
    }

    #[test]
    fn test_stmtrs_valid_with_availbal() {
        let input = "\
            <BANKACCTFROM>\
                <ACCTID>1234567890\
                <ACCTTYPE>CHECKING\
            </BANKACCTFROM>\
            <BANKTRANLIST>\
                <DTSTART>20250715080000\
                <DTEND>20250716090000\
            </BANKTRANLIST>\
            <AVAILBAL>\
                <BALAMT>500.00\
                <DTASOF>20250715080000\
            </AVAILBAL>\
            </STMTRS>";
        let mut tokens = tokenize(input);

        let result = Stmtrs::parse(&mut tokens);
        assert!(result.is_ok());
        let stmtrs = result.unwrap();
        assert!(stmtrs.availbal.is_some());
        assert_eq!(stmtrs.bankacctfrom.acct_id, "1234567890");
    }

    #[test]
    fn test_stmtrs_valid_with_bank_id() {
        let input = "\
            <CURDEF>CAD\
            <BANKACCTFROM>\
                <ACCTID>1234567890\
                <ACCTTYPE>SAVINGS\
                <BANKID>123456789\
            </BANKACCTFROM>\
            <BANKTRANLIST>\
                <DTSTART>20250715080000\
                <DTEND>20250716090000\
            </BANKTRANLIST>\
            <LEDGERBAL>\
                <BALAMT>2000.00\
                <DTASOF>20250715080000\
            </LEDGERBAL>\
            <AVAILBAL>\
                <BALAMT>1500.00\
                <DTASOF>20250715080000\
            </AVAILBAL>\
            </STMTRS>";
        let mut tokens = tokenize(input);

        let result = Stmtrs::parse(&mut tokens);
        assert!(result.is_ok());
        let stmtrs = result.unwrap();
        assert_eq!(stmtrs.currency, Some("CAD".to_string()));
        assert_eq!(stmtrs.bankacctfrom.acct_id, "1234567890");
        assert_eq!(stmtrs.bankacctfrom.acct_type, "SAVINGS");
        assert_eq!(stmtrs.bankacctfrom.bank_id, Some("123456789".to_string()));
        assert!(stmtrs.ledgerbal.is_some());
        assert!(stmtrs.availbal.is_some());
    }

    #[test]
    fn test_stmtrs_missing_bankacctfrom() {
        let input = "\
            <BANKTRANLIST>\
                <DTSTART>20250715080000\
                <DTEND>20250716090000\
            </BANKTRANLIST>\
            </STMTRS>";
        let mut tokens = tokenize(input);

        let result = Stmtrs::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::MissingRequiredValue(msg)) if msg.contains("BANKACCTFROM is a required value")
        ));
    }

    #[test]
    fn test_stmtrs_missing_banktranlist() {
        let input = "\
            <BANKACCTFROM>\
                <ACCTID>1234567890\
                <ACCTTYPE>CHECKING\
            </BANKACCTFROM>\
            </STMTRS>";
        let mut tokens = tokenize(input);

        let result = Stmtrs::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::MissingRequiredValue(msg)) if msg.contains("BANKTRANSLIST is a required value")
        ));
    }

    #[test]
    fn test_stmtrs_unknown_tag() {
        let input = "\
            <UNKNOWNTAG>value\
            <BANKACCTFROM>\
                <ACCTID>1234567890\
                <ACCTTYPE>CHECKING\
            </BANKACCTFROM>\
            <BANKTRANLIST>\
                <DTSTART>20250715080000\
                <DTEND>20250716090000\
            </BANKTRANLIST>\
            </STMTRS>";
        let mut tokens = tokenize(input);

        let result = Stmtrs::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedToken(msg)) if msg.contains("UNKNOWNTAG") && msg.contains("STMTRS")
        ));
    }

    #[test]
    fn test_stmtrs_unexpected_eof_after_curdef() {
        let input = "<CURDEF>";
        let mut tokens = tokenize(input);

        let result = Stmtrs::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedEOF(msg)) if msg.contains("Expected token following the CURDEF token")
        ));
    }

    #[test]
    fn test_stmtrs_unexpected_eof_missing_closing_tag() {
        let input = "\
            <CURDEF>USD\
            <BANKACCTFROM>\
                <ACCTID>1234567890\
                <ACCTTYPE>CHECKING\
            </BANKACCTFROM>\
            <BANKTRANLIST>\
                <DTSTART>20250715080000\
                <DTEND>20250716090000\
            </BANKTRANLIST>";
        let mut tokens = tokenize(input);

        let result = Stmtrs::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedEOF(msg)) if msg.contains("Was still expecting the '/STMTRS'")
        ));
    }
}

#[cfg(test)]
mod test_bankmsgsrsv1 {
    use super::*;
    use crate::tokenize;

    #[test]
    fn test_bankmsgsrsv1_valid_minimal() {
        let input = "\
            <STMTTRNRS>\
                <STMTRS>\
                    <BANKACCTFROM>\
                        <ACCTID>1234567890\
                        <ACCTTYPE>CHECKING\
                    </BANKACCTFROM>\
                    <BANKTRANLIST>\
                        <DTSTART>20250715080000\
                        <DTEND>20250716090000\
                    </BANKTRANLIST>\
                </STMTRS>\
            </STMTTRNRS>\
            </BANKMSGSRSV1>";
        let mut tokens = tokenize(input);

        let result = BankMsgSrsV1::parse(&mut tokens);
        assert!(result.is_ok());
        let bankmsgsrsv1 = result.unwrap();
        assert!(bankmsgsrsv1.stmttrns.trnuid.is_none());
        assert!(bankmsgsrsv1.stmttrns.status.is_none());
        assert_eq!(
            bankmsgsrsv1.stmttrns.stmtrs.bankacctfrom.acct_id,
            "1234567890"
        );
        assert_eq!(
            bankmsgsrsv1.stmttrns.stmtrs.bankacctfrom.acct_type,
            "CHECKING"
        );
    }

    #[test]
    fn test_bankmsgsrsv1_missing_stmttrnrs() {
        let input = "</BANKMSGSRSV1>";
        let mut tokens = tokenize(input);

        let result = BankMsgSrsV1::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::MissingRequiredValue(msg)) if msg.contains("STMTTRNRS is a required value")
        ));
    }

    #[test]
    fn test_bankmsgsrsv1_unknown_tag() {
        let input = "\
            <UNKNOWNTAG>value\
            <STMTTRNRS>\
                <STMTRS>\
                    <BANKACCTFROM>\
                        <ACCTID>1234567890\
                        <ACCTTYPE>CHECKING\
                    </BANKACCTFROM>\
                    <BANKTRANLIST>\
                        <DTSTART>20250715080000\
                        <DTEND>20250716090000\
                    </BANKTRANLIST>\
                </STMTRS>\
            </STMTTRNRS>\
            </BANKMSGSRSV1>";
        let mut tokens = tokenize(input);

        let result = BankMsgSrsV1::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedToken(msg)) if msg.contains("UNKNOWNTAG") && msg.contains("BANKMSGSRSV1")
        ));
    }

    #[test]
    fn test_bankmsgsrsv1_unexpected_eof_missing_closing_tag() {
        let input = "\
            <STMTTRNRS>\
                <STMTRS>\
                    <BANKACCTFROM>\
                        <ACCTID>1234567890\
                        <ACCTTYPE>CHECKING\
                    </BANKACCTFROM>\
                    <BANKTRANLIST>\
                        <DTSTART>20250715080000\
                        <DTEND>20250716090000\
                    </BANKTRANLIST>\
                </STMTRS>\
            </STMTTRNRS>";
        let mut tokens = tokenize(input);

        let result = BankMsgSrsV1::parse(&mut tokens);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(QFXParsingError::UnexpectedEOF(msg)) if msg.contains("Was still expecting the '/BANKMSGSRSV1' token")
        ));
    }
}
