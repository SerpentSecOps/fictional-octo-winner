import React, { useState, useCallback, useEffect } from 'react';
import ReactFlow, {
  Node,
  Edge,
  Controls,
  Background,
  useNodesState,
  useEdgesState,
  addEdge,
  Connection,
  BackgroundVariant,
} from 'reactflow';
import 'reactflow/dist/style.css';
import { useAppStore } from '../store/appStore';
import { getCanvasState, saveCanvasState } from '../api/canvas';
import { createProject, listProjects } from '../api/rag';
import { Plus, Save, FolderPlus } from 'lucide-react';
import { showError, showSuccess } from '../utils/toast';
import { logError } from '../utils/logger';

const Canvas: React.FC = () => {
  const { selectedProject, setSelectedProject, projects, setProjects } = useAppStore();
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);
  const [isSaving, setIsSaving] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [isCreatingProject, setIsCreatingProject] = useState(false);
  const [newProjectName, setNewProjectName] = useState('');
  const [showNewProject, setShowNewProject] = useState(false);

  // Load canvas state when project changes
  useEffect(() => {
    if (selectedProject) {
      loadCanvasState();
    }
  }, [selectedProject]);

  const loadCanvasState = async () => {
    if (!selectedProject) return;

    setIsLoading(true);
    try {
      const state = await getCanvasState(selectedProject.id);
      if (state) {
        // Convert API format to React Flow format
        const flowNodes: Node[] = state.nodes.map((n) => ({
          id: n.id,
          type: n.node_type,
          position: n.position,
          data: n.data,
        }));
        const flowEdges: Edge[] = state.edges;
        setNodes(flowNodes);
        setEdges(flowEdges);
      } else {
        // New canvas - start with a default node
        setNodes([
          {
            id: '1',
            type: 'default',
            position: { x: 250, y: 100 },
            data: { label: 'Start Here - Double click to edit' },
          },
        ]);
        setEdges([]);
      }
    } catch (error) {
      logError('Failed to load canvas:', error);
      showError('Failed to load canvas');
    } finally {
      setIsLoading(false);
    }
  };

  const handleSave = async () => {
    if (!selectedProject) return;

    setIsSaving(true);
    try {
      // Convert React Flow format to API format
      const state = {
        nodes: nodes.map((n) => ({
          id: n.id,
          node_type: n.type || 'default',
          position: n.position,
          data: n.data,
        })),
        edges: edges.map((e) => ({
          id: e.id,
          source: e.source,
          target: e.target,
        })),
      };

      await saveCanvasState(selectedProject.id, state);
      showSuccess('Canvas saved successfully!');
    } catch (error) {
      logError('Failed to save canvas:', error);
      showError('Failed to save canvas');
    } finally {
      setIsSaving(false);
    }
  };

  const onConnect = useCallback(
    (params: Connection) => setEdges((eds) => addEdge(params, eds)),
    [setEdges]
  );

  const addTextNode = () => {
    const newNode: Node = {
      id: `node_${Date.now()}`,
      type: 'default',
      position: { x: Math.random() * 500, y: Math.random() * 500 },
      data: { label: 'New Note' },
    };
    setNodes((nds) => [...nds, newNode]);
  };

  const handleCreateProject = async () => {
    if (!newProjectName.trim()) return;

    setIsCreatingProject(true);
    try {
      const project = await createProject(newProjectName);
      const updatedProjects = await listProjects();
      setProjects(updatedProjects);
      setSelectedProject(project);
      setNewProjectName('');
      setShowNewProject(false);
      showSuccess('Project created successfully!');
    } catch (error) {
      logError('Failed to create project:', error);
      showError('Failed to create project');
    } finally {
      setIsCreatingProject(false);
    }
  };

  return (
    <div className="h-full flex flex-col bg-white dark:bg-gray-900">
      {/* Header */}
      <div className="border-b border-gray-200 dark:border-gray-700 p-4 bg-white dark:bg-gray-800">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-4">
            <h1 className="text-xl font-semibold text-gray-900 dark:text-white">Canvas</h1>

            {/* Project selector */}
            <select
              value={selectedProject?.id || ''}
              onChange={(e) => {
                const project = projects.find(
                  (p) => p.id === parseInt(e.target.value)
                );
                setSelectedProject(project || null);
              }}
              className="px-3 py-1 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm"
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
              className="px-3 py-1 bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded hover:bg-gray-300 dark:hover:bg-gray-600 flex items-center space-x-1 text-sm"
            >
              <FolderPlus size={16} />
              <span>New Project</span>
            </button>
          </div>

          {selectedProject && (
            <div className="flex items-center space-x-2">
              <button
                onClick={addTextNode}
                className="px-3 py-1 bg-primary-600 text-white rounded hover:bg-primary-700 flex items-center space-x-1 text-sm"
              >
                <Plus size={16} />
                <span>Add Node</span>
              </button>
              <button
                onClick={handleSave}
                disabled={isSaving}
                className="px-3 py-1 bg-green-600 text-white rounded hover:bg-green-700 disabled:opacity-50 flex items-center space-x-1 text-sm"
              >
                <Save size={16} />
                <span>{isSaving ? 'Saving...' : 'Save'}</span>
              </button>
            </div>
          )}
        </div>

        {/* New project form */}
        {showNewProject && (
          <div className="mt-3 flex items-center space-x-2">
            <input
              type="text"
              value={newProjectName}
              onChange={(e) => setNewProjectName(e.target.value)}
              placeholder="Project name"
              className="px-3 py-1 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm"
              onKeyDown={(e) => e.key === 'Enter' && handleCreateProject()}
            />
            <button
              onClick={handleCreateProject}
              disabled={isCreatingProject}
              className="px-3 py-1 bg-primary-600 text-white rounded hover:bg-primary-700 disabled:opacity-50 text-sm"
            >
              {isCreatingProject ? 'Creating...' : 'Create'}
            </button>
            <button
              onClick={() => {
                setShowNewProject(false);
                setNewProjectName('');
              }}
              className="px-3 py-1 bg-gray-300 dark:bg-gray-600 text-gray-700 dark:text-gray-200 rounded hover:bg-gray-400 dark:hover:bg-gray-500 text-sm"
            >
              Cancel
            </button>
          </div>
        )}
      </div>

      {/* Canvas */}
      <div className="flex-1">
        {isLoading ? (
          <div className="flex items-center justify-center h-full text-gray-500 dark:text-gray-400">
            <p>Loading canvas...</p>
          </div>
        ) : selectedProject ? (
          <ReactFlow
            nodes={nodes}
            edges={edges}
            onNodesChange={onNodesChange}
            onEdgesChange={onEdgesChange}
            onConnect={onConnect}
            fitView
          >
            <Background variant={BackgroundVariant.Dots} />
            <Controls />
          </ReactFlow>
        ) : (
          <div className="flex items-center justify-center h-full text-gray-500 dark:text-gray-400">
            <p>Select or create a project to use the canvas</p>
          </div>
        )}
      </div>
    </div>
  );
};

export default Canvas;
