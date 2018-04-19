# Renvoyer des données

## Définir le modèle
Dans `models.rs`

```rust,no_run,ignore
#[derive(Serialize, Debug)]
pub struct Review {
    pub product_id: i32,
    pub reviewer: String,
    pub review: String,
}
```

Dans `lib.rs`

```rust,no_run,ignore
mod models;
```

## Traduire avec les entrées / sorties
Dans `reviews.rs`

```rust,no_run,ignore
use models;
```

### Définir la structure JSON de sortie

```rust,no_run,ignore
#[derive(Debug, Serialize)]
pub struct Product {
    pub id: i32,
    pub reviews: Vec<Review>,
}

#[derive(Debug, Serialize)]
pub struct Review {
    pub reviewer: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rating: Option<Rating>,
}

#[derive(Debug, Serialize)]
pub struct Rating {
    pub stars: i32,
    pub color: Color,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Blue,
    Yellow,
    Red,
}
```

### Convertir le modèle en sortie

```rust,no_run,ignore
pub fn rating_nb_to_rating(rating: i32) -> Rating {
    Rating {
        stars: rating,
        color: match rating {
            1 => Color::Red,
            2 | 3 => Color::Yellow,
            _ => Color::Blue,
        },
    }
}

pub fn reviews_with_ratings(
    reviews: Vec<models::Review>,
    ratings: HashMap<String, i32>,
) -> Vec<Review> {
    reviews
        .iter()
        .map(|review| {
            let reviewer = review.reviewer.clone();
            Review {
                rating: ratings.get(&reviewer).map(|&r| rating_nb_to_rating(r)),
                reviewer: reviewer,
                text: review.review.clone(),
            }
        })
        .collect()
}
```

### Renvoyer la réponse

```rust,no_run,ignore
let ratings = HashMap::new();
let reviews = vec![];

let build_response_from_ratings = move |ratings| {
    futures::future::result({
        // build the response
        let product = Product {
            id: product_id,
            reviews: reviews_with_ratings(reviews, ratings),
        };
        // return a 200 response with the reviews as json
        Ok(HttpResponse::Ok().json(product))
    })
};

build_response_from_ratings(ratings).responder()
```

### Définir la structure JSON en entrée

```rust,no_run,ignore
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewReview {
    reviewer: String,
    text: String,
    rating: i32,
}
```

### Convertir l'entrée en modèle

```rust,no_run,ignore
let review_to_save = models::Review {
    product_id: product_id as i32,
    reviewer: review.reviewer.clone(),
    review: review.text.clone(),
};

```

### Renvoyer la réponse

```rust,no_run,ignore
futures::future::result({
    Ok(HttpResponse::Ok().json(review.clone()))
}).responder()
```

## Résultat

Récupérer les reviews d'un product donne une liste vide
```
curl localhost:9080/reviews/0 -i
HTTP/1.1 200 OK
content-length: 21
content-type: application/json
date: Thu, 19 Apr 2018 21:59:21 GMT

{"id":0,"reviews":[]}
```

Poster une review la renvoie telle quelle
```
curl localhost:9080/reviews/0 -i -H 'Content-Type: application/json' -d '{"reviewer":"moi","rating":3,"text":"mon avis"}'
HTTP/1.1 200 OK
content-length: 47
content-type: application/json
date: Thu, 19 Apr 2018 22:00:57 GMT

{"reviewer":"moi","text":"mon avis","rating":3}
```

Poster un JSON invalide répond une erreur 400 et un message dans les logs

```
curl localhost:9080/reviews/0 -i -H 'Content-Type: application/json' -d '{"reviewer":"moi","rating":3}'
HTTP/1.1 400 Bad Request
content-length: 0
date: Thu, 19 Apr 2018 22:01:54 GMT
````

````
{"msg":"Error occured during request handling: Json deserialize error: missing field `text` at line 1 column 29","level":"WARN","ts":"2018-04-20T00:01:55.287447+02:00","logger":"app"}
````
