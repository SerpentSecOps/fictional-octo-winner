import React from 'react';
import { useAppStore } from '../store/appStore';
import type { ViewType } from '../store/appStore';
import {
  MessageSquare,
  Database,
  Layout,
  Code,
  Settings,
  FolderOpen,
} from 'lucide-react';

const Sidebar: React.FC = () => {
  const { currentView, setCurrentView, selectedProject } = useAppStore();

  const navItems: Array<{ id: ViewType; label: string; icon: React.ReactNode }> = [
    { id: 'chat', label: 'Chat', icon: <MessageSquare size={20} /> },
    { id: 'rag', label: 'RAG', icon: <Database size={20} /> },
    { id: 'canvas', label: 'Canvas', icon: <Layout size={20} /> },
    { id: 'codelab', label: 'Code Lab', icon: <Code size={20} /> },
    { id: 'settings', label: 'Settings', icon: <Settings size={20} /> },
  ];

  return (
    <div className="w-16 bg-gray-900 flex flex-col items-center py-4 space-y-4">
      {navItems.map((item) => (
        <button
          key={item.id}
          onClick={() => setCurrentView(item.id)}
          className={`w-12 h-12 flex items-center justify-center rounded-lg transition-colors ${
            currentView === item.id
              ? 'bg-primary-600 text-white'
              : 'text-gray-400 hover:bg-gray-800 hover:text-white'
          }`}
          title={item.label}
        >
          {item.icon}
        </button>
      ))}

      {selectedProject && (
        <div className="mt-auto pt-4 border-t border-gray-800">
          <div className="w-12 h-12 flex items-center justify-center">
            <FolderOpen size={20} className="text-gray-400" />
          </div>
        </div>
      )}
    </div>
  );
};

export default Sidebar;
