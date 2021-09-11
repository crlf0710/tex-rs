fn main() {
    #[cfg(feature = "trace")]
    {
        use tracing_subscriber::{layer::SubscriberExt, Registry};
        use tracing_tree::HierarchicalLayer;
        let env = "TEX_LOG";
        match std::env::var(env) {
            Ok(s) if !s.is_empty() => {
                let filter = tracing_subscriber::EnvFilter::from_env(env);
                let layer = HierarchicalLayer::default()
                    .with_indent_lines(true)
                    .with_indent_amount(2)
                    .with_bracketed_fields(true)
                    .with_writer(std::io::stderr);

                let subscriber = Registry::default().with(filter).with(layer);
                tracing::subscriber::set_global_default(subscriber).unwrap();
            }
            _ => {}
        }
    }

    let mut globals: Box<tex::TeXGlobals> = Default::default();
    tex::entry(&mut globals);
    drop(globals);
}
