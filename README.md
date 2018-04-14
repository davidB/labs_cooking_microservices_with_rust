# Service Ratings

* Squelette de service avec Gotham
  * Répondre aux requêtes en GET et en POST
  * Lire les données du path
  * Lire les données du body
  * Répondre une struct implémentant `IntoResponse`

```
curl http://localhost:7878/hello
curl http://localhost:7878/hello/Devoxx
curl http://localhost:7878/hello -X POST -d 'Rust'
curl http://localhost:7878/goodbye/Devoxx
```


* À faire:
  * Répondre sur GET /ratings/:product_id
  * Répondre sur POST /ratings/:product_id

```
curl http://localhost/ratings/5
curl http://localhost/ratings/5 -X POST -d '{"reviewer": "my_name", "rating": 3}'
```
