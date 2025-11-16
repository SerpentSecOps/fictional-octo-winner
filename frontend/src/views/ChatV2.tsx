import React, { useState, useRef, useEffect } from 'react';
import { useAppStore } from '../store/appStore';
import { sendChatMessageStream } from '../api/chat';
import {
  createConversation,
  listConversations,
  getConversationWithMessages,
  deleteConversation as deleteConversationApi,
  addMessage,
  updateConversationTitle,
} from '../api/conversation';
import type { Conversation, Message as DbMessage } from '../api/types';
import { Send, Loader2, Plus, Trash2, Edit2, Check, X } from 'lucide-react';
import ReactMarkdown from 'react-markdown';
import { showError, showSuccess } from '../utils/toast';

interface UIMessage {
  id: string;
  role: 'system' | 'user' | 'assistant';
  content: string;
}

const ChatV2: React.FC = () => {
  const { providers } = useAppStore();
  const [conversations, setConversations] = useState<Conversation[]>([]);
  const [selectedConversation, setSelectedConversation] = useState<Conversation | null>(
    null
  );
  const [messages, setMessages] = useState<UIMessage[]>([]);
  const [input, setInput] = useState('');
  const [isStreaming, setIsStreaming] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [selectedProvider, setSelectedProvider] = useState('');
  const [selectedModel, setSelectedModel] = useState('');
  const [temperature, setTemperature] = useState(0.7);
  const [editingTitle, setEditingTitle] = useState<number | null>(null);
  const [editTitle, setEditTitle] = useState('');
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

  // Load conversations on mount
  useEffect(() => {
    loadConversations();
  }, []);

  // Load messages when conversation changes
  useEffect(() => {
    if (selectedConversation) {
      loadMessages(selectedConversation.id);
    } else {
      setMessages([]);
    }
  }, [selectedConversation]);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const loadConversations = async () => {
    try {
      const convos = await listConversations();
      setConversations(convos);

      // Select first conversation if none selected
      if (!selectedConversation && convos.length > 0) {
        setSelectedConversation(convos[0]);
      }
    } catch (error) {
      console.error('Failed to load conversations:', error);
      showError('Failed to load conversations');
    }
  };

  const loadMessages = async (conversationId: number) => {
    setIsLoading(true);
    try {
      const data = await getConversationWithMessages(conversationId);
      const uiMessages: UIMessage[] = data.messages.map((msg) => ({
        id: msg.id.toString(),
        role: msg.role as 'system' | 'user' | 'assistant',
        content: msg.content,
      }));
      setMessages(uiMessages);
    } catch (error) {
      console.error('Failed to load messages:', error);
      showError('Failed to load messages');
    } finally {
      setIsLoading(false);
    }
  };

  const handleNewConversation = async () => {
    if (!selectedProvider || !selectedModel) {
      showError('Please select a provider and model first');
      return;
    }

    try {
      const conv = await createConversation({
        title: 'New Conversation',
        provider_id: selectedProvider,
        model: selectedModel,
      });
      setConversations([conv, ...conversations]);
      setSelectedConversation(conv);
      setMessages([]);
      showSuccess('Created new conversation');
    } catch (error) {
      console.error('Failed to create conversation:', error);
      showError('Failed to create conversation');
    }
  };

  const handleDeleteConversation = async (conv: Conversation, e: React.MouseEvent) => {
    e.stopPropagation();

    if (!confirm(`Delete conversation "${conv.title}"?`)) {
      return;
    }

    try {
      await deleteConversationApi(conv.id);
      const newConvos = conversations.filter((c) => c.id !== conv.id);
      setConversations(newConvos);

      if (selectedConversation?.id === conv.id) {
        setSelectedConversation(newConvos[0] || null);
      }

      showSuccess('Conversation deleted');
    } catch (error) {
      console.error('Failed to delete conversation:', error);
      showError('Failed to delete conversation');
    }
  };

  const handleRenameConversation = async (conv: Conversation) => {
    if (!editTitle.trim()) {
      setEditingTitle(null);
      return;
    }

    try {
      await updateConversationTitle(conv.id, editTitle.trim());
      const updatedConvos = conversations.map((c) =>
        c.id === conv.id ? { ...c, title: editTitle.trim() } : c
      );
      setConversations(updatedConvos);

      if (selectedConversation?.id === conv.id) {
        setSelectedConversation({ ...selectedConversation, title: editTitle.trim() });
      }

      setEditingTitle(null);
      showSuccess('Conversation renamed');
    } catch (error) {
      console.error('Failed to rename conversation:', error);
      showError('Failed to rename conversation');
    }
  };

  const handleSend = async () => {
    if (!input.trim() || !selectedProvider || !selectedModel || isStreaming) return;

    // Create new conversation if none selected
    if (!selectedConversation) {
      await handleNewConversation();
      // Wait a bit for state to update
      await new Promise((resolve) => setTimeout(resolve, 100));
    }

    if (!selectedConversation) {
      showError('Failed to create conversation');
      return;
    }

    const userMessage: UIMessage = {
      id: Date.now().toString(),
      role: 'user',
      content: input.trim(),
    };

    setMessages((prev) => [...prev, userMessage]);
    setInput('');
    setIsStreaming(true);

    try {
      // Save user message to DB
      await addMessage({
        conversation_id: selectedConversation.id,
        role: 'user',
        content: userMessage.content,
      });

      // Prepare assistant message
      const assistantMessage: UIMessage = {
        id: (Date.now() + 1).toString(),
        role: 'assistant',
        content: '',
      };

      setMessages((prev) => [...prev, assistantMessage]);

      // Streaming chat
      const requestId = `req_${Date.now()}`;
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
          setMessages((prev) => {
            const newMessages = [...prev];
            const lastMessage = newMessages[newMessages.length - 1];
            if (lastMessage && lastMessage.role === 'assistant') {
              lastMessage.content += chunk.delta;
            }
            return newMessages;
          });
        },
        async () => {
          setIsStreaming(false);

          // Save assistant message to DB
          const finalContent =
            messages[messages.length - 1]?.role === 'assistant'
              ? messages[messages.length - 1].content
              : '';

          if (finalContent && selectedConversation) {
            try {
              await addMessage({
                conversation_id: selectedConversation.id,
                role: 'assistant',
                content: finalContent,
              });

              // Auto-generate title from first message
              if (messages.length === 2) {
                const title = userMessage.content.slice(0, 50) + (userMessage.content.length > 50 ? '...' : '');
                await updateConversationTitle(selectedConversation.id, title);
                const updatedConvos = conversations.map((c) =>
                  c.id === selectedConversation.id ? { ...c, title } : c
                );
                setConversations(updatedConvos);
                setSelectedConversation({ ...selectedConversation, title });
              }
            } catch (error) {
              console.error('Failed to save assistant message:', error);
            }
          }
        }
      );
    } catch (error) {
      console.error('Chat error:', error);
      showError(`Chat error: ${error instanceof Error ? error.message : 'Unknown error'}`);
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
    <div className="h-full flex bg-white dark:bg-gray-900">
      {/* Conversations Sidebar */}
      <div className="w-64 border-r border-gray-200 dark:border-gray-700 flex flex-col bg-gray-50 dark:bg-gray-800">
        <div className="p-3 border-b border-gray-200 dark:border-gray-700">
          <button
            onClick={handleNewConversation}
            disabled={!selectedProvider || !selectedModel}
            className="w-full px-3 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center space-x-2"
          >
            <Plus size={16} />
            <span>New Chat</span>
          </button>
        </div>

        <div className="flex-1 overflow-y-auto p-2">
          {conversations.length === 0 ? (
            <p className="text-sm text-gray-500 dark:text-gray-400 text-center mt-4">
              No conversations yet
            </p>
          ) : (
            <div className="space-y-1">
              {conversations.map((conv) => (
                <div
                  key={conv.id}
                  onClick={() => setSelectedConversation(conv)}
                  className={`p-2 rounded cursor-pointer transition-colors group ${
                    selectedConversation?.id === conv.id
                      ? 'bg-primary-100 dark:bg-primary-900/30'
                      : 'hover:bg-gray-100 dark:hover:bg-gray-700'
                  }`}
                >
                  {editingTitle === conv.id ? (
                    <div className="flex items-center space-x-1">
                      <input
                        type="text"
                        value={editTitle}
                        onChange={(e) => setEditTitle(e.target.value)}
                        onKeyDown={(e) => {
                          if (e.key === 'Enter') handleRenameConversation(conv);
                          if (e.key === 'Escape') setEditingTitle(null);
                        }}
                        className="flex-1 px-2 py-1 text-sm border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-900 dark:text-white"
                        autoFocus
                      />
                      <button
                        onClick={() => handleRenameConversation(conv)}
                        className="p-1 text-green-600 hover:text-green-700"
                      >
                        <Check size={14} />
                      </button>
                      <button
                        onClick={() => setEditingTitle(null)}
                        className="p-1 text-red-600 hover:text-red-700"
                      >
                        <X size={14} />
                      </button>
                    </div>
                  ) : (
                    <div className="flex items-center justify-between">
                      <span className="text-sm font-medium text-gray-900 dark:text-white truncate flex-1">
                        {conv.title}
                      </span>
                      <div className="flex items-center space-x-1 opacity-0 group-hover:opacity-100 transition-opacity">
                        <button
                          onClick={(e) => {
                            e.stopPropagation();
                            setEditingTitle(conv.id);
                            setEditTitle(conv.title);
                          }}
                          className="p-1 text-gray-500 hover:text-gray-700 dark:hover:text-gray-300"
                        >
                          <Edit2 size={14} />
                        </button>
                        <button
                          onClick={(e) => handleDeleteConversation(conv, e)}
                          className="p-1 text-red-500 hover:text-red-700"
                        >
                          <Trash2 size={14} />
                        </button>
                      </div>
                    </div>
                  )}
                  <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                    {new Date(conv.updated_at).toLocaleDateString()}
                  </p>
                </div>
              ))}
            </div>
          )}
        </div>
      </div>

      {/* Chat Area */}
      <div className="flex-1 flex flex-col">
        {/* Header */}
        <div className="border-b border-gray-200 dark:border-gray-700 p-4 bg-white dark:bg-gray-800">
          <div className="flex items-center justify-between">
            <h1 className="text-xl font-semibold text-gray-900 dark:text-white">
              {selectedConversation?.title || 'Chat'}
            </h1>
            <div className="flex items-center space-x-4">
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

              <input
                type="text"
                value={selectedModel}
                onChange={(e) => setSelectedModel(e.target.value)}
                placeholder="Model name"
                className="px-3 py-1 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm w-48"
              />

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
            </div>
          </div>
        </div>

        {/* Messages */}
        <div className="flex-1 overflow-y-auto p-4 space-y-4">
          {isLoading ? (
            <div className="flex items-center justify-center h-full">
              <Loader2 className="animate-spin text-primary-600" size={32} />
            </div>
          ) : messages.length === 0 ? (
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
    </div>
  );
};

export default ChatV2;
