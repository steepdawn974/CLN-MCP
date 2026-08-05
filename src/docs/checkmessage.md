checkmessage -- Command to verify a signature on a message

The **checkmessage** RPC command verifies that a message was signed by a specific node.

- **message** (string): The message that was signed
- **signature** (string): The signature (zbase32 encoded)
- **pubkey** (pubkey, optional): The pubkey of the signer. If not provided, the pubkey is recovered from the signature.
