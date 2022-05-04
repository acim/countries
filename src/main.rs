use anyhow::Result;
use prettytable::{cell, row, Table};
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();

    // let countries = client
    //     .get("https://datahub.io/core/country-codes/r/country-codes.json")
    //     .send()
    //     .await?
    //     .text()
    //     .await?;

    // let data = serde_json::from_str::<Root>(&countries)?;

    // data.iter().for_each(|c| {
    //     println!(
    //         "Name: {:?} Display name: {:?} Continent: {:?}",
    //         c.official_name_en, c.intermediate_region_name, c.region_name
    //     )
    // });

    let html = client
        .get("https://unstats.un.org/unsd/methodology/m49/overview")
        .send()
        .await?
        .text()
        .await?;

    let mut table = Table::new();
    table.add_row(row![
        "M49 Code",
        "ISO-alpha2 Code",
        "Country or Area",
        "Intermediate Region Name",
        "Region Code",
        "Region Name"
    ]);

    let document = Html::parse_document(&html);
    let tr_selector = Selector::parse("table#downloadTableEN tbody tr").unwrap();
    for tr in document.select(&tr_selector) {
        let td_selector = Selector::parse("td").unwrap();
        let tds: Vec<String> = tr.select(&td_selector).map(|x| x.inner_html()).collect();
        table.add_row(row![tds[9], tds[10], tds[8], tds[7], tds[2], tds[3]]);
    }

    table.printstd();

    Ok(())
}

pub type Root = Vec<Country>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Country {
    #[serde(rename = "CLDR display name")]
    pub cldr_display_name: Option<String>,
    pub capital: Option<String>,
    pub continent: Option<String>,
    #[serde(rename = "DS")]
    pub ds: Option<String>,
    #[serde(rename = "Developed / Developing Countries")]
    pub developed_developing_countries: Option<String>,
    pub dial: Option<String>,
    #[serde(rename = "EDGAR")]
    pub edgar: Option<String>,
    #[serde(rename = "FIFA")]
    pub fifa: Option<String>,
    #[serde(rename = "FIPS")]
    pub fips: Option<String>,
    #[serde(rename = "GAUL")]
    pub gaul: Option<String>,
    #[serde(rename = "Geoname ID")]
    pub geoname_id: Option<f64>,
    #[serde(rename = "Global Code")]
    pub global_code: Option<String>,
    #[serde(rename = "Global Name")]
    pub global_name: Option<String>,
    #[serde(rename = "IOC")]
    pub ioc: Option<String>,
    #[serde(rename = "ISO3166-1-Alpha-2")]
    pub iso3166_1_alpha_2: Option<String>,
    #[serde(rename = "ISO3166-1-Alpha-3")]
    pub iso3166_1_alpha_3: Option<String>,
    #[serde(rename = "ISO3166-1-numeric")]
    pub iso3166_1_numeric: Option<String>,
    #[serde(rename = "ISO4217-currency_alphabetic_code")]
    pub iso4217_currency_alphabetic_code: Option<String>,
    #[serde(rename = "ISO4217-currency_country_name")]
    pub iso4217_currency_country_name: Option<String>,
    #[serde(rename = "ISO4217-currency_minor_unit")]
    pub iso4217_currency_minor_unit: Option<String>,
    #[serde(rename = "ISO4217-currency_name")]
    pub iso4217_currency_name: Option<String>,
    #[serde(rename = "ISO4217-currency_numeric_code")]
    pub iso4217_currency_numeric_code: Option<String>,
    #[serde(rename = "ITU")]
    pub itu: Option<String>,
    #[serde(rename = "Intermediate Region Code")]
    pub intermediate_region_code: Option<String>,
    #[serde(rename = "Intermediate Region Name")]
    pub intermediate_region_name: Option<String>,
    #[serde(rename = "Land Locked Developing Countries (LLDC)")]
    pub land_locked_developing_countries_lldc: Option<String>,
    pub languages: Option<String>,
    #[serde(rename = "Least Developed Countries (LDC)")]
    pub least_developed_countries_ldc: Option<String>,
    pub m49: Option<f64>,
    #[serde(rename = "MARC")]
    pub marc: Option<String>,
    #[serde(rename = "Region Code")]
    pub region_code: Option<String>,
    #[serde(rename = "Region Name")]
    pub region_name: Option<String>,
    #[serde(rename = "Small Island Developing States (SIDS)")]
    pub small_island_developing_states_sids: Option<String>,
    #[serde(rename = "Sub-region Code")]
    pub sub_region_code: Option<String>,
    #[serde(rename = "Sub-region Name")]
    pub sub_region_name: Option<String>,
    #[serde(rename = "TLD")]
    pub tld: Option<String>,
    #[serde(rename = "UNTERM Arabic Formal")]
    pub unterm_arabic_formal: Option<String>,
    #[serde(rename = "UNTERM Arabic Short")]
    pub unterm_arabic_short: Option<String>,
    #[serde(rename = "UNTERM Chinese Formal")]
    pub unterm_chinese_formal: Option<String>,
    #[serde(rename = "UNTERM Chinese Short")]
    pub unterm_chinese_short: Option<String>,
    #[serde(rename = "UNTERM English Formal")]
    pub unterm_english_formal: Option<String>,
    #[serde(rename = "UNTERM English Short")]
    pub unterm_english_short: Option<String>,
    #[serde(rename = "UNTERM French Formal")]
    pub unterm_french_formal: Option<String>,
    #[serde(rename = "UNTERM French Short")]
    pub unterm_french_short: Option<String>,
    #[serde(rename = "UNTERM Russian Formal")]
    pub unterm_russian_formal: Option<String>,
    #[serde(rename = "UNTERM Russian Short")]
    pub unterm_russian_short: Option<String>,
    #[serde(rename = "UNTERM Spanish Formal")]
    pub unterm_spanish_formal: Option<String>,
    #[serde(rename = "UNTERM Spanish Short")]
    pub unterm_spanish_short: Option<String>,
    #[serde(rename = "WMO")]
    pub wmo: Option<String>,
    #[serde(rename = "is_independent")]
    pub is_independent: Option<String>,
    #[serde(rename = "official_name_ar")]
    pub official_name_ar: Option<String>,
    #[serde(rename = "official_name_cn")]
    pub official_name_cn: Option<String>,
    #[serde(rename = "official_name_en")]
    pub official_name_en: Option<String>,
    #[serde(rename = "official_name_es")]
    pub official_name_es: Option<String>,
    #[serde(rename = "official_name_fr")]
    pub official_name_fr: Option<String>,
    #[serde(rename = "official_name_ru")]
    pub official_name_ru: Option<String>,
}
