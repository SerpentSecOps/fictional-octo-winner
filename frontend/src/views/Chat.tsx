import React, { useState, useRef, useEffect } from 'react';
import { useAppStore } from '../store/appStore';
import { sendChatMessageStream } from '../api/chat';
import { ragChat } from '../api/rag';
import type { ChatMessage, ChunkMatch } from '../api/types';
import { Send, Loader2, Database } from 'lucide-react';
import ReactMarkdown from 'react-markdown';

interface Message extends ChatMessage {
  id: string;
  sources?: ChunkMatch[];
}

const Chat: React.FC = () => {
  const { providers, selectedProject } = useAppStore();
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState('');
  const [isStreaming, setIsStreaming] = useState(false);
  const [selectedProvider, setSelectedProvider] = useState('');
  const [selectedModel, setSelectedModel] = useState('');
  const [useRag, setUseRag] = useState(false);
  const [temperature, setTemperature] = useState(0.7);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const enabledProviders = providers.filter((p) => p.enabled && p.has_api_key);

  // Auto-select first enabled provider
  useEffect(() => {
    if (!selectedProvider && enabledProviders.length > 0) {
      const firstProvider = enabledProviders[0];
      setSelectedProvider(firstProvider.provider_id);
      if (firstProvider.default_model) {
        setSelectedModel(firstProvider.default_model);
      }
    }
  }, [enabledProviders, selectedProvider]);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleSend = async () => {
    if (!input.trim() || !selectedProvider || !selectedModel || isStreaming) return;

    const userMessage: Message = {
      id: Date.now().toString(),
      role: 'user',
      content: input.trim(),
    };

    setMessages((prev) => [...prev, userMessage]);
    setInput('');
    setIsStreaming(true);

    try {
      if (useRag && selectedProject) {
        // Use RAG chat
        const response = await ragChat({
          project_id: selectedProject.id,
          query: userMessage.content,
          provider_id: selectedProvider,
          model: selectedModel,
          top_k: 5,
          temperature,
          max_tokens: 4096,
        });

        const assistantMessage: Message = {
          id: (Date.now() + 1).toString(),
          role: 'assistant',
          content: response.response,
          sources: response.sources,
        };

        setMessages((prev) => [...prev, assistantMessage]);
      } else {
        // Use streaming chat
        const requestId = `req_${Date.now()}`;
        const assistantMessage: Message = {
          id: (Date.now() + 1).toString(),
          role: 'assistant',
          content: '',
        };

        setMessages((prev) => [...prev, assistantMessage]);

        await sendChatMessageStream(
          {
            provider_id: selectedProvider,
            model: selectedModel,
            messages: [...messages, userMessage].map((m) => ({
              role: m.role,
              content: m.content,
            })),
            temperature,
            max_tokens: 4096,
            top_p: undefined,
            stream: true,
          },
          requestId,
          (chunk) => {
            // Update the last message with the chunk
            setMessages((prev) => {
              const newMessages = [...prev];
              const lastMessage = newMessages[newMessages.length - 1];
              if (lastMessage && lastMessage.role === 'assistant') {
                lastMessage.content += chunk.delta;
              }
              return newMessages;
            });
          },
          () => {
            setIsStreaming(false);
          }
        );
      }
    } catch (error) {
      console.error('Chat error:', error);
      const errorMessage: Message = {
        id: (Date.now() + 1).toString(),
        role: 'assistant',
        content: `Error: ${error instanceof Error ? error.message : 'Unknown error'}`,
      };
      setMessages((prev) => [...prev, errorMessage]);
    } finally {
      setIsStreaming(false);
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  return (
    <div className="h-full flex flex-col bg-white dark:bg-gray-900">
      {/* Header */}
      <div className="border-b border-gray-200 dark:border-gray-700 p-4 bg-white dark:bg-gray-800">
        <div className="flex items-center justify-between">
          <h1 className="text-xl font-semibold text-gray-900 dark:text-white">Chat</h1>
          <div className="flex items-center space-x-4">
            {/* Provider selector */}
            <select
              value={selectedProvider}
              onChange={(e) => {
                setSelectedProvider(e.target.value);
                const provider = providers.find((p) => p.provider_id === e.target.value);
                if (provider?.default_model) {
                  setSelectedModel(provider.default_model);
                }
              }}
              className="px-3 py-1 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm"
            >
              <option value="">Select Provider</option>
              {enabledProviders.map((p) => (
                <option key={p.provider_id} value={p.provider_id}>
                  {p.provider_id}
                </option>
              ))}
            </select>

            {/* Model input */}
            <input
              type="text"
              value={selectedModel}
              onChange={(e) => setSelectedModel(e.target.value)}
              placeholder="Model name"
              className="px-3 py-1 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm w-48"
            />

            {/* Temperature */}
            <div className="flex items-center space-x-2">
              <label className="text-sm text-gray-600 dark:text-gray-400">Temp:</label>
              <input
                type="number"
                value={temperature}
                onChange={(e) => setTemperature(parseFloat(e.target.value))}
                step="0.1"
                min="0"
                max="2"
                className="px-2 py-1 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm w-16"
              />
            </div>

            {/* RAG toggle */}
            {selectedProject && (
              <button
                onClick={() => setUseRag(!useRag)}
                className={`flex items-center space-x-1 px-3 py-1 rounded text-sm ${
                  useRag
                    ? 'bg-primary-600 text-white'
                    : 'bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300'
                }`}
              >
                <Database size={16} />
                <span>RAG</span>
              </button>
            )}
          </div>
        </div>
      </div>

      {/* Messages */}
      <div className="flex-1 overflow-y-auto p-4 space-y-4">
        {messages.length === 0 ? (
          <div className="text-center text-gray-500 dark:text-gray-400 mt-20">
            <p>Start a conversation</p>
            {enabledProviders.length === 0 && (
              <p className="mt-2 text-sm">
                Configure a provider in Settings to get started
              </p>
            )}
          </div>
        ) : (
          messages.map((message) => (
            <div
              key={message.id}
              className={`flex ${
                message.role === 'user' ? 'justify-end' : 'justify-start'
              }`}
            >
              <div
                className={`max-w-3xl rounded-lg p-4 ${
                  message.role === 'user'
                    ? 'bg-primary-600 text-white'
                    : 'bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-white'
                }`}
              >
                <div className="text-xs font-semibold mb-1 opacity-70">
                  {message.role === 'user' ? 'You' : 'Assistant'}
                </div>
                <div className="markdown-content">
                  <ReactMarkdown>{message.content}</ReactMarkdown>
                </div>
                {message.sources && message.sources.length > 0 && (
                  <div className="mt-3 pt-3 border-t border-gray-300 dark:border-gray-600">
                    <div className="text-xs font-semibold mb-2">Sources:</div>
                    <div className="space-y-1">
                      {message.sources.map((source, idx) => (
                        <div key={idx} className="text-xs opacity-80">
                          {source.document_name} (similarity: {source.similarity.toFixed(2)})
                        </div>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            </div>
          ))
        )}
        <div ref={messagesEndRef} />
      </div>

      {/* Input */}
      <div className="border-t border-gray-200 dark:border-gray-700 p-4 bg-white dark:bg-gray-800">
        <div className="flex space-x-2">
          <textarea
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Type a message... (Shift+Enter for new line)"
            rows={3}
            className="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white resize-none"
          />
          <button
            onClick={handleSend}
            disabled={isStreaming || !input.trim() || !selectedProvider || !selectedModel}
            className="px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center"
          >
            {isStreaming ? (
              <Loader2 size={20} className="animate-spin" />
            ) : (
              <Send size={20} />
            )}
          </button>
        </div>
      </div>
    </div>
  );
};

export default Chat;
