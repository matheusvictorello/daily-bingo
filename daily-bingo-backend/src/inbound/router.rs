use axum::Router;
use axum::middleware::from_fn_with_state;
use tower_http::trace::TraceLayer;

use super::AppState;
use super::middleware;
use super::route::auth;
use super::route::bingo;

pub fn router<S: AppState>(app_state: S) -> Router {
    let public = Router::new().nest("/", auth::router());

    let protected = Router::new()
        .nest("/bingos", bingo::router())
        .route_layer(
            from_fn_with_state(
                app_state.clone(),
                middleware::auth::authorization::<S>,
            ),
        );

    public
        .merge(protected)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}
