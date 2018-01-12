
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



impl BillFormatting for Bill {
    fn format_as_sewobe_csv(&self, conf: &SewobeConfiguration) -> Vec<Vec<String>> {
        unimplemented!()
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
        return raw_header.split(";").map(|s|s.to_string()).collect();
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

    #[test]
    fn simple_sewobe_csv_works() {

        let bill: Bill = unimplemented!();
        let conf: SewobeConfiguration = unimplemented!();

        let should = vec![
            //header
            vec![
                "Mitgliedsnummer","R vs G","Rechnungsnr.","Rechnungsname","Rechnungsdatum","Positionsnr.","Positionsname","Positionsbeschreibung","Anzahl","Preis pro Einheit","2 == Lastschrift und 1 == Ueberweisung","Empfang per Mail","Zahlungsziel in Tagen","SEPA Intervall (0 fuer einmalig)","Datum Rechnungsstellung","Datum Faelligkeit","Datum Positionsende","Mehrwertsteuersatz","Beschreibung"," Spendenfähig","Spende","Buchhaltungskonto","Steuerschluessel","Unterkonto Kantine"
            ].iter().map(|s|s.to_string()).collect(),
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

        for j in 0..should[0].len()-1 {
            assert_eq!(should[0][j], is_header[j]);
        }

        for i in 1..should.len()-1 {
            for j in 0..should[i].len()-1 {
                assert_eq!(should[i][j], is_content[i-1][j])
            }
        }
    }

    #[test]
    fn simple_general_csv_works() {
        assert!(1 + 1 == 4);
    }

}