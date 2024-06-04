use std::convert::Infallible;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct ResponseData {
    pub status: String,
}

pub async fn it_compiles() -> Result<HttpResponse, Infallible> {
    Ok(HttpResponse::Ok().json(ResponseData {
        status: "ok".into(),
    }))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        it_compiles().await.unwrap();
    }
}
