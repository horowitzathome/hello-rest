use axum::{routing, Router};
use axum_prometheus::{metrics_exporter_prometheus::PrometheusHandle, PrometheusMetricLayer};
use dotenv::dotenv;
use tower_http::cors::{Any, CorsLayer};
use tracing::*;

mod listen;

pub type Result<T, E = anyhow::Error> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, db!");

    // Tracing!
    register_tracing();

    // Now trace the started message
    info!("Started Hello DB");

    // Metrics
    let (prometheus_layer, metric_handle) = define_metrics();

    // Prepare environment and initialize DB
    dotenv().ok();

    // Do Route stuff
    let router = create_router_with_prometheus(prometheus_layer, metric_handle);

    // Start Web Server at port 8080
    use tokio::signal::unix as usig;
    let mut shutdown = usig::signal(usig::SignalKind::terminate())?;
    let server = axum::Server::bind(&std::net::SocketAddr::from(([0, 0, 0, 0], 8080)))
        .serve(router.into_make_service())
        .with_graceful_shutdown(async move {
            shutdown.recv().await;
        });

    // Wait for either Watcher or WebServer to exit and write log hwo exited (the Watcher should never exit by its own)
    tokio::select! {
        _ = server => info!("axum server exited"),
    }

    // Finish
    Ok(())
}

fn register_tracing() {
    tracing_subscriber::fmt()
        .json()
        .flatten_event(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_level(true)
        .with_max_level(Level::INFO)
        .init();
}

fn define_metrics() -> (PrometheusMetricLayer<'static>, PrometheusHandle) {
    axum_prometheus::PrometheusMetricLayer::pair()
}

fn create_router_with_prometheus(prometheus_layer: PrometheusMetricLayer<'static>, metric_handle: PrometheusHandle) -> Router {
    let routers = create_router()
        .route("/actuator/prometheus", routing::get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer);

    routers
    //Router::new().nest("/hello", routers)
}

fn create_router() -> Router {
    let cors = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    Router::new()
        // Here the business routes later
        .route("/", routing::get(listen::root_root))
        .route("/hello", routing::get(listen::root_hello))
        .route("/hello/dog", routing::get(listen::get_dog))
        .route("/dog", routing::get(listen::get_dog_root))
        //.layer(Extension(reader_deployment))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        // Reminder: routes added *after* TraceLayer are not subject to its logging behavior
        .route("/actuator/health", routing::get(listen::health_root))
        .route("/hello/actuator/health", routing::get(listen::health))
        .layer(cors)
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use dotenv::dotenv;

    #[ctor::ctor]
    fn init() {
        println!("We are in init to initialize environment and database");
        dotenv().ok();
    }

    #[test]
    fn test_env() {
        //dotenv().ok();
        let pg_res = std::env::var("DATABASE_URL");

        println!("Postgres URL = {:?}", pg_res);

        let now: DateTime<Utc> = Utc::now();

        println!("UTC now is: {}", now);
        println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
        println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    }
}
