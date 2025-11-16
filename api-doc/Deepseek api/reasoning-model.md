# Reasoning Model (deepseek-reasoner)

## Overview

The `deepseek-reasoner` model is DeepSeek's advanced reasoning model that generates Chain of Thought (CoT) analysis before providing final answers. This enables transparent, step-by-step problem solving for complex tasks.

## Key Features

- **Chain of Thought**: Explicit reasoning process before answers
- **Extended Output**: Up to 64K tokens for complex reasoning
- **Transparent Logic**: View the model's thinking process
- **Distillation Ready**: Extract and analyze reasoning patterns

---

## Model Specifications

**Model Name**: `deepseek-reasoner` (DeepSeek-V3.2-Exp)

**Context Window**: 128,000 tokens

**Output Limits**:
- Default: 32,768 tokens
- Maximum: 65,536 tokens

**Pricing**:
- Input (cache miss): $0.28 per 1M tokens
- Input (cache hit): $0.028 per 1M tokens
- Output: $0.42 per 1M tokens

---

## Output Structure

The model provides two distinct content fields:

### reasoning_content

The Chain of Thought analysis - the model's internal reasoning process.

```json
{
  "reasoning_content": "Let me think through this step by step:\n1. First, I need to...\n2. Then, I should consider...\n3. Finally..."
}
```

### content

The final answer presented to the user.

```json
{
  "content": "Based on my analysis, the answer is..."
}
```

---

## Basic Usage

### Python Example

```python
from openai import OpenAI

# Upgrade SDK first: pip3 install -U openai

client = OpenAI(
    api_key="YOUR_API_KEY",
    base_url="https://api.deepseek.com"
)

response = client.chat.completions.create(
    model="deepseek-reasoner",
    messages=[
        {
            "role": "user",
            "content": "If a train travels 120 km in 2 hours, then speeds up and travels 180 km in the next 1.5 hours, what is the average speed for the entire journey?"
        }
    ],
    max_tokens=32768
)

# Access the reasoning process
print("Reasoning:")
print(response.choices[0].message.reasoning_content)

print("\nFinal Answer:")
print(response.choices[0].message.content)
```

### Example Output

**reasoning_content**:
```
Let me break this down step by step:

1. First segment: 120 km in 2 hours
   - Speed = 120 / 2 = 60 km/h

2. Second segment: 180 km in 1.5 hours
   - Speed = 180 / 1.5 = 120 km/h

3. Total distance: 120 + 180 = 300 km

4. Total time: 2 + 1.5 = 3.5 hours

5. Average speed = Total distance / Total time
   - Average speed = 300 / 3.5 ≈ 85.71 km/h
```

**content**:
```
The average speed for the entire journey is approximately 85.71 km/h.
```

---

## Parameters

### Supported Parameters

