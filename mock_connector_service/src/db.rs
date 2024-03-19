use serde::Deserialize;
use tokio_postgres::{Client, Error, NoTls, Statement};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

//Service to generate and fetch data from a database,
//simulating fetching the Virk API
#[derive(Deserialize, Debug)]
pub struct DBEntry {
    pub id: u32,
    pub col1: String,
    pub col2: String,
}

pub struct DBService {
    pub client: Client,
    pub load_statement: Statement
}

impl DBService {
    pub async fn new() -> Self {
        let (client, connection) = match tokio_postgres::connect("host=localhost user=postgres password=postgres dbname=data port=5431", NoTls).await {
            Ok((client, connection)) => (client, connection),
            Err(e) => panic!("{:?}", e),
        };
        println!("client and connection established");
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("db connection error: {}", e);
            }
        });

        let _ = match client.query(
            "CREATE TABLE IF NOT EXISTS data(id SERIAL PRIMARY KEY, col1 TEXT, col2 TEXT);",&[]).await {
                Ok(res) => res,
                Err(e) => panic!("{:?}", e),
            };

        let load_statement = match client.prepare("SELECT * FROM data WHERE id = $1").await {
            Ok(statement) => statement,
            Err(e) => panic!("{:?}", e),
        };

        let _ = populate_db(&client).await;
    
        Self {
            client,
            load_statement
        }
    }

    pub async fn load_entry(&self, id: u32) -> Result<DBEntry, Error> {
        let row = self.client.query_one(&self.load_statement, &[&id]).await?;
        let entry = DBEntry {
            id: row.try_get("id")?,
            col1: row.try_get("col1")?,
            col2: row.try_get("col2")?
        };
        Ok(entry)
    }
}

async fn populate_db(client: &Client) -> Result<(), Error>{
    
    let statement = client.prepare(
        "INSERT INTO data (col2, col3) VALUES ($1, $2)"
        ).await?;
    
    for _ in 0..100000000 {
        let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

        let rand_string2: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

        client.query(&statement, &[&rand_string, &rand_string2]).await?;
    }

    Ok(())
}