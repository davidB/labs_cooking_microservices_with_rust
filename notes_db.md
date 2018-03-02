# CFP Micro-service with Rust

WIP

## Services ideas

### A distributed calculator

- like a recursive expression evaluator
- support of constant integer
- support of value read from db
- support of operator '+'
- support of (....) to group evaluation and send to a service

### A store + aggregator for test result

### A path finder

### A karma service

## Elements to include into our lab

- The idea that I want to explore is what it would be like to create a small microservice in Rust. How hard is it? Is it even possible? What tooling will I need? There’s a healthy list of questions that comes with this idea. from [Creating Expedient Microservices in Rust and Diesel - via @codeship | via @codeship](https://blog.codeship.com/creating-expedient-microservices-in-rust-and-diesel/)

### highlights

#### tools

- rustup
- cargo
- clippy
- crates.io
- rls
- profiler
  - [Making TRust-DNS faster than BIND9](https://bluejekyll.github.io/blog/rust/2017/12/29/making-trust-dns-fast.html)
  - [Rust Optimization.md](https://gist.github.com/jFransham/369a86eff00e5f280ed25121454acec1)
  - [Rust Profiling with Instruments and FlameGraph on OSX: CPU/Time](http://carol-nichols.com/2015/12/09/rust-profiling-on-osx-cpu-time/)
- benchmarks
  - libtest benchmarks
  - [Criterion.rs](https://bheisler.github.io/post/benchmarking-with-criterion-rs/)
- debugger
  - [Debugging Rust programs with lldb on MacOS | Bryce Fisher-Fleig](https://bryce.fisher-fleig.org/blog/debugging-rust-programs-with-lldb/index.html)

#### libs

- failure (successor of error_chain)
- [purpliminal/rust-dotenv: A `dotenv` implementation for Rust.](https://github.com/purpliminal/rust-dotenv)
- logger
  - [env_logger](https://crates.io/crates/env_logger)
  - [slog](https://crates.io/crates/slog)
  - [slog-envlogger](https://crates.io/crates/slog-envlogger)
- serde (rustc-serialize)
- hyper/reqwest
- concurrent
    - thread
    - future/tokio
    - [Xudong-Huang/may: rust stackful coroutine library](https://github.com/Xudong-Huang/may)
    - smid
- db
    - diesel

#### build

##### Docker Alpine

(from [michiel/docker-rust-microservice](https://github.com/michiel/docker-rust-microservice))

[Alpine Linux](https://alpinelinux.org/) uses [musl-libc](https://www.musl-libc.org/) instead of glibc, which is the default for most common distributions.

```sh
  rustup target add x86_64-unknown-linux-musl
````

Once that is installed we can explicitly target it when building the service,

```sh
  cargo build --target x86_64-unknown-linux-musl --release
````

### Links

- [Rust 101 - YouTube](https://www.youtube.com/watch?v=FMqydRampuo) [slides](http://talks.edunham.net/lca2017/rust101.pdf)
- [awesome-rust](https://github.com/rust-unofficial/awesome-rust)
- [Matthias Endler - Idiomatic Rust - YouTube](https://www.youtube.com/watch?v=P2mooqNMxMs)
- https://github.com/rust-unofficial/patterns
- https://github.com/mre/idiomatic-rust
- [Go kit - Frequently asked questions](https://gokit.io/faq/)
  - [The Hunt for a Logger Interface](http://go-talks.appspot.com/github.com/ChrisHines/talks/structured-logging/structured-logging.slide#40)
  - [Peter Bourgon · Logging v. instrumentation](https://peter.bourgon.org/blog/2016/02/07/logging-v-instrumentation.html)


### Microservices

- https://12factor.net/
  1. Codebase: One codebase tracked in revision control, many deploys
  2. Dependencies: Explicitly declare and isolate dependencies
  3. Config: Store config in the environment
  4. Backing services: Treat backing services as attached resources
  5. Build, release, run: Strictly separate build and run stages
  6. Processes: Execute the app as one or more stateless processes
  7. Port binding: Export services via port binding
  8. Concurrency: Scale out via the process model
  9. Disposability: Maximize robustness with fast startup and graceful shutdown
  10. Dev/prod parity: Keep development, staging, and production as similar as possible
  11. Logs: Treat logs as event streams
  12. Admin processes: Run admin/management tasks as one-off processes
- configuration
  - environment
  - secret
  - static (cold)
  - dynamic (hot)
- deployement
  - tools
    - spinnaker
    - helm
    - [brigade](https://brigade.sh/) (event-driven scripting for k8s)
      - [Microservices, Service Mesh, and CI/CD Pipelines: Making It All Work Together [I] - Brian Redmond - YouTube](https://www.youtube.com/watch?v=UbLG_qUyCgM&index=13&list=PLj6h78yzYM2P-3-xqvmWaZbbI1sW-ulZb)
        - [KubeCon 2017 Demo — Istio and Brigade CI/CD – Brian Redmond – Medium](https://medium.com/@chzbrgr71/kubecon-2017-demo-istio-and-brigade-ci-cd-9db5ef15a942)
      - kashti (a dashboard for brigade project)
  - container images
  - healthcheck, readiness, liveness,
  - chaos automated platform (CHaP)
  - squeeze testing
  - canary (AB testing)
    - ACA (automated canary analysis)
    - advanced routing
      - route rules
      - traffic shaping
    - observability
      - metrics
      - logs
      - tracing
    - chaos testing
      - fault injection
        - delay
        - faults
  - deployment strategy
    - green / blue
    - only one / highlander
- AB testing
  - traffic splitting
  - traffic routing
  - observability
- observability, apm, alerts, telemetry
  - dashboard : grafana, skywalking vs jaeger
  - snap ?
    - [Snap and Kubernetes: together at last – Intel SDI – Medium](https://medium.com/intel-sdi/how-we-integrated-kubernetes-with-snap-and-come-up-with-kubesnap-466ac25fc0ff)
    - [intelsdi-x/snap-integration-kubernetes](https://github.com/intelsdi-x/snap-integration-kubernetes)
  - logstash, syslog
  - prometheus vs influxdb vs postgresql/timescale
  - topology
- network, routing
  - topology
  - service discovery
  - ingress
  - egress
  - envoy
  - istio vs conduit
  - service meshes and proxies as side-car
    - observability
    - resiliency
    - traffic control
    - security
    - policy enforcement
    - part of infrastructure
    - zero code change
    - control plane (pilots + mixers + secutiry)
    - outbound features
      - service authentication
      - load balancing
      - timeout, retries, deadline, circuit breaker
      - connection pool sizing
      - fine grained routing
      - telemetry
      - request tracing
      - fault injection
    - inbound features
      - service authentication
      - authorization
      - rate limits
      - load shedding
      - request tracing
      - fault injection
  - circuit breaker
  - retry / deadline
- security
  - vault, secret
  - istio
    - concerns
      - insiders
      - hijacked service
      - microservice attack surface
      - workload mobility
      - brittle fine-grained models
      - securing resources not just endpoints
      - audit & compliance
    - wants
      - workload mobility
      - remote admin & development
      - shared & 3rd party services
      - user & service identity
      - lower cost
    - spiffe.io
  - orchestrate key & certificates
    - generation
    - deployment
    - rotation
    - revocation
- protocol
  - http/1.1
  - http/2
  - with or without tls
  - json + http
  - protobuf + gRPC
  - graphql
  - kafka
  - syslog
- db
  - dgraph
  - sql (diesel)
  - cassandra, mongodb

### presentation ideas
- mindmapping for demo ?
  - https://coggle.it/
- wall of names, logo and show the selected one (and why in notes)
- graphviz (dot syntax) + d3js prog (or blender or pixi4js)
  - display graph
  - highlight (auto-zoom + relayout to fit slide) on selected node (and edges and direct nodes deep by number 1-9)
  - controls:
    - select node by click or by name
    - select deep by number (or slider or wheels or up/down arrow) (LoD)
    - goto "next" node (follow outgoing edge "next") with right arrow
    - goto "prev" node (follow incoming (reverse) edge "next") with left arrow
  - inspiration:
    - http://graphviz.it/#/gallery/world.gv / https://github.com/mstefaniuk/graph-viz-d3-js
    - https://maptimeboston.github.io/d3-maptime/#/
    - https://28mm.github.io/notes/d3-terraform-graphs-2
    - https://github.com/dagrejs/dagre-d3 + https://github.com/dagrejs/graphlib-dot
    - http://www.d3noob.org/2013/03/d3js-force-directed-graph-example-basic.html
  - integrate logo + link (to project,...)

- for the code
  - use git + 1 tag (or 1 branch ?) per step ?
