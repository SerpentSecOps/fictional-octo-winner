# JSON Mode

## Overview

DeepSeek's JSON Output feature enables models to produce valid, structured JSON strings. This is essential for structured data extraction, parsing workflows, and integration with downstream systems.

## How to Enable JSON Mode

### Three Required Steps

#### 1. Set Response Format

Configure the `response_format` parameter:

```python
response_format={'type': 'json_object'}
```

#### 2. Include 'json' in Prompt

Your system or user prompt must include the word "json" and provide an example of the desired format:

```python
messages = [
    {
        "role": "system",
        "content": "Extract information and return as JSON with fields: question and answer"
    },
    {
        "role": "user",
        "content": "Which is the longest river in the world? The Nile River."
    }
]
```

#### 3. Set Adequate max_tokens

Ensure `max_tokens` is large enough to prevent JSON truncation:

```python
max_tokens=1000  # Adjust based on expected output size
```

---

## Basic Example

### Python Implementation

```python
from openai import OpenAI

client = OpenAI(
    api_key="YOUR_API_KEY",
    base_url="https://api.deepseek.com"
)

response = client.chat.completions.create(
    model="deepseek-chat",
    messages=[
        {
            "role": "system",
            "content": "Extract information as JSON with 'name', 'age', and 'city' fields"
        },
        {
            "role": "user",
            "content": "John is 30 years old and lives in Paris"
        }
    ],
    response_format={'type': 'json_object'},
    max_tokens=500
)

import json
result = json.loads(response.choices[0].message.content)
print(result)
# Output: {"name": "John", "age": 30, "city": "Paris"}
```

---

## Advanced Examples

### Data Extraction

```python
response = client.chat.completions.create(
    model="deepseek-chat",
    messages=[
        {
            "role": "system",
            "content": """Extract product information as JSON with these fields:
            - name: product name
            - price: price in USD
            - category: product category
            - in_stock: boolean availability"""
        },
        {
            "role": "user",
            "content": "The iPhone 15 Pro costs $999 and is available in the Electronics category"
        }
    ],
    response_format={'type': 'json_object'},
    max_tokens=300
)

data = json.loads(response.choices[0].message.content)
```

**Output:**
```json
{
  "name": "iPhone 15 Pro",
  "price": 999,
  "category": "Electronics",
  "in_stock": true
}
```

### Structured Analysis

```python
response = client.chat.completions.create(
    model="deepseek-chat",
    messages=[
        {
            "role": "system",
            "content": """Analyze the sentiment and return JSON:
            {
                "sentiment": "positive|negative|neutral",
                "confidence": 0-1,
                "keywords": ["array", "of", "keywords"]
            }"""
        },
        {
            "role": "user",
            "content": "This product is absolutely amazing! Best purchase ever."
        }
    ],
    response_format={'type': 'json_object'},
    max_tokens=200
)
```

**Output:**
```json
{
  "sentiment": "positive",
  "confidence": 0.95,
  "keywords": ["amazing", "best", "purchase"]
}
```

### Nested JSON Structures

```python
response = client.chat.completions.create(
    model="deepseek-chat",
    messages=[
        {
            "role": "system",
            "content": """Extract user data as JSON:
            {
                "user": {
                    "name": "string",
                    "contact": {
                        "email": "string",
                        "phone": "string"
                    }
                },
                "preferences": ["array"]
            }"""
        },
        {
            "role": "user",
            "content": "Alice's email is alice@example.com, phone is 555-1234, and she likes coffee and reading"
        }
    ],
    response_format={'type': 'json_object'},
    max_tokens=400
)
```

**Output:**
```json
{
  "user": {
    "name": "Alice",
    "contact": {
      "email": "alice@example.com",
      "phone": "555-1234"
    }
  },
  "preferences": ["coffee", "reading"]
}
```

---

## Best Practices

### 1. Provide Clear Schema

Always specify the expected JSON structure in your system prompt:

```python
system_prompt = """Return JSON with this exact structure:
{
    "field1": "description",
    "field2": 123,
    "field3": ["array"]
}"""
```

### 2. Include Examples

Provide example JSON in your prompt:

```python
system_prompt = """Extract data as JSON. Example:
{
    "title": "Product Name",
    "price": 99.99
}"""
```

### 3. Set Adequate Token Limits

Calculate expected output size and add buffer:

```python
# For a ~200 character JSON response
max_tokens=300  # ~50% buffer
```

### 4. Handle Empty Content

The API may occasionally return empty content. Implement error handling:

```python
try:
    content = response.choices[0].message.content
    if not content:
        # Handle empty response
        print("Empty response received")
    else:
        data = json.loads(content)
except json.JSONDecodeError as e:
    print(f"Invalid JSON: {e}")
```

### 5. Validate Output

Always validate the JSON structure matches expectations:

```python
import json

def validate_response(content, required_fields):
    try:
        data = json.loads(content)
        for field in required_fields:
            if field not in data:
                return False, f"Missing field: {field}"
        return True, data
    except json.JSONDecodeError:
        return False, "Invalid JSON"

content = response.choices[0].message.content
valid, result = validate_response(content, ["name", "age", "city"])
```

---

## Common Issues and Solutions

### Issue: Empty Content

**Problem**: API returns empty content occasionally

**Solution**:
- Modify your prompts to be more specific
- Increase `max_tokens`
- Retry with adjusted parameters
- Add fallback handling

```python
content = response.choices[0].message.content
if not content:
    # Retry with modified prompt
    pass
```

### Issue: Truncated JSON

**Problem**: JSON is cut off mid-structure

**Solution**: Increase `max_tokens`

```python
# Before
max_tokens=100  # Too small

# After
max_tokens=500  # Adequate for most responses
```

### Issue: Invalid JSON Format

**Problem**: Output is not valid JSON

**Solution**:
- Ensure prompt includes the word "json"
- Provide clear structure example
- Use `response_format={'type': 'json_object'}`

```python
# Bad prompt
"Extract the information"

# Good prompt
"Extract information as JSON with fields: name, age, city"
```

---

## Model Support

### deepseek-chat ✓

Full JSON mode support

```python
model="deepseek-chat",
response_format={'type': 'json_object'}
```

### deepseek-reasoner ✓

JSON mode supported

```python
model="deepseek-reasoner",
response_format={'type': 'json_object'}
```

---

## Use Cases

### Data Extraction
- Extract structured information from text
- Parse unstructured content into databases
- Convert natural language to structured data

### API Integration
- Generate API request payloads
- Transform responses for downstream services
- Create configuration files

### Classification Tasks
- Sentiment analysis with structured output
- Category classification
- Entity recognition

### Form Generation
- Convert descriptions to form schemas
- Generate UI component configurations
- Create validation rules

---

## Limitations

1. **Occasional Empty Responses**: The API may return empty content (being actively addressed)
2. **No Schema Validation**: Model doesn't enforce strict schema compliance (use Strict Mode in function calling for that)
3. **Prompt Dependency**: Output quality depends heavily on prompt clarity
4. **Token Overhead**: JSON formatting uses additional tokens

---

## Related Features

- **[Function Calling](./function-calling.md)**: For strict schema enforcement
- **[API Reference](./api-reference.md)**: Complete parameter documentation
- **[Reasoning Model](./reasoning-model.md)**: Using JSON with reasoning mode
