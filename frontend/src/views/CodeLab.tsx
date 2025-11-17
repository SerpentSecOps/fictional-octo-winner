import React, { useState, useRef } from 'react';
import Editor, { type Monaco } from '@monaco-editor/react';
import { Code, AlertCircle } from 'lucide-react';
import { showInfo } from '../utils/toast';
import type { editor } from 'monaco-editor';

const CodeLab: React.FC = () => {
  const [code, setCode] = useState(`// JavaScript/TypeScript Code Editor
// TODO: Implement ESLint integration via Tauri command

function greet(name) {
  return \`Hello, \${name}!\`;
}

console.log(greet("World"));
`);
  const [language, setLanguage] = useState('javascript');
  const editorRef = useRef<editor.IStandaloneCodeEditor | null>(null);

  const handleEditorDidMount = (editor: editor.IStandaloneCodeEditor, monaco: Monaco) => {
    editorRef.current = editor;
  };

  // TODO: Implement lint_code Tauri command
  const handleLint = async () => {
    showInfo('Linting feature TODO: Requires ESLint integration via Tauri command');
  };

  return (
    <div className="h-full flex flex-col bg-white dark:bg-gray-900">
      {/* Header */}
      <div className="border-b border-gray-200 dark:border-gray-700 p-4 bg-white dark:bg-gray-800">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-4">
            <h1 className="text-xl font-semibold text-gray-900 dark:text-white">
              Code Lab
            </h1>

            <select
              value={language}
              onChange={(e) => setLanguage(e.target.value)}
              className="px-3 py-1 border border-gray-300 dark:border-gray-600 rounded bg-white dark:bg-gray-700 text-gray-900 dark:text-white text-sm"
            >
              <option value="javascript">JavaScript</option>
              <option value="typescript">TypeScript</option>
              <option value="python">Python</option>
              <option value="rust">Rust</option>
              <option value="json">JSON</option>
            </select>
          </div>

          <div className="flex items-center space-x-2">
            <button
              onClick={handleLint}
              className="px-3 py-1 bg-primary-600 text-white rounded hover:bg-primary-700 flex items-center space-x-1 text-sm"
            >
              <Code size={16} />
              <span>Lint Code (TODO)</span>
            </button>
          </div>
        </div>
      </div>

      {/* Editor */}
      <div className="flex-1 flex">
        <div className="flex-1">
          <Editor
            height="100%"
            language={language}
            value={code}
            onChange={(value) => setCode(value || '')}
            onMount={handleEditorDidMount}
            theme="vs-dark"
            options={{
              minimap: { enabled: false },
              fontSize: 14,
              lineNumbers: 'on',
              roundedSelection: false,
              scrollBeyondLastLine: false,
              automaticLayout: true,
            }}
          />
        </div>

        {/* Problems panel */}
        <div className="w-80 border-l border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 p-4">
          <div className="flex items-center space-x-2 mb-4">
            <AlertCircle size={20} className="text-yellow-500" />
            <h2 className="font-semibold text-gray-900 dark:text-white">Problems</h2>
          </div>

          <div className="text-sm text-gray-600 dark:text-gray-400">
            <p className="mb-2">TODO: Implement linting system</p>
            <ul className="list-disc pl-5 space-y-1">
              <li>Add Tauri command for ESLint</li>
              <li>Parse lint diagnostics</li>
              <li>Display in this panel</li>
              <li>Add inline markers in editor</li>
            </ul>
          </div>

          <div className="mt-6 p-3 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded">
            <p className="text-xs text-blue-800 dark:text-blue-200">
              <strong>Note:</strong> The Code Lab provides a Monaco-powered editor
              with syntax highlighting. Full ESLint integration can be added by
              implementing a Tauri command that runs ESLint via Node.js subprocess
              and returns diagnostics.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default CodeLab;
