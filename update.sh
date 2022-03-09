#!/bin/sh

cd python && ./generate.sh
cd ..
cd typescript && npm run update-docs