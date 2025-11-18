import React, { useState } from 'react';
import { useAppStore } from '../store/appStore';
import { updateProvider, testProviderConnection, getProviders } from '../api/config';
import { Check, Loader2 } from 'lucide-react';

const AVAILABLE_PROVIDERS = [
  {
    id: 'claude',
    name: 'Anthropic Claude',
    defaultBaseUrl: 'https://api.anthropic.com',
    defaultModel: 'claude-3-5-sonnet-20241022',
    models: [
      'claude-3-5-sonnet-20241022',
      'claude-3-5-haiku-20241022',
      'claude-3-opus-20240229',
      'claude-3-sonnet-20240229',
      'claude-3-haiku-20240307',
    ],
  },
  {
    id: 'deepseek',
    name: 'DeepSeek',
    defaultBaseUrl: 'https://api.deepseek.com',
    defaultModel: 'deepseek-chat',
    models: [
      'deepseek-chat',
      'deepseek-coder',
    ],
  },
  {
    id: 'gemini',
    name: 'Google Gemini',
    defaultBaseUrl: 'https://generativelanguage.googleapis.com/v1',
    defaultModel: 'gemini-1.5-pro',
    models: [
      'gemini-1.5-pro',
      'gemini-1.5-flash',
      'gemini-1.0-pro',
      'embedding-001',
    ],
  },
];

