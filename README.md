# reviews - 1 : actix-web

* Squelette de service avec actix-web
  * Répondre aux requêtes en GET et en POST
  * Lire les données du path
  * Lire les données du body
  * Répondre une struct implémentant `Deserialize`

```
curl http://localhost:9080/hello/Devoxx
curl http://localhost:9080/hello -H 'Content-Type: application/json' -X POST -d '{"say hello to": "Rust"}'
```

* À faire:
  * Répondre sur GET /reviews/:product_id
  * Répondre sur POST /reviews/:product_id

```
curl http://localhost/reviews/5
curl http://localhost/reviews/5 -H 'Content-Type: application/json' -X POST -d '{"reviewer": "my_name", "rating": 3, "text": "my review"}'
```
