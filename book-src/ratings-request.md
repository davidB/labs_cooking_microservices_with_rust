# Communiquer avec le service ratings

Dans `reviews.rs`

## Nouveaux imports

```rust,no_run,ignore
use actix_web::client::ClientRequest;
use actix_web::HttpMessage;
```

## Récupérer les ratings

### Définir la structure du json

```rust,no_run,ignore
#[derive(Debug, Deserialize)]
pub struct RatingsResponse {
    id: i32,
    pub ratings: HashMap<String, i32>,
}
```

### Appeler le service

Remplacer l'assignation de `ratings` par:
```rust,no_run,ignore
let ratings = ClientRequest::get(&format!("{}/ratings/{}", ::CONFIG.ratings_url, product_id))
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
    });
```

Et remplacer
```rust,no_run,ignore
build_response_from_ratings(ratings).responder()
```

par
```rust,no_run,ignore
ratings.and_then(build_response_from_ratings).responder()
```


## Sauvegarder un nouveau rating

### Appeler le service

Remplacer
```rust,no_run,ignore
state
    .db
    .send(db::SaveReview {
        review: review_to_save,
    })
    .from_err()
    .and_then(move |_| Ok(HttpResponse::Ok().json(review.clone())))
    .responder()
```

par:
```rust,no_run,ignore
ClientRequest::post(&format!("{}/ratings/{}", ::CONFIG.ratings_url, product_id))
    .json(review.clone())
    .unwrap()
    .send()
    .map(|_| ())
    .or_else(|err| {
        // in case of error, log it and ignore it
        error!("{:?}", err);
        Ok(())
    })
    .and_then(move |_| {
        state
            .db
            .send(db::SaveReview {
                review: review_to_save,
            })
            .from_err()
            .and_then(move |_| Ok(HttpResponse::Ok().json(review.clone())))
    })
    .responder()
```

## Résultat

Les ratings sont maintenant dispo quand on demande des avis, et l'application `ratings` a loggé qu'elle a été appellée
```
curl localhost:9081/reviews/0
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

```
{"msg":"requesting ratings for product 0","level":"INFO","ts":"2018-04-20T00:52:35.348631+02:00","logger":"app"}
{"msg":"[RESPONSE][d65d721f-b0d4-4689-a1fd-7b455fedd76d][HTTP/1.1][200 OK][755µs]","level":"INFO","ts":"2018-04-20T00:52:35.348900+02:00","logger":"app"}
```

À la sauvegarde d'un avis, la note est sauvegardée dans l'application `ratings`
```
curl localhost:9081/reviews/3 -H 'Content-Type: application/json' -d '{"reviewer":"moi","rating":3,"text":"mon avis"}'
{"reviewer":"moi","text":"mon avis","rating":3}

curl localhost:9081/reviews/3
{
   "reviews" : [
      {
         "reviewer" : "moi",
         "text" : "mon avis",
         "rating" : {
            "color" : "yellow",
            "stars" : 3
         }
      }
   ],
   "id" : 3
}
```

```
{"msg":"saving new rating NewRating { reviewer: \"moi\", rating: 3 } for product 3","level":"INFO","ts":"2018-04-20T01:00:22.008725+02:00","logger":"app"}
{"msg":"[RESPONSE][4c0951b3-f302-4759-9458-506a6b1a9f97][HTTP/1.1][200 OK][822µs]","level":"INFO","ts":"2018-04-20T01:00:22.009032+02:00","logger":"app"}
```
