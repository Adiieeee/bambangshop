use dashmap::DashMap;
use lazy_static::lazy_static;
use crate::model::subscriber::Subscriber;

//Singelation of Database
lazy_static! {
    pub static ref SUBSCRIBERS: DashMap<String, DashMap<String, Subscriber>> = DashMap::new();
}

pub structSubscriberRepository;

impl structSubscriberRepository{
}