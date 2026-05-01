use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{Layer, time::LocalTime},
    layer::SubscriberExt,
};

use crate::simulation::Simulation;

mod grid;
mod render;
mod simulation;

fn init_subscriber() -> WorkerGuard {
    let file = tracing_appender::rolling::daily("./logs", "log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file);

    let console = Layer::new()
        .with_writer(std::io::stdout)
        .with_timer(LocalTime::rfc_3339())
        .pretty();

    let inspector = Layer::new()
        .with_writer(non_blocking)
        .with_timer(LocalTime::rfc_3339())
        .json();

    let subscriber = tracing_subscriber::registry().with(console).with(inspector);

    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global collector");

    guard
}

fn main() {
    let _guard = init_subscriber();
    tracing::info!("Tracing subscriber initialized.");

    let mut simulation = Simulation::new(50, 50);

    let mut renderer = render::RenderContext::new();
    renderer.run(&mut simulation);

    tracing::info!("Program end.");
}
