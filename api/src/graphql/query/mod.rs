mod member;
mod owner;
mod user;
mod credential;

// Add your other ones here to create a unified Query object
// e.x. Query(SomeQuery, OtherQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(user::Query, owner::Query, member::Query, credential::Query);
