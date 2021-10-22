import { readFileSync, writeFileSync } from "fs";

import shell from "shelljs";

const hash = shell.exec("git show -s --format=%h").trim();

const pkg = JSON.parse(readFileSync("./package.json"));
let version = pkg.version.split(".").map(i => parseInt(i));
version[version.length - 1] = `0-${hash}`;
pkg.version = version.join(".");

writeFileSync("./package.json", JSON.stringify(pkg, null, 2), "utf-8");