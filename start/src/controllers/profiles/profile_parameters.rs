#![allow(unused)]
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewProfileRouteParam {
    user_name: String,
    full_name: String,
    description: String,
    region: String,
    main_url: String,
    avatar: Vec<u8>
}