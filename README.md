# reviews - 4 : futur

* Appel à une API externe
  * Appel à une API pour avoir un mot aléatoire
  * Chaînage des futures pour utiliser cette requête plus le résultat en base de données

```
curl http://localhost:9080/hello/1
curl http://localhost:9080/hello/1 -H 'Content-Type: application/json' -X POST -d '{"say hello to": "Rust"}'
curl http://localhost:9080/hello/1
curl http://localhost:9080/random/1
```

* À faire:
  * Récupérer les ratings depuis le service ratings
  * Sauvegarder une nouvelle note dans le service ratings

```
curl http://localhost/reviews/5
curl http://localhost/reviews/5 -H 'Content-Type: application/json' -X POST -d '{"reviewer": "my_name", "rating": 3, "text": "my review"}'
```
