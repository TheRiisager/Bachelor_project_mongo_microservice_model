use futures::StreamExt;
use mongodb::{error::Error, options::{ClientOptions, ReplaceOptions}, Client, Database, Collection, bson::doc};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DBEntry {
    pub id: u32,
    pub col1: String,
    pub col2: String,
}

pub struct MongoDBService {
    client: Client,
    db: Database,
    collection: Collection<DBEntry>
}

impl MongoDBService {
    pub async fn new() -> Result<Self, Error> {
        let client_options = ClientOptions::parse("mongodb://user:pass@localhost").await?;
        let client = Client::with_options(client_options)?;
        let db = client.database("virk");
        let collection = db.collection::<DBEntry>("data");

        Ok(MongoDBService { 
            client, 
            db,
            collection 
        })
    }

    pub async fn save_entry(&self, entry: DBEntry) -> Result<i64, Error> {
        let id = entry.id.clone();
        let options = ReplaceOptions::builder().upsert(true).build();
        let res = &self.collection.replace_one(doc! {"id": entry.id}, entry, options).await?;
        let id = match &res.upserted_id {
            Some(id) => {id.as_i64().unwrap()},
            None => id.into(),
        };

        Ok(id)
    }

    pub async fn get_entry(&self, id: i64) -> Result<DBEntry, Error> {
        let res = self.collection.find_one(doc!{"id": id}, None).await?;
        match res {
            Some(res) => Ok(res),
            None => panic!("no entry found with id {:?}", id)
        }
    }

    pub async fn get_all_ids(&self) -> Result<Vec<u32>, Error> {
        let mut res = self.collection.find(None, None).await?;
        let mut list: Vec<u32> = Vec::new(); 
        while let Some(Ok(entry)) = res.next().await {
            list.push(entry.id);
        }
        Ok(list)
    }
}