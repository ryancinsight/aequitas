use tracing::subscriber::DefaultGuard;
use tracing_subscriber::fmt::format::FmtSpan;

pub fn trace_phases() -> DefaultGuard {
    tracing::subscriber::set_default(
        tracing_subscriber::fmt()
            .with_span_events(FmtSpan::FULL)
            .with_test_writer()
            .finish(),
    )
}
