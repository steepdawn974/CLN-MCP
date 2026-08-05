close -- Command to close a channel

The **close** RPC command closes a channel with a peer.

- **id** (pubkey/channel_id): The pubkey or channel_id of the channel to close
- **unilateraltimeout** (u64, optional): Timeout in seconds for unilateral close (0 = immediate)
- **fee_negotiation_step** (string, optional): Fee negotiation step
