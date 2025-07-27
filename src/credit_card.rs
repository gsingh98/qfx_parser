use crate::parse_ofx_datetime;
use crate::Parseable;
use crate::QFXParsingError;
use crate::Status;
use chrono::DateTime;
use chrono::Utc;

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

#[derive(Clone)]
pub struct BankTranList {
    pub dt_start: Option<String>,
    pub dt_end: Option<String>,
    pub transactions: Vec<Stmttrn>, // TODO: Should not have pub(crate) visibility
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
                                "BANKTRANS is a required value in CCSTMTRS".to_string(),
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
