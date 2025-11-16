# Getting Started with DeepSeek API

## Authentication Setup

### Obtaining Your API Key

1. Visit the DeepSeek platform dashboard
2. Navigate to the API keys section
3. Generate a new API key
4. Store it securely (never commit to version control)

### Base URLs

- **Standard**: `https://api.deepseek.com`
- **OpenAI Compatible**: `https://api.deepseek.com/v1`

## Your First API Call

### Using cURL

```bash
curl https://api.deepseek.com/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "model": "deepseek-chat",
    "messages": [
      {"role": "system", "content": "You are a helpful assistant."},
      {"role": "user", "content": "Hello!"}
    ]
  }'
```

### Using Python (OpenAI SDK)

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
        {"role": "user", "content": "Hello!"}
    ]
)

print(response.choices[0].message.content)
```

### Using Node.js

```javascript
const OpenAI = require('openai');

const client = new OpenAI({
    apiKey: 'YOUR_API_KEY',
    baseURL: 'https://api.deepseek.com'
});

async function main() {
    const response = await client.chat.completions.create({
        model: 'deepseek-chat',
        messages: [
            { role: 'system', content: 'You are a helpful assistant.' },
            { role: 'user', content: 'Hello!' }
        ]
    });

    console.log(response.choices[0].message.content);
}

main();
```

## Streaming Responses

To receive responses in real-time, set `stream: true`:

```python
response = client.chat.completions.create(
    model="deepseek-chat",
    messages=[
        {"role": "user", "content": "Tell me a story"}
    ],
    stream=True
)

for chunk in response:
    if chunk.choices[0].delta.content:
        print(chunk.choices[0].delta.content, end='')
```

## Key Concepts

### Messages

Conversations are represented as arrays of message objects with roles:
- **system**: Sets the assistant's behavior
- **user**: User's input
- **assistant**: Model's responses
- **tool**: Results from function calls

### Models

- **deepseek-chat**: Standard chat model for general tasks
- **deepseek-reasoner**: Reasoning model with Chain of Thought analysis

### Parameters

Common parameters to control generation:
- `temperature` (0-2): Controls randomness (default: 1)
- `max_tokens`: Maximum output length
- `top_p` (0-1): Nucleus sampling parameter (default: 1)
- `stream`: Enable streaming responses (default: false)

## Next Steps

- Learn about [Models and Pricing](./models-and-pricing.md)
- Explore [Function Calling](./function-calling.md)
- Review [API Reference](./api-reference.md)
