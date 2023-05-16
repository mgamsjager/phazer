use chrono::{DateTime, Utc};
use rss::Item;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct CustomFeederItem {
    pub channel_owner: String,
    pub item: Item,
}

impl PartialOrd for CustomFeederItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a_date = self
            .item
            .pub_date
            .as_ref()
            .unwrap_or(&Utc::now().to_rfc2822())
            .clone();
        let b_date = other
            .item
            .pub_date
            .as_ref()
            .unwrap_or(&Utc::now().to_rfc2822())
            .clone();
        let a = DateTime::parse_from_rfc2822(a_date.as_str()).unwrap();
        let o = DateTime::parse_from_rfc2822(b_date.as_str()).unwrap();

        Some(a.cmp(&o).reverse())
    }
}

impl Ord for CustomFeederItem {
    fn cmp(&self, other: &Self) -> Ordering {
        let a_date = self
            .item
            .pub_date
            .as_ref()
            .unwrap_or(&Utc::now().to_rfc2822())
            .clone();
        let b_date = other
            .item
            .pub_date
            .as_ref()
            .unwrap_or(&Utc::now().to_rfc2822())
            .clone();
        let a = DateTime::parse_from_rfc2822(a_date.as_str()).unwrap();
        let o = DateTime::parse_from_rfc2822(b_date.as_str()).unwrap();
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
        let date = self
            .item
            .pub_date
            .as_ref()
            .unwrap_or(&Utc::now().to_rfc2822())
            .clone();
        DateTime::parse_from_rfc2822(date.as_str())
            .unwrap_or_default()
            .format("%d-%m-%Y %H:%M")
            .to_string()
    }

    pub fn get_guid(&self) -> &str {
        self.item.guid.as_ref().unwrap().value.as_str()
    }
}
