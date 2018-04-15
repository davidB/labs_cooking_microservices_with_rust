# ratings - 2 : serde

* Utilisation de serde
  * Dérivation des traits `Serialize` / `Deserialize`
  * Les attributs de serde
  * Écrire du json avec `serde_json::to_*`
  * Lire le json avec `serde_json::from_*`

```
curl http://localhost:7878/hello -X POST -d '{"say hello to": "Rust"}'
curl http://localhost:7878/goodbye/Devoxx
```

* À faire:
  * Créer les structs pour la réponse avec les ratings
  * Implémenter le trait `IntoResponse` de Gotham
  * Créer la struct pour créer un rating

```
curl http://localhost/ratings/5
curl http://localhost/ratings/5 -X POST -d '{"reviewer": "my_name", "rating": 3}'
```
