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
                "/BANKACCTFROM" => {
                    return Ok(Self {
                        acct_id: s_acct_id.ok_or(QFXParsingError::MissingRequiredValue(
                            "ACCTID is a required value in BANKACCTFROM".to_string(),
                        ))?,
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
