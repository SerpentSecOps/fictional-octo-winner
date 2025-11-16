# Model Selection Guide

## Quick Decision Matrix

| Requirement | deepseek-chat | deepseek-reasoner |
|-------------|--------------|-------------------|
| Fast responses | ✅ Best | ❌ Slower |
| Function calling | ✅ Yes | ❌ No |
| Low cost | ✅ Lower | ❌ Higher |
| Simple queries | ✅ Ideal | ⚠️ Overkill |
| Complex reasoning | ⚠️ Basic | ✅ Excellent |
| Chain of Thought | ❌ No | ✅ Yes |
| Long outputs | ⚠️ Up to 8K | ✅ Up to 65K |
| Transparency | ❌ No reasoning | ✅ Shows thinking |

## Choose deepseek-chat When:

### Speed is Critical
- Real-time chat applications
- Interactive systems
- High-throughput services
- User-facing applications

### Using Function Calling
- Tool integration required
- API calls needed
- External service integration
- Database queries

### Standard Conversation Tasks
- General Q&A
- Content generation
- Text summarization
- Translation

### Cost Optimization is Priority
- High-volume applications
- Budget-constrained projects
- Non-critical accuracy requirements

### Examples:

```python
# Customer support chatbot
response = client.chat.completions.create(
    model="deepseek-chat",  # Fast, cost-effective
    messages=[
        {"role": "system", "content": "You are a support agent"},
        {"role": "user", "content": "How do I reset my password?"}
    ]
)
```

```python
# Content generation
response = client.chat.completions.create(
    model="deepseek-chat",  # Quick generation
    messages=[{"role": "user", "content": "Write a blog intro about AI"}],
    temperature=0.8
)
```

## Choose deepseek-reasoner When:

### Complex Reasoning is Required
- Multi-step problem solving
- Strategic planning
- Complex decision making
- Algorithm design

### Need Chain of Thought Analysis
- Educational explanations
- Debugging reasoning
- Verification of logic
- Transparent decision-making

### Problem Requires Step-by-Step Thinking
- Mathematical proofs
- Logic puzzles
- Code analysis
- Scientific reasoning

### Quality is More Important Than Speed
- Research applications
- Critical decisions
- Academic use cases
- High-stakes scenarios

### Examples:

```python
# Complex math problem
response = client.chat.completions.create(
    model="deepseek-reasoner",  # Shows work
    messages=[{
        "role": "user",
        "content": "A farmer has chickens and rabbits. There are 30 heads and 88 legs. How many of each?"
    }]
)

# Access reasoning
print("Reasoning:", response.choices[0].message.reasoning_content)
print("Answer:", response.choices[0].message.content)
```

```python
# Code debugging with explanation
response = client.chat.completions.create(
    model="deepseek-reasoner",  # Detailed analysis
    messages=[{
        "role": "user",
        "content": "Why doesn't this code work? [code here]"
    }]
)
```

## Hybrid Approach

Use both models strategically:

### Initial Triage with deepseek-chat

```python
# Quick classification
triage = client.chat.completions.create(
    model="deepseek-chat",
    messages=[{
        "role": "user",
        "content": f"Is this a complex problem requiring deep reasoning? {user_query}"
    }]
)

# Route based on complexity
if "complex" in triage.choices[0].message.content.lower():
    model = "deepseek-reasoner"
else:
    model = "deepseek-chat"

# Process with appropriate model
response = client.chat.completions.create(model=model, ...)
```

### Function Calling + Reasoning

```python
# Use deepseek-chat for function calls
tools_response = client.chat.completions.create(
    model="deepseek-chat",
    messages=[...],
    tools=[weather_tool, calculator_tool]
)

# Execute tool calls
results = execute_tools(tools_response)

# Use reasoner for final analysis
final_response = client.chat.completions.create(
    model="deepseek-reasoner",
    messages=[
        {"role": "user", "content": "Analyze these results: " + results}
    ]
)
```

## Use Case Examples

### E-commerce Chatbot
**Model**: deepseek-chat
- **Why**: Fast responses, function calling for product lookup
- **Priority**: Speed, cost

### Research Assistant
**Model**: deepseek-reasoner
- **Why**: Complex analysis, transparent reasoning
- **Priority**: Quality, transparency

### Code Completion
**Model**: deepseek-chat
- **Why**: Quick suggestions, FIM support
- **Priority**: Speed, real-time

### Homework Tutor
**Model**: deepseek-reasoner
- **Why**: Step-by-step explanations, educational value
- **Priority**: Understanding, quality

### Content Moderation
**Model**: deepseek-chat
- **Why**: Fast classification, high volume
- **Priority**: Speed, throughput

### Legal Analysis
**Model**: deepseek-reasoner
- **Why**: Complex reasoning, high stakes
- **Priority**: Accuracy, transparency

## Cost vs Quality Trade-off

```
deepseek-chat:          Fast + Cheap + Good for most tasks
deepseek-reasoner:      Slow + Expensive + Best for complex tasks

Choose based on:
- Task complexity
- Budget constraints
- Speed requirements
- Quality needs
```

## Performance Comparison

| Metric | deepseek-chat | deepseek-reasoner |
|--------|--------------|-------------------|
| Response Time | ~1-2s | ~3-10s |
| Cost per Query | $0.001-0.01 | $0.01-0.10 |
| Output Quality (simple) | ★★★★★ | ★★★★★ |
| Output Quality (complex) | ★★★☆☆ | ★★★★★ |
| Token Efficiency | ★★★★★ | ★★★☆☆ |

## Summary

**Start with deepseek-chat** for most applications. Switch to **deepseek-reasoner** only when you specifically need:
- Transparent reasoning
- Complex problem solving
- Educational explanations
- High-accuracy requirements

**When in doubt**: Try deepseek-chat first—it's faster and cheaper, and handles most tasks well.
