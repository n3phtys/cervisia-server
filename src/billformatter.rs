use rustix_bl::datastore::Bill;
use rustix_bl;
use chrono::prelude::*;
use chrono::offset::LocalResult;

pub struct SewobeConfiguration {
    pub static_csv_headerline: String,
    pub template_for_csv_line: String,
}

//following template placeholders will be string-replaced inside the configured template_for_csv_line when calling format_as_sewobe_csv()

static TEMPLATE_PLACEHOLDER_MEMBER_NR: &'static str = "TEMPLATE_PLACEHOLDER_MEMBER_NR";
static TEMPLATE_PLACEHOLDER_BILL_NR: &'static str = "TEMPLATE_PLACEHOLDER_BILL_NR";
static TEMPLATE_PLACEHOLDER_BILL_TITLE: &'static str = "TEMPLATE_PLACEHOLDER_BILL_TITLE";
static TEMPLATE_PLACEHOLDER_BILL_DATE: &'static str = "TEMPLATE_PLACEHOLDER_BILL_DATE";
static TEMPLATE_PLACEHOLDER_POSITION_NR: &'static str = "TEMPLATE_PLACEHOLDER_POSITION_NR";
static TEMPLATE_PLACEHOLDER_POSITION_NAME: &'static str = "TEMPLATE_PLACEHOLDER_POSITION_NAME";
static TEMPLATE_PLACEHOLDER_POSITION_DESCRIPTION: &'static str = "TEMPLATE_PLACEHOLDER_POSITION_DESCRIPTION";
static TEMPLATE_PLACEHOLDER_AMOUNT: &'static str = "TEMPLATE_PLACEHOLDER_AMOUNT";
static TEMPLATE_PLACEHOLDER_PRICE_PER_UNIT: &'static str = "TEMPLATE_PLACEHOLDER_PRICE_PER_UNIT";
static TEMPLATE_PLACEHOLDER_BILLING_DATE: &'static str = "TEMPLATE_PLACEHOLDER_BILLING_DATE";
static TEMPLATE_PLACEHOLDER_DUE_DATE: &'static str = "TEMPLATE_PLACEHOLDER_DUE_DATE";
static TEMPLATE_PLACEHOLDER_POSITION_TERMINATION_DATE: &'static str = "TEMPLATE_PLACEHOLDER_POSITION_TERMINATION_DATE";
//unused static TEMPLATE_PLACEHOLDER_TAX_RATE: &'static str = "TEMPLATE_PLACEHOLDER_TAX_RATE";
static TEMPLATE_PLACEHOLDER_REMARK: &'static str = "TEMPLATE_PLACEHOLDER_REMARK";
//unused static TEMPLATE_PLACEHOLDER_BOOKKEEPING_ACCOUNT: &'static str = "TEMPLATE_PLACEHOLDER_BOOKKEEPING_ACCOUNT";
//unused static TEMPLATE_PLACEHOLDER_SUB_BOOKKEEPING_ACCOUNT: &'static str = "TEMPLATE_PLACEHOLDER_SUB_BOOKKEEPING_ACCOUNT";


pub trait BillFormatting {
    //takes configuration and outputs a sewobe csv string
    fn format_as_sewobe_csv(&self, conf: &SewobeConfiguration) -> Vec<Vec<String>>;

    //outputs reduced bill string for one specific person
    fn format_as_personalized_documentation(&self, user_id: u32) -> Vec<Vec<String>>;


    fn sewobe_header(&self, conf: &SewobeConfiguration) -> Vec<String>;
    fn documentation_header(&self, conf: &SewobeConfiguration) -> Vec<String>;

    //outputs bill for everyone in the bill (ordered alphabetically by name)
    fn format_as_documentation(&self) -> Vec<Vec<String>> {
        self.list_of_user_ids().iter().flat_map(|id| self.format_as_personalized_documentation(*id)).collect()
    }

    fn list_of_user_ids(&self) -> Vec<u32>;
    fn includes_user(&self, user_id: u32) -> bool {
        return self.list_of_user_ids().contains(&user_id);
    }
}


pub struct SewobeCSVLine {
    pub external_user_id: String,
    pub use_r_vs_g: bool,
    pub bill_external_id: String,
    pub bill_name: String,
    pub bill_date: DateTime<Utc>,
    pub position_index: u16,
    pub position_name: String,
    pub position_description: String,
    pub position_count: u32,
    pub price_per_unit_cents: u32,
    pub use_inbox: bool,
    pub receive_mail: bool,
    pub payment_target_days: u32,
    pub sepa_interval: u32,
    pub bill_date_sent: DateTime<Utc>,
    pub bill_date_late: DateTime<Utc>,
    pub position_ends_date: DateTime<Utc>,
    pub tax_rate: String,
    pub description: String,
    pub is_not_donation: bool,
    pub donation_remark: String,
    pub billkeeping_account: String,
    pub tax_key: String,
    pub subaccount: String,
}

