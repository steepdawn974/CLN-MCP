getroute -- Command to find the best route for a payment to a lightning node.

The **getroute** RPC command returns the best route for sending an amount to a node.

- **id** (pubkey): The destination node pubkey
- **amount_msat** (string): Amount to send in millisatoshi
- **riskfactor** (u64, optional): Risk factor (default 0)
