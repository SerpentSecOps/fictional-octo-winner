import { create } from 'zustand';
import type { MaskedProviderConfig, Project } from '../api/types';

export type ViewType = 'chat' | 'canvas' | 'codelab' | 'settings';

interface AppState {
  // Current view
  currentView: ViewType;
  setCurrentView: (view: ViewType) => void;

  // Providers
  providers: MaskedProviderConfig[];
  setProviders: (providers: MaskedProviderConfig[]) => void;

  // Selected provider
  selectedProvider: string | null;
  setSelectedProvider: (providerId: string | null) => void;

  // Projects
  projects: Project[];
  setProjects: (projects: Project[]) => void;

  // Selected project
  selectedProject: Project | null;
  setSelectedProject: (project: Project | null) => void;

  // Loading state
  isLoading: boolean;
  setIsLoading: (loading: boolean) => void;

  // Error state
  error: string | null;
  setError: (error: string | null) => void;
}

export const useAppStore = create<AppState>((set) => ({
  currentView: 'chat',
  setCurrentView: (view) => set({ currentView: view }),

  providers: [],
  setProviders: (providers) => set({ providers }),

  selectedProvider: null,
  setSelectedProvider: (providerId) => set({ selectedProvider: providerId }),

  projects: [],
  setProjects: (projects) => set({ projects }),

  selectedProject: null,
  setSelectedProject: (project) => set({ selectedProject: project }),

  isLoading: false,
  setIsLoading: (loading) => set({ isLoading: loading }),

  error: null,
  setError: (error) => set({ error }),
}));
