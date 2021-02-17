use serde::{Deserialize, Serialize};
use sqlx;

/// @TODO use Lang enum in model structs once support arrives in sqlx crate
/// https://github.com/launchbadge/sqlx/issues/313
#[derive(sqlx::Type, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[sqlx(rename = "VARCHAR")]
#[sqlx(rename_all = "lowercase")]
pub enum Lang {
    En,
    Tr,
}
