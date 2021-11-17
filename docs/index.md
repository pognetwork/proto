# Shared Protocol Buffers ![](https://img.shields.io/npm/v/@pognetwork/champ-proto?style=flat-square&logo=npm)

[**pognetwork/proto**](https://github.cpm/pognetwork/proto) contains protocol buffers which are shared across pog projects and can be used to interact with the different APIs safely.

APIs are append only. Fields that aren't used anymore can only be updated for major new API versions or deprecated and replaced. Old fields should first be marked as deprecated and later commented out without their id being reused.

## Supported languages

### Rust

Currently prepackaged types are only provided for rust:

`cargo.toml`

```toml
[dependencies]
pog-proto = {git = "https://github.com/pognetwork/proto"}
```

Cargo pins dependencies, so after pushing a new change to this repo, be sure to run `$ cargo update` in the project depending on this. Also, remember to reload your IDE since the rust language server often doesn't pick up the changes in dependencies.

### TypeScript/JavaScript

New builds are automatically released and versioned with the sortened version of the commit hash, e.g. `0.1.0-d15c488`.

`$ npm install @pognetwork/champ-proto@latest google-protobuf @types/google-protobuf @improbable-eng/grpc-web`

```ts
import { grpc } from "@improbable-eng/grpc-web";
import {
  GrpcWebImpl,
  NodeUserClientImpl,
} from "@pognetwork/champ-proto/rpc/node_user";

const rpc = new GrpcWebImpl("http://localhost:9090", {
  debug: false,
  metadata: new grpc.Metadata({ Authorization: "bar" }),
});

const user = new NodeUserClientImpl(rpc);

user
  .Login({ password: "foo", username: "bar" })
  .then((reply) => console.log(reply.token))
  .catch((e) => console.error(e));
```
