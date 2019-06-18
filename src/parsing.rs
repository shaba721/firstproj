

use reqwest::Client;
use std::time::{Duration, Instant};
use std::result;
use serde::{Deserialize, Serialize};

use reqwest::StatusCode;
use regex::RegexSet;

use serde_json::{Value, Result, Number, Map};
use serde::de::Unexpected::StructVariant;
use std::collections::HashMap;
use regex::Regex;
use std::io::SeekFrom::Start;
use std::str::FromStr;
use std::panic::resume_unwind;
use std::path::Prefix::Verbatim;
use core::panicking::panic_fmt;


#[derive(Serialize, Deserialize)]
pub struct Stock {
    #[serde(rename = "unique_image_url_prefixes")]
    unique_image_url_prefixes: Vec<Option<serde_json::Value>>,

    #[serde(rename = "products_and_categories")]
    products_and_categories: HashMap<String, Vec<ProductsAndCategory>>,

    #[serde(rename = "last_mobile_api_update")]
    last_mobile_api_update: String,

    #[serde(rename = "release_date")]
    release_date: String,

    #[serde(rename = "release_week")]
    release_week: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProductsAndCategory {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "image_url")]
    image_url: String,

    #[serde(rename = "image_url_hi")]
    image_url_hi: String,

    #[serde(rename = "price")]
    price: i64,

    #[serde(rename = "sale_price")]
    sale_price: i64,

    #[serde(rename = "new_item")]
    new_item: bool,

    #[serde(rename = "position")]
    position: i64,

    #[serde(rename = "category_name")]
    category_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TopLevel {
    #[serde(rename = "styles")]
    styles: Vec<Style>,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "can_add_styles")]
    can_add_styles: bool,

    #[serde(rename = "can_buy_multiple")]
    can_buy_multiple: bool,

    #[serde(rename = "ino")]
    ino: String,

    #[serde(rename = "cod_blocked")]
    cod_blocked: bool,

    #[serde(rename = "canada_blocked")]
    canada_blocked: bool,

    #[serde(rename = "purchasable_qty")]
    purchasable_qty: i64,

    #[serde(rename = "new_item")]
    new_item: bool,

    #[serde(rename = "apparel")]
    apparel: bool,

    #[serde(rename = "handling")]
    handling: i64,

    #[serde(rename = "no_free_shipping")]
    no_free_shipping: bool,

    #[serde(rename = "can_buy_multiple_with_limit")]
    can_buy_multiple_with_limit: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Style {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "currency")]
    currency: String,

    #[serde(rename = "description")]
    description: Option<serde_json::Value>,

    #[serde(rename = "image_url")]
    image_url: String,

    #[serde(rename = "image_url_hi")]
    image_url_hi: String,

    #[serde(rename = "swatch_url")]
    swatch_url: String,

    #[serde(rename = "swatch_url_hi")]
    swatch_url_hi: String,

    #[serde(rename = "mobile_zoomed_url")]
    mobile_zoomed_url: String,

    #[serde(rename = "mobile_zoomed_url_hi")]
    mobile_zoomed_url_hi: String,

    #[serde(rename = "bigger_zoomed_url")]
    bigger_zoomed_url: String,

    #[serde(rename = "sizes")]
    sizes: Vec<Size>,

    #[serde(rename = "additional")]
    additional: Vec<Additional>,
}

#[derive(Serialize, Deserialize)]
pub struct Additional {
    #[serde(rename = "swatch_url")]
    swatch_url: String,

    #[serde(rename = "swatch_url_hi")]
    swatch_url_hi: String,

    #[serde(rename = "image_url")]
    image_url: String,

    #[serde(rename = "image_url_hi")]
    image_url_hi: String,

    #[serde(rename = "zoomed_url")]
    zoomed_url: String,

    #[serde(rename = "zoomed_url_hi")]
    zoomed_url_hi: String,

    #[serde(rename = "bigger_zoomed_url")]
    bigger_zoomed_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Size {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "stock_level")]
    stock_level: i64,
}

fn getting_json(){
    let requesttime = Instant::now();

    let client = Client::new();
    let resp = client.get("https://www.supremenewyork.com/shop.json")
        .send().expect("couldnt get it")
        .text().expect("failed");
    let duration = requesttime.elapsed();
    println!("It took {}ms to request the Supreme shop endpoint ", duration.as_millis());
    find_product(resp);



}

pub struct productid{
    pub productid: i64,
    pub stid: i64,
    pub sid: i64
}



pub fn find_product(storejson: String) {

    let parser: Stock = serde_json::from_str(&storejson)
        .expect("failed to parse json");

    for (category, product) in parser.products_and_categories.iter(){

        for Products in product{
            if Products.name.contains("pendant"){
                let productid = Products.id;




                }

            }
        }

    panic!("failed to find product");

}

fn productjson(productid: i64){
    let client = Client::new();

    let url = format!("https://www.supremenewyork.com/shop/{}.json", productid);
    let resp = client.get(&url)
        .send().expect("failed to find endpoint")
        .text().expect("failed to get text");
    findst(&resp.to_lowercase());
    findsizes(&resp);

}




fn findsizes(productjson: &String) {
    let product = productjson.to_lowercase();
    let parser: TopLevel = serde_json::from_str(&product).expect("dont fail");

    for product in parser.styles.iter() {
        for size in &product.sizes {
            if size.name.contains("n/a") {
                let style = product.id;



            }
        }
    }
    panic!("failed");
}



pub fn findst(productjson: &String) {

    let parser: TopLevel  = serde_json::from_str(&productjson).expect("failed st");
    for product in parser.styles{
        if product.name.contains("gold"){
            let style = product.id;



        }
    }
    panic!("failed");
}






