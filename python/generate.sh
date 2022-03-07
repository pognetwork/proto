#!/bin/sh

protoc --proto_path=../ --python_out=./pog_proto \
    node/api.proto \
    node/p2p.proto \
    node/rpc/lattice.proto \
    node/rpc/node_admin.proto \
    node/rpc/node_user.proto \
    node/rpc/node_wallet_manager.proto \
