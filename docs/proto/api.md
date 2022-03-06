# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [node/api.proto](#node_api-proto)
    - [BlockData](#API-BlockData)
    - [BlockHeader](#API-BlockHeader)
    - [SignedBlock](#API-SignedBlock)
    - [Transaction](#API-Transaction)
    - [Transaction.TxClaim](#API-Transaction-TxClaim)
    - [Transaction.TxDelegate](#API-Transaction-TxDelegate)
    - [Transaction.TxOpen](#API-Transaction-TxOpen)
    - [Transaction.TxSend](#API-Transaction-TxSend)
  
    - [AccountType](#API-AccountType)
    - [BlockVersion](#API-BlockVersion)
    - [SigType](#API-SigType)
  
- [Scalar Value Types](#scalar-value-types)



<a name="node_api-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## node/api.proto



<a name="API-BlockData"></a>

### BlockData



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| version | [BlockVersion](#API-BlockVersion) |  |  |
| signatureType | [SigType](#API-SigType) |  |  |
| balance | [uint64](#uint64) |  | the new account balance after applying all transactions |
| height | [uint64](#uint64) |  | the block height (block index) |
| previous | [bytes](#bytes) |  | previous block, empty if first block |
| transactions | [Transaction](#API-Transaction) | repeated | transactions contained in a block - Can&#39;t contain duplicates - Can&#39;t contain more than 255 transactions - Orderd (as per protobuf specification) |






<a name="API-BlockHeader"></a>

### BlockHeader



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| signature | [bytes](#bytes) |  | 64 bytes (for Ed25519, might be expanded later) |
| publicKey | [bytes](#bytes) |  | 32 bytes (for Ed25519) account&#39;s public key this needs to be provided since account addresses are hashed public keys |
| timestamp | [uint64](#uint64) |  | seconds of UTC time since Unix epoch will be set by the first node receiving the transaction |






<a name="API-SignedBlock"></a>

### SignedBlock



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| header | [BlockHeader](#API-BlockHeader) |  |  |
| data | [BlockData](#API-BlockData) |  |  |






<a name="API-Transaction"></a>

### Transaction



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| txOpen | [Transaction.TxOpen](#API-Transaction-TxOpen) |  |  |
| txSend | [Transaction.TxSend](#API-Transaction-TxSend) |  |  |
| txClaim | [Transaction.TxClaim](#API-Transaction-TxClaim) |  |  |
| txDelegate | [Transaction.TxDelegate](#API-Transaction-TxDelegate) |  |  |






<a name="API-Transaction-TxClaim"></a>

### Transaction.TxClaim
Block Claim collects a transaction from another account&#39;s block


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| sendTransactionID | [bytes](#bytes) |  |  |






<a name="API-Transaction-TxDelegate"></a>

### Transaction.TxDelegate
Block delegate assigns a new representative to an account


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| representative | [bytes](#bytes) |  |  |






<a name="API-Transaction-TxOpen"></a>

### Transaction.TxOpen
Initialize a new Account


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type | [AccountType](#API-AccountType) |  |  |






<a name="API-Transaction-TxSend"></a>

### Transaction.TxSend
Block send is a send transaction in the sender&#39;s account


| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| receiver | [bytes](#bytes) |  |  |
| amount | [uint64](#uint64) |  |  |
| data | [bytes](#bytes) |  |  |





 


<a name="API-AccountType"></a>

### AccountType
should be represented as u8

| Name | Number | Description |
| ---- | ------ | ----------- |
| Managed | 0 |  |
| Autonomous | 1 |  |



<a name="API-BlockVersion"></a>

### BlockVersion
should be represented as u8

| Name | Number | Description |
| ---- | ------ | ----------- |
| V1 | 0 |  |



<a name="API-SigType"></a>

### SigType
should be represented as u8

| Name | Number | Description |
| ---- | ------ | ----------- |
| Ed25519 | 0 |  |


 

 

 



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

