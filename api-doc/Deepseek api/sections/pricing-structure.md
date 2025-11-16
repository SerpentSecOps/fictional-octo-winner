# Pricing Structure

## Token-Based Pricing

All prices are calculated per 1 million tokens.

## Pricing Table

| Token Type | Cost per 1M Tokens | Savings |
|------------|-------------------|---------|
| **Input tokens (cache hit)** | $0.028 | 90% off |
| **Input tokens (cache miss)** | $0.28 | Standard |
| **Output tokens** | $0.42 | Standard |

## Cost Calculation Formula

```
Total Cost = (Number of Tokens / 1,000,000) × Price per 1M Tokens
```

## Detailed Examples

### Example 1: Simple Query (No Cache)

**Input**: 10,000 tokens (cache miss)
```
10,000 / 1,000,000 × $0.28 = $0.0028
```

**Output**: 5,000 tokens
```
5,000 / 1,000,000 × $0.42 = $0.0021
```

**Total Cost**: $0.0049

### Example 2: With Cache Hit

**Input**: 10,000 tokens (cache hit)
```
10,000 / 1,000,000 × $0.028 = $0.00028
```

**Output**: 5,000 tokens
```
5,000 / 1,000,000 × $0.42 = $0.0021
```

**Total Cost**: $0.00238 (52% savings!)

### Example 3: Large Document Processing

**Input**: 100,000 tokens (50% cache hit, 50% cache miss)
```
Cache hit: 50,000 / 1,000,000 × $0.028 = $0.0014
Cache miss: 50,000 / 1,000,000 × $0.28 = $0.014
Input total: $0.0154
```

**Output**: 2,000 tokens
```
2,000 / 1,000,000 × $0.42 = $0.00084
```

**Total Cost**: $0.01624

## Billing Details

### Payment Priority

Charges are deducted from your account balance in this order:
1. **Granted balance** (promotional credits, bonuses)
2. **Topped-up balance** (purchased credits)

### Account Management

- Monitor balance in your dashboard
- Set up alerts for low balance
- Top up before running out of credits
- Review usage reports regularly

## Important Notes

### Pricing Changes
- Pricing is subject to change
- DeepSeek reserves the right to adjust rates
- Check official pricing page for updates
- Subscribe to announcements

### Cost Optimization Tips

1. **Use Context Caching**: Save 90% on repeated content
2. **Optimize Prompts**: Shorter prompts = lower costs
3. **Set max_tokens**: Limit output length
4. **Use deepseek-chat**: When reasoning not needed
5. **Batch Requests**: Process multiple queries efficiently

## Token Estimation

### Rough Guidelines
- **1 token** ≈ 4 characters in English
- **1 token** ≈ ¾ of a word
- **100 tokens** ≈ 75 words
- **1,000 tokens** ≈ 750 words

### Example Texts
- "Hello, how are you?" ≈ **6 tokens**
- Short paragraph (100 words) ≈ **133 tokens**
- Page of text (500 words) ≈ **667 tokens**
- Full article (2000 words) ≈ **2,667 tokens**

## Comparing Model Costs

### deepseek-chat
- Lower per-token cost
- Faster responses = less wait time
- Suitable for high-volume applications

### deepseek-reasoner
- Higher per-token cost (includes reasoning)
- More tokens per response
- Better for accuracy-critical tasks
