import shell from "shelljs";
import { files } from "../files.js";

if (!shell.which('protoc')) {
  shell.echo('This script requires protoc');
  shell.exit(1);
}

shell.mkdir("-p", ["generated/node"])

shell.cd("../")

for (const file of files) {
  shell.exec(`protoc --plugin=./typescript/node_modules/.bin/protoc-gen-ts_proto --ts_proto_opt=outputClientImpl=grpc-web --ts_proto_opt=esModuleInterop=true  --ts_proto_out=./typescript/generated/node ./node/${file}`)
}
