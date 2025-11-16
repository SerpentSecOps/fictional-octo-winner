# Models and Pricing

## Available Models

### deepseek-chat (DeepSeek-V3.2-Exp)

**Non-thinking Mode** - Standard chat completion model

**Specifications:**
- **Context Window**: 128K tokens
- **Max Output**: 4K tokens (default) / 8K tokens (maximum)

**Supported Features:**
- JSON output formatting
- Function calling
- Chat prefix completion
- Fill-in-the-middle (FIM) completion

**Use Cases:**
- General conversation
- Content generation
- Structured data extraction
- Tool integration

---

### deepseek-reasoner (DeepSeek-V3.2-Exp)

**Thinking Mode** - Advanced reasoning with Chain of Thought

**Specifications:**
- **Context Window**: 128K tokens
- **Max Output**: 32K tokens (default) / 64K tokens (maximum)

**Supported Features:**
- JSON output formatting
- Chat prefix completion

**Note**: Function calling requests are automatically processed via `deepseek-chat` instead.

**Use Cases:**
- Complex problem solving
- Multi-step reasoning
- Mathematical calculations
- Logic puzzles

---

## Pricing

All prices are per 1 million tokens.

| Token Type | Cost per 1M Tokens |
|------------|-------------------|
| **Input tokens (cache hit)** | $0.028 |
| **Input tokens (cache miss)** | $0.28 |
| **Output tokens** | $0.42 |

### Cost Calculation

```
Total Cost = (Number of Tokens / 1,000,000) × Price per 1M Tokens
```

**Example:**
```
Input: 10,000 tokens (cache miss) = 0.01M × $0.28 = $0.0028
Output: 5,000 tokens = 0.005M × $0.42 = $0.0021
Total: $0.0049
```

## Context Caching

DeepSeek's context caching system helps reduce costs for repeated content:

- **Cache Hit**: Tokens retrieved from cache cost only **$0.028 per 1M tokens** (90% savings)
- **Cache Miss**: Tokens not in cache cost standard **$0.28 per 1M tokens**

The API response includes cache usage details:
```json
{
  "usage": {
    "prompt_tokens": 15000,
    "completion_tokens": 500,
    "total_tokens": 15500,
    "prompt_cache_hit_tokens": 12000,
    "prompt_cache_miss_tokens": 3000
  }
}
```

## Billing Details

### Payment Priority

Charges are deducted from your account balance in this order:
1. **Granted balance** (promotional credits)
2. **Topped-up balance** (purchased credits)

### Important Notes

- Pricing is subject to change - monitor the official pricing page
- DeepSeek reserves the right to adjust pricing
- Check your dashboard regularly for balance updates

## Token Limits

### deepseek-chat

| Limit Type | Value |
|-----------|-------|
| Context window | 128,000 tokens |
| Default max output | 4,096 tokens |
| Maximum max output | 8,192 tokens |

### deepseek-reasoner

| Limit Type | Value |
|-----------|-------|
| Context window | 128,000 tokens |
| Default max output | 32,768 tokens |
| Maximum max output | 65,536 tokens |

## Model Selection Guide

Choose **deepseek-chat** when:
- You need fast responses
- Using function calling
- Standard conversation tasks
- Cost optimization is priority

Choose **deepseek-reasoner** when:
- Complex reasoning is required
- You need Chain of Thought analysis
- Problem requires step-by-step thinking
- Quality is more important than speed
