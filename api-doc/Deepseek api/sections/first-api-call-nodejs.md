# First API Call - Using Node.js

## Installation

```bash
npm install openai
```

## Basic Usage

```javascript
const OpenAI = require('openai');

const client = new OpenAI({
    apiKey: 'YOUR_API_KEY',
    baseURL: 'https://api.deepseek.com'
});

async function main() {
    const response = await client.chat.completions.create({
        model: 'deepseek-chat',
        messages: [
            { role: 'system', content: 'You are a helpful assistant.' },
            { role: 'user', content: 'Hello!' }
        ]
    });

    console.log(response.choices[0].message.content);
}

main();
```

## With Environment Variables

```javascript
const OpenAI = require('openai');
require('dotenv').config();

const client = new OpenAI({
    apiKey: process.env.DEEPSEEK_API_KEY,
    baseURL: 'https://api.deepseek.com'
});

async function generateText() {
    try {
        const response = await client.chat.completions.create({
            model: 'deepseek-chat',
            messages: [
                { role: 'user', content: 'Explain async/await in JavaScript' }
            ],
            temperature: 0.7,
            max_tokens: 200
        });

        console.log(response.choices[0].message.content);
    } catch (error) {
        console.error('Error:', error.message);
    }
}

generateText();
```

## TypeScript Example

```typescript
import OpenAI from 'openai';

const client = new OpenAI({
    apiKey: process.env.DEEPSEEK_API_KEY!,
    baseURL: 'https://api.deepseek.com'
});

async function chat(message: string): Promise<string> {
    const response = await client.chat.completions.create({
        model: 'deepseek-chat',
        messages: [{ role: 'user', content: message }]
    });

    return response.choices[0].message.content || '';
}

chat('What is TypeScript?').then(console.log);
```
