# Error Codes

## Overview

This document lists all error codes returned by the DeepSeek API, their causes, and recommended solutions.

---

## HTTP Status Codes

### 400 - Bad Request

**Cause**: Invalid request body format

**Description**: The request body doesn't match the expected format or contains syntax errors.

**Solutions**:
- Review the error message for specific hints
- Check JSON syntax for validity
- Verify all required parameters are included
- Ensure parameter types match the API specification
- Refer to [API Reference](./api-reference.md) for correct format

**Example Error**:
```json
{
  "error": {
    "message": "Invalid request body: missing required field 'messages'",
    "type": "invalid_request_error",
    "code": 400
  }
}
```

**Common Causes**:
- Missing required fields (`messages`, `model`)
- Invalid JSON syntax
- Wrong parameter types
- Including `reasoning_content` in multi-turn conversations with reasoner model

---

### 401 - Unauthorized

**Cause**: Authentication fails due to wrong API key

**Description**: The provided API key is invalid, expired, or missing.

**Solutions**:
- Verify your API key is correct
- Check for extra spaces or newlines in the key
- Generate a new API key from your dashboard if needed
- Ensure the key is properly formatted in the Authorization header

**Example Error**:
```json
{
  "error": {
    "message": "Invalid authentication credentials",
    "type": "authentication_error",
    "code": 401
  }
}
```

**Correct Authentication**:
```bash
curl https://api.deepseek.com/chat/completions \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '...'
```

```python
client = OpenAI(
    api_key="YOUR_API_KEY",  # Correct format
    base_url="https://api.deepseek.com"
)
```

---

### 402 - Payment Required

**Cause**: You have run out of balance

**Description**: Your account balance is insufficient to process the request.

**Solutions**:
- Check your account balance in the dashboard
- Top up your account with funds
- Review your usage and billing history
- Consider implementing usage limits in your application

**Example Error**:
```json
{
  "error": {
    "message": "Insufficient balance. Please top up your account.",
    "type": "insufficient_balance",
    "code": 402
  }
}
```

**Prevention Tips**:
- Monitor your balance regularly
- Set up balance alerts
- Implement usage tracking in your application
- Use context caching to reduce costs

---

### 422 - Unprocessable Entity

**Cause**: Your request contains invalid parameters

**Description**: The request is well-formed but contains invalid parameter values.

**Solutions**:
- Review the error message for specific parameter issues
- Check parameter values against allowed ranges
- Verify enum values are correct
- Consult [API Reference](./api-reference.md) for valid parameter options

**Example Error**:
```json
{
  "error": {
    "message": "Invalid parameter: temperature must be between 0 and 2",
    "type": "invalid_request_error",
    "code": 422
  }
}
```

**Common Invalid Parameters**:

| Parameter | Invalid Example | Valid Example |
|-----------|----------------|---------------|
| temperature | 3.0 | 0.0 - 2.0 |
| top_p | 1.5 | 0.0 - 1.0 |
| model | "gpt-4" | "deepseek-chat" or "deepseek-reasoner" |
| max_tokens | -100 | Positive integer |
| frequency_penalty | 5.0 | -2.0 to 2.0 |
| presence_penalty | -3.0 | -2.0 to 2.0 |

---

### 429 - Too Many Requests

**Cause**: You are sending requests too quickly

**Description**: Your request rate exceeds the allowed limit.

**Solutions**:
- Implement rate limiting in your application
- Add delays between requests
- Use exponential backoff for retries
- Consider batching requests if possible
- Evaluate alternative LLM providers for high-volume needs

**Example Error**:
```json
{
  "error": {
    "message": "Rate limit exceeded. Please slow down your requests.",
    "type": "rate_limit_error",
    "code": 429
  }
}
```

**Retry Strategy with Exponential Backoff**:

```python
import time
from openai import OpenAI, RateLimitError

client = OpenAI(
    api_key="YOUR_API_KEY",
    base_url="https://api.deepseek.com"
)

def make_request_with_retry(messages, max_retries=5):
    for attempt in range(max_retries):
        try:
            response = client.chat.completions.create(
                model="deepseek-chat",
                messages=messages
            )
            return response
        except RateLimitError:
            if attempt < max_retries - 1:
                wait_time = (2 ** attempt) + 1  # Exponential backoff
                print(f"Rate limited. Waiting {wait_time} seconds...")
                time.sleep(wait_time)
            else:
                raise

    return None
```

**Rate Limiting Best Practices**:
```python
import time

class RateLimiter:
    def __init__(self, requests_per_minute):
        self.requests_per_minute = requests_per_minute
        self.min_interval = 60.0 / requests_per_minute
        self.last_request = 0

    def wait_if_needed(self):
        elapsed = time.time() - self.last_request
        if elapsed < self.min_interval:
            time.sleep(self.min_interval - elapsed)
        self.last_request = time.time()

# Usage
limiter = RateLimiter(requests_per_minute=10)

for message in messages_to_send:
    limiter.wait_if_needed()
    response = client.chat.completions.create(...)
```

---

### 500 - Internal Server Error

**Cause**: Our server encounters an issue

**Description**: The DeepSeek server experienced an internal error.

**Solutions**:
- Retry your request after a short wait
- Implement exponential backoff retry logic
- Contact support if the problem persists
- Check API status page for service incidents

**Example Error**:
```json
{
  "error": {
    "message": "Internal server error. Please try again.",
    "type": "server_error",
    "code": 500
  }
}
```

**Retry Implementation**:
```python
import time
from openai import OpenAI, APIError

def make_request_with_retry(client, messages, max_retries=3):
    for attempt in range(max_retries):
        try:
            return client.chat.completions.create(
                model="deepseek-chat",
                messages=messages
            )
        except APIError as e:
            if e.status_code == 500 and attempt < max_retries - 1:
                wait_time = 2 ** attempt  # 1s, 2s, 4s
                print(f"Server error. Retrying in {wait_time}s...")
                time.sleep(wait_time)
            else:
                raise
```

