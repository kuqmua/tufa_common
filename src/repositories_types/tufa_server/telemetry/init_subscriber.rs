pub fn init_subscriber<'a>(
    subscriber: impl tracing::Subscriber + Send + Sync,
) -> Result<(), crate::repositories_types::tufa_server::telemetry::init_subcriber_error_enum::InitSubcriberErrorEnum<'a>>{
    if let Err(e) = tracing_log::LogTracer::init() {
        return Err(crate::repositories_types::tufa_server::telemetry::init_subcriber_error_enum::InitSubcriberErrorEnum::SetLogger {
            error: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        });
    }
    if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
        return Err(crate::repositories_types::tufa_server::telemetry::init_subcriber_error_enum::InitSubcriberErrorEnum::SetGlobalDefault {
            error: e,
            code_occurence: crate::code_occurence_tufa_common!(),
        });
    }
    Ok(())
}
