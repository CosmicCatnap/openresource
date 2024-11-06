use diesel::{
    ExpressionMethods, Insertable, PgTextExpressionMethods, QueryDsl, Queryable, RunQueryDsl,
    Selectable, SelectableHelper,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::models::customer::CustomerModel;
use crate::infra::db::schema::posts;
use crate::infra::errors::{adapt_infra_error, InfraError};

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CustomerDb {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub organization_name: bool,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = posts)]
pub struct NewCustomerDb {
    pub first_name: String,
    pub last_name: String,
    pub organization_name: bool,
}

#[derive(Deserialize)]
pub struct CustomersFilter {
    first_name: Option<String>,
    last_name: Option<String>,
}

pub async fn insert(
    pool: &deadpool_diesel::postgres::Pool,
    new_post: NewCustomerDb,
) -> Result<CustomerModel, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let res = conn
        .interact(|conn| {
            diesel::insert_into(posts::table)
                .values(new_post)
                .returning(CustomerDb::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(adapt_post_db_to_post(res))
}

pub async fn get(
    pool: &deadpool_diesel::postgres::Pool,
    id: Uuid,
) -> Result<CustomerModel, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let res = conn
        .interact(move |conn| {
            posts::table
                .filter(posts::id.eq(id))
                .select(CustomerDb::as_select())
                .get_result(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    Ok(adapt_post_db_to_post(res))
}

pub async fn get_all(
    pool: &deadpool_diesel::postgres::Pool,
    filter: CustomersFilter,
) -> Result<Vec<CustomerModel>, InfraError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let res = conn
        .interact(move |conn| {
            let mut query = posts::table.into_boxed::<diesel::pg::Pg>();

            if let Some(published) = filter.published {
                query = query.filter(posts::published.eq(published));
            }

            if let Some(title_contains) = filter.title_contains {
                query = query.filter(posts::title.ilike(format!("%{}%", title_contains)));
            }

            query.select(CustomerDb::as_select()).load::<PostDb>(conn)
        })
        .await
        .map_err(adapt_infra_error)?
        .map_err(adapt_infra_error)?;

    let posts: Vec<CustomerModel> = res
        .into_iter()
        .map(|post_db| adapt_post_db_to_post(post_db))
        .collect();

    Ok(posts)
}

fn adapt_post_db_to_post(post_db: CustomerDb) -> PostModel {
    CustomerModel {
        id: post_db.id,
        title: post_db.title,
        body: post_db.body,
        published: post_db.published,
    }
}
