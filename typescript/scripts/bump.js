import { readFileSync, writeFileSync } from "fs";

const pkg = JSON.parse(readFileSync("./package.json"));

let version = pkg.version.split(".").map(i => parseInt(i));
version[version.length - 1] += 1;
pkg.version = version.join(".");

writeFileSync("./package.json", JSON.stringify(pkg, null, 2), "utf-8");