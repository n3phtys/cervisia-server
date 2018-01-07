
use rustix_bl::datastore::Bill;
use rustix_bl;

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
    fn format_as_sewobe_csv(&self, conf: &SewobeConfiguration) -> String;

    //outputs reduced bill string for one specific person
    fn format_as_personalized_documentation(&self, user_id: u32) -> String;

    //outputs bill for everyone in the bill (ordered alphabetically by name)
    fn format_as_documentation(&self) -> String {
        self.list_of_user_ids().iter().map(|id| self.format_as_personalized_documentation(*id)).fold("".to_string(), |acc, b| acc + &b)
    }

    fn list_of_user_ids(&self) -> Vec<u32>;
    fn includes_user(&self, user_id: u32) -> bool {
        return self.list_of_user_ids().contains(&user_id);
    }
}



impl BillFormatting for Bill {
    fn format_as_sewobe_csv(&self, conf: &SewobeConfiguration) -> String {
        unimplemented!()
    }

    fn format_as_personalized_documentation(&self, user_id: u32) -> String {
        //TODO: define format
        let mut lines: Vec<String> = Vec::new();

        //add header later on, only data for now
        let userdata: &rustix_bl::datastore::BillUserInstance = self.finalized_data.user_consumption.get(&user_id).unwrap();


        for (dayindex, daydata) in &userdata.per_day {
            //personally_consumed
            //specials_consumed
            //ffa_giveouts
            //giveouts_to_user_id
        }

        return lines.iter().fold("".to_string(), |acc, b| acc + &b);
    }

    fn list_of_user_ids(&self) -> Vec<u32> {
        let mut v: Vec<u32> = Vec::new();
        for (key, value) in &self.finalized_data.all_users {
            v.push(*key);
        }
        return v;
    }
}