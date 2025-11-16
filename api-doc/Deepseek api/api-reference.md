# API Reference

## Create Chat Completion

Creates a model response for a chat conversation.

### Endpoint

```
POST /chat/completions
```

**Base URL**: `https://api.deepseek.com`

### Authentication

```
Authorization: Bearer YOUR_API_KEY
```

---

## Request Parameters

### Required Parameters

#### messages

**Type**: Array of objects (minimum 1)

Array of conversation messages. Each message has one of these structures:

**System Message:**
```json
{
  "role": "system",
  "content": "string"
}
```

**User Message:**
```json
{
  "role": "user",
  "content": "string"
}
```

**Assistant Message:**
```json
{
  "role": "assistant",
  "content": "string or null"
}
```

**Tool Message:**
```json
{
  "role": "tool",
  "content": "string",
  "tool_call_id": "string"
}
```

#### model

**Type**: String

**Options**: `deepseek-chat` or `deepseek-reasoner`

The model to use for completion.

---

### Optional Parameters

#### temperature

**Type**: Number
**Range**: 0-2
**Default**: 1

Controls randomness in the output. Higher values make output more random, lower values make it more focused and deterministic.

#### top_p

**Type**: Number
**Range**: 0-1
**Default**: 1

Nucleus sampling parameter. Alternative to temperature for controlling randomness.

#### max_tokens

**Type**: Integer
**Default**: Model-dependent (4K for chat, 32K for reasoner)

Maximum number of tokens to generate in the completion.

#### frequency_penalty

**Type**: Number
**Range**: -2 to 2
**Default**: 0

Penalizes tokens based on their frequency in the text so far. Positive values reduce repetition.

#### presence_penalty

**Type**: Number
**Range**: -2 to 2
**Default**: 0

Penalizes tokens that have already appeared in the text. Positive values encourage topic diversity.

#### stream

**Type**: Boolean
**Default**: false

Whether to stream the response as server-sent events.

#### logprobs

**Type**: Boolean
**Default**: false

Whether to return log probabilities of output tokens.

#### top_logprobs

**Type**: Integer
**Range**: 0-20

Number of most likely tokens to return with their log probabilities. Requires `logprobs: true`.

#### response_format

**Type**: Object
**Default**: `{type: "text"}`

**Options**:
```json
{
  "type": "text"
}
```
or
```json
{
  "type": "json_object"
}
```

Controls output format. Use `json_object` for structured JSON output.

#### tools

**Type**: Array of objects
**Maximum**: 128 functions

Array of functions the model can call. See [Function Calling](./function-calling.md) for details.

#### tool_choice

**Type**: String or Object
**Options**: `"none"`, `"auto"`, `"required"`, or object specifying function
**Default**: `"auto"` (when tools provided) or `"none"` (no tools)

Controls how the model uses tools.

#### stop

**Type**: String or Array of strings
**Maximum**: 16 sequences

Stop generation when these sequences are encountered.

---

## Response Format

### Success Response (200)

**Non-streaming:**

```json
{
  "id": "chatcmpl-123",
  "object": "chat.completion",
  "created": 1234567890,
  "model": "deepseek-chat",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "Hello! How can I help you today?",
        "tool_calls": null
      },
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": 20,
    "completion_tokens": 10,
    "total_tokens": 30,
    "prompt_cache_hit_tokens": 0,
    "prompt_cache_miss_tokens": 20
  }
}
```

### Response Fields

#### choices

Array of completion choices (usually contains one element).

**Fields:**
- `index`: Choice index
- `message`: The generated message
- `finish_reason`: Why generation stopped

**Finish Reasons:**
- `stop`: Natural completion
- `length`: Reached max_tokens limit
- `content_filter`: Content was filtered
- `tool_calls`: Model wants to call a function
- `insufficient_system_resource`: System resource exhausted

#### usage

Token usage statistics.

**Fields:**
- `prompt_tokens`: Total input tokens
- `completion_tokens`: Total output tokens
- `total_tokens`: Sum of prompt and completion tokens
- `prompt_cache_hit_tokens`: Input tokens retrieved from cache
- `prompt_cache_miss_tokens`: Input tokens not in cache

---

## Streaming Response

When `stream: true`, the response is sent as server-sent events:

```
data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1234567890,"model":"deepseek-chat","choices":[{"index":0,"delta":{"role":"assistant","content":"Hello"},"finish_reason":null}]}

data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1234567890,"model":"deepseek-chat","choices":[{"index":0,"delta":{"content":"!"},"finish_reason":null}]}

data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1234567890,"model":"deepseek-chat","choices":[{"index":0,"delta":{},"finish_reason":"stop"}]}

data: [DONE]
```

**Key Differences:**
- Object type is `chat.completion.chunk`
- Messages contain `delta` instead of `message`
- Stream ends with `data: [DONE]`

---

## Example Requests

### Basic Chat

```python
from openai import OpenAI

client = OpenAI(
    api_key="YOUR_API_KEY",
    base_url="https://api.deepseek.com"
)

response = client.chat.completions.create(
    model="deepseek-chat",
    messages=[
        {"role": "system", "content": "You are a helpful assistant."},
        {"role": "user", "content": "What is the capital of France?"}
    ]
)

print(response.choices[0].message.content)
```

### With Temperature Control

```python
response = client.chat.completions.create(
    model="deepseek-chat",
    messages=[
        {"role": "user", "content": "Write a creative story"}
    ],
    temperature=1.5,
    max_tokens=500
)
```

### Streaming Response

```python
response = client.chat.completions.create(
    model="deepseek-chat",
    messages=[
        {"role": "user", "content": "Count to 10"}
    ],
    stream=True
)

for chunk in response:
    if chunk.choices[0].delta.content:
        print(chunk.choices[0].delta.content, end='')
```

### JSON Output

```python
response = client.chat.completions.create(
    model="deepseek-chat",
    messages=[
        {"role": "system", "content": "Extract information as JSON"},
        {"role": "user", "content": "John is 30 years old and lives in Paris"}
    ],
    response_format={"type": "json_object"}
)
```

---

## Error Responses

See [Error Codes](./error-codes.md) for complete error handling documentation.

Common errors:
- `400`: Invalid request format
- `401`: Authentication failed
- `422`: Invalid parameters
- `429`: Rate limit exceeded
- `500`: Server error
- `503`: Service overloaded
