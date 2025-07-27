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
    pub banktranslist: BankTranList, // TODO: Should not be a pub(crate) visibility
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
