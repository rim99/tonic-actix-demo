# Tonic + Actix

Run app

```
cargo run
```

Curl test

```
grpcurl --plaintext -d '{"name": "hello"}' localhost:50051 helloworld.Greeter/SayHello
```
