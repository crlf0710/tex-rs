fn main() {
    #[cfg(feature = "trace")]
    {
        let subscriber_builder = tracing_subscriber::FmtSubscriber::builder();
        #[cfg(feature = "debugging")]
        let subscriber_builder = subscriber_builder
            .with_max_level(tracing::Level::TRACE)
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE);
        #[cfg(not(feature = "debugging"))]
        let subscriber_builder = subscriber_builder.with_max_level(tracing::Level::TRACE);
        subscriber_builder.init();
    }
    let mut globals = tex::TeXGlobals::default();
    tex::entry(&mut globals);
}
