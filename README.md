# ratings -  : healthcheck

* Logs
  * `env!` pour accèder aux variables d'environment pendant la compile, [cargo en met plusieurs à dispo](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates)
  * `time` pour avoir un timestamp

```
curl localhost:9080/health
```

* À faire:
  * Jouer avec
