# Streaming Responses

## Overview

Streaming allows you to receive responses in real-time as they are generated, instead of waiting for the complete response.

## Python Example

```python
from openai import OpenAI

client = OpenAI(
    api_key="YOUR_API_KEY",
    base_url="https://api.deepseek.com"
)

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

## Node.js Example

```javascript
const OpenAI = require('openai');

const client = new OpenAI({
    apiKey: 'YOUR_API_KEY',
    baseURL: 'https://api.deepseek.com'
});

async function streamResponse() {
    const stream = await client.chat.completions.create({
        model: 'deepseek-chat',
        messages: [{ role: 'user', content: 'Tell me a story' }],
        stream: true
    });

    for await (const chunk of stream) {
        const content = chunk.choices[0]?.delta?.content || '';
        process.stdout.write(content);
    }
}

streamResponse();
```

## cURL Example

```bash
curl https://api.deepseek.com/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -d '{
    "model": "deepseek-chat",
    "messages": [{"role": "user", "content": "Tell me a story"}],
    "stream": true
  }'
```

## Stream Response Format

```
data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1234567890,"model":"deepseek-chat","choices":[{"index":0,"delta":{"role":"assistant","content":"Once"},"finish_reason":null}]}

data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1234567890,"model":"deepseek-chat","choices":[{"index":0,"delta":{"content":" upon"},"finish_reason":null}]}

data: {"id":"chatcmpl-123","object":"chat.completion.chunk","created":1234567890,"model":"deepseek-chat","choices":[{"index":0,"delta":{"content":" a"},"finish_reason":null}]}

data: [DONE]
```

## Benefits of Streaming

- **Improved UX**: Users see responses immediately
- **Lower Perceived Latency**: Feels faster than waiting
- **Progressive Display**: Show content as it generates
- **Better for Long Responses**: Especially useful for lengthy outputs

## Use Cases

- Chatbot interfaces
- Real-time content generation
- Interactive applications
- Long-form text generation
