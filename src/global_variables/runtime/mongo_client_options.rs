pub static MONGO_CLIENT_OPTIONS: once_cell::sync::Lazy<mongodb::options::ClientOptions> = once_cell::sync::Lazy::new(
    || 
    {
        futures::executor::block_on(
            mongodb::options::ClientOptions::parse(
                {
                    use crate::traits::get_mongo_url::GetMongoUrl;
                    crate::global_variables::runtime::config::CONFIG.get_mongo_url()
                }
            )
        )
        .expect("cannot construct mongo client options")
    }
);
