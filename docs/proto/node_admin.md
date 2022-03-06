# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [node/rpc/node_admin.proto](#node_rpc_node_admin-proto)
    - [Empty](#NODEADMIN-Empty)
    - [GetBlockPoolSizeReply](#NODEADMIN-GetBlockPoolSizeReply)
    - [GetChainReply](#NODEADMIN-GetChainReply)
    - [GetLogsReply](#NODEADMIN-GetLogsReply)
    - [GetLogsRequest](#NODEADMIN-GetLogsRequest)
    - [GetModeReply](#NODEADMIN-GetModeReply)
    - [GetNodeNameReply](#NODEADMIN-GetNodeNameReply)
    - [GetNodeStatusReply](#NODEADMIN-GetNodeStatusReply)
    - [GetPeersResponse](#NODEADMIN-GetPeersResponse)
    - [GetPendingBlocksReply](#NODEADMIN-GetPendingBlocksReply)
    - [GetVersionResponse](#NODEADMIN-GetVersionResponse)
    - [Peer](#NODEADMIN-Peer)
    - [SetModeRequest](#NODEADMIN-SetModeRequest)
    - [SetNodeNameRequest](#NODEADMIN-SetNodeNameRequest)
    - [UpgradeNodeRequest](#NODEADMIN-UpgradeNodeRequest)
  
    - [Mode](#NODEADMIN-Mode)
    - [Status](#NODEADMIN-Status)
  
    - [NodeAdmin](#NODEADMIN-NodeAdmin)
  
- [Scalar Value Types](#scalar-value-types)



<a name="node_rpc_node_admin-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## node/rpc/node_admin.proto



<a name="NODEADMIN-Empty"></a>

### Empty







<a name="NODEADMIN-GetBlockPoolSizeReply"></a>

### GetBlockPoolSizeReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| length | [uint64](#uint64) |  |  |






<a name="NODEADMIN-GetChainReply"></a>

### GetChainReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| current_chain | [string](#string) |  |  |






<a name="NODEADMIN-GetLogsReply"></a>

### GetLogsReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| logs | [string](#string) | repeated |  |






<a name="NODEADMIN-GetLogsRequest"></a>

### GetLogsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| since | [uint64](#uint64) |  |  |






<a name="NODEADMIN-GetModeReply"></a>

### GetModeReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| current_mode | [Mode](#NODEADMIN-Mode) |  |  |






<a name="NODEADMIN-GetNodeNameReply"></a>

### GetNodeNameReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |






<a name="NODEADMIN-GetNodeStatusReply"></a>

### GetNodeStatusReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| status | [Status](#NODEADMIN-Status) |  |  |






<a name="NODEADMIN-GetPeersResponse"></a>

### GetPeersResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| peers | [Peer](#NODEADMIN-Peer) | repeated |  |






<a name="NODEADMIN-GetPendingBlocksReply"></a>

### GetPendingBlocksReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| blocks | [API.SignedBlock](#API-SignedBlock) | repeated |  |






<a name="NODEADMIN-GetVersionResponse"></a>

### GetVersionResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| version | [string](#string) |  |  |






<a name="NODEADMIN-Peer"></a>

### Peer



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ip | [string](#string) |  |  |
| prime | [bool](#bool) |  |  |






<a name="NODEADMIN-SetModeRequest"></a>

### SetModeRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| mode | [Mode](#NODEADMIN-Mode) |  |  |






<a name="NODEADMIN-SetNodeNameRequest"></a>

### SetNodeNameRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| new_name | [string](#string) |  |  |






<a name="NODEADMIN-UpgradeNodeRequest"></a>

### UpgradeNodeRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| version | [string](#string) |  |  |





 


<a name="NODEADMIN-Mode"></a>

### Mode


| Name | Number | Description |
| ---- | ------ | ----------- |
| PRIME | 0 | prime delegates |
| VALIDATING | 1 | non-prime delegates |
| OBSERVER | 2 | non-interacting node |
| LIGHT | 3 | node without the full transaction history |



<a name="NODEADMIN-Status"></a>

### Status


| Name | Number | Description |
| ---- | ------ | ----------- |
| RUNNING | 0 |  |
| OUT_OF_SYNC | 1 |  |
| WAITING_FOR_PEERS | 2 |  |
| STARTING | 3 |  |


 

 


<a name="NODEADMIN-NodeAdmin"></a>

### NodeAdmin


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetPeers | [Empty](#NODEADMIN-Empty) | [GetPeersResponse](#NODEADMIN-GetPeersResponse) |  |
| GetVersion | [Empty](#NODEADMIN-Empty) | [GetVersionResponse](#NODEADMIN-GetVersionResponse) |  |
| UpgradeNode | [UpgradeNodeRequest](#NODEADMIN-UpgradeNodeRequest) | [Empty](#NODEADMIN-Empty) |  |
| GetPendingBlocks | [Empty](#NODEADMIN-Empty) | [GetPendingBlocksReply](#NODEADMIN-GetPendingBlocksReply) |  |
| GetBlockPoolSize | [Empty](#NODEADMIN-Empty) | [GetBlockPoolSizeReply](#NODEADMIN-GetBlockPoolSizeReply) |  |
| GetNodeStatus | [Empty](#NODEADMIN-Empty) | [GetNodeStatusReply](#NODEADMIN-GetNodeStatusReply) |  |
| GetMode | [Empty](#NODEADMIN-Empty) | [GetModeReply](#NODEADMIN-GetModeReply) |  |
| SetMode | [SetModeRequest](#NODEADMIN-SetModeRequest) | [Empty](#NODEADMIN-Empty) |  |
| GetNodeName | [Empty](#NODEADMIN-Empty) | [GetNodeNameReply](#NODEADMIN-GetNodeNameReply) |  |
| SetNodeName | [SetNodeNameRequest](#NODEADMIN-SetNodeNameRequest) | [Empty](#NODEADMIN-Empty) |  |
| GetChain | [Empty](#NODEADMIN-Empty) | [GetChainReply](#NODEADMIN-GetChainReply) |  |
| GetLogs | [GetLogsRequest](#NODEADMIN-GetLogsRequest) | [GetLogsReply](#NODEADMIN-GetLogsReply) |  |

 



## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |

