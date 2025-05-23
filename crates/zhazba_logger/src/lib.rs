use tracing::dispatcher::set_global_default;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_error::ErrorLayer;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt};


pub fn init_logger() {
  let file_appender = RollingFileAppender::new(
    if env!("ENV") == "DEBUG" {
      Rotation::NEVER
    } else {
      Rotation::DAILY
    },
    if env!("ENV") == "DEBUG" {
      "./logs"
    } else {
      todo!()
    },
    "zhazba.log",
  );
  let fmt_layer = fmt::Layer::new()
    .with_writer(file_appender)
    .with_ansi(false)
    .with_line_number(true);
  let filter = EnvFilter::from_default_env();
  let error_layer = ErrorLayer::default();

  let subscriber = tracing_subscriber::registry()
    .with(filter)
    .with(error_layer)
    .with(fmt_layer);
  set_global_default(subscriber.into())
    .expect("setting tracing default failed");
}
