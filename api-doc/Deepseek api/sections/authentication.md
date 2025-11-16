# Authentication Setup

## Obtaining Your API Key

1. Visit the DeepSeek platform dashboard
2. Navigate to the API keys section
3. Generate a new API key
4. Store it securely (never commit to version control)

## Base URLs

- **Standard**: `https://api.deepseek.com`
- **OpenAI Compatible**: `https://api.deepseek.com/v1`

## Authentication Format

Include your API key in the Authorization header:

```
Authorization: Bearer YOUR_API_KEY
```

## Security Best Practices

- Never commit API keys to version control
- Store keys in environment variables
- Rotate keys periodically
- Use separate keys for development and production
- Revoke keys immediately if compromised
