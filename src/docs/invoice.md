invoice -- Command to create a bolt11 invoice

The **invoice** RPC command creates a bolt11 invoice that can be paid by another node.

- **amount_msat** (string): Amount in millisatoshi (or "any" for zero-amount invoice)
- **label** (string): Unique label for the invoice
- **description** (string): Description shown on the invoice
- **expiry** (u64, optional): Expiry in seconds (default 3600)
