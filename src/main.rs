use anyhow::Result;

struct ResponseError(anyhow::Error);

impl From<anyhow::Error> for ResponseError {
    fn from(error: anyhow::Error) -> Self {
        Self(error)
    }
}

impl axum::response::IntoResponse for ResponseError {
    fn into_response(self) -> axum::response::Response {
        let status_code = if let Some(error) = self.0.downcast_ref::<ApiError>() {
            match error {
                ApiError::NotFound => axum::http::StatusCode::NOT_FOUND,
                ApiError::BadRequest => axum::http::StatusCode::BAD_REQUEST,
            }
        } else {
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        };
        (status_code, self.0.to_string()).into_response()
    }
}

type ResultResponse<T> = std::result::Result<T, ResponseError>;

fn might_fail() -> Result<()> {
    Ok(())
}

#[derive(thiserror::Error, Debug)]
enum ApiError {
    #[error("not found")]
    NotFound,
    #[error("bad request")]
    BadRequest,
}

fn might_fail_2() -> Result<(), ApiError> {
    Ok(())
}

async fn get_foobar() -> ResultResponse<()> {
    might_fail()?;
    might_fail_2().map_err(Into::<anyhow::Error>::into)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = axum::Router::new().route("/foobar", axum::routing::get(get_foobar));

    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
