# Communiquer avec le service ratings

Dans `reviews.rs`

## Nouveaux imports

```rust,no_run
use actix_web::client::ClientRequest;
use actix_web::HttpMessage;
```

## Récupérer les ratings

### Définir la structure du json

```rust,no_run
#[derive(Debug, Deserialize)]
pub struct RatingsResponse {
    id: i32,
    pub ratings: HashMap<String, i32>,
}
```

### Appeler le service

```rust,no_run
ClientRequest::get(&format!("{}/ratings/{}", ::CONFIG.ratings_url, product_id))
    .finish()
    .unwrap()
    .send()
    .map_err(error::Error::from)
    .and_then(move |resp| {
        resp.json()
            .from_err()
            .and_then(|ratings: RatingsResponse| Ok(ratings.ratings))
    })
    .or_else(|err| {
        // in case of error, log it and continue with an empty list of ratings
        error!("{:?}", err);
        Ok(HashMap::new())
    })
```

### Chainer la future

```rust,no_run
...
        .and_then(move |ratings| {
            ...
        }).responder()
```

### Résultat

```
curl localhost:9080/reviews/0
{
   "reviews" : [
      {
         "rating" : {
            "stars" : 5,
            "color" : "blue"
         },
         "text" : "An extremely entertaining play by Shakespeare. The slapstick humour is refreshing!",
         "reviewer" : "Reviewer1"
      },
      {
         "reviewer" : "Reviewer2",
         "rating" : {
            "color" : "blue",
            "stars" : 4
         },
         "text" : "Absolutely fun and entertaining. The play lacks thematic depth when compared to other plays by Shakespeare."
      }
   ],
   "id" : 0
}
```

## Sauvegarder un nouveau rating

### Appeler le service

```rust,no_run
ClientRequest::post(&format!("{}/ratings/{}", ::CONFIG.ratings_url, product_id))
    .json(review.0)
    .unwrap()
    .send()
    .map(|_| ())
    .or_else(|err| {
        // in case of error, log it and ignore it
        error!("{:?}", err);
        Ok(())
    })
```

### Chainer la future

```rust,no_run
...
        .and_then(move |_| {
            ...
        }).responder()
```
