# reviews - 2 : actix

* Acteurs avec actix
  * Un acteur et des messages typés
  * L'isoler avec `SyncArbiter`
  * Envoyer un message
  * Envoyer un message et attendre le résultat dans le future

```
curl http://localhost:9080/hello
curl http://localhost:9080/hello -H 'Content-Type: application/json' -X POST -d '{"say hello to": "Rust"}'
curl http://localhost:9080/hello
```

* À faire:
  * Ajouter un type de message GetReviews avec un product_id
  * Ajouter un type de message SaveReview

```
curl http://localhost/reviews/5
curl http://localhost/reviews/5 -H 'Content-Type: application/json' -X POST -d '{"reviewer": "my_name", "rating": 3, "text": "my review"}'
```
