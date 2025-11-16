import React, { useEffect } from 'react';
import { Toaster } from 'react-hot-toast';
import Sidebar from './components/Sidebar';
import Settings from './views/Settings';
import ChatV2 from './views/ChatV2';
import Canvas from './views/Canvas';
import CodeLab from './views/CodeLab';
import RAG from './views/RAG';
import { useAppStore } from './store/appStore';
import { getProviders } from './api/config';
import { listProjects } from './api/rag';

const App: React.FC = () => {
  const { currentView, setProviders, setProjects, setError } = useAppStore();
  const [isInitialLoading, setIsInitialLoading] = React.useState(true);

  useEffect(() => {
    // Load initial data
    const loadData = async () => {
      setIsInitialLoading(true);
      try {
        const [providers, projects] = await Promise.all([
          getProviders(),
          listProjects(),
        ]);
        setProviders(providers);
        setProjects(projects);
      } catch (error) {
        console.error('Failed to load initial data:', error);
        setError(error instanceof Error ? error.message : 'Failed to load data');
      } finally {
        setIsInitialLoading(false);
      }
    };

    loadData();
  }, [setProviders, setProjects, setError]);

  const renderView = () => {
    switch (currentView) {
      case 'chat':
        return <ChatV2 />;
      case 'rag':
        return <RAG />;
      case 'canvas':
        return <Canvas />;
      case 'codelab':
        return <CodeLab />;
      case 'settings':
        return <Settings />;
      default:
        return <ChatV2 />;
    }
  };

  if (isInitialLoading) {
    return (
      <div className="flex h-screen items-center justify-center bg-gray-100 dark:bg-gray-900">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-primary-600 mx-auto mb-4"></div>
          <p className="text-gray-600 dark:text-gray-400">Loading LLM Workbench...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex h-screen bg-gray-100 dark:bg-gray-900">
      <Toaster />
      <Sidebar />
      <main className="flex-1 overflow-hidden">{renderView()}</main>
    </div>
  );
};

export default App;
