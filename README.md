# Service Ratings

* Utilisation de serde
  * Dérivation des traits `Serialize` / `Deserialize`
  * Les attributs de serde
  * Écrire du json dans une String avec `serde_json::to_string`
  * Lire le json avec `serde_json::from_*`

* À faire:
  * Créer les structs pour la réponse avec les ratings
  * Implémenter le trait `IntoResponse` de Gotham
  * Créer la struct pour créer un rating
