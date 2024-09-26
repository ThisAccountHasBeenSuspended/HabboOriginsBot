use mongodb::{options::ClientOptions, Client, Collection};

pub fn client(opts: Option<ClientOptions>) -> &'static Client {
    use std::sync::OnceLock;
    static VAL: OnceLock<Client> = OnceLock::new();
    VAL.get_or_init(move || {
        let o = unsafe { opts.unwrap_unchecked() };
        match Client::with_options(o) {
            Ok(r) => r,
            Err(e) => panic!("{}", e),
        }
    })
}

pub async fn init() {
    let settings = crate::settings();
    let mongodb_uri = settings.get_mongodb_uri();

    let options = match mongodb::options::ClientOptions::parse(mongodb_uri).await {
        Ok(r) => r,
        Err(e) => panic!("{}", e),
    };
    let _ = client(Some(options));
}

pub fn get_coll<T>(name: &str) -> Collection<T>
where
    T: Sync + Send,
{
    let client = client(None);
    let db = client.database("Cluster0");
    db.collection::<T>(name)
}
