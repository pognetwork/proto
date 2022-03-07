# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [node/rpc/block.proto](#node_rpc_block-proto)
    - [BalanceReply](#BLOCK-BalanceReply)
    - [BalanceRequest](#BLOCK-BalanceRequest)
    - [BlockByIDReply](#BLOCK-BlockByIDReply)
    - [BlockByIDRequest](#BLOCK-BlockByIDRequest)
    - [BlockHeightReply](#BLOCK-BlockHeightReply)
    - [BlockHeightRequest](#BLOCK-BlockHeightRequest)
    - [DelegateReply](#BLOCK-DelegateReply)
    - [DelegateRequest](#BLOCK-DelegateRequest)
    - [Empty](#BLOCK-Empty)
    - [PendingBlockReply](#BLOCK-PendingBlockReply)
    - [TXByIDReply](#BLOCK-TXByIDReply)
    - [TXByIDRequest](#BLOCK-TXByIDRequest)
    - [TXByIndexReply](#BLOCK-TXByIndexReply)
    - [TXByIndexRequest](#BLOCK-TXByIndexRequest)
    - [UnacknowledgedTXReply](#BLOCK-UnacknowledgedTXReply)
    - [VotingPowerReply](#BLOCK-VotingPowerReply)
    - [VotingPowerRequest](#BLOCK-VotingPowerRequest)
  
    - [BlockService](#BLOCK-BlockService)
  
- [Scalar Value Types](#scalar-value-types)



<a name="node_rpc_block-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## node/rpc/block.proto



<a name="BLOCK-BalanceReply"></a>

### BalanceReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| balance | [uint64](#uint64) |  | Reply contains the balance queried |






<a name="BLOCK-BalanceRequest"></a>

### BalanceRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [bytes](#bytes) |  | Balance Request takes the address for query |






<a name="BLOCK-BlockByIDReply"></a>

### BlockByIDReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| block | [API.Block](#API-Block) |  |  |






<a name="BLOCK-BlockByIDRequest"></a>

### BlockByIDRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| hash | [bytes](#bytes) |  |  |






<a name="BLOCK-BlockHeightReply"></a>

### BlockHeightReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| nextHeight | [uint64](#uint64) |  |  |






<a name="BLOCK-BlockHeightRequest"></a>

### BlockHeightRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [bytes](#bytes) |  |  |
| getNext | [bool](#bool) |  |  |






<a name="BLOCK-DelegateReply"></a>

### DelegateReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| delegateAddress | [bytes](#bytes) |  |  |






<a name="BLOCK-DelegateRequest"></a>

### DelegateRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [bytes](#bytes) |  |  |






<a name="BLOCK-Empty"></a>

### Empty







<a name="BLOCK-PendingBlockReply"></a>

### PendingBlockReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| blocks | [API.Block](#API-Block) | repeated |  |






<a name="BLOCK-TXByIDReply"></a>

### TXByIDReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| transaction | [API.Transaction](#API-Transaction) |  |  |






<a name="BLOCK-TXByIDRequest"></a>

### TXByIDRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| transactionID | [bytes](#bytes) |  |  |






<a name="BLOCK-TXByIndexReply"></a>

### TXByIndexReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| transaction | [API.Transaction](#API-Transaction) |  |  |






<a name="BLOCK-TXByIndexRequest"></a>

### TXByIndexRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| height | [uint64](#uint64) |  |  |
| index | [uint64](#uint64) |  |  |






<a name="BLOCK-UnacknowledgedTXReply"></a>

### UnacknowledgedTXReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| transactions | [API.Transaction](#API-Transaction) | repeated |  |






<a name="BLOCK-VotingPowerReply"></a>

### VotingPowerReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| power | [uint32](#uint32) |  |  |






<a name="BLOCK-VotingPowerRequest"></a>

### VotingPowerRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [bytes](#bytes) |  |  |
| getActive | [bool](#bool) |  |  |





 

 

 


<a name="BLOCK-BlockService"></a>

### BlockService


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetBalance | [BalanceRequest](#BLOCK-BalanceRequest) | [BalanceReply](#BLOCK-BalanceReply) | GetBalance accepts BalanceRequests and returns BalanceReplies |
| GetBlockHeight | [BlockHeightRequest](#BLOCK-BlockHeightRequest) | [BlockHeightReply](#BLOCK-BlockHeightReply) | GetNextBlockHeight uses an address to get the latest block index to then return the next index |
| GetVotingPower | [VotingPowerRequest](#BLOCK-VotingPowerRequest) | [VotingPowerReply](#BLOCK-VotingPowerReply) | GetVotingPower uses an address to get the voting power of an account |
| GetBlockByID | [BlockByIDRequest](#BLOCK-BlockByIDRequest) | [BlockByIDReply](#BLOCK-BlockByIDReply) | GetBlockByID uses the hash of a block to get the data |
| GetDelegate | [DelegateRequest](#BLOCK-DelegateRequest) | [DelegateReply](#BLOCK-DelegateReply) | GetDelegate uses an account address to return a address of the delegate |
| GetPendingBlocks | [Empty](#BLOCK-Empty) | [PendingBlockReply](#BLOCK-PendingBlockReply) | GetPendingBlocks gets all blocks that havent been processed |
| GetUnacknowledgedTX | [Empty](#BLOCK-Empty) | [UnacknowledgedTXReply](#BLOCK-UnacknowledgedTXReply) | GetUnacknowledgedTX gets a number of how many transactions are unacknowledged |
| GetTXByID | [TXByIDRequest](#BLOCK-TXByIDRequest) | [TXByIDReply](#BLOCK-TXByIDReply) | GetTXByID uses a transaction ID to get a transaction |
| GetTXByIndex | [TXByIndexRequest](#BLOCK-TXByIndexRequest) | [TXByIndexReply](#BLOCK-TXByIndexReply) | GetTXByIndex uses an index for inside a block to return a transaction |

 



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

