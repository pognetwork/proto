# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [node/rpc/node_wallet_manager.proto](#node_rpc_node_wallet_manager-proto)
    - [AddWalletReply](#NODEWALLETMANAGER-AddWalletReply)
    - [AddWalletRequest](#NODEWALLETMANAGER-AddWalletRequest)
    - [DecryptMessageReply](#NODEWALLETMANAGER-DecryptMessageReply)
    - [DecryptMessageRequest](#NODEWALLETMANAGER-DecryptMessageRequest)
    - [Empty](#NODEWALLETMANAGER-Empty)
    - [EncryptMessageReply](#NODEWALLETMANAGER-EncryptMessageReply)
    - [EncryptMessageRequest](#NODEWALLETMANAGER-EncryptMessageRequest)
    - [GetWalletReply](#NODEWALLETMANAGER-GetWalletReply)
    - [GetWalletRequest](#NODEWALLETMANAGER-GetWalletRequest)
    - [GetWalletsReply](#NODEWALLETMANAGER-GetWalletsReply)
    - [RemoveWalletRequest](#NODEWALLETMANAGER-RemoveWalletRequest)
    - [SignBlockReply](#NODEWALLETMANAGER-SignBlockReply)
    - [SignBlockRequest](#NODEWALLETMANAGER-SignBlockRequest)
    - [SignMessageReply](#NODEWALLETMANAGER-SignMessageReply)
    - [SignMessageRequest](#NODEWALLETMANAGER-SignMessageRequest)
    - [VerifySignatureRequest](#NODEWALLETMANAGER-VerifySignatureRequest)
  
    - [NodeWalletManager](#NODEWALLETMANAGER-NodeWalletManager)
  
- [Scalar Value Types](#scalar-value-types)



<a name="node_rpc_node_wallet_manager-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## node/rpc/node_wallet_manager.proto



<a name="NODEWALLETMANAGER-AddWalletReply"></a>

### AddWalletReply







<a name="NODEWALLETMANAGER-AddWalletRequest"></a>

### AddWalletRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| username | [string](#string) |  |  |
| password | [string](#string) |  |  |






<a name="NODEWALLETMANAGER-DecryptMessageReply"></a>

### DecryptMessageReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| decrypted_message | [string](#string) |  |  |






<a name="NODEWALLETMANAGER-DecryptMessageRequest"></a>

### DecryptMessageRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| encrypted_message | [string](#string) |  |  |
| address | [bytes](#bytes) |  |  |






<a name="NODEWALLETMANAGER-Empty"></a>

### Empty







<a name="NODEWALLETMANAGER-EncryptMessageReply"></a>

### EncryptMessageReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| encrypted_message | [string](#string) |  |  |






<a name="NODEWALLETMANAGER-EncryptMessageRequest"></a>

### EncryptMessageRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unencrypted_message | [string](#string) |  |  |
| address | [bytes](#bytes) |  |  |






<a name="NODEWALLETMANAGER-GetWalletReply"></a>

### GetWalletReply







<a name="NODEWALLETMANAGER-GetWalletRequest"></a>

### GetWalletRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [bytes](#bytes) |  |  |






<a name="NODEWALLETMANAGER-GetWalletsReply"></a>

### GetWalletsReply







<a name="NODEWALLETMANAGER-RemoveWalletRequest"></a>

### RemoveWalletRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| address | [bytes](#bytes) |  |  |






<a name="NODEWALLETMANAGER-SignBlockReply"></a>

### SignBlockReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| signed_block | [API.SignedBlock](#API-SignedBlock) |  |  |






<a name="NODEWALLETMANAGER-SignBlockRequest"></a>

### SignBlockRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unsigned_block | [API.SignedBlock](#API-SignedBlock) |  |  |
| address | [bytes](#bytes) |  |  |






<a name="NODEWALLETMANAGER-SignMessageReply"></a>

### SignMessageReply



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| signed_data | [string](#string) |  |  |






<a name="NODEWALLETMANAGER-SignMessageRequest"></a>

### SignMessageRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unsigned_data | [string](#string) |  |  |
| address | [bytes](#bytes) |  |  |






<a name="NODEWALLETMANAGER-VerifySignatureRequest"></a>

### VerifySignatureRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data | [string](#string) |  |  |
| address | [bytes](#bytes) |  |  |





 

 

 


<a name="NODEWALLETMANAGER-NodeWalletManager"></a>

### NodeWalletManager


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetWallets | [Empty](#NODEWALLETMANAGER-Empty) | [GetWalletsReply](#NODEWALLETMANAGER-GetWalletsReply) |  |
| GetWallet | [GetWalletRequest](#NODEWALLETMANAGER-GetWalletRequest) | [GetWalletReply](#NODEWALLETMANAGER-GetWalletReply) |  |
| AddWallet | [AddWalletRequest](#NODEWALLETMANAGER-AddWalletRequest) | [AddWalletReply](#NODEWALLETMANAGER-AddWalletReply) |  |
| RemoveWallet | [RemoveWalletRequest](#NODEWALLETMANAGER-RemoveWalletRequest) | [Empty](#NODEWALLETMANAGER-Empty) |  |
| SignMessage | [SignMessageRequest](#NODEWALLETMANAGER-SignMessageRequest) | [SignMessageReply](#NODEWALLETMANAGER-SignMessageReply) |  |
| SignBlock | [SignBlockRequest](#NODEWALLETMANAGER-SignBlockRequest) | [SignBlockReply](#NODEWALLETMANAGER-SignBlockReply) |  |
| VerifySignature | [VerifySignatureRequest](#NODEWALLETMANAGER-VerifySignatureRequest) | [Empty](#NODEWALLETMANAGER-Empty) |  |
| EncryptMessage | [EncryptMessageRequest](#NODEWALLETMANAGER-EncryptMessageRequest) | [EncryptMessageReply](#NODEWALLETMANAGER-EncryptMessageReply) |  |
| DecryptMessage | [DecryptMessageRequest](#NODEWALLETMANAGER-DecryptMessageRequest) | [DecryptMessageReply](#NODEWALLETMANAGER-DecryptMessageReply) |  |

 



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

