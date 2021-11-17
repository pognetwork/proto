export const rpc = [
  "rpc/node_admin.proto",
  "rpc/node_wallet_manager.proto",
  "rpc/block.proto",
  "rpc/node_user.proto",
];

export const apiFiles = [
  "api.proto",
]

export const files = [
  ...rpc,
  ...apiFiles
];
