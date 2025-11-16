# deepseek-chat Model

## Overview

**deepseek-chat (DeepSeek-V3.2-Exp)** - Standard chat completion model for general-purpose tasks.

## Specifications

- **Context Window**: 128,000 tokens
- **Max Output**: 4,096 tokens (default) / 8,192 tokens (maximum)
- **Mode**: Non-thinking mode (direct responses)

## Supported Features

### JSON Output Formatting
Generate structured JSON responses for data extraction and API integration.

### Function Calling
Integrate with external tools and APIs through function calls.

### Chat Prefix Completion
Continue existing conversations or complete partial responses.

### Fill-in-the-Middle (FIM) Completion
Complete code or text with context from both before and after the insertion point.

## Use Cases

### General Conversation
- Customer support chatbots
- Virtual assistants
- Q&A systems

### Content Generation
- Article writing
- Email composition
- Creative writing

### Structured Data Extraction
- Parse unstructured text
- Extract entities
- Generate JSON from descriptions

### Tool Integration
- API integration
- Database queries
- External service calls

## Example Usage

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
        {"role": "user", "content": "Explain quantum computing"}
    ],
    temperature=0.7,
    max_tokens=500
)

print(response.choices[0].message.content)
```

## Performance Characteristics

- **Speed**: Fast response times
- **Cost**: Lower cost per token
- **Quality**: High quality for general tasks
- **Reliability**: Consistent outputs

## When to Use

Choose **deepseek-chat** when:
- You need fast responses
- Using function calling features
- Standard conversation or generation tasks
- Cost optimization is a priority
- You don't need explicit reasoning steps
