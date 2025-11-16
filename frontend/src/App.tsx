import React, { useEffect } from 'react';
import Sidebar from './components/Sidebar';
import Settings from './views/Settings';
import Chat from './views/Chat';
import Canvas from './views/Canvas';
import CodeLab from './views/CodeLab';
import { useAppStore } from './store/appStore';
import { getProviders } from './api/config';
import { listProjects } from './api/rag';

const App: React.FC = () => {
  const { currentView, setProviders, setProjects, setError } = useAppStore();

  useEffect(() => {
    // Load initial data
    const loadData = async () => {
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
      }
    };

    loadData();
  }, [setProviders, setProjects, setError]);

  const renderView = () => {
    switch (currentView) {
      case 'chat':
        return <Chat />;
      case 'canvas':
        return <Canvas />;
      case 'codelab':
        return <CodeLab />;
      case 'settings':
        return <Settings />;
      default:
        return <Chat />;
    }
  };

  return (
    <div className="flex h-screen bg-gray-100 dark:bg-gray-900">
      <Sidebar />
      <main className="flex-1 overflow-hidden">{renderView()}</main>
    </div>
  );
};

export default App;
