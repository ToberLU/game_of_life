mod grid;
mod render;
mod simulation;

fn main() {
    tracing_subscriber::fmt::init();
    let mut renderer = render::RenderContext::new();
    renderer.run();
}
