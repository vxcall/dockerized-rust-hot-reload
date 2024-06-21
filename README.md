# :fire: dockerized-rust-hot-reload
best practice of HOT reloading dockerized rust code.
U can reach to [my tiny blog post](https://vxcall.github.io/posts/turbocharging_rust_achieve_lightning_fast_hot_reloading_with_docker_and_mold/) about this.

run following to experience how fast it is.

```shell
docker compose up --build -d
docker compose logs -f
```

then access 

```shell
localhost:8080/api/jobs
```

to see the initial content.

after that, change arbitrally Rust code like

```rs
fn jobs() -> &'static str {
    "all jobs"
}
```

to

```rs
fn jobs() -> &'static str {
    "changedddddddd!!"
}
```

then reload the page to confirm it's been changed successfully!
