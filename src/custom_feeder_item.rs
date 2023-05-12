use chrono::DateTime;
use rss::Item;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct CustomFeederItem {
    pub channel_owner: String,
    pub item: Item,
}

impl PartialOrd for CustomFeederItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = DateTime::parse_from_rfc2822(self.item.pub_date.as_ref().unwrap()).unwrap();
        let o = DateTime::parse_from_rfc2822(other.item.pub_date.as_ref().unwrap()).unwrap();

        Some(a.cmp(&o).reverse())
    }
}

impl Ord for CustomFeederItem {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = DateTime::parse_from_rfc2822(self.item.pub_date.as_ref().unwrap()).unwrap();
        let o = DateTime::parse_from_rfc2822(other.item.pub_date.as_ref().unwrap()).unwrap();
        a.cmp(&o).reverse()
    }
}

impl PartialEq<Self> for CustomFeederItem {
    fn eq(&self, other: &Self) -> bool {
        self.item
            .guid
            .as_ref()
            .unwrap()
            .value
            .eq(&other.item.guid.as_ref().unwrap().value)
    }
}

impl Eq for CustomFeederItem {}

impl CustomFeederItem {
    pub fn get_pub_date(&self) -> String {
        let date = self.item.pub_date.as_ref().unwrap();
        DateTime::parse_from_rfc2822(date)
            .unwrap_or_default()
            .format("%d-%m-%Y %H:%M")
            .to_string()
    }

    pub fn get_guid(&self) -> &str {
        self.item.guid.as_ref().unwrap().value.as_str()
    }
}
