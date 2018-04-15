use rustix_bl::datastore::Bill;
use rustix_bl;
use chrono::prelude::*;
use time;
use chrono::offset::LocalResult;
use std;

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
    fn format_as_sewobe_csv(&self) -> Vec<Vec<String>>;

    //outputs reduced bill string for one specific person
    fn format_as_personalized_documentation(&self, user_id: u32) -> Vec<Vec<String>>;


    fn sewobe_header(&self) -> Vec<String>;
    fn documentation_header(&self) -> Vec<String>;

    //outputs bill for everyone in the bill (ordered alphabetically by name)
    fn format_as_documentation(&self) -> Vec<Vec<String>> {
        self.list_of_user_ids().iter().flat_map(|id| self.format_as_personalized_documentation(*id)).collect()
    }

    fn list_of_user_ids(&self) -> Vec<u32>;
    fn includes_user(&self, user_id: u32) -> bool {
        return self.list_of_user_ids().contains(&user_id);
    }
}

fn cents_to_currency_string(cents: i32) -> String {
    let before = cents / 100;
    let after2 = cents % 10;
    let after1 = (cents % 100 - after2) / 10;
    let after1 = if after1 >= 0 {after1} else {after1 * -1i32};
    return format!("{},{}{}", before, after1, after2);
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
    pub price_per_unit_cents: i32,
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


pub struct OversightCSVLine {
    pub username: String,
    pub user_id: String, //external_id
    pub is_billed: bool,
    pub day: DateTime<Utc>,
    pub item_name: String,
    pub item_count: u32,
    pub item_cost_cents: i32,
    pub budget_cents_outgoing: i32, //negative for incoming
    //pub giveout_message: String,
    pub donor: String,
    pub donor_id: String,
    pub recipient: String,
    pub recipient_id: String,
    pub is_special: bool,
    pub is_giveout: bool,
    pub is_count: bool,
    pub is_budget: bool,
    pub is_incoming_donation: bool,
    pub is_ffa: bool,
}

impl OversightCSVLine {


    pub fn ffa_giveout(username: String, user_id: String, is_billed: bool, item_name: String, item_count: u32, item_cost_cents: i32, day: DateTime<Utc>) -> Self {
        return OversightCSVLine {
            username: username,
            user_id: user_id,
            is_billed: is_billed,
            day: day,
            item_name: item_name,
            item_count: item_count,
            item_cost_cents: item_cost_cents,
            budget_cents_outgoing: 0,
            donor: String::new(),
            donor_id: String::new(),
            recipient: String::new(),
            recipient_id: String::new(),
            is_special: false,
            is_giveout: true,
            is_count: false,
            is_budget: false,
            is_incoming_donation: false,
            is_ffa: true,
        };
    }
    pub fn budget_outgoing(username: String, user_id: String, is_billed: bool, budget_cents_outgoing: i32, day: DateTime<Utc>, recipient: String, recipient_id : String) -> Self {
        return OversightCSVLine {
            username: username,
            user_id: user_id,
            is_billed: is_billed,
            day: day,
            item_name: String::new(),
            item_count: 1,
            item_cost_cents: budget_cents_outgoing,
            budget_cents_outgoing: budget_cents_outgoing,
            donor: String::new(),
            donor_id: String::new(),
            recipient: recipient,
            recipient_id: recipient_id,
            is_special: false,
            is_giveout: true,
            is_count: false,
            is_budget: true,
            is_incoming_donation: false,
            is_ffa: false,
        };
    }
    pub fn budget_incoming(username: String, user_id: String, is_billed: bool, budget_cents_incoming: i32, day: DateTime<Utc>, donor: String, donor_id: String) -> Self {
        return OversightCSVLine {
            username: username,
            user_id: user_id,
            is_billed: is_billed,
            day: day,
            item_name: String::new(),
            item_count: 1,
            item_cost_cents: (-1i32) * budget_cents_incoming,
            budget_cents_outgoing: (-1i32) * budget_cents_incoming,
            donor: donor,
            donor_id: donor_id,
            recipient: String::new(),
            recipient_id: String::new(),
            is_special: false,
            is_giveout: true,
            is_count: false,
            is_budget: true,
            is_incoming_donation: true,
            is_ffa: false,
        };
    }
    pub fn count_giveout_outgoing(username: String, user_id: String, is_billed: bool, item_name: String, item_count: u32, item_cost_cents: i32, day: DateTime<Utc>, recipient: String, recipient_id: String) -> Self {
        return OversightCSVLine {
            username: username,
            user_id: user_id,
            is_billed: is_billed,
            day: day,
            item_name: item_name,
            item_count: item_count,
            item_cost_cents: item_cost_cents,
            budget_cents_outgoing: 0,
            donor: String::new(),
            donor_id: String::new(),
            recipient: recipient,
            recipient_id: recipient_id,
            is_special: false,
            is_giveout: true,
            is_count: true,
            is_budget: false,
            is_incoming_donation: false,
            is_ffa: false,
        };
    }
    pub fn normal_purchase(username: String, user_id: String, is_billed: bool, item_name: String, item_count: u32, item_cost_cents: i32, day: DateTime<Utc>) -> Self {
        return OversightCSVLine {
            username: username,
            user_id: user_id,
            is_billed: is_billed,
            day: day,
            item_name: item_name,
            item_count: item_count,
            item_cost_cents: item_cost_cents,
            budget_cents_outgoing: 0,
            donor: String::new(),
            donor_id: String::new(),
            recipient: String::new(),
            recipient_id: String::new(),
            is_special: true,
            is_giveout: false,
            is_count: false,
            is_budget: false,
            is_incoming_donation: false,
            is_ffa: false,
        };
    }
    pub fn special_purchase(username: String, user_id: String, is_billed: bool, item_name: String, item_cost_cents: i32, day: DateTime<Utc>) -> Self {
        return OversightCSVLine {
            username: username,
            user_id: user_id,
            is_billed: is_billed,
            day: day,
            item_name: item_name,
            item_count: 1,
            item_cost_cents: item_cost_cents,
            budget_cents_outgoing: 0,
            donor: String::new(),
            donor_id: String::new(),
            recipient: String::new(),
            recipient_id: String::new(),
            is_special: false,
            is_giveout: false,
            is_count: false,
            is_budget: false,
            is_incoming_donation: false,
            is_ffa: false,
        };
    }


    fn fmt(&self) -> Vec<String> {
        return vec![
            self.username.to_string(), self.user_id.to_string(), if self.is_billed {"true".to_string()} else {"false".to_string()},
            self.day.format(DATE_FORMAT_STRING).to_string(), self.item_name.to_string(), self.item_count.to_string(),
            cents_to_currency_string(self.item_cost_cents), self.budget_cents_outgoing.to_string(), self.donor.to_string(),
            self.donor_id.to_string(), self.recipient.to_string(), self.recipient_id.to_string(),
            if self.is_special {"true".to_string()} else {"false".to_string()}, if self.is_giveout {"true".to_string()} else {"false".to_string()}, if self.is_count {"true".to_string()} else {"false".to_string()},
            if self.is_budget {"true".to_string()} else {"false".to_string()}, if self.is_incoming_donation {"true".to_string()} else {"false".to_string()}, if self.is_ffa {"true".to_string()} else {"false".to_string()},
        ];
    }
}

impl SewobeCSVLine {
    //TODO: should get an export date, or not? or a finalization date? to calculate from there
    //TODO: remark should contain FROM and TO as readable date
    fn new(timestamp_from: i64, timestamp_to: i64, external_user_id: &str, position_name: &str, position_description: &str, position_index: u16, position_count: u32, position_price_per_unit: i32) -> Self {
        let utc_timestamp_from = Utc.timestamp(timestamp_from / 1000, 0);
        let utc_timestamp_to = Utc.timestamp(timestamp_to / 1000, 0);

        let billing_creation_date = utc_timestamp_to.clone(); //TODO: replace by first export date

        let bill_id: String = billing_creation_date.format("%y%m%d%S").to_string() + external_user_id;



        return SewobeCSVLine {
            external_user_id: external_user_id.to_string(),
            use_r_vs_g: true,
            bill_external_id: bill_id,
            bill_name: "Kantinenabrechnung ".to_string() + &billing_creation_date.format("%m/%y").to_string(),
            bill_date: billing_creation_date,
            position_index: position_index,
            position_name: position_name.to_string(),
            position_description: position_description.to_string(),
            position_count: position_count,
            price_per_unit_cents: position_price_per_unit,
            use_inbox: true,
            receive_mail: true,
            payment_target_days: 30,
            sepa_interval: 0,
            bill_date_sent: billing_creation_date,
            bill_date_late: billing_creation_date + time::Duration::seconds(14 * 24 * 60 * 60),
            position_ends_date: billing_creation_date + time::Duration::seconds(100 * 365  * 24 * 60 * 60),
            tax_rate: "0".to_string(),
            description: "KA ".to_string() + &utc_timestamp_from.format(DATE_FORMAT_STRING_SHORT).to_string() + "-" + &utc_timestamp_to.format(DATE_FORMAT_STRING_SHORT).to_string(),
            is_not_donation: true,
            donation_remark: "".to_string(),
            billkeeping_account: "1112".to_string(),
            tax_key: "1".to_string(),
            subaccount: "8293".to_string(),
        };
    }

    fn fmt(&self) -> Vec<String> {
        vec![
            self.external_user_id.to_string(), if self.use_r_vs_g { "2".to_string() } else { "1".to_string() }, self.bill_external_id.to_string(), self.bill_date.format(DATE_FORMAT_STRING).to_string(), self.position_index.to_string(), self.position_name.to_string(), self.position_description.to_string(), self.position_count.to_string(), cents_to_currency_string(self.price_per_unit_cents), if self.use_inbox {"2".to_string()} else {"1".to_string()}, if self.receive_mail {"2".to_string()} else {"1".to_string()}, self.payment_target_days.to_string(), self.sepa_interval.to_string(), self.bill_date_sent.format(DATE_FORMAT_STRING).to_string(), self.bill_date_late.format(DATE_FORMAT_STRING).to_string(), self.position_ends_date.format(DATE_FORMAT_STRING).to_string(), self.tax_rate.to_string(), self.description.to_string(), if self.is_not_donation {"0".to_string()} else {"1".to_string()}, self.donation_remark.to_string(), self.billkeeping_account.to_string(), self.tax_rate.to_string(), self.subaccount.to_string()
        ]
    }
}

static DATE_FORMAT_STRING: &'static str = "%d.%m.%Y";
static DATE_FORMAT_STRING_SHORT: &'static str = "%d.%m.%y";

pub trait InOrderableu32 {
    fn in_order_keys(&self) -> Vec<u32>;
}
pub trait InOrderableusize {
    fn in_order_keys(&self) -> Vec<usize>;
}

impl<T> InOrderableu32 for std::collections::HashMap<u32, T> {
    fn in_order_keys(&self) -> Vec<u32> {
        let mut v: Vec<u32> = self.keys().map(|x:&u32| *x).collect();
        v.sort();
        return v;
    }
}

impl<T> InOrderableusize for std::collections::HashMap<usize, T> {
    fn in_order_keys(&self) -> Vec<usize> {
        let mut v: Vec<usize> = self.keys().map(|x:&usize| *x).collect();
        v.sort();
        return v;
    }
}


impl BillFormatting for Bill {
    fn format_as_sewobe_csv(&self) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let timestamp_to: i64 = self.timestamp_to;
        let timestamp_from: i64 = self.timestamp_from;

        //for every user
        let items = self.finalized_data.all_items.clone();
        let users = self.finalized_data.all_users.clone();



        //filter out unbilled user_ids
        for user_id in &users.in_order_keys() {
            let consumption = self.finalized_data.user_consumption.get(user_id).unwrap();

            if users.get(user_id).is_some() && users.get(user_id).unwrap().external_user_id.is_some() && users.get(user_id).unwrap().is_billed && !self.users_that_will_not_be_billed.contains(user_id) {
                let external_user_id: String = users.get(user_id).unwrap().clone().external_user_id.unwrap().to_string();
                let mut position_index = 0u16;
                for day in &consumption.per_day.in_order_keys() {
                    let daycontent = consumption.per_day.get(day).unwrap();
                    //for every item

                    for item_id_purchase in &daycontent.personally_consumed.in_order_keys() {
                        let count = daycontent.personally_consumed.get(item_id_purchase).unwrap();
                        let item: rustix_bl::datastore::Item = items.get(item_id_purchase).unwrap().clone();
                        result.push(SewobeCSVLine::new(timestamp_from, timestamp_to, &external_user_id, &item.name, "Selbst gekauft", position_index, *count, item.cost_cents as i32).fmt());
                        position_index += 1;
                    }
                        for special in &daycontent.specials_consumed {
                            result.push(SewobeCSVLine::new(timestamp_from, timestamp_to, &external_user_id,  &special.name, "Speziell abgestrichen", position_index, 1, special.price as i32).fmt());
                            position_index += 1;
                        }

                        for item_id_ffa in &daycontent.ffa_giveouts.in_order_keys() {
                            let count = daycontent.ffa_giveouts.get(item_id_ffa).unwrap();
                            let item: rustix_bl::datastore::Item = items.get(item_id_ffa).unwrap().clone();
                            result.push(SewobeCSVLine::new(timestamp_from, timestamp_to, &external_user_id, &item.name, "An alle ausgegeben", position_index, *count, item.cost_cents as i32).fmt());
                            position_index += 1;
                        }

                        for other_user_id in &daycontent.giveouts_to_user_id.in_order_keys() {
                            let paid_for = daycontent.giveouts_to_user_id.get(other_user_id).unwrap();

                            let other_user: rustix_bl::datastore::User = users.get(other_user_id).unwrap().clone();

                            let budget_given: u64 = paid_for.budget_given;
                            let budget_gotten: u64 = paid_for.budget_gotten;

                            //if budget given or gotten > 0, also add to bill
                            if budget_given > 0 {
                                result.push(SewobeCSVLine::new(timestamp_from, timestamp_to, &external_user_id, &format!("Guthaben verschenkt an {}", other_user.username), &format!("Guthaben verbraucht: {} Cents (intern verrechnet)", budget_given), position_index, 1, budget_given as i32).fmt());
                                position_index += 1;
                            }
                            if budget_gotten > 0 {
                                result.push(SewobeCSVLine::new(timestamp_from, timestamp_to, &external_user_id, &format!("Guthaben erhalten von {}", other_user.username), &format!("Guthaben verbraucht: {} Cents (intern verrechnet)", budget_gotten), position_index, 1, -1i32 * (budget_gotten as i32)).fmt());
                                position_index += 1;
                            }



                            for item_id in &paid_for.count_giveouts_used.in_order_keys() {
                                let count = paid_for.count_giveouts_used.get(item_id).unwrap();
                                let item: rustix_bl::datastore::Item = items.get(&item_id).unwrap().clone();
                                result.push(SewobeCSVLine::new(timestamp_from, timestamp_to, &external_user_id, &item.name, &format!("Ausgegeben an und verbraucht von {}", other_user.username), position_index, *count, item.cost_cents as i32).fmt());
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
        let mut result: Vec<Vec<String>> = Vec::new();
        let timestamp_to: i64 = self.timestamp_to;
        let timestamp_from: i64 = self.timestamp_from;

        //for every user
        let items = self.finalized_data.all_items.clone();
        let users = self.finalized_data.all_users.clone();



        //filter out unbilled user_ids
        for user_id in &users.in_order_keys() {
            let is_billed: bool = users.get(user_id).is_some() && users.get(user_id).unwrap().external_user_id.is_some() && users.get(user_id).unwrap().is_billed && !self.users_that_will_not_be_billed.contains(user_id);
            let consumption = self.finalized_data.user_consumption.get(user_id).unwrap();

            if users.get(user_id).is_some() && users.get(user_id).unwrap().external_user_id.is_some() {
                let external_user_id: String = users.get(user_id).unwrap().clone().external_user_id.unwrap().to_string();
                let mut position_index = 0u16;
                for day in &consumption.per_day.in_order_keys() {
                    let daycontent = consumption.per_day.get(day).unwrap();

                    let day_timestamp: DateTime<Utc> = Utc.timestamp(timestamp_from, 0) + time::Duration::seconds((60i64 * 60i64 * 24i64) * (*day as i64));

                    //for every item

                    for item_id_purchase in &daycontent.personally_consumed.in_order_keys() {
                        let count = daycontent.personally_consumed.get(item_id_purchase).unwrap();
                        let item: rustix_bl::datastore::Item = items.get(item_id_purchase).unwrap().clone();
                        result.push(OversightCSVLine::normal_purchase(users.get(user_id).unwrap().username.to_string(), external_user_id.to_string(), is_billed, item.name, *count, item.cost_cents as i32, day_timestamp).fmt());
                        position_index += 1;
                    }
                    for special in &daycontent.specials_consumed {
                        result.push(OversightCSVLine::special_purchase(users.get(user_id).unwrap().username.to_string(), external_user_id.to_string(), is_billed, special.name.to_string(), (special.price) as i32, day_timestamp).fmt());
                        position_index += 1;
                    }

                    for item_id_ffa in &daycontent.ffa_giveouts.in_order_keys() {
                        let count = daycontent.ffa_giveouts.get(item_id_ffa).unwrap();
                        let item: rustix_bl::datastore::Item = items.get(item_id_ffa).unwrap().clone();
                        result.push(OversightCSVLine::ffa_giveout(users.get(user_id).unwrap().username.to_string(), external_user_id.to_string(), is_billed, item.name, *count, item.cost_cents as i32, day_timestamp).fmt());
                        position_index += 1;
                    }

                    for other_user_id in &daycontent.giveouts_to_user_id.in_order_keys() {
                        let paid_for = daycontent.giveouts_to_user_id.get(other_user_id).unwrap();

                        let other_user: rustix_bl::datastore::User = users.get(other_user_id).unwrap().clone();
                        let other_user_name: String = other_user.username;
                        let other_user_id : String = other_user.external_user_id.unwrap_or("".to_string());

                        let budget_given: u64 = paid_for.budget_given;
                        let budget_gotten: u64 = paid_for.budget_gotten;

                        //if budget given or gotten > 0, also add to bill
                        if budget_given > 0 {
                            result.push(OversightCSVLine::budget_outgoing(users.get(user_id).unwrap().username.to_string(), external_user_id.to_string(), is_billed, budget_given as i32, day_timestamp, other_user_name.to_string(), other_user_id.to_string()).fmt());
                            position_index += 1;
                        }
                        if budget_gotten > 0 {
                            result.push(OversightCSVLine::budget_incoming(users.get(user_id).unwrap().username.to_string(), external_user_id.to_string(), is_billed, budget_gotten as i32, day_timestamp, other_user_name.to_string(), other_user_id.to_string()).fmt());
                            position_index += 1;
                        }



                        for item_id in &paid_for.count_giveouts_used.in_order_keys() {
                            let count = paid_for.count_giveouts_used.get(item_id).unwrap();
                            let item: rustix_bl::datastore::Item = items.get(&item_id).unwrap().clone();
                            result.push(OversightCSVLine::count_giveout_outgoing(users.get(user_id).unwrap().username.to_string(), external_user_id.to_string(), is_billed, item.name, *count, item.cost_cents as i32, day_timestamp,  other_user_name.to_string(), other_user_id.to_string()).fmt());
                            position_index += 1;}
                    }


                    //list received (per donor), paid, donated (per recipient, or ffa)
                    //list amount of user budget ingoing and outgoing (per donor/recipient, but independent of item, as unique item position)
                }
            }
        }


        return result;
    }

    fn list_of_user_ids(&self) -> Vec<u32> {
        let mut v: Vec<u32> = Vec::new();
        for (key, value) in &self.finalized_data.all_users {
            v.push(*key);
        }
        return v;
    }
    fn sewobe_header(&self) -> Vec<String> {
        let raw_header = "Mitgliedsnummer;R vs G;Rechnungsnr.;Rechnungsname;Rechnungsdatum;Positionsnr.;Positionsname;Positionsbeschreibung;Anzahl;Preis pro Einheit;2 == Lastschrift und 1 == Ueberweisung;Empfang per Mail;Zahlungsziel in Tagen;SEPA Intervall (0 fuer einmalig);Datum Rechnungsstellung;Datum Faelligkeit;Datum Positionsende;Mehrwertsteuersatz;Beschreibung; Spendenfähig;Spende;Buchhaltungskonto;Steuerschluessel;Unterkonto Kantine";
        return raw_header.split(";").map(|s| s.to_string()).collect();
    }

    fn documentation_header(&self) -> Vec<String> {
        let raw_header = "username;user_id;is_billed;day;item_name;item_count;item_cost_per_unit;budget_cents_outgoing;donor;donor_id;recipient;recipient_id;is_special;is_giveout;is_count;is_budget;is_incoming_donation;is_ffa";
        return raw_header.split(";").map(|s| s.to_string()).collect();
    }
}


#[cfg(test)]
mod tests {
    use rustix_bl;
    use billformatter::BillFormatting;
    use rustix_bl::datastore::*;
    use std::collections::*;
    use rustix_bl::datastore::*;
    use chrono;
    use chrono::*;
    use billformatter::DATE_FORMAT_STRING;


    #[test]
    fn stringsort_works() {
        let mut a = vec!["b", "ab", "a"];
        a.sort();
        let mut b = vec!["b", "a", "ab"];
        b.sort();
        assert_eq!(a, b);
    }

    #[test]
    fn simple_sewobe_csv_works() {
        let bill: Bill = Bill {

            timestamp_from: 1500000000,
            timestamp_to: 2000000000,
            comment: "No comment here".to_string(),
            users: UserGroup::AllUsers,
            bill_state: BillState::ExportedAtLeastOnce,
            users_that_will_not_be_billed: {
                let mut s = HashSet::new();
                s.insert(1);
                s
            },
            finalized_data: ExportableBillData {
                all_users: {
                    let mut m = HashMap::new();
                    m.insert(0, rustix_bl::datastore::User {
                        username: "alice".to_string(),
                        external_user_id: Some("ExternalUserId0".to_string()),
                        user_id: 0,
                        is_billed: true,
                        highlight_in_ui: false,
                        deleted: false,
                    });
                    m.insert(1, rustix_bl::datastore::User {
                        username: "bob".to_string(),
                        external_user_id: None,
                        user_id: 1,
                        is_billed: true,
                        highlight_in_ui: false,
                        deleted: false,
                    });
                    m.insert(2, rustix_bl::datastore::User {
                        username: "charlie".to_string(),
                        external_user_id: None,
                        user_id: 2,
                        is_billed: false,
                        highlight_in_ui: false,
                        deleted: false,
                    });
                    m
                },
                all_items: {
                    let mut m = HashMap::new();
                    m.insert(0, rustix_bl::datastore::Item {
                        name: "beer".to_string(),
                        item_id: 0,
                        category: None,
                        cost_cents: 95,
                        deleted: false,
                    });
                    m.insert(1, rustix_bl::datastore::Item {
                        name: "soda".to_string(),
                        item_id: 1,
                        category: None,
                        cost_cents: 85,
                        deleted: false,
                    });
                    m
                },
                user_consumption: {
                    let mut consumption_map = HashMap::new();

                    //user 0 has consumed both items on two separate days and been given count and budget by 0 and 1
                    //user 1 has consumed both items and given count to 0
                    //user 2 has consumed items 1 and given budget to 0


                    consumption_map.insert(0, rustix_bl::datastore::BillUserInstance {user_id:0, per_day: {
                        let mut day_hashmap = HashMap::new();
                        let day_index_1 = 0;
                        let day_index_2 = 3;
                        day_hashmap.insert(day_index_1, rustix_bl::datastore::BillUserDayInstance {
                            personally_consumed: {
                                let mut hm = HashMap::new();

                                hm.insert(0, 3);
                                hm.insert(1,19);

                                hm
                            },
                            specials_consumed: Vec::new(),
                            ffa_giveouts: HashMap::new(),
                            giveouts_to_user_id: HashMap::new(),
                        });
                        day_hashmap.insert(day_index_2, rustix_bl::datastore::BillUserDayInstance {
                            personally_consumed: {
                                let mut hm = HashMap::new();
                                hm.insert(0, 99);
                                hm
                            },
                            specials_consumed: vec![rustix_bl::datastore::PricedSpecial {
                            name: "Banana".to_string(),
                            purchase_id: 0,
                            price: 12345,
                            }],
                            ffa_giveouts: {
                                let mut hm = HashMap::new();
                                hm.insert(0, 9);
                                hm.insert(1, 1234);
                                hm
                            },
                            giveouts_to_user_id: {
                                let mut hm = HashMap::new();

                                hm.insert(1, rustix_bl::datastore::PaidFor{
                                    recipient_id: 1,
                                    count_giveouts_used: HashMap::new(),
                                    budget_given: 0,
                                    budget_gotten: 25,
                                });
                                hm.insert(2, rustix_bl::datastore::PaidFor{
                                    recipient_id: 2,
                                    count_giveouts_used: HashMap::new(),
                                    budget_given: 45,
                                    budget_gotten: 140,
                                });

                                hm
                            },
                        });
                        day_hashmap
                    }});
                    consumption_map.insert(1, rustix_bl::datastore::BillUserInstance {
                        user_id: 1,
                        per_day: HashMap::new(),
                    });
                    consumption_map.insert(2, rustix_bl::datastore::BillUserInstance {
                        user_id: 2,
                        per_day: HashMap::new(),
                    });

                    consumption_map
                },
            },
        };

        let should: Vec<Vec<String>> = vec![
            //header
            vec![
                "Mitgliedsnummer", "R vs G", "Rechnungsnr.", "Rechnungsname", "Rechnungsdatum", "Positionsnr.", "Positionsname", "Positionsbeschreibung", "Anzahl", "Preis pro Einheit", "2 == Lastschrift und 1 == Ueberweisung", "Empfang per Mail", "Zahlungsziel in Tagen", "SEPA Intervall (0 fuer einmalig)", "Datum Rechnungsstellung", "Datum Faelligkeit", "Datum Positionsende", "Mehrwertsteuersatz", "Beschreibung", " Spendenfähig", "Spende", "Buchhaltungskonto", "Steuerschluessel", "Unterkonto Kantine"
            ].iter().map(|s| s.to_string()).collect(),
        ];


        let should_lines = vec!["ExternalUserId0;2;70012420ExternalUserId0;24.01.1970;0;beer;Selbst gekauft;3;0,95;2;2;30;0;24.01.1970;07.02.1970;30.12.2069;0;KA 18.01.70-24.01.70;0;;1112;0;8293", "ExternalUserId0;2;70012420ExternalUserId0;24.01.1970;1;soda;Selbst gekauft;19;0,85;2;2;30;0;24.01.1970;07.02.1970;30.12.2069;0;KA 18.01.70-24.01.70;0;;1112;0;8293", "ExternalUserId0;2;70012420ExternalUserId0;24.01.1970;2;beer;Selbst gekauft;99;0,95;2;2;30;0;24.01.1970;07.02.1970;30.12.2069;0;KA 18.01.70-24.01.70;0;;1112;0;8293", "ExternalUserId0;2;70012420ExternalUserId0;24.01.1970;3;Banana;Speziell abgestrichen;1;123,45;2;2;30;0;24.01.1970;07.02.1970;30.12.2069;0;KA 18.01.70-24.01.70;0;;1112;0;8293", "ExternalUserId0;2;70012420ExternalUserId0;24.01.1970;4;beer;An alle ausgegeben;9;0,95;2;2;30;0;24.01.1970;07.02.1970;30.12.2069;0;KA 18.01.70-24.01.70;0;;1112;0;8293", "ExternalUserId0;2;70012420ExternalUserId0;24.01.1970;5;soda;An alle ausgegeben;1234;0,85;2;2;30;0;24.01.1970;07.02.1970;30.12.2069;0;KA 18.01.70-24.01.70;0;;1112;0;8293", "ExternalUserId0;2;70012420ExternalUserId0;24.01.1970;6;Guthaben erhalten von bob;Guthaben verbraucht: 25 Cents (intern verrechnet);1;0,2-5;2;2;30;0;24.01.1970;07.02.1970;30.12.2069;0;KA 18.01.70-24.01.70;0;;1112;0;8293", "ExternalUserId0;2;70012420ExternalUserId0;24.01.1970;7;Guthaben verschenkt an charlie;Guthaben verbraucht: 45 Cents (intern verrechnet);1;0,45;2;2;30;0;24.01.1970;07.02.1970;30.12.2069;0;KA 18.01.70-24.01.70;0;;1112;0;8293", "ExternalUserId0;2;70012420ExternalUserId0;24.01.1970;8;Guthaben erhalten von charlie;Guthaben verbraucht: 140 Cents (intern verrechnet);1;-1,40;2;2;30;0;24.01.1970;07.02.1970;30.12.2069;0;KA 18.01.70-24.01.70;0;;1112;0;8293"];


        let is_content = bill.format_as_sewobe_csv();
        let is_header = bill.sewobe_header();

        let mut is_lines: Vec<String> = is_content.iter().map(|vec| vec.join(";")).collect();


        let is_all: String = is_lines.join("\n");

        println!("Sewobe CSV Test Output:\n{}", is_all);

        println!("Lines should:\n{:?}\nvs lines is:\n{:?}", should_lines, is_lines);


        assert!(is_lines[0].contains("beer"));
        assert!(should_lines[0].contains("beer"));

        assert_eq!(is_lines.len(), 9);
        assert_eq!(should_lines.len(), 9);
        assert_eq!(should_lines, is_lines);


        for j in 0..should[0].len() - 1 {
            assert_eq!(should[0][j], is_header[j]);
        }

    }

    #[test]
    fn date_format_works() {

        let day_timestamp: chrono::DateTime<chrono::Utc> = chrono::Utc.timestamp(1523794067, 347000);
        assert_eq!("15.04.2018".to_string(), day_timestamp.format(DATE_FORMAT_STRING).to_string())
    }


    #[test]
    fn simple_general_csv_works() {

        let bill: Bill = Bill {

            timestamp_from: 1500000000,
            timestamp_to: 2000000000,
            comment: "No comment here".to_string(),
            users: UserGroup::AllUsers,
            bill_state: BillState::ExportedAtLeastOnce,
            users_that_will_not_be_billed: {
                let mut s = HashSet::new();
                s.insert(1);
                s
            },
            finalized_data: ExportableBillData {
                all_users: {
                    let mut m = HashMap::new();
                    m.insert(0, rustix_bl::datastore::User {
                        username: "alice".to_string(),
                        external_user_id: Some("ExternalUserId0".to_string()),
                        user_id: 0,
                        is_billed: true,
                        highlight_in_ui: false,
                        deleted: false,
                    });
                    m.insert(1, rustix_bl::datastore::User {
                        username: "bob".to_string(),
                        external_user_id: None,
                        user_id: 1,
                        is_billed: true,
                        highlight_in_ui: false,
                        deleted: false,
                    });
                    m.insert(2, rustix_bl::datastore::User {
                        username: "charlie".to_string(),
                        external_user_id: None,
                        user_id: 2,
                        is_billed: false,
                        highlight_in_ui: false,
                        deleted: false,
                    });
                    m
                },
                all_items: {
                    let mut m = HashMap::new();
                    m.insert(0, rustix_bl::datastore::Item {
                        name: "beer".to_string(),
                        item_id: 0,
                        category: None,
                        cost_cents: 95,
                        deleted: false,
                    });
                    m.insert(1, rustix_bl::datastore::Item {
                        name: "soda".to_string(),
                        item_id: 1,
                        category: None,
                        cost_cents: 85,
                        deleted: false,
                    });
                    m
                },
                user_consumption: {
                    let mut consumption_map = HashMap::new();

                    //user 0 has consumed both items on two separate days and been given count and budget by 0 and 1
                    //user 1 has consumed both items and given count to 0
                    //user 2 has consumed items 1 and given budget to 0


                    consumption_map.insert(0, rustix_bl::datastore::BillUserInstance {user_id:0, per_day: {
                        let mut day_hashmap = HashMap::new();
                        let day_index_1 = 0;
                        let day_index_2 = 3;
                        day_hashmap.insert(day_index_1, rustix_bl::datastore::BillUserDayInstance {
                            personally_consumed: {
                                let mut hm = HashMap::new();

                                hm.insert(0, 3);
                                hm.insert(1,19);

                                hm
                            },
                            specials_consumed: Vec::new(),
                            ffa_giveouts: HashMap::new(),
                            giveouts_to_user_id: HashMap::new(),
                        });
                        day_hashmap.insert(day_index_2, rustix_bl::datastore::BillUserDayInstance {
                            personally_consumed: {
                                let mut hm = HashMap::new();
                                hm.insert(0, 99);
                                hm
                            },
                            specials_consumed: vec![rustix_bl::datastore::PricedSpecial {
                                name: "Banana".to_string(),
                                purchase_id: 0,
                                price: 12345,
                            }],
                            ffa_giveouts: {
                                let mut hm = HashMap::new();
                                hm.insert(0, 9);
                                hm.insert(1, 1234);
                                hm
                            },
                            giveouts_to_user_id: {
                                let mut hm = HashMap::new();

                                hm.insert(1, rustix_bl::datastore::PaidFor{
                                    recipient_id: 1,
                                    count_giveouts_used: HashMap::new(),
                                    budget_given: 0,
                                    budget_gotten: 25,
                                });
                                hm.insert(2, rustix_bl::datastore::PaidFor{
                                    recipient_id: 2,
                                    count_giveouts_used: HashMap::new(),
                                    budget_given: 45,
                                    budget_gotten: 140,
                                });

                                hm
                            },
                        });
                        day_hashmap
                    }});
                    consumption_map.insert(1, rustix_bl::datastore::BillUserInstance {
                        user_id: 1,
                        per_day: HashMap::new(),
                    });
                    consumption_map.insert(2, rustix_bl::datastore::BillUserInstance {
                        user_id: 2,
                        per_day: HashMap::new(),
                    });

                    consumption_map
                },
            },
        };

        let should_header: Vec<String> = vec!["username", "user_id", "is_billed", "day", "item_name", "item_count", "item_cost_per_unit", "budget_cents_outgoing", "donor", "donor_id", "recipient", "recipient_id", "is_special", "is_giveout", "is_count", "is_budget", "is_incoming_donation", "is_ffa"].iter().map(|s|s.to_string()).collect();
        let should_lines: Vec<Vec<String>> = vec![vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "14.07.2017".to_string(), "beer".to_string(), "3".to_string(), "0,95".to_string(), "0".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "true".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "14.07.2017".to_string(), "soda".to_string(), "19".to_string(), "0,85".to_string(), "0".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "true".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "beer".to_string(), "99".to_string(), "0,95".to_string(), "0".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "true".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "Banana".to_string(), "1".to_string(), "123,45".to_string(), "0".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "beer".to_string(), "9".to_string(), "0,95".to_string(), "0".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "false".to_string(), "true".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "true".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "soda".to_string(), "1234".to_string(), "0,85".to_string(), "0".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "false".to_string(), "true".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "true".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "".to_string(), "1".to_string(), "0,2-5".to_string(), "-25".to_string(), "bob".to_string(), "".to_string(), "".to_string(), "".to_string(), "false".to_string(), "true".to_string(), "false".to_string(), "true".to_string(), "true".to_string(), "false".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "".to_string(), "1".to_string(), "0,45".to_string(), "45".to_string(), "".to_string(), "".to_string(), "charlie".to_string(), "".to_string(), "false".to_string(), "true".to_string(), "false".to_string(), "true".to_string(), "false".to_string(), "false".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "".to_string(), "1".to_string(), "-1,40".to_string(), "-140".to_string(), "charlie".to_string(), "".to_string(), "".to_string(), "".to_string(), "false".to_string(), "true".to_string(), "false".to_string(), "true".to_string(), "true".to_string(), "false".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "14.07.2017".to_string(), "beer".to_string(), "3".to_string(), "0,95".to_string(), "0".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "true".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "14.07.2017".to_string(), "soda".to_string(), "19".to_string(), "0,85".to_string(), "0".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "true".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "beer".to_string(), "99".to_string(), "0,95".to_string(), "0".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "true".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "Banana".to_string(), "1".to_string(), "123,45".to_string(), "0".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "beer".to_string(), "9".to_string(), "0,95".to_string(), "0".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "false".to_string(), "true".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "true".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "soda".to_string(), "1234".to_string(), "0,85".to_string(), "0".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "false".to_string(), "true".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "true".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "".to_string(), "1".to_string(), "0,2-5".to_string(), "-25".to_string(), "bob".to_string(), "".to_string(), "".to_string(), "".to_string(), "false".to_string(), "true".to_string(), "false".to_string(), "true".to_string(), "true".to_string(), "false".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "".to_string(), "1".to_string(), "0,45".to_string(), "45".to_string(), "".to_string(), "".to_string(), "charlie".to_string(), "".to_string(), "false".to_string(), "true".to_string(), "false".to_string(), "true".to_string(), "false".to_string(), "false".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "".to_string(), "1".to_string(), "-1,40".to_string(), "-140".to_string(), "charlie".to_string(), "".to_string(), "".to_string(), "".to_string(), "false".to_string(), "true".to_string(), "false".to_string(), "true".to_string(), "true".to_string(), "false".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "14.07.2017".to_string(), "beer".to_string(), "3".to_string(), "0,95".to_string(), "0".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "true".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "14.07.2017".to_string(), "soda".to_string(), "19".to_string(), "0,85".to_string(), "0".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "true".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "beer".to_string(), "99".to_string(), "0,95".to_string(), "0".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "true".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "Banana".to_string(), "1".to_string(), "123,45".to_string(), "0".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "false".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "beer".to_string(), "9".to_string(), "0,95".to_string(), "0".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "false".to_string(), "true".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "true".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "soda".to_string(), "1234".to_string(), "0,85".to_string(), "0".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "false".to_string(), "true".to_string(), "false".to_string(), "false".to_string(), "false".to_string(), "true".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "".to_string(), "1".to_string(), "0,2-5".to_string(), "-25".to_string(), "bob".to_string(), "".to_string(), "".to_string(), "".to_string(), "false".to_string(), "true".to_string(), "false".to_string(), "true".to_string(), "true".to_string(), "false".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "".to_string(), "1".to_string(), "0,45".to_string(), "45".to_string(), "".to_string(), "".to_string(), "charlie".to_string(), "".to_string(), "false".to_string(), "true".to_string(), "false".to_string(), "true".to_string(), "false".to_string(), "false".to_string()], vec!["alice".to_string(), "ExternalUserId0".to_string(), "true".to_string(), "17.07.2017".to_string(), "".to_string(), "1".to_string(), "-1,40".to_string(), "-140".to_string(), "charlie".to_string(), "".to_string(), "".to_string(), "".to_string(), "false".to_string(), "true".to_string(), "false".to_string(), "true".to_string(), "true".to_string(), "false".to_string()]];

        let is_header = bill.documentation_header();

        assert_eq!(should_header, is_header);


        let is_lines = bill.format_as_documentation();


        assert_eq!(should_lines, is_lines);


        for row in &is_lines {
            assert_eq!(row.len(), should_header.len());
        }
    }
}