const Settings: React.FC = () => {
  const { providers, setProviders } = useAppStore();
  const [activeProvider, setActiveProvider] = useState<string | null>(null);
  const [formData, setFormData] = useState<{
    api_key: string;
    base_url: string;
    default_model: string;
    enabled: boolean;
  }>({
    api_key: '',
    base_url: '',
    default_model: '',
    enabled: true,
  });
  const [testing, setTesting] = useState(false);
  const [testResult, setTestResult] = useState<{ success: boolean; message: string } | null>(
    null
  );
  const [saving, setSaving] = useState(false);

  const handleSelectProvider = (providerId: string) => {
    setActiveProvider(providerId);
    setTestResult(null);

    const existingConfig = providers.find((p) => p.provider_id === providerId);
    const providerDef = AVAILABLE_PROVIDERS.find((p) => p.id === providerId);

    setFormData({
      api_key: '', // Never pre-fill API key
      base_url: existingConfig?.base_url || providerDef?.defaultBaseUrl || '',
      default_model:
        existingConfig?.default_model || providerDef?.defaultModel || '',
      enabled: existingConfig?.enabled ?? true,
    });
  };

  const handleSave = async () => {
    if (!activeProvider) return;

    setSaving(true);
    setTestResult(null);

    try {
      await updateProvider({
        provider_id: activeProvider,
        api_key: formData.api_key || undefined,
        base_url: formData.base_url || undefined,
        default_model: formData.default_model || undefined,
        enabled: formData.enabled,
      });

      // Refresh providers list
      const updated = await getProviders();
      setProviders(updated);

      setTestResult({ success: true, message: 'Provider configured successfully!' });
      setFormData({ ...formData, api_key: '' }); // Clear API key input
    } catch (error) {
      setTestResult({
        success: false,
        message: error instanceof Error ? error.message : 'Failed to save',
      });
    } finally {
      setSaving(false);
    }
  };

  const handleTest = async () => {
    if (!activeProvider) return;

    setTesting(true);
    setTestResult(null);

    try {
      const result = await testProviderConnection(activeProvider);
      setTestResult({ success: true, message: result });
    } catch (error) {
      setTestResult({
        success: false,
        message: error instanceof Error ? error.message : 'Connection test failed',
      });
    } finally {
      setTesting(false);
    }
  };

  const currentProviderConfig = activeProvider
    ? providers.find((p) => p.provider_id === activeProvider)
    : null;

  return (
    <div className="h-full flex">
      {/* Provider list */}
      <div className="w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 p-4">
        <h2 className="text-lg font-semibold mb-4 text-gray-900 dark:text-white">
          LLM Providers
        </h2>
        <div className="space-y-2">
          {AVAILABLE_PROVIDERS.map((provider) => {
            const config = providers.find((p) => p.provider_id === provider.id);
            const isConfigured = config?.has_api_key;

            return (
              <button
                key={provider.id}
                onClick={() => handleSelectProvider(provider.id)}
                className={`w-full text-left px-3 py-2 rounded-lg transition-colors ${
                  activeProvider === provider.id
                    ? 'bg-primary-600 text-white'
                    : 'bg-gray-100 dark:bg-gray-700 text-gray-900 dark:text-white hover:bg-gray-200 dark:hover:bg-gray-600'
                }`}
              >
                <div className="flex items-center justify-between">
                  <span className="font-medium">{provider.name}</span>
                  {isConfigured && (
                    <Check size={16} className="text-green-500" />
                  )}
                </div>
              </button>
            );
          })}
        </div>
      </div>

      {/* Configuration form */}
      <div className="flex-1 p-6 overflow-auto bg-gray-50 dark:bg-gray-900">
        {activeProvider ? (
          <div className="max-w-2xl">
            <h1 className="text-2xl font-bold mb-6 text-gray-900 dark:text-white">
              {AVAILABLE_PROVIDERS.find((p) => p.id === activeProvider)?.name} Configuration
            </h1>

            {currentProviderConfig?.has_api_key && (
              <div className="mb-4 p-4 bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800 rounded-lg">
                <p className="text-green-800 dark:text-green-200">
                  ✓ API key is configured and encrypted
                </p>
              </div>
            )}

            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300">
                  API Key
                </label>
                <input
                  type="password"
                  value={formData.api_key}
                  onChange={(e) =>
                    setFormData({ ...formData, api_key: e.target.value })
                  }
                  placeholder={currentProviderConfig?.has_api_key ? '(configured)' : 'Enter API key'}
                  className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
                />
                <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                  API keys are encrypted and stored securely in your OS keychain
                </p>
              </div>

              <div>
                <label className="block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300">
                  Base URL
                </label>
                <input
                  type="text"
                  value={formData.base_url}
                  onChange={(e) =>
                    setFormData({ ...formData, base_url: e.target.value })
                  }
                  placeholder="https://api.example.com"
                  className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
                />
              </div>

              <div>
                <label className="block text-sm font-medium mb-1 text-gray-700 dark:text-gray-300">
                  Default Model
                </label>
                <select
                  value={formData.default_model}
                  onChange={(e) =>
                    setFormData({ ...formData, default_model: e.target.value })
                  }
                  className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
                >
                  <option value="">Select a model...</option>
                  {AVAILABLE_PROVIDERS.find((p) => p.id === activeProvider)?.models.map((model) => (
                    <option key={model} value={model}>
                      {model}
                    </option>
                  ))}
                </select>
                <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                  Select the default model for this provider
                </p>
              </div>

              <div className="flex items-center">
                <input
                  type="checkbox"
                  id="enabled"
                  checked={formData.enabled}
                  onChange={(e) =>
                    setFormData({ ...formData, enabled: e.target.checked })
                  }
                  className="mr-2"
                />
                <label htmlFor="enabled" className="text-sm text-gray-700 dark:text-gray-300">
                  Enable this provider
                </label>
              </div>

              {testResult && (
                <div
                  className={`p-4 rounded-lg ${
                    testResult.success
                      ? 'bg-green-50 dark:bg-green-900/20 border border-green-200 dark:border-green-800'
                      : 'bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800'
                  }`}
                >
                  <p
                    className={
                      testResult.success
                        ? 'text-green-800 dark:text-green-200'
                        : 'text-red-800 dark:text-red-200'
                    }
                  >
                    {testResult.message}
                  </p>
                </div>
              )}

              <div className="flex space-x-3 pt-4">
                <button
                  onClick={handleSave}
                  disabled={saving}
                  className="px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 disabled:opacity-50 flex items-center"
                >
                  {saving && <Loader2 size={16} className="animate-spin mr-2" />}
                  Save Configuration
                </button>

                {currentProviderConfig?.has_api_key && (
                  <button
                    onClick={handleTest}
                    disabled={testing}
                    className="px-4 py-2 bg-gray-600 text-white rounded-lg hover:bg-gray-700 disabled:opacity-50 flex items-center"
                  >
                    {testing && <Loader2 size={16} className="animate-spin mr-2" />}
                    Test Connection
                  </button>
                )}
              </div>
            </div>
          </div>
        ) : (
          <div className="text-center text-gray-500 dark:text-gray-400 mt-20">
            <p>Select a provider from the left to configure</p>
          </div>
        )}
      </div>
    </div>
  );
};

export default Settings;
