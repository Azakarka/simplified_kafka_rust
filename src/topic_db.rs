use std::collections::HashMap;
use std::str;
use std::sync::Arc;
use tokio::sync::broadcast::{channel, Sender, Receiver};
use tokio::sync::Mutex;

type Topic = String;
pub(crate) type IncomingData = String;

pub type Channel<T> = (Sender<T>, Receiver<T>);
pub type TopicMap = Arc<Mutex<HashMap<Topic , Channel<IncomingData>>>>;

const CHANNEL_SIZE: usize = 1024;

pub fn init_topic_db() -> TopicMap {
    Arc::new(Mutex::new(HashMap::new()))
}

pub async fn get_sender(topic_db: &mut TopicMap, topic: &str) -> Sender<IncomingData> {
    let mut locked_topic_map = topic_db.lock().await;
    if !locked_topic_map.contains_key(topic) {
        let channel = channel(CHANNEL_SIZE);
        locked_topic_map.insert(topic.to_string(), channel);
    }
    locked_topic_map.get_mut(topic).unwrap().0.clone()
}

pub async fn get_receiver(topic_db: &mut TopicMap, topic: &str) -> Receiver<IncomingData> {
    get_sender(topic_db, topic).await.subscribe()
}
