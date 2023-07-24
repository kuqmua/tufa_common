pub type DynArcGetAppInfoSendSync = std::sync::Arc<
    dyn crate::repositories_types::tufa_server::try_build_actix_web_dev_server::GetAppInfo
        + Send
        + Sync,
>;
