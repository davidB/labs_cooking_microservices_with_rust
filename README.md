# Service Ratings

* Utilisation de serde
  * `lazy_static` pour avoir une variable accessible de partout
  * `Arc` pour y accèder de plusieurs threads
  * `Mutex` pour pouvoir la modifier

```
curl http://localhost:7878/hello
curl http://localhost:7878/hello -X POST -d '{"say hello to": "Rust"}'
curl http://localhost:7878/hello
```

* À faire:
  * Sauvegarder les ratings dans une HashMap
  * Répondre la liste des ratings sauvegardés en filtrant par product id

```
curl http://localhost/ratings/5 -X POST -d '{"reviewer": "my_name", "rating": 3}'
curl http://localhost/ratings/5
```
