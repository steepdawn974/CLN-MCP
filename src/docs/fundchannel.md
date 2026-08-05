fundchannel -- Command to open a channel with a connected peer

The **fundchannel** RPC command opens a payment channel with a connected peer by funding it with on-chain funds.

- **id** (pubkey): The pubkey of the peer to open a channel with
- **amount** (string): Amount in satoshis to fund the channel
- **feerate** (string, optional): Fee rate for the funding transaction
- **announce** (bool, optional): Whether to announce the channel publicly (default true)
