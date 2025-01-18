pub mod command;
pub mod query;


#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use dashmap::DashMap;

    use crate::adapters::inmemory::InMemoryRepository;

    use crate::app::command::create_short_url::CreateShortUrlCommand;
    use crate::app::query::get_full_url::GetFullUrlQuery;
    use crate::id_provider::FakeIDProvider;

    #[tokio::test]
    async fn create_and_get_short_url() {
        let store = Arc::new(DashMap::new());

        let repo = InMemoryRepository::new(store.clone());

        let id_provider = FakeIDProvider::new("123".to_owned());

        let create_command = CreateShortUrlCommand::new(id_provider, repo.clone());

        let get_query = GetFullUrlQuery::new(repo);

        let result = create_command.execute("https://google.com").await;
        let result2 = get_query.execute(&result.unwrap()).await.unwrap();

        assert_eq!(result2, "https://google.com/".to_owned())
    }


}
