import shell from "shelljs";

const files = [
  "api.proto",
  "rpc/admin.proto",
  "rpc/private.proto",
  "rpc/account.proto",
];

if (!shell.which('protoc')) {
  shell.echo('This script requires protoc');
  shell.exit(1);
}

shell.cd("../")

for (const file of files) {
  shell.exec(`protoc --plugin=./typescript/node_modules/.bin/protoc-gen-ts_proto --ts_proto_opt=outputClientImpl=grpc-web  --ts_proto_out=./typescript/generated ./node/${file}`)
}