---

### 503 - Service Unavailable

**Cause**: The server is overloaded due to high traffic

**Description**: The service is temporarily unavailable due to high demand.

**Solutions**:
- Retry your request after a brief wait (recommended: 5-10 seconds)
- Implement retry logic with exponential backoff
- Check API status page for service health
- Consider request timing during off-peak hours

**Example Error**:
```json
{
  "error": {
    "message": "Service temporarily unavailable due to high traffic. Please retry.",
    "type": "service_unavailable",
    "code": 503
  }
}
```

**Handling 503 Errors**:
```python
import time
from openai import OpenAI, APIError

def make_request_with_503_handling(client, messages):
    max_retries = 5
    base_delay = 5  # Start with 5 seconds

    for attempt in range(max_retries):
        try:
            return client.chat.completions.create(
                model="deepseek-chat",
                messages=messages
            )
        except APIError as e:
            if e.status_code == 503 and attempt < max_retries - 1:
                wait_time = base_delay * (2 ** attempt)  # 5s, 10s, 20s, 40s, 80s
                print(f"Service unavailable. Waiting {wait_time}s before retry {attempt + 1}/{max_retries}...")
                time.sleep(wait_time)
            else:
                raise

    return None
```

---

## Error Handling Best Practices

### 1. Comprehensive Error Handling

```python
from openai import OpenAI, APIError, RateLimitError, AuthenticationError

client = OpenAI(
    api_key="YOUR_API_KEY",
    base_url="https://api.deepseek.com"
)

def safe_api_call(messages):
    try:
        response = client.chat.completions.create(
            model="deepseek-chat",
            messages=messages
        )
        return response
    except AuthenticationError:
        print("Authentication failed. Check your API key.")
        return None
    except RateLimitError:
        print("Rate limit exceeded. Please slow down.")
        return None
    except APIError as e:
        if e.status_code == 400:
            print(f"Bad request: {e.message}")
        elif e.status_code == 402:
            print("Insufficient balance. Please top up.")
        elif e.status_code == 422:
            print(f"Invalid parameters: {e.message}")
        elif e.status_code == 500:
            print("Server error. Will retry...")
        elif e.status_code == 503:
            print("Service unavailable. Will retry...")
        return None
    except Exception as e:
        print(f"Unexpected error: {e}")
        return None
```

### 2. Retry Logic with Exponential Backoff

```python
import time
from typing import Optional

def make_request_with_smart_retry(
    client,
    messages,
    max_retries=5,
    initial_delay=1
) -> Optional[any]:
    """
    Make API request with smart retry logic
    """
    retryable_errors = [429, 500, 503]

    for attempt in range(max_retries):
        try:
            return client.chat.completions.create(
                model="deepseek-chat",
                messages=messages
            )
        except APIError as e:
            # Don't retry non-retryable errors
            if e.status_code not in retryable_errors:
                raise

            # Last attempt - don't wait
            if attempt == max_retries - 1:
                raise

            # Calculate wait time
            if e.status_code == 429:
                wait_time = initial_delay * (2 ** attempt)
            elif e.status_code == 503:
                wait_time = 5 * (2 ** attempt)
            else:  # 500
                wait_time = 2 ** attempt

            print(f"Error {e.status_code}. Retry {attempt + 1}/{max_retries} after {wait_time}s")
            time.sleep(wait_time)

    return None
```

### 3. Logging and Monitoring

```python
import logging
from datetime import datetime

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

def make_monitored_request(client, messages):
    start_time = datetime.now()

    try:
        response = client.chat.completions.create(
            model="deepseek-chat",
            messages=messages
        )

        duration = (datetime.now() - start_time).total_seconds()
        logger.info(f"Request successful. Duration: {duration}s, Tokens: {response.usage.total_tokens}")

        return response

    except APIError as e:
        duration = (datetime.now() - start_time).total_seconds()
        logger.error(f"Request failed. Status: {e.status_code}, Duration: {duration}s, Error: {e.message}")
        raise
```

### 4. Graceful Degradation

```python
def get_completion_with_fallback(client, messages):
    """
    Try to get completion with fallback strategies
    """
    try:
        # First attempt with reasoning model
        return client.chat.completions.create(
            model="deepseek-reasoner",
            messages=messages
        )
    except APIError as e:
        if e.status_code == 503:
            # Fallback to standard model if reasoner is overloaded
            print("Reasoner unavailable, using standard model...")
            return client.chat.completions.create(
                model="deepseek-chat",
                messages=messages
            )
        raise
```

---

## Quick Reference Table

| Code | Error | Retry? | Action |
|------|-------|--------|--------|
| 400 | Bad Request | No | Fix request format |
| 401 | Unauthorized | No | Check API key |
| 402 | Payment Required | No | Top up balance |
| 422 | Invalid Parameters | No | Fix parameters |
| 429 | Rate Limit | Yes | Implement backoff |
| 500 | Server Error | Yes | Retry with backoff |
| 503 | Service Unavailable | Yes | Retry with backoff |

---

## Support Resources

- **API Status**: Check the platform for service status
- **Documentation**: https://api-docs.deepseek.com/
- **Discord**: Join the community for support
- **Email**: Contact platform support for persistent issues
- **Twitter**: Follow for service updates

---

## Related Documentation

- [Getting Started](./getting-started.md) - Authentication setup
- [API Reference](./api-reference.md) - Complete API documentation
- [Models and Pricing](./models-and-pricing.md) - Understand costs and limits
