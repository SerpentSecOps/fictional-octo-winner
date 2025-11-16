# Context Caching

## Overview

DeepSeek's context caching system automatically caches frequently used content to reduce costs by up to 90%.

## How It Works

When you send similar prompts or reuse content:
1. **First Request**: Content is processed normally (cache miss)
2. **Subsequent Requests**: Cached content is retrieved (cache hit)
3. **Cost Savings**: Cache hits cost only $0.028 per 1M tokens vs $0.28

## Cost Comparison

| Cache Status | Cost per 1M Tokens | Savings |
|--------------|-------------------|---------|
| **Cache Miss** | $0.28 | Baseline |
| **Cache Hit** | $0.028 | **90% off** |

## API Response Format

The API response includes detailed cache usage statistics:

```json
{
  "usage": {
    "prompt_tokens": 15000,
    "completion_tokens": 500,
    "total_tokens": 15500,
    "prompt_cache_hit_tokens": 12000,
    "prompt_cache_miss_tokens": 3000
  }
}
```

### Field Descriptions

- **prompt_tokens**: Total input tokens
- **completion_tokens**: Total output tokens
- **total_tokens**: Sum of prompt + completion
- **prompt_cache_hit_tokens**: Tokens retrieved from cache
- **prompt_cache_miss_tokens**: Tokens not in cache

## Example Calculation

**Scenario**: Processing a 15,000-token prompt with 12,000 tokens cached

**Cache Hit Cost**:
```
12,000 / 1,000,000 × $0.028 = $0.000336
```

**Cache Miss Cost**:
```
3,000 / 1,000,000 × $0.28 = $0.00084
```

**Total Input Cost**: $0.001176

**Without Caching**:
```
15,000 / 1,000,000 × $0.28 = $0.0042
```

**Savings**: $0.0042 - $0.001176 = **$0.003024 (72% savings)**

## Use Cases for Caching

### Document Processing
Process the same document multiple times with different queries:
```python
base_document = "..." # Large document (10,000 tokens)

# First query - cache miss
response1 = client.chat.completions.create(
    model="deepseek-chat",
    messages=[
        {"role": "system", "content": base_document},
        {"role": "user", "content": "Summarize this"}
    ]
)

# Second query - cache hit!
response2 = client.chat.completions.create(
    model="deepseek-chat",
    messages=[
        {"role": "system", "content": base_document},
        {"role": "user", "content": "Extract key points"}
    ]
)
```

### System Prompts
Reuse the same system prompt across many requests:
```python
system_prompt = "You are an expert Python programmer..."

# All requests with this system prompt benefit from caching
for user_query in queries:
    response = client.chat.completions.create(
        model="deepseek-chat",
        messages=[
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_query}
        ]
    )
```

### Multi-Turn Conversations
Maintain conversation history efficiently:
```python
conversation = [
    {"role": "system", "content": "You are helpful"},
    {"role": "user", "content": "Hello"},
    {"role": "assistant", "content": "Hi there!"},
    # Previous messages cached in subsequent turns
]

conversation.append({"role": "user", "content": "Tell me about AI"})
response = client.chat.completions.create(
    model="deepseek-chat",
    messages=conversation
)
```

## Best Practices

### 1. Reuse Content
Keep consistent prompts and system messages to maximize cache hits.

### 2. Structure Prompts Strategically
Place reusable content at the beginning:
```python
# Good - cacheable content first
messages = [
    {"role": "system", "content": long_instructions},  # Cached
    {"role": "user", "content": variable_query}        # New each time
]
```

### 3. Monitor Cache Performance
Track cache hit rates in your usage statistics:
```python
usage = response.usage
cache_hit_rate = usage.prompt_cache_hit_tokens / usage.prompt_tokens
print(f"Cache hit rate: {cache_hit_rate * 100:.1f}%")
```

### 4. Batch Similar Requests
Group requests with similar content together to maximize caching benefits.

## Cache Behavior

### What Gets Cached
- System prompts
- User messages
- Document content
- Conversation history

### Cache Duration
- Automatic cache management
- Recently used content stays cached
- Rarely used content may be evicted

### Cache Scope
- Per-account caching
- Not shared across accounts
- Automatic and transparent
