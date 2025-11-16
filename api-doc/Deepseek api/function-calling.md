# Function Calling

## Overview

Function Calling enables DeepSeek models to invoke external tools and APIs, expanding capabilities beyond text generation. The model can intelligently determine when to call functions, what parameters to use, and synthesize results into natural language responses.

## How It Works

### Workflow

1. **User Query**: Submit a question that requires external data
2. **Model Response**: Model returns a function call with parameters
3. **Tool Execution**: Execute the function and provide results back
4. **Final Response**: Model synthesizes the output into natural language

## Basic Implementation

### 1. Define Your Functions

```python
from openai import OpenAI

client = OpenAI(
    api_key="YOUR_API_KEY",
    base_url="https://api.deepseek.com"
)

tools = [
    {
        "type": "function",
        "function": {
            "name": "get_weather",
            "description": "Get the current weather for a location",
            "parameters": {
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g., San Francisco, CA"
                    },
                    "unit": {
                        "type": "string",
                        "enum": ["celsius", "fahrenheit"],
                        "description": "Temperature unit"
                    }
                },
                "required": ["location"]
            }
        }
    }
]
```

### 2. Make the API Call

```python
messages = [
    {"role": "user", "content": "What's the weather in Paris?"}
]

response = client.chat.completions.create(
    model="deepseek-chat",
    messages=messages,
    tools=tools,
    tool_choice="auto"
)
```

### 3. Handle Function Calls

```python
# Check if model wants to call a function
if response.choices[0].message.tool_calls:
    tool_call = response.choices[0].message.tool_calls[0]
    function_name = tool_call.function.name
    function_args = json.loads(tool_call.function.arguments)

    # Execute your function
    if function_name == "get_weather":
        result = get_weather(
            location=function_args["location"],
            unit=function_args.get("unit", "celsius")
        )

    # Add the result to conversation
    messages.append(response.choices[0].message)
    messages.append({
        "role": "tool",
        "content": json.dumps(result),
        "tool_call_id": tool_call.id
    })

    # Get final response
    final_response = client.chat.completions.create(
        model="deepseek-chat",
        messages=messages
    )

    print(final_response.choices[0].message.content)
```

### Complete Example

```python
import json
from openai import OpenAI

client = OpenAI(
    api_key="YOUR_API_KEY",
    base_url="https://api.deepseek.com"
)

# Define function
def get_weather(location, unit="celsius"):
    # Your actual implementation here
    return {
        "location": location,
        "temperature": 22,
        "unit": unit,
        "condition": "sunny"
    }

# Define tool schema
tools = [{
    "type": "function",
    "function": {
        "name": "get_weather",
        "description": "Get weather of a location",
        "parameters": {
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "City and state, e.g., San Francisco, CA"
                },
                "unit": {
                    "type": "string",
                    "enum": ["celsius", "fahrenheit"]
                }
            },
            "required": ["location"]
        }
    }
}]

# Initial request
messages = [{"role": "user", "content": "What's the weather in Tokyo?"}]

response = client.chat.completions.create(
    model="deepseek-chat",
    messages=messages,
    tools=tools
)

# Process function call
if response.choices[0].message.tool_calls:
    tool_call = response.choices[0].message.tool_calls[0]
    args = json.loads(tool_call.function.arguments)

    # Execute function
    result = get_weather(**args)

    # Continue conversation
    messages.append(response.choices[0].message)
    messages.append({
        "role": "tool",
        "content": json.dumps(result),
        "tool_call_id": tool_call.id
    })

    # Get final answer
    final = client.chat.completions.create(
        model="deepseek-chat",
        messages=messages
    )

    print(final.choices[0].message.content)
```

---

## Strict Mode (Beta)

Strict mode ensures the model strictly adheres to JSON schema format requirements, eliminating hallucinated parameters or incorrect data types.

### Enabling Strict Mode

```python
from openai import OpenAI

client = OpenAI(
    api_key="YOUR_API_KEY",
    base_url="https://api.deepseek.com/beta"  # Note: /beta endpoint
)
```

### Strict Function Definition

```python
tools = [{
    "type": "function",
    "function": {
        "name": "get_user_info",
        "description": "Retrieve user information",
        "strict": True,  # Enable strict mode
        "parameters": {
            "type": "object",
            "properties": {
                "user_id": {
                    "type": "string",
                    "description": "The user's ID"
                },
                "include_email": {
                    "type": "boolean",
                    "description": "Whether to include email"
                }
            },
            "required": ["user_id", "include_email"],  # All properties required
            "additionalProperties": False  # No additional properties allowed
        }
    }
}]
```

### Strict Mode Requirements

