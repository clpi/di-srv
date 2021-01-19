use tokio::io::AsyncWriteExt;
use crate::{models::Response, state::State, handlers::user::*};
use futures::{Stream, StreamExt, TryStreamExt};
use actix_web::error::ParseError;
use actix_multipart::Multipart;
use actix_web::{Error, web::{self, scope},
    HttpRequest, HttpResponse,
};

pub fn routes(base: &str) -> actix_web::Scope {
    scope(base)
}

pub async fn upload_user_img(
    data: web::Data<State>,
    path: web::Path<String>,
    mut pl: Multipart
    ) -> Result<HttpResponse, Error>
{
    while let Ok(Some(mut field)) = pl.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| ParseError::Incomplete)?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| ParseError::Incomplete)?;
        let filepath = format!("./tmp/{}", &filename);
        let mut f = tokio::fs::File::create(filepath).await?;
        while let Some(c) = field.next().await {
            f.write_all(&c.unwrap()).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}
