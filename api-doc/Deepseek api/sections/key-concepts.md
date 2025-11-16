# Key Concepts

## Messages

Conversations are represented as arrays of message objects with different roles:

### System Messages
Sets the assistant's behavior and personality:
```json
{"role": "system", "content": "You are a helpful assistant."}
```

### User Messages
User's input or questions:
```json
{"role": "user", "content": "What is Python?"}
```

### Assistant Messages
Model's previous responses (for multi-turn conversations):
```json
{"role": "assistant", "content": "Python is a programming language..."}
```

### Tool Messages
Results from function calls:
```json
{"role": "tool", "content": "{\"temperature\": 72}", "tool_call_id": "call_123"}
```

## Models

### deepseek-chat
- Standard chat model for general tasks
- 128K context window
- Max output: 4K tokens (default) / 8K tokens (maximum)
- Supports function calling, JSON output, FIM

### deepseek-reasoner
- Reasoning model with Chain of Thought analysis
- 128K context window
- Max output: 32K tokens (default) / 64K tokens (maximum)
- Best for complex reasoning tasks

## Common Parameters

### temperature (0-2)
Controls randomness in the output:
- **0**: Deterministic, focused
- **1**: Balanced (default)
- **2**: More creative, random

### max_tokens
Maximum number of tokens to generate:
```python
max_tokens=500  # Limit response length
```

### top_p (0-1)
Nucleus sampling parameter (alternative to temperature):
- **0.1**: Very focused
- **1.0**: Full distribution (default)

### stream (boolean)
Enable real-time streaming responses:
```python
stream=True  # Enable streaming
stream=False  # Wait for complete response (default)
```

## Token Usage

Understanding tokens:
- 1 token ≈ 4 characters in English
- 1 token ≈ ¾ of a word
- Both input and output consume tokens

Example:
- "Hello, how are you?" ≈ 6 tokens
- Longer responses cost more

## Context Window

- Maximum total tokens (input + output) per request
- deepseek-chat: 128,000 tokens
- deepseek-reasoner: 128,000 tokens

## Next Steps

- Learn about [Models and Pricing](./models-and-pricing.md)
- Explore [Function Calling](./function-calling.md)
- Review [API Reference](./api-reference.md)
