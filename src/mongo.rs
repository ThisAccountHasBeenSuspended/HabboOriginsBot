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
    let mdb = settings.get_mongodb();

    let options = match mongodb::options::ClientOptions::parse(mdb.get_uri()).await {
        Ok(r) => r,
        Err(e) => panic!("{}", e),
    };
    let _ = client(Some(options));
}

pub fn get_coll<T>(name: &str) -> Collection<T>
where
    T: Sync + Send,
{
    let settings = crate::settings();
    let mdb = settings.get_mongodb();

    let client = client(None);
    let db = client.database(mdb.get_database());
    db.collection::<T>(name)
}