impl SewobeCSVLine {
    //TODO: should get an export date, or not? or a finalization date? to calculate from there
    //TODO: remark should contain FROM and TO as readable date
    fn new(timestamp_from: i64, timestamp_to: i64, external_user_id: &str, position_name: &str, position_description: &str, position_index: u16, position_count: u32, position_price_per_unit: u32) -> Self {
        let utc_timestamp_from = Utc.timestamp(timestamp_from, 0);
        let utc_timestamp_to = Utc.timestamp(timestamp_to, 0);

        unimplemented!()
    }

    fn fmt(&self) -> Vec<String> {
        unimplemented!()
    }
}


impl BillFormatting for Bill {
    fn format_as_sewobe_csv(&self, conf: &SewobeConfiguration) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let timestamp_to: i64 = self.timestamp_to;
        let timestamp_from: i64 = self.timestamp_from;

        //for every user
        let items = self.finalized_data.all_items.clone();
        let users = self.finalized_data.all_users.clone();

        //filter out unbilled user_ids
        for (user_id, consumption) in &self.finalized_data.user_consumption {
            if users.get(user_id).is_some() && users.get(user_id).unwrap().external_user_id.is_some() && users.get(user_id).unwrap().is_billed && !self.users_that_will_not_be_billed.contains(user_id) {
                let external_user_id: String = users.get(user_id).unwrap().username.to_string();
                let mut position_index = 0u16;
                for (day, daycontent) in &consumption.per_day {
                    //for every item

                    for (item_id_purchase, count) in &daycontent.personally_consumed {
                        let item: rustix_bl::datastore::Item = items.get(item_id_purchase).unwrap().clone();
                        result.push(SewobeCSVLine::new(timestamp_from, timestamp_to, &external_user_id, &item.name, "Selbst gekauft", position_index, *count, item.cost_cents).fmt());
                        position_index += 1;
                    }
                        for special in &daycontent.specials_consumed {
                            result.push(SewobeCSVLine::new(timestamp_from, timestamp_to, &external_user_id,  &special.name, "Speziell abgestrichen", position_index, 1, special.price).fmt());
                            position_index += 1;
                        }

                        for (item_id_ffa, count) in &daycontent.ffa_giveouts {
                            let item: rustix_bl::datastore::Item = items.get(item_id_ffa).unwrap().clone();
                            result.push(SewobeCSVLine::new(timestamp_from, timestamp_to, &external_user_id, &item.name, "An alle ausgegeben", position_index, *count, item.cost_cents).fmt());
                            position_index += 1;
                        }

                        for (other_user_id, paid_for) in &daycontent.giveouts_to_user_id {

                            let other_user: rustix_bl::datastore::User = users.get(other_user_id).unwrap().clone();

                            let budget_given: u64 = paid_for.budget_given;
                            let budget_gotten: u64 = paid_for.budget_gotten;

                            //if budget given or gotten > 0, also add to bill
                            if budget_given > 0 {
                                result.push(SewobeCSVLine::new(timestamp_from, timestamp_to, &external_user_id, &format!("Guthaben verschenkt an {}", other_user.username), &format!("Guthaben verbraucht: {} Cents (intern verrechnet)", budget_given), position_index, 1, 0).fmt());
                                position_index += 1;
                            }
                            if budget_gotten > 0 {
                                result.push(SewobeCSVLine::new(timestamp_from, timestamp_to, &external_user_id, &format!("Guthaben erhalten von {}", other_user.username), &format!("Guthaben verbraucht: {} Cents (intern verrechnet)", budget_gotten), position_index, 1, 0).fmt());
                                position_index += 1;
                            }



                            for (item_id, count) in &paid_for.count_giveouts_used {
                                let item: rustix_bl::datastore::Item = items.get(&item_id).unwrap().clone();
                                result.push(SewobeCSVLine::new(timestamp_from, timestamp_to, &external_user_id, &item.name, &format!("Ausgegeben an und verbraucht von {}", other_user.username), position_index, *count, item.cost_cents).fmt());
                                position_index += 1;}
                        }


                        //list received (per donor), paid, donated (per recipient, or ffa)
                        //list amount of user budget ingoing and outgoing (per donor/recipient, but independent of item, as unique item position)
                    }
                }
            }


