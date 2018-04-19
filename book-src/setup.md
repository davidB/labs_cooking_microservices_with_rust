# Préparation

## Branche git

```bash
git checkout reviews-todo
```

## Avoir le hands on en local

```bash
cargo install mdbook
mdbook serve
```

## Lancer le service ratings

```bash
cd ratings
PORT=7878 RUST_LOG=info cargo run
```

Vérifier qu'il marche:
```bash
curl localhost:7878/ratings/0
curl localhost:7878/ratings/0 -d '{"reviewer":"me","rating":3}'
curl localhost:7878/ratings/0
```

## Lancer le service avec rebuild automatique

```bash
cargo install cargo-watch
cd reviews
RUST_LOG=warn RATINGS_URL=http://127.0.0.1:7878 cargo watch --ignore db.sqlite -x 'run'
```
