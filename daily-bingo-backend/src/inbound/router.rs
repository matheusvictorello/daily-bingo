use axum::Router;
use axum::middleware::from_fn_with_state;
use tower_http::trace::TraceLayer;

use super::AppState;
use super::middleware;
use super::route::auth;

pub fn router<S: AppState>(app_state: S) -> Router {
    let public = Router::new().nest("/", auth::router());

    // let protected = Router::new()
    //     .nest("/assets", asset::router())
    //     .nest("/permissions", permission::router())
    //     .nest("/wallet", wallet::router())
    //     .route_layer(
    //         from_fn_with_state(
    //             app_state.clone(),
    //             middleware::auth::authorization::<S>,
    //         ),
    //     )
    //     // .route_layer(
    //     //     middleware::from_fn_with_state(
    //     //         state.clone(),
    //     //         permissions::permissions_middleware,
    //     //     )
    //     // )
    //     ;

    public
        // .merge(protected)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}
