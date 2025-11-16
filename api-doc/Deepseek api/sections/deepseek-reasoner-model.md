# deepseek-reasoner Model

## Overview

**deepseek-reasoner (DeepSeek-V3.2-Exp)** - Advanced reasoning model with Chain of Thought capabilities.

## Specifications

- **Context Window**: 128,000 tokens
- **Max Output**: 32,768 tokens (default) / 65,536 tokens (maximum)
- **Mode**: Thinking mode (shows reasoning process)

## Supported Features

### JSON Output Formatting
Generate structured JSON responses with reasoning included.

### Chat Prefix Completion
Continue existing conversations with maintained reasoning context.

### Chain of Thought Analysis
Explicit step-by-step reasoning process visible in responses.

## Important Note

**Function Calling**: Requests with function calling are automatically redirected to `deepseek-chat` for processing.

## Use Cases

### Complex Problem Solving
- Multi-step problems
- Strategy development
- Complex decision making

### Multi-Step Reasoning
- Logical deduction
- Planning and scheduling
- Algorithm design

### Mathematical Calculations
- Advanced mathematics
- Statistical analysis
- Proofs and derivations

### Logic Puzzles
- Constraint satisfaction
- Game theory
- Puzzle solving

## Example Usage

```python
from openai import OpenAI

client = OpenAI(
    api_key="YOUR_API_KEY",
    base_url="https://api.deepseek.com"
)

response = client.chat.completions.create(
    model="deepseek-reasoner",
    messages=[
        {
            "role": "user",
            "content": "If a train travels 120 km in 2 hours, then speeds up and travels 180 km in 1.5 hours, what is the average speed?"
        }
    ],
    max_tokens=10000
)

# Access reasoning process
print("Reasoning:", response.choices[0].message.reasoning_content)
print("\nAnswer:", response.choices[0].message.content)
```

## Performance Characteristics

- **Speed**: Slower than deepseek-chat (due to reasoning)
- **Cost**: Higher token usage (includes reasoning tokens)
- **Quality**: Superior for complex tasks
- **Transparency**: Shows thinking process

## Output Structure

The model provides two types of content:

### reasoning_content
The model's internal reasoning process and steps.

### content
The final answer or conclusion.

## When to Use

Choose **deepseek-reasoner** when:
- Complex reasoning is required
- You need transparent Chain of Thought analysis
- Problem requires step-by-step thinking
- Quality and accuracy are more important than speed
- You want to verify the reasoning process
- Educational or explanatory purposes

## When NOT to Use

Avoid **deepseek-reasoner** when:
- Simple queries or conversations
- Function calling is required (use deepseek-chat)
- Speed is critical
- Cost optimization is priority
- Reasoning transparency not needed
