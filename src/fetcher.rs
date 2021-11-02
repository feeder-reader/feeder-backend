use futures::future::FutureExt;
use std::time::Duration;
use futures_timer::Delay;
use futures::lock::Mutex;
use feeder_types::*;
use actix_web::Error;
use rss::Channel;
use atom_syndication::Feed;
use actix_web::client::Client;

pub struct Fetcher {
    entries: Mutex<Vec<Entry>>,
}

impl Fetcher {
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(Vec::new()),
        }
    }

    pub async fn update_loop(&self) -> Result<(), Error> {
        let now_future = Delay::new(Duration::from_secs(5));
        now_future.await;
         let mut client = Client::default();

       let mut response = client.get("http://feeds.feedburner.com/TheHackersNews")
          .send()
          .await?;
        let body = response.body().await?;
        let channel = Channel::read_from(&body[..]).unwrap();
        let items: Vec<Entry> = channel.items.into_iter().map(|item| {
            Entry {
                id: item.guid.expect("entry w/out guid").value,
                title: item.title.expect("entry w/out title"),
                link: item.link.expect("entry w/out link"),
                summary: item.description.expect("entry w/out description"),
                source: "TheHackersNews".to_owned(),
                new: true,
            }
        }).collect();
        let mut entries = self.entries.lock().await;
        entries.extend_from_slice(&items);

        info!("hi");
        Ok(())
    }

    pub async fn get_entries(&self) -> Vec<Entry> {
        let entries = self.entries.lock().await;
        entries.to_vec()
    }
}
