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
pub fn rating_nb_to_rating(rating: &i32) -> Rating {
    Rating {
        stars: *rating,
        color: match *rating {
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
                rating: ratings.get(&reviewer).map(rating_nb_to_rating),
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
futures::future::ok(Ok(vec![]))
    .and_then(move |mut reviews: Result<Vec<models::Review>, ()>| {
        if reviews.is_err() {
            error!("{:?}", reviews);
            reviews = Ok(vec![]);
        }
        // build the response
        let product = Product {
            id: product_id,
            reviews: reviews_with_ratings(reviews.unwrap(), ratings),
        };
        // return a 200 response with the reviews as json
        Ok(HttpResponse::Ok().json(product))
    })
    .responder()
```

### Définir la structure JSON en entrée

```rust,no_run,ignore
#[derive(Debug, Deserialize, Serialize)]
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
futures::future::ok(Ok(NewReview {
    reviewer: review.reviewer.clone(),
    text: review.text.clone(),
    rating: review.rating,
}))
    .and_then(|res: Result<NewReview, ()>| match res {
        Ok(review) => Ok(HttpResponse::Ok().json(review)),
        Err(_) => Ok(HttpResponse::InternalServerError().into()),
    })
    .responder()
```