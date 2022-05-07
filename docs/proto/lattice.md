# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [node/rpc/lattice.proto](#node_rpc_lattice-proto)
    - [BalanceReply](#LATTICE-BalanceReply)
    - [BalanceRequest](#LATTICE-BalanceRequest)
    - [BlockByIDReply](#LATTICE-BlockByIDReply)
    - [BlockByIDRequest](#LATTICE-BlockByIDRequest)
    - [BlockHeightReply](#LATTICE-BlockHeightReply)
    - [BlockHeightRequest](#LATTICE-BlockHeightRequest)
    - [DelegateReply](#LATTICE-DelegateReply)
    - [DelegateRequest](#LATTICE-DelegateRequest)
    - [Empty](#LATTICE-Empty)
    - [GetBlocksReply](#LATTICE-GetBlocksReply)
    - [GetBlocksRequest](#LATTICE-GetBlocksRequest)
    - [PendingBlockReply](#LATTICE-PendingBlockReply)
    - [TXByIDReply](#LATTICE-TXByIDReply)
    - [TXByIDRequest](#LATTICE-TXByIDRequest)
    - [TXByIndexReply](#LATTICE-TXByIndexReply)
    - [TXByIndexRequest](#LATTICE-TXByIndexRequest)
    - [UnacknowledgedTXReply](#LATTICE-UnacknowledgedTXReply)
    - [VotingPowerReply](#LATTICE-VotingPowerReply)
    - [VotingPowerRequest](#LATTICE-VotingPowerRequest)
  
    - [sortBy](#LATTICE-sortBy)
  
    - [Lattice](#LATTICE-Lattice)
  
- [Scalar Value Types](#scalar-value-types)



<a name="node_rpc_lattice-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## node/rpc/lattice.proto



<a name="LATTICE-BalanceReply"></a>

### BalanceReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| balance | [uint64](#uint64) |  | Reply contains the balance queried |






<a name="LATTICE-BalanceRequest"></a>

### BalanceRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [bytes](#bytes) |  | Balance Request takes the address for query |






<a name="LATTICE-BlockByIDReply"></a>

### BlockByIDReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| block | [API.Block](#API-Block) |  |  |






<a name="LATTICE-BlockByIDRequest"></a>

### BlockByIDRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| hash | [bytes](#bytes) |  |  |






<a name="LATTICE-BlockHeightReply"></a>

### BlockHeightReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| nextHeight | [uint64](#uint64) |  |  |






<a name="LATTICE-BlockHeightRequest"></a>

### BlockHeightRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [bytes](#bytes) |  |  |
| getNext | [bool](#bool) |  |  |






<a name="LATTICE-DelegateReply"></a>

### DelegateReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| delegateAddress | [bytes](#bytes) |  |  |






<a name="LATTICE-DelegateRequest"></a>

### DelegateRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [bytes](#bytes) |  |  |






<a name="LATTICE-Empty"></a>

### Empty







<a name="LATTICE-GetBlocksReply"></a>

### GetBlocksReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| blocks | [API.Block](#API-Block) | repeated |  |






<a name="LATTICE-GetBlocksRequest"></a>

### GetBlocksRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| limit | [uint32](#uint32) |  |  |
| offset | [uint32](#uint32) |  |  |
| sortBy | [sortBy](#LATTICE-sortBy) |  |  |






<a name="LATTICE-PendingBlockReply"></a>

### PendingBlockReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| blocks | [API.Block](#API-Block) | repeated |  |






<a name="LATTICE-TXByIDReply"></a>

### TXByIDReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| transaction | [API.Transaction](#API-Transaction) |  |  |






<a name="LATTICE-TXByIDRequest"></a>

### TXByIDRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| transactionID | [bytes](#bytes) |  |  |






<a name="LATTICE-TXByIndexReply"></a>

### TXByIndexReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| transaction | [API.Transaction](#API-Transaction) |  |  |






<a name="LATTICE-TXByIndexRequest"></a>

### TXByIndexRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| height | [uint64](#uint64) |  |  |
| index | [uint64](#uint64) |  |  |






<a name="LATTICE-UnacknowledgedTXReply"></a>

### UnacknowledgedTXReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| transactions | [API.Transaction](#API-Transaction) | repeated |  |






<a name="LATTICE-VotingPowerReply"></a>

### VotingPowerReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| power | [uint64](#uint64) |  |  |






<a name="LATTICE-VotingPowerRequest"></a>

### VotingPowerRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [bytes](#bytes) |  |  |
| getActive | [bool](#bool) |  |  |





 


<a name="LATTICE-sortBy"></a>

### sortBy


| Name | Number | Description |
| ---- | ------ | ----------- |
| data | 0 |  |
| data_desc | 1 |  |


 

 


<a name="LATTICE-Lattice"></a>

### Lattice


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetBalance | [BalanceRequest](#LATTICE-BalanceRequest) | [BalanceReply](#LATTICE-BalanceReply) | GetBalance accepts BalanceRequests and returns BalanceReplies |
| GetBlockHeight | [BlockHeightRequest](#LATTICE-BlockHeightRequest) | [BlockHeightReply](#LATTICE-BlockHeightReply) | GetNextBlockHeight uses an address to get the latest block index to then return the next index |
| GetVotingPower | [VotingPowerRequest](#LATTICE-VotingPowerRequest) | [VotingPowerReply](#LATTICE-VotingPowerReply) | GetVotingPower uses an address to get the voting power of an account |
| GetBlockByID | [BlockByIDRequest](#LATTICE-BlockByIDRequest) | [BlockByIDReply](#LATTICE-BlockByIDReply) | GetBlockByID uses the hash of a block to get the data |
| GetDelegate | [DelegateRequest](#LATTICE-DelegateRequest) | [DelegateReply](#LATTICE-DelegateReply) | GetDelegate uses an account address to return a address of the delegate |
| GetPendingBlocks | [Empty](#LATTICE-Empty) | [PendingBlockReply](#LATTICE-PendingBlockReply) | GetPendingBlocks gets all blocks that havent been processed |
| GetBlocks | [GetBlocksRequest](#LATTICE-GetBlocksRequest) | [GetBlocksReply](#LATTICE-GetBlocksReply) | GetUnacknowledgedTX gets a number of how many transactions are unacknowledged |
| GetUnacknowledgedTX | [Empty](#LATTICE-Empty) | [UnacknowledgedTXReply](#LATTICE-UnacknowledgedTXReply) | GetUnacknowledgedTX gets a number of how many transactions are |
| GetTXByID | [TXByIDRequest](#LATTICE-TXByIDRequest) | [TXByIDReply](#LATTICE-TXByIDReply) | GetTXByID uses a transaction ID to get a transaction |
| GetTXByIndex | [TXByIndexRequest](#LATTICE-TXByIndexRequest) | [TXByIndexReply](#LATTICE-TXByIndexReply) | GetTXByIndex uses an index for inside a block to return a transaction |

 



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

