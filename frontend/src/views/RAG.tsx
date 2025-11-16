import React, { useState, useEffect } from 'react';
import { useAppStore } from '../store/appStore';
import {
  createProject,
  listProjects,
  listDocuments,
  addDocument,
  ragChat,
} from '../api/rag';
import type { Project, Document, ChunkMatch } from '../api/types';
import { open } from '@tauri-apps/api/dialog';
import { readTextFile } from '@tauri-apps/api/fs';
import { Upload, FileText, Trash2, Search, Loader2, MessageSquare } from 'lucide-react';

const RAG: React.FC = () => {
  const { providers, selectedProject, setSelectedProject, projects, setProjects } =
    useAppStore();
  const [documents, setDocuments] = useState<Document[]>([]);
  const [loading, setLoading] = useState(false);
  const [uploadingDoc, setUploadingDoc] = useState(false);
  const [newProjectName, setNewProjectName] = useState('');
  const [showNewProject, setShowNewProject] = useState(false);
  const [selectedProvider, setSelectedProvider] = useState('');
  const [query, setQuery] = useState('');
  const [searchResults, setSearchResults] = useState<ChunkMatch[] | null>(null);
  const [chatResponse, setChatResponse] = useState<string | null>(null);
  const [isSearching, setIsSearching] = useState(false);

  const enabledProviders = providers.filter((p) => p.enabled && p.has_api_key);

  // Auto-select first enabled provider
  useEffect(() => {
    if (!selectedProvider && enabledProviders.length > 0) {
      setSelectedProvider(enabledProviders[0].provider_id);
    }
  }, [enabledProviders, selectedProvider]);

  useEffect(() => {
    if (selectedProject) {
      loadDocuments();
    }
  }, [selectedProject]);

  const loadDocuments = async () => {
    if (!selectedProject) return;
    try {
      const docs = await listDocuments(selectedProject.id);
      setDocuments(docs);
    } catch (error) {
      console.error('Failed to load documents:', error);
    }
  };

  const handleCreateProject = async () => {
    if (!newProjectName.trim()) return;
    try {
      const project = await createProject(newProjectName);
      const updatedProjects = await listProjects();
      setProjects(updatedProjects);
      setSelectedProject(project);
      setNewProjectName('');
      setShowNewProject(false);
    } catch (error) {
      console.error('Failed to create project:', error);
      alert('Failed to create project');
    }
  };

  const handleUploadDocument = async () => {
    if (!selectedProject || !selectedProvider) {
      alert('Please select a project and provider first');
      return;
    }

    try {
      // Open file picker
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: 'Text Documents',
            extensions: ['txt', 'md', 'markdown'],
          },
        ],
      });

      if (!selected || Array.isArray(selected)) return;

      setUploadingDoc(true);

      // Read file content
      const content = await readTextFile(selected);
      const fileName = selected.split('/').pop() || 'document.txt';

      // Add document
      const response = await addDocument({
        project_id: selectedProject.id,
        name: fileName,
        content,
        provider_id: selectedProvider,
      });

      alert(
        `Document uploaded! Created ${response.chunks_created} chunks for indexing.`
      );

      // Reload documents list
      await loadDocuments();
    } catch (error) {
      console.error('Upload failed:', error);
      alert(`Upload failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
    } finally {
      setUploadingDoc(false);
    }
  };

  const handleRagChat = async () => {
    if (!selectedProject || !selectedProvider || !query.trim()) return;

    setIsSearching(true);
    setChatResponse(null);
    setSearchResults(null);

    try {
      const response = await ragChat({
        project_id: selectedProject.id,
        query: query.trim(),
        provider_id: selectedProvider,
        model: enabledProviders.find((p) => p.provider_id === selectedProvider)
          ?.default_model || '',
        top_k: 5,
        temperature: 0.7,
        max_tokens: 4096,
      });

      setChatResponse(response.response);
      setSearchResults(response.sources);
    } catch (error) {
      console.error('RAG chat failed:', error);
      alert(`Query failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
    } finally {
      setIsSearching(false);
    }
  };

  return (
    <div className="h-full flex bg-white dark:bg-gray-900">
      {/* Sidebar */}
      <div className="w-80 border-r border-gray-200 dark:border-gray-700 flex flex-col">
        {/* Project Selection */}
        <div className="p-4 border-b border-gray-200 dark:border-gray-700">
          <h2 className="text-lg font-semibold mb-3 text-gray-900 dark:text-white">
            RAG Projects
          </h2>

          <select
            value={selectedProject?.id || ''}
            onChange={(e) => {
              const project = projects.find((p) => p.id === parseInt(e.target.value));
              setSelectedProject(project || null);
            }}
            className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm mb-2"
          >
            <option value="">Select Project</option>
            {projects.map((p) => (
              <option key={p.id} value={p.id}>
                {p.name}
              </option>
            ))}
          </select>

          <button
            onClick={() => setShowNewProject(!showNewProject)}
            className="w-full px-3 py-2 bg-primary-600 text-white rounded hover:bg-primary-700 text-sm"
          >
            New Project
          </button>

          {showNewProject && (
            <div className="mt-2 space-y-2">
              <input
                type="text"
                value={newProjectName}
                onChange={(e) => setNewProjectName(e.target.value)}
                placeholder="Project name"
                className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm"
                onKeyDown={(e) => e.key === 'Enter' && handleCreateProject()}
              />
              <div className="flex space-x-2">
                <button
                  onClick={handleCreateProject}
                  className="flex-1 px-3 py-1 bg-green-600 text-white rounded hover:bg-green-700 text-sm"
                >
                  Create
                </button>
                <button
                  onClick={() => {
                    setShowNewProject(false);
                    setNewProjectName('');
                  }}
                  className="flex-1 px-3 py-1 bg-gray-300 dark:bg-gray-600 text-gray-700 dark:text-gray-200 rounded hover:bg-gray-400 dark:hover:bg-gray-500 text-sm"
                >
                  Cancel
                </button>
              </div>
            </div>
          )}
        </div>

        {/* Provider Selection */}
        {selectedProject && (
          <div className="p-4 border-b border-gray-200 dark:border-gray-700">
            <label className="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300">
              Embedding Provider
            </label>
            <select
              value={selectedProvider}
              onChange={(e) => setSelectedProvider(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm"
            >
              <option value="">Select Provider</option>
              {enabledProviders.map((p) => (
                <option key={p.provider_id} value={p.provider_id}>
                  {p.provider_id}
                </option>
              ))}
            </select>
          </div>
        )}

        {/* Documents List */}
        {selectedProject && (
          <div className="flex-1 overflow-y-auto p-4">
            <div className="flex items-center justify-between mb-3">
              <h3 className="font-semibold text-gray-900 dark:text-white">Documents</h3>
              <button
                onClick={handleUploadDocument}
                disabled={uploadingDoc || !selectedProvider}
                className="px-3 py-1 bg-green-600 text-white rounded hover:bg-green-700 disabled:opacity-50 text-sm flex items-center space-x-1"
              >
                {uploadingDoc ? (
                  <Loader2 size={14} className="animate-spin" />
                ) : (
                  <Upload size={14} />
                )}
                <span>Upload</span>
              </button>
            </div>

            {documents.length === 0 ? (
              <p className="text-sm text-gray-500 dark:text-gray-400">
                No documents yet. Upload a .txt or .md file to get started.
              </p>
            ) : (
              <div className="space-y-2">
                {documents.map((doc) => (
                  <div
                    key={doc.id}
                    className="p-3 bg-gray-100 dark:bg-gray-800 rounded flex items-start justify-between"
                  >
                    <div className="flex items-start space-x-2 flex-1">
                      <FileText size={16} className="text-gray-500 mt-1" />
                      <div>
                        <p className="text-sm font-medium text-gray-900 dark:text-white">
                          {doc.name}
                        </p>
                        <p className="text-xs text-gray-500 dark:text-gray-400">
                          {new Date(doc.created_at).toLocaleDateString()}
                        </p>
                      </div>
                    </div>
                    <button className="text-red-500 hover:text-red-700">
                      <Trash2 size={16} />
                    </button>
                  </div>
                ))}
              </div>
            )}
          </div>
        )}
      </div>

      {/* Main Area - Query & Results */}
      <div className="flex-1 flex flex-col">
        <div className="p-4 border-b border-gray-200 dark:border-gray-700">
          <h1 className="text-xl font-semibold text-gray-900 dark:text-white">
            RAG Search & Chat
          </h1>
        </div>

        <div className="flex-1 overflow-y-auto p-6">
          {!selectedProject ? (
            <div className="text-center text-gray-500 dark:text-gray-400 mt-20">
              <p>Select or create a project to get started</p>
            </div>
          ) : documents.length === 0 ? (
            <div className="text-center text-gray-500 dark:text-gray-400 mt-20">
              <p>Upload documents to enable RAG search</p>
            </div>
          ) : (
            <div className="max-w-4xl mx-auto space-y-6">
              {/* Query Input */}
              <div>
                <label className="block text-sm font-medium mb-2 text-gray-700 dark:text-gray-300">
                  Ask a Question
                </label>
                <div className="flex space-x-2">
                  <textarea
                    value={query}
                    onChange={(e) => setQuery(e.target.value)}
                    placeholder="What would you like to know about your documents?"
                    rows={3}
                    className="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-white resize-none"
                    onKeyDown={(e) => {
                      if (e.key === 'Enter' && !e.shiftKey) {
                        e.preventDefault();
                        handleRagChat();
                      }
                    }}
                  />
                  <button
                    onClick={handleRagChat}
                    disabled={isSearching || !query.trim()}
                    className="px-4 py-2 bg-primary-600 text-white rounded-lg hover:bg-primary-700 disabled:opacity-50 flex items-center space-x-2"
                  >
                    {isSearching ? (
                      <Loader2 size={20} className="animate-spin" />
                    ) : (
                      <MessageSquare size={20} />
                    )}
                    <span>Ask</span>
                  </button>
                </div>
              </div>

              {/* Response */}
              {chatResponse && (
                <div className="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4">
                  <h3 className="font-semibold mb-2 text-gray-900 dark:text-white">
                    Answer
                  </h3>
                  <p className="text-gray-800 dark:text-gray-200 whitespace-pre-wrap">
                    {chatResponse}
                  </p>
                </div>
              )}

              {/* Sources */}
              {searchResults && searchResults.length > 0 && (
                <div>
                  <h3 className="font-semibold mb-3 text-gray-900 dark:text-white">
                    Sources Used
                  </h3>
                  <div className="space-y-3">
                    {searchResults.map((result, idx) => (
                      <div
                        key={idx}
                        className="bg-gray-100 dark:bg-gray-800 rounded-lg p-4"
                      >
                        <div className="flex items-center justify-between mb-2">
                          <span className="text-sm font-medium text-gray-900 dark:text-white">
                            {result.document_name}
                          </span>
                          <span className="text-xs text-gray-500 dark:text-gray-400">
                            Similarity: {(result.similarity * 100).toFixed(1)}%
                          </span>
                        </div>
                        <p className="text-sm text-gray-700 dark:text-gray-300">
                          {result.chunk.content.substring(0, 200)}
                          {result.chunk.content.length > 200 && '...'}
                        </p>
                      </div>
                    ))}
                  </div>
                </div>
              )}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default RAG;
