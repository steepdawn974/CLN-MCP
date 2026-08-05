call_rpc_method -- Generic tool to call any CLN RPC method

This tool allows calling any Core Lightning RPC method directly. Use this for methods that don't have a dedicated tool.

- **method** (string): The CLN RPC method name (e.g. "getinfo", "listpeers", "invoice", "sendpay")
- **params** (object, optional): Parameters as a JSON object. Use null for no parameters.