**Activation:**
- Use base URL: `https://api.deepseek.com/beta`
- Set `"strict": true` in function definition
- Include `"additionalProperties": false` in object schemas

**All Properties Must Be Required:**

```json
{
  "type": "object",
  "properties": {
    "name": {"type": "string"},
    "age": {"type": "integer"}
  },
  "required": ["name", "age"],
  "additionalProperties": false
}
```

---

## JSON Schema Support

### Supported Types

**Primitives:**
- `object`
- `string`
- `number`
- `integer`
- `boolean`

**Collections:**
- `array`
- `enum`

**Composition:**
- `anyOf`
- `$ref`
- `$def`

### Type-Specific Constraints

#### Objects

```json
{
  "type": "object",
  "properties": {
    "name": {"type": "string"},
    "age": {"type": "integer"}
  },
  "required": ["name", "age"],
  "additionalProperties": false
}
```

#### Strings

**Supported:**
- `pattern`: Regex validation
- `format`: Predefined formats
  - `email`
  - `hostname`
  - `ipv4`
  - `ipv6`
  - `uuid`

**Not Supported:**
- `minLength`
- `maxLength`

```json
{
  "type": "string",
  "pattern": "^[A-Z]{3}$",
  "format": "email"
}
```

#### Numbers

**Supported:**
- `const`: Exact value
- `default`: Default value
- `minimum` / `maximum`: Range limits
- `exclusiveMinimum` / `exclusiveMaximum`: Exclusive limits
- `multipleOf`: Divisibility constraint

```json
{
  "type": "number",
  "minimum": 0,
  "maximum": 100,
  "multipleOf": 0.1
}
```

#### Arrays

```json
{
  "type": "array",
  "items": {
    "type": "string"
  }
}
```

**Note:** `minItems` and `maxItems` are not supported.

#### Enums

```json
{
  "type": "string",
  "enum": ["red", "green", "blue"]
}
```

### Advanced Features

#### Reusable Components with $ref

```json
{
  "$defs": {
    "address": {
      "type": "object",
      "properties": {
        "street": {"type": "string"},
        "city": {"type": "string"}
      },
      "required": ["street", "city"],
      "additionalProperties": false
    }
  },
  "type": "object",
  "properties": {
    "home_address": {"$ref": "#/$defs/address"},
    "work_address": {"$ref": "#/$defs/address"}
  },
  "required": ["home_address", "work_address"],
  "additionalProperties": false
}
```

#### Recursive Structures

```json
{
  "$defs": {
    "node": {
      "type": "object",
      "properties": {
        "value": {"type": "string"},
        "children": {
          "type": "array",
          "items": {"$ref": "#/$defs/node"}
        }
      },
      "required": ["value", "children"],
      "additionalProperties": false
    }
  }
}
```

---

## Tool Choice Options

### auto (default)

Model decides whether to call a function:

```python
response = client.chat.completions.create(
    model="deepseek-chat",
    messages=messages,
    tools=tools,
    tool_choice="auto"
)
```

### none

Disable function calling:

```python
response = client.chat.completions.create(
    model="deepseek-chat",
    messages=messages,
    tools=tools,
    tool_choice="none"
)
```

### required

Force the model to call a function:

```python
response = client.chat.completions.create(
    model="deepseek-chat",
    messages=messages,
    tools=tools,
    tool_choice="required"
)
```

### Specific Function

Force a specific function call:

```python
response = client.chat.completions.create(
    model="deepseek-chat",
    messages=messages,
    tools=tools,
    tool_choice={
        "type": "function",
        "function": {"name": "get_weather"}
    }
)
```

---

## Limitations

- **Maximum Functions**: 128 functions per request
- **Model Support**: Only `deepseek-chat` supports function calling
  - `deepseek-reasoner` will redirect function calling requests to `deepseek-chat`
- **Beta Features**: Strict mode is in beta and may have breaking changes

---

## Best Practices

1. **Clear Descriptions**: Write detailed function and parameter descriptions
2. **Required Fields**: Always specify which parameters are required
3. **Error Handling**: Implement robust error handling for function execution
4. **Validation**: Validate function arguments before execution
5. **Security**: Never execute untrusted code from function arguments
6. **Timeouts**: Implement timeouts for external API calls
7. **Fallbacks**: Provide fallback responses if function execution fails

---

## Common Use Cases

- **Weather Information**: Real-time weather data
- **Database Queries**: Retrieve information from databases
- **API Integration**: Connect to external services
- **Calculations**: Complex mathematical computations
- **Data Transformation**: Format and transform data
- **System Actions**: Execute system commands (with caution)