| Parameter | Support | Notes |
|-----------|---------|-------|
| `max_tokens` | ✓ | Default 32K, max 64K (includes reasoning) |
| `response_format` | ✓ | JSON mode supported |
| `stream` | ✓ | Streaming supported |
| `stop` | ✓ | Stop sequences |
| `temperature` | ✗ | No effect (doesn't error) |
| `top_p` | ✗ | No effect (doesn't error) |
| `presence_penalty` | ✗ | No effect (doesn't error) |
| `frequency_penalty` | ✗ | No effect (doesn't error) |
| `logprobs` | ✗ | Errors if used |
| `top_logprobs` | ✗ | Errors if used |

**Important**: temperature, top_p, presence_penalty, and frequency_penalty are ignored but won't cause errors. logprobs and top_logprobs will cause errors.

---

## Multi-Round Conversations

### Important: Handling reasoning_content

**Critical**: Previous reasoning content is **not** included in context for subsequent turns. You must remove the `reasoning_content` field before sending messages back.

### Correct Implementation

```python
messages = [
    {"role": "user", "content": "What is 15 * 23?"}
]

# First turn
response = client.chat.completions.create(
    model="deepseek-reasoner",
    messages=messages
)

# Add assistant message WITHOUT reasoning_content
assistant_message = {
    "role": "assistant",
    "content": response.choices[0].message.content
    # Do NOT include reasoning_content here
}
messages.append(assistant_message)

# Second turn
messages.append({"role": "user", "content": "Now multiply that by 2"})

response = client.chat.completions.create(
    model="deepseek-reasoner",
    messages=messages
)
```

### What Happens If You Include reasoning_content

Including `reasoning_content` in subsequent requests will cause a **400 error**.

```python
# ❌ WRONG - Will cause 400 error
assistant_message = {
    "role": "assistant",
    "content": response.choices[0].message.content,
    "reasoning_content": response.choices[0].message.reasoning_content  # Don't do this!
}
```

---

## Supported Features

### ✓ JSON Output

The reasoning model supports JSON mode:

```python
response = client.chat.completions.create(
    model="deepseek-reasoner",
    messages=[
        {
            "role": "system",
            "content": "Solve the problem and return JSON with 'reasoning_steps' and 'answer' fields"
        },
        {
            "role": "user",
            "content": "What is the square root of 144?"
        }
    ],
    response_format={'type': 'json_object'},
    max_tokens=10000
)
```

### ✓ Chat Prefix Completion (Beta)

Supported but in beta stage.

### ✓ Streaming

```python
response = client.chat.completions.create(
    model="deepseek-reasoner",
    messages=[
        {"role": "user", "content": "Explain quantum entanglement"}
    ],
    stream=True
)

for chunk in response:
    if hasattr(chunk.choices[0].delta, 'reasoning_content'):
        print(chunk.choices[0].delta.reasoning_content, end='')
    if hasattr(chunk.choices[0].delta, 'content'):
        print(chunk.choices[0].delta.content, end='')
```

---

## Unsupported Features

### ✗ Function Calling

The reasoning model does not support function calling. If you send a function calling request to `deepseek-reasoner`, it will be automatically processed using `deepseek-chat` instead.

```python
# This will use deepseek-chat instead
response = client.chat.completions.create(
    model="deepseek-reasoner",
    messages=[{"role": "user", "content": "What's the weather?"}],
    tools=[weather_tool]  # Triggers automatic fallback to deepseek-chat
)
```

### ✗ Fill-in-the-Middle (FIM)

FIM completion is not supported in beta stage.

---

## Use Cases

### Complex Problem Solving

```python
response = client.chat.completions.create(
    model="deepseek-reasoner",
    messages=[{
        "role": "user",
        "content": "A farmer has chickens and rabbits. There are 30 heads and 88 legs total. How many chickens and how many rabbits are there?"
    }]
)
```

### Mathematical Reasoning

```python
response = client.chat.completions.create(
    model="deepseek-reasoner",
    messages=[{
        "role": "user",
        "content": "Prove that the sum of angles in a triangle is 180 degrees"
    }]
)
```

### Logic Puzzles

```python
response = client.chat.completions.create(
    model="deepseek-reasoner",
    messages=[{
        "role": "user",
        "content": "There are 5 houses in 5 different colors. In each house lives a person with a different nationality. The 5 owners drink a certain beverage, smoke a certain brand of cigar and keep a certain pet. Who owns the fish?"
    }]
)
```

### Step-by-Step Analysis

```python
response = client.chat.completions.create(
    model="deepseek-reasoner",
    messages=[{
        "role": "user",
        "content": "Analyze the pros and cons of implementing microservices architecture for a e-commerce platform"
    }]
)
```

---

## Best Practices

### 1. Use Appropriate Token Limits

Reasoning requires more tokens. Set `max_tokens` generously:

```python
max_tokens=32768  # Default - good for most cases
max_tokens=65536  # Maximum - for very complex reasoning
```

### 2. Extract Reasoning for Analysis

Save reasoning content for analysis and improvement:

```python
reasoning = response.choices[0].message.reasoning_content
# Store for training, analysis, or display
```

### 3. Clean Messages for Multi-Turn

Always remove `reasoning_content` before continuing conversations:

```python
clean_message = {
    "role": "assistant",
    "content": response.choices[0].message.content
}
```

### 4. Choose the Right Model

Use `deepseek-reasoner` when:
- Complex reasoning is needed
- Step-by-step thinking is valuable
- Transparency in logic is important
- Quality is more important than speed

Use `deepseek-chat` when:
- Fast responses are needed
- Simple queries
- Function calling is required
- Cost optimization is priority

### 5. Handle Streaming Properly

Both reasoning and content can be streamed:

```python
reasoning_buffer = []
content_buffer = []

for chunk in response:
    delta = chunk.choices[0].delta

    if hasattr(delta, 'reasoning_content') and delta.reasoning_content:
        reasoning_buffer.append(delta.reasoning_content)

    if hasattr(delta, 'content') and delta.content:
        content_buffer.append(delta.content)

reasoning = ''.join(reasoning_buffer)
content = ''.join(content_buffer)
```

---

## SDK Requirements

Ensure you have the latest OpenAI SDK:

```bash
pip3 install -U openai
```

The `reasoning_content` field requires newer SDK versions.

---

## Cost Considerations

Reasoning mode uses more output tokens than standard mode:

**Example Cost Calculation:**
```
Input: 100 tokens (cache miss) = $0.000028
Output (reasoning + answer): 5000 tokens = $0.0021
Total: ~$0.002128 per request
```

Consider using cache hits for repeated prompts to reduce costs by 90%.

---

## Related Documentation

- [Models and Pricing](./models-and-pricing.md) - Detailed pricing information
- [JSON Mode](./json-mode.md) - Using JSON with reasoning
- [API Reference](./api-reference.md) - Complete API documentation
