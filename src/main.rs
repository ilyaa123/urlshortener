use std::sync::Arc;
use dashmap::DashMap;

pub mod adapters;
pub mod app;
pub mod id_provider;
pub mod di;
pub mod ports;
pub mod error;

#[tokio::main]
async fn main() {
    let store = Arc::new(DashMap::new());

    let in_memory_repository = adapters::inmemory::InMemoryRepository::new(store);

    let idp = id_provider::NanoIDProvider;

    let container = Arc::new(di::Container::new(idp, in_memory_repository.clone(), in_memory_repository));

    let server = ports::httpapi::Server::new(3001, container);

    server.run().await;
}
