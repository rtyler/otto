/*
 * The relational data service largely is meant to expose information from an underlying database
 */
use dotenv::dotenv;
use log::*;
use tide::Request;

use async_graphql::dataloader::{DataLoader, Loader};
use async_graphql::futures_util::TryStreamExt;
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{
    Context, EmptyMutation, EmptySubscription, FieldError, Object, Result, Schema, SimpleObject,
};
use async_trait::async_trait;
use sqlx::{Pool, SqlitePool};
use std::collections::HashMap;
use tide::{http::mime, Body, Response, StatusCode};
use uuid::Uuid;

/**
 * QueryState is a simple struct to pass data through to async-graphql implementations
 */
struct QueryState {
    pool: SqlitePool,
    data_loader: DataLoader<ProjectLoader>,
}

/**
 * Simple/empty healthcheck endpoint which can be used to determine whether the webservice is at
 * least minimally functional
 */
async fn healthcheck(_req: Request<()>) -> tide::Result {
    Ok(tide::Response::builder(200)
        .body("{}")
        .content_type("application/json")
        .build())
}

/**
 * Main web service set up
 */
#[async_std::main]
async fn main() -> async_graphql::Result<()> {
    use std::{env, net::TcpListener, os::unix::io::FromRawFd};
    pretty_env_logger::init();
    dotenv().ok();
    let pool: SqlitePool = Pool::connect(&env::var("DATABASE_URL")?).await?;
    debug!("Connecting to: {}", env::var("DATABASE_URL")?);

    let qs = QueryState {
        pool: pool.clone(),
        data_loader: DataLoader::new(ProjectLoader::new(pool.clone())),
    };

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(qs)
        .finish();

    let mut app = tide::new();
    app.at("/health").get(healthcheck);
    app.at("/graphql")
        .post(async_graphql_tide::endpoint(schema));
    app.at("/").get(|_| async move {
        let mut resp = Response::new(StatusCode::Ok);
        resp.set_body(Body::from_string(playground_source(
            GraphQLPlaygroundConfig::new("/graphql"),
        )));
        resp.set_content_type(mime::HTML);
        Ok(resp)
    });

    if let Some(fd) = env::var("LISTEN_FD").ok().and_then(|fd| fd.parse().ok()) {
        app.listen(unsafe { TcpListener::from_raw_fd(fd) }).await?;
    } else {
        app.listen("http://localhost:7674").await?;
    }
    Ok(())
}

#[derive(sqlx::FromRow, Clone, Debug, SimpleObject)]
pub struct Project {
    uuid: Uuid,
    path: String,
    title: String,
}

pub struct ProjectLoader(SqlitePool);
impl ProjectLoader {
    fn new(pool: SqlitePool) -> Self {
        Self(pool)
    }
}

#[async_trait]
impl Loader<Uuid> for ProjectLoader {
    type Value = Project;
    type Error = FieldError;

    async fn load(&self, keys: &[Uuid]) -> Result<HashMap<Uuid, Self::Value>, Self::Error> {
        use std::str::FromStr;
        use sqlx::Row;

        let uuids = keys.iter().map(|u| u.to_string()).collect::<Vec<String>>();
        // Doing awful things to allow the bulk query with the Uuid since sqlx cannot map a
        // hyphenated string back out to a normal Uuid
        let mut records = sqlx::query("SELECT * FROM projects WHERE uuid IN ($1)")
                .bind(uuids.join(","))
                .fetch(&self.0);

        let mut out = HashMap::new();
        while let Some(row) = records.try_next().await? {
            let project = Project {
                uuid: Uuid::from_str(row.get("uuid"))?,
                title: row.get("title"),
                path: row.get("path"),
            };
            out.insert(project.uuid, project);
        }

        Ok(out)
    }
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn project(&self, ctx: &Context<'_>, id: Uuid) -> Result<Option<Project>> {
        Ok(ctx
            .data_unchecked::<QueryState>()
            .data_loader
            .load_one(id)
            .await?)
    }
}

#[derive(Clone)]
struct AppState {
    schema: Schema<QueryRoot, EmptyMutation, EmptySubscription>,
}

#[cfg(test)]
mod tests {}
