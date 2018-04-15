# reviews - 3 : diesel

* Connection à une base de données avec diesel
  * Créer la DB avec diesel cli
  * Table pour stocker un nom et un id

```
curl http://localhost:9080/hello/1
curl http://localhost:9080/hello/1 -H 'Content-Type: application/json' -X POST -d '{"say hello to": "Rust"}'
curl http://localhost:9080/hello/1
```

* À faire:
  * Créer la migration pour les review
  * Ajouter un type de message SaveReview
  * Ajouter un type de message GetReviews

```
curl http://localhost/reviews/5
curl http://localhost/reviews/5 -H 'Content-Type: application/json' -X POST -d '{"reviewer": "my_name", "rating": 3, "text": "my review"}'
```
