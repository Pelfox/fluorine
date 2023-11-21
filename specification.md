# Protocol specification

## Available types

Everything is sent in **big-endian** ordering, if other isn't specified. For integer types,
see [Rust's integer types specification](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types).

### String

Representation of a string in bytes, always prefixed with its length.

#### Example:

* Input: `Hello`
* Output: `[5, 72, 101, 108, 108, 111]`

### Byte Array

Array of bytes of any length. Isn't prefixed with length.

## Packet structure

Packet can be *compressed* or *uncompressed*. Which type the all packets will be is specified
in the [`Handshake`](#handshake-c-s) packet.

:warning: First packet, which is [`Handshake`](#handshake-c-s), is always sent in uncompressed format.

| Name | Type                      | Notes                                 |
|------|---------------------------|---------------------------------------|
| Size | `u8`                      | Total length of `ID` + `Data` fields. |
| ID   | `u8`                      | [Unique packet ID](#packet-ids).      |
| Data | [Byte Array](#byte-array) | Depends on each packet.               |

## Packets

### Packet IDs

Direction shows which direction packet is sent.

* `C -> S` means that packet is sent from client to server.
* `S -> C` is the opposite of `C -> S`.

| ID  | Name                                | Direction |
|-----|-------------------------------------|-----------|
| 0x0 | [Handshake](#handshake-c-s)         | C -> S    |
| 0x1 | [Handshake ACK](#handshake-ack-s-c) | S -> C    |
|     |                                     |           |
|     |                                     |           |

### Handshake (C->S)

Sent by the client on the first connection to the server.

| Name                    | Type              | Notes                                                |
|-------------------------|-------------------|------------------------------------------------------|
| `version`               | [string](#string) | Version of a client.                                 |
| `enable_compression`    | `boolean`         | Should server enable the compression?                |
| `compression_threshold` | `i64`             | After which packet size compression will be enabled? |


### Handshake ACK (S->C)

Sent by the server when client's handshake has been acknowledged. Doesn't contain any fields.
