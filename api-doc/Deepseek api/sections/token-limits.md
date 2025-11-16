# Token Limits

## Overview

Token limits define the maximum number of tokens that can be processed in a single API request, including both input (prompt) and output (completion).

## deepseek-chat Limits

| Limit Type | Value | Notes |
|-----------|-------|-------|
| **Context Window** | 128,000 tokens | Total input + output |
| **Default Max Output** | 4,096 tokens | Standard generation |
| **Maximum Max Output** | 8,192 tokens | Configurable limit |

### Example Configuration

```python
response = client.chat.completions.create(
    model="deepseek-chat",
    messages=[...],
    max_tokens=8192  # Maximum allowed
)
```

## deepseek-reasoner Limits

| Limit Type | Value | Notes |
|-----------|-------|-------|
| **Context Window** | 128,000 tokens | Total input + output |
| **Default Max Output** | 32,768 tokens | Includes reasoning |
| **Maximum Max Output** | 65,536 tokens | Configurable limit |

### Example Configuration

```python
response = client.chat.completions.create(
    model="deepseek-reasoner",
    messages=[...],
    max_tokens=65536  # Maximum allowed
)
```

## Understanding Context Window

The context window is the total capacity for both input and output tokens:

```
Context Window = Input Tokens + Output Tokens
```

### Example Scenario

**deepseek-chat** (128K context window):
- Input: 120,000 tokens
- Available for output: 8,000 tokens (capped at max 8,192)

**deepseek-reasoner** (128K context window):
- Input: 100,000 tokens
- Available for output: 28,000 tokens (up to 65,536 if within context)

## Setting max_tokens

### Optimal Settings

**For Short Responses** (Q&A, summaries):
```python
max_tokens=500  # Quick, concise answers
```

**For Medium Responses** (Explanations, descriptions):
```python
max_tokens=2000  # Detailed explanations
```

**For Long Responses** (Articles, analysis):
```python
max_tokens=4096  # deepseek-chat
max_tokens=32768  # deepseek-reasoner
```

## Token Estimation

### Character to Token Conversion

Approximate guidelines for English text:
- **1 token** ≈ 4 characters
- **1 token** ≈ ¾ of a word
- **100 tokens** ≈ 75 words ≈ 300 characters

### Common Text Lengths

| Text Type | Approx Words | Approx Tokens |
|-----------|--------------|---------------|
| Tweet | 40 | 53 |
| Paragraph | 100 | 133 |
| Email | 250 | 333 |
| Blog post | 1,000 | 1,333 |
| Article | 2,000 | 2,667 |
| Document | 10,000 | 13,333 |

## Handling Token Limits

### Check Token Count

```python
import tiktoken

# Estimate tokens in text
def count_tokens(text):
    encoding = tiktoken.encoding_for_model("gpt-4")
    return len(encoding.encode(text))

prompt = "Your long prompt here..."
token_count = count_tokens(prompt)
print(f"Prompt uses {token_count} tokens")
```

### Truncate Long Inputs

```python
def truncate_to_limit(text, max_tokens=100000):
    encoding = tiktoken.encoding_for_model("gpt-4")
    tokens = encoding.encode(text)

    if len(tokens) > max_tokens:
        tokens = tokens[:max_tokens]

    return encoding.decode(tokens)

safe_prompt = truncate_to_limit(long_document, 100000)
```

### Split Large Documents

```python
def chunk_document(text, chunk_size=50000):
    encoding = tiktoken.encoding_for_model("gpt-4")
    tokens = encoding.encode(text)

    chunks = []
    for i in range(0, len(tokens), chunk_size):
        chunk_tokens = tokens[i:i + chunk_size]
        chunks.append(encoding.decode(chunk_tokens))

    return chunks

# Process document in chunks
for chunk in chunk_document(large_document):
    response = client.chat.completions.create(
        model="deepseek-chat",
        messages=[{"role": "user", "content": chunk}]
    )
```

## Error Handling

### Token Limit Exceeded

```python
from openai import OpenAI, APIError

try:
    response = client.chat.completions.create(
        model="deepseek-chat",
        messages=[{"role": "user", "content": very_long_text}]
    )
except APIError as e:
    if "maximum context length" in str(e).lower():
        print("Token limit exceeded. Reduce input or max_tokens.")
    else:
        raise
```

## Best Practices

1. **Monitor Token Usage**: Track token consumption in responses
2. **Set Reasonable Limits**: Don't max out unnecessarily
3. **Optimize Prompts**: Be concise to save tokens
4. **Use Caching**: Reduce effective token usage
5. **Plan for Limits**: Design systems with token limits in mind
