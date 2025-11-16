# First API Call - Using Python

## Installation

```bash
pip install openai
```

## Basic Usage

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

## With Environment Variables

```python
import os
from openai import OpenAI

# Set API key as environment variable: export DEEPSEEK_API_KEY=your_key_here
client = OpenAI(
    api_key=os.getenv("DEEPSEEK_API_KEY"),
    base_url="https://api.deepseek.com"
)

response = client.chat.completions.create(
    model="deepseek-chat",
    messages=[
        {"role": "user", "content": "Explain recursion simply"}
    ],
    temperature=0.7,
    max_tokens=200
)

print(response.choices[0].message.content)
```

## Error Handling

```python
from openai import OpenAI, APIError, RateLimitError

client = OpenAI(
    api_key="YOUR_API_KEY",
    base_url="https://api.deepseek.com"
)

try:
    response = client.chat.completions.create(
        model="deepseek-chat",
        messages=[{"role": "user", "content": "Hello"}]
    )
    print(response.choices[0].message.content)
except RateLimitError:
    print("Rate limit exceeded. Please slow down requests.")
except APIError as e:
    print(f"API error: {e}")
```
