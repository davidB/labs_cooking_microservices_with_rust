# Préparation

## Branche git

```
git checkout reviews-todo
```

## Avoir le hands on en local

```
cargo install mdbook
mdbook serve
```

## Lancer le service ratings

```
PORT=7878 RUST_LOG=info cargo run -p ratings
```

Vérifier qu'il marche:
```
curl localhost:7878/ratings/0
curl localhost:7878/ratings/0 -d '{"reviewer":"me","rating":3}'
curl localhost:7878/ratings/0
```

## Lancer le service avec rebuild automatique

```
cargo install cargo-watch
cd reviews
RATINGS_URL=http://127.0.0.1:7878 cargo watch -x 'run'
```
