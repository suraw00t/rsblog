#![allow(unused)]
use serde::Deserialize;
use utoipa::IntoParams;

pub mod tokens;
pub mod users;

#[derive(Deserialize, IntoParams)]
pub struct PaginationParams {
    page: Option<usize>,
    size: Option<usize>,
}

impl PaginationParams {
    pub fn get_page(&self) -> usize {
        self.page.unwrap_or(0)
    }

    pub fn get_size(&self) -> usize {
        self.size.unwrap_or(50)
    }
}

#[derive(Deserialize, IntoParams)]
pub struct BindingParams {
    pub size: Option<usize>,
}

impl BindingParams {
    pub fn get_size(&self) -> usize {
        self.size.unwrap_or(50)
    }
}
