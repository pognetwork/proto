import shell from "shelljs";
import { files } from "../files.js";

if (!shell.which('protoc')) {
  shell.echo('This script requires protoc');
  shell.exit(1);
}
shell.cd("../")

for (const file of files) {
  const out = file.split(".")[0];
  shell.exec(`
    docker run --rm \
      -v $(pwd)/docs/generated:/out \
      -v $(pwd)/:/protos \
      pseudomuto/protoc-gen-doc --doc_opt=markdown,${out}.md node/${file}
  `)
}
