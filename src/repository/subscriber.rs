use bambangshop::REQWEST_CLIENT;
use dashmap::DashMap;
use lazy_static::lazy_static;
use rocket::{serde::json::to_string, tokio, log};
use crate::model::{notification::Notification, subscriber::Subscriber};

// Singleton of Database
lazy_static!{
    static ref SUBSCRIBER: DashMap<String, DashMap<String, Subscriber>> = DashMap::new();
}

pub struct SubscriberRepository;

impl SubscriberRepository{
    pub fn add(product_type: &str, subscriber: Subscriber) -> Subscriber {
        let subscriber_value = subscriber.clone();
        if SUBSCRIBER.get(product_type).is_none(){
            SUBSCRIBER.insert(String::from(product_type), DashMap::new());

        };

        SUBSCRIBER.get(product_type).unwrap()
            .insert(subscriber_value.url.clone(), subscriber_value);
        return subscriber;
    }

    pub fn list_all(product_type: &str) -> Vec<Subscriber> {
        if SUBSCRIBER.get(product_type).is_none(){
            SUBSCRIBER.insert(String::from(product_type), DashMap::new());
        };

        return SUBSCRIBER.get(product_type).unwrap().iter()
            .map(|f| f.value().clone()).collect();
    }

    pub fn delete(product_type: &str, url: &str) -> Option<Subscriber>{
        // membuat product_type baru apabila belum ada product_type tersebut
        if SUBSCRIBER.get(product_type).is_none() {
            SUBSCRIBER.insert(String::from(product_type), DashMap::new());
        }

        // Menghapus subscriber dengan product_type & url
        let result = SUBSCRIBER.get(product_type).unwrap().remove(url);
        if !result.is_none(){
            return Some(result.unwrap().1);
        }
        return None;
    }

}

impl Subscriber{
    #[tokio::main]
    pub async fn update(&self, payload: Notification){
        REQWEST_CLIENT
            .post(&self.url)
            .header("Content-Type", "JSON")
            .body(to_string(&payload).unwrap())
            .send().await.ok();
        log::warn_!("Sent {} notification of: [{}] {}, to: {}",
            payload.status, payload.product_type, payload.product_title, self.url);
    }
}