        return result;
    }

    fn format_as_personalized_documentation(&self, user_id: u32) -> Vec<Vec<String>> {
        //TODO: define format
        let mut lines: Vec<Vec<String>> = Vec::new();

        //add header later on, only data for now
        let userdata: &rustix_bl::datastore::BillUserInstance = self.finalized_data.user_consumption.get(&user_id).unwrap();


        for (dayindex, daydata) in &userdata.per_day {
            //personally_consumed
            //specials_consumed
            //ffa_giveouts
            //giveouts_to_user_id
        }

        return lines;
    }

    fn list_of_user_ids(&self) -> Vec<u32> {
        let mut v: Vec<u32> = Vec::new();
        for (key, value) in &self.finalized_data.all_users {
            v.push(*key);
        }
        return v;
    }
    fn sewobe_header(&self, conf: &SewobeConfiguration) -> Vec<String> {
        let raw_header = "Mitgliedsnummer;R vs G;Rechnungsnr.;Rechnungsname;Rechnungsdatum;Positionsnr.;Positionsname;Positionsbeschreibung;Anzahl;Preis pro Einheit;2 == Lastschrift und 1 == Ueberweisung;Empfang per Mail;Zahlungsziel in Tagen;SEPA Intervall (0 fuer einmalig);Datum Rechnungsstellung;Datum Faelligkeit;Datum Positionsende;Mehrwertsteuersatz;Beschreibung; Spendenfähig;Spende;Buchhaltungskonto;Steuerschluessel;Unterkonto Kantine";
        return raw_header.split(";").map(|s| s.to_string()).collect();
    }

    fn documentation_header(&self, conf: &SewobeConfiguration) -> Vec<String> {
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {
    use billformatter::BillFormatting;
    use billformatter::SewobeConfiguration;
    use rustix_bl::datastore::Bill;
    use std::collections::*;
    use rustix_bl::datastore::*;

    #[test]
    fn simple_sewobe_csv_works() {
        let bill: Bill = Bill {
            //TODO: input some basic data to test all functionality

            timestamp_from: 0,
            timestamp_to: 0,
            comment: String::new(),
            users: UserGroup::AllUsers,
            bill_state: BillState::ExportedAtLeastOnce,
            users_that_will_not_be_billed: HashSet::new(),
            finalized_data: ExportableBillData {
                all_users: HashMap::new(),
                all_items: HashMap::new(),
                user_consumption: HashMap::new(),
            },
        };
        let conf: SewobeConfiguration = SewobeConfiguration {
            static_csv_headerline: String::new(),
            template_for_csv_line: String::new(),
        };

        let should = vec![
            //header
            vec![
                "Mitgliedsnummer", "R vs G", "Rechnungsnr.", "Rechnungsname", "Rechnungsdatum", "Positionsnr.", "Positionsname", "Positionsbeschreibung", "Anzahl", "Preis pro Einheit", "2 == Lastschrift und 1 == Ueberweisung", "Empfang per Mail", "Zahlungsziel in Tagen", "SEPA Intervall (0 fuer einmalig)", "Datum Rechnungsstellung", "Datum Faelligkeit", "Datum Positionsende", "Mehrwertsteuersatz", "Beschreibung", " Spendenfähig", "Spende", "Buchhaltungskonto", "Steuerschluessel", "Unterkonto Kantine"
            ].iter().map(|s| s.to_string()).collect(),
            //first line
            vec!["11293".to_string(), "2".to_string(), "1706241129301".to_string(), "Kantinenrechnung 06/2017".to_string(), "24.06.2017".to_string(),
                 "1".to_string(), "Edel".to_string(), "Edel".to_string(), "4".to_string(), "0,95".to_string(), "2".to_string(), "2".to_string(), "30".to_string(), "0".to_string(), "24.06.2017".to_string(),
                 "01.07.2017".to_string(), "31.05.2117".to_string(), "0".to_string(), "auto-gen by avbier".to_string(), "0".to_string(), "".to_string(), "1112".to_string(), "1".to_string(), "8293".to_string()],
            //second line
            vec!["11293".to_string(), "2".to_string(), "1706241129301".to_string(), "Kantinenrechnung 06/2017".to_string(), "24.06.2017".to_string(),
                 "1".to_string(), "Pils".to_string(), "Pils".to_string(), "4".to_string(), "0,95".to_string(), "2".to_string(), "2".to_string(), "30".to_string(), "0".to_string(), "24.06.2017".to_string(),
                 "01.07.2017".to_string(), "31.05.2117".to_string(), "0".to_string(), "auto-gen by avbier".to_string(), "0".to_string(), "".to_string(), "1112".to_string(), "1".to_string(), "8293".to_string()],
        ];

        let is_content = bill.format_as_sewobe_csv(&conf);
        let is_header = bill.sewobe_header(&conf);

        assert_eq!(should.len() - 1, is_content.len());

        for j in 0..should[0].len() - 1 {
            assert_eq!(should[0][j], is_header[j]);
        }

        for i in 1..should.len() - 1 {
            for j in 0..should[i].len() - 1 {
                assert_eq!(should[i][j], is_content[i - 1][j])
            }
        }
    }

    #[test]
    fn simple_general_csv_works() {
        assert!(1 + 1 == 2);
    }
}