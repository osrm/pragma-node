use super::DieselResult;
use crate::schema::currencies;
use bigdecimal::BigDecimal;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, ToSchema)]
pub struct Currency {
    pub id: Uuid,
    pub name: String,
    pub decimals: BigDecimal,
    pub is_abstract: bool,
    pub ethereum_address: Option<String>,
}

impl Currency {
    pub fn get_all(conn: &mut PgConnection) -> DieselResult<Vec<String>> {
        currencies::table.select(currencies::name).get_results(conn)
    }

    pub fn get_abstract_all(conn: &mut PgConnection) -> DieselResult<Vec<String>> {
        currencies::table
            .select(currencies::name)
            .filter(currencies::abstract_.eq(true))
            .get_results(conn)
    }

    pub fn get_decimals_all(conn: &mut PgConnection) -> DieselResult<Vec<(String, BigDecimal)>> {
        currencies::table
            .select((currencies::name, currencies::decimals))
            .get_results::<(String, BigDecimal)>(conn)
    }
}
