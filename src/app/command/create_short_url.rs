use crate::{error::AppError, id_provider::IDProvider};

pub trait CreateShortUrlRepository {
    fn save(&self, full_url: String, id: String) -> impl std::future::Future<Output = Result<(), AppError>> + std::marker::Send;
}

pub struct CreateShortUrlCommand<I, R> where I: IDProvider, R: CreateShortUrlRepository {
    id_provider: I,
    repo: R
}


impl<I, R> CreateShortUrlCommand<I, R> where I: IDProvider, R: CreateShortUrlRepository {
    pub fn new(id_provider: I, repo: R) -> Self {
        Self { id_provider, repo }
    }

    pub async fn execute(&self, full_url: &str) -> Result<String, AppError> {
        let parsed_url = url::Url::parse(&full_url).map_err(|_| AppError::URLAParseError)?;
        let id = self.id_provider.provide();
        self.repo.save(parsed_url.to_string(), id.clone()).await?;
        Ok(id)
    }
}



#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use dashmap::DashMap;

    use crate::adapters::inmemory::InMemoryRepository;

    use super::*;

    #[tokio::test]
    async fn get_short_url() {
        let id_provider = crate::id_provider::FakeIDProvider::new("123".to_owned());
        let store = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store);

        let command = CreateShortUrlCommand::new(id_provider, repo);
        let result = command.execute("https://google.com").await;
        assert_ne!(result, Ok("".to_owned()));
    }


    #[tokio::test]
    async fn get_two_different_short_url() {
        let id_provider = crate::id_provider::NanoIDProvider;
        let store = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store);

        let command = CreateShortUrlCommand::new(id_provider, repo);
        let result = command.execute("https://google.com").await;


        let result2 = command.execute("https://google.com").await;
        assert_ne!(result, result2);
    }

    #[tokio::test]
    async fn after_save_store_should_have_one_item() {
        let id_provider = crate::id_provider::NanoIDProvider;
        let store = Arc::new(DashMap::new());
        let repo = InMemoryRepository::new(store.clone());

        let command = CreateShortUrlCommand::new(id_provider, repo);
        let id = command.execute("https://google.com").await.unwrap();

        let full_url = store.get(&id).unwrap();

        assert_eq!(store.len(), 1);
        assert_eq!(full_url.value(), "https://google.com/")
    }



}
