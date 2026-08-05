setchannel -- Command to update channel fee policy

The **setchannel** RPC command updates the fee policy for a channel.

- **id** (pubkey/short_channel_id/"all"): The channel to update, or "all" for all channels
- **feebase** (string, optional): Base fee in millisatoshi
- **feeppm** (string, optional): Parts per million fee
- **htlcmin** (string, optional): Minimum HTLC in millisatoshi
- **htlcmax** (string, optional): Maximum HTLC in millisatoshi
