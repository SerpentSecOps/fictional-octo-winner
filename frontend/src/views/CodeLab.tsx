import React, { useState, useRef } from 'react';
import Editor, { type Monaco } from '@monaco-editor/react';
import { Code, AlertCircle, CheckCircle } from 'lucide-react';
import { showInfo, showSuccess } from '../utils/toast';
import type { editor } from 'monaco-editor';

interface LintProblem {
  line: number;
  column: number;
  endLine: number;
  endColumn: number;
  severity: 'error' | 'warning' | 'info';
  message: string;
}

const CodeLab: React.FC = () => {
  const [code, setCode] = useState(`// JavaScript/TypeScript Code Editor
// Press "Lint Code" to check for issues

function greet(name) {
  return \`Hello, \${name}!\`;
}

console.log(greet("World"));

// Try adding some issues:
// - var x = 1; (prefer const/let)
// - Missing semicolons
// - Unused variables
`);
  const [language, setLanguage] = useState('javascript');
  const [problems, setProblems] = useState<LintProblem[]>([]);
  const editorRef = useRef<editor.IStandaloneCodeEditor | null>(null);
  const monacoRef = useRef<Monaco | null>(null);

  const handleEditorDidMount = (editor: editor.IStandaloneCodeEditor, monaco: Monaco) => {
    editorRef.current = editor;
    monacoRef.current = monaco;
  };

  // Basic linting rules for JavaScript/TypeScript
  const handleLint = async () => {
    if (!editorRef.current || !monacoRef.current) {
      showInfo('Editor not ready');
      return;
    }

    const model = editorRef.current.getModel();
    if (!model) return;

    const monaco = monacoRef.current;
    const diagnostics: LintProblem[] = [];
    const lines = code.split('\n');

    lines.forEach((line, lineIndex) => {
      const lineNum = lineIndex + 1;

      // Check for var usage (should use const/let)
      if (line.includes('var ') && !line.trim().startsWith('//')) {
        const column = line.indexOf('var ') + 1;
        diagnostics.push({
          line: lineNum,
          column,
          endLine: lineNum,
          endColumn: column + 3,
          severity: 'warning',
          message: "Unexpected 'var', use 'let' or 'const' instead",
        });
      }

      // Check for console.log (should remove in production)
      if (line.includes('console.log') && !line.trim().startsWith('//')) {
        const column = line.indexOf('console.log') + 1;
        diagnostics.push({
          line: lineNum,
          column,
          endLine: lineNum,
          endColumn: column + 11,
          severity: 'info',
          message: 'Unexpected console statement',
        });
      }

      // Check for missing semicolons
      const trimmed = line.trim();
      if (
        trimmed &&
        !trimmed.startsWith('//') &&
        !trimmed.startsWith('/*') &&
        !trimmed.endsWith(';') &&
        !trimmed.endsWith('{') &&
        !trimmed.endsWith('}') &&
        !trimmed.startsWith('function') &&
        !trimmed.startsWith('if') &&
        !trimmed.startsWith('for') &&
        !trimmed.startsWith('while') &&
        !trimmed.startsWith('return') &&
        !trimmed.endsWith(',')
      ) {
        diagnostics.push({
          line: lineNum,
          column: line.length,
          endLine: lineNum,
          endColumn: line.length,
          severity: 'warning',
          message: 'Missing semicolon',
        });
      }
    });

    // Convert to Monaco markers
    const markers = diagnostics.map((problem) => ({
      startLineNumber: problem.line,
      startColumn: problem.column,
      endLineNumber: problem.endLine,
      endColumn: problem.endColumn,
      severity:
        problem.severity === 'error'
          ? monaco.MarkerSeverity.Error
          : problem.severity === 'warning'
          ? monaco.MarkerSeverity.Warning
          : monaco.MarkerSeverity.Info,
      message: problem.message,
    }));

    monaco.editor.setModelMarkers(model, 'eslint', markers);
    setProblems(diagnostics);

    if (diagnostics.length === 0) {
      showSuccess('No problems found!');
    } else {
      showInfo(`Found ${diagnostics.length} problem${diagnostics.length > 1 ? 's' : ''}`);
    }
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
              className="px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 flex items-center space-x-1 text-sm"
            >
              <Code size={16} />
              <span>Lint Code</span>
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
        <div className="w-80 border-l border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 p-4 overflow-y-auto">
          <div className="flex items-center space-x-2 mb-4">
            {problems.length === 0 ? (
              <CheckCircle size={20} className="text-green-500" />
            ) : (
              <AlertCircle size={20} className="text-yellow-500" />
            )}
            <h2 className="font-semibold text-gray-900 dark:text-white">
              Problems {problems.length > 0 && `(${problems.length})`}
            </h2>
          </div>

          {problems.length === 0 ? (
            <div className="text-sm text-gray-600 dark:text-gray-400">
              <p>No problems detected. Click "Lint Code" to check for issues.</p>
            </div>
          ) : (
            <div className="space-y-2">
              {problems.map((problem, index) => (
                <div
                  key={index}
                  className={`p-2 rounded border text-xs ${
                    problem.severity === 'error'
                      ? 'bg-red-50 dark:bg-red-900/20 border-red-200 dark:border-red-800'
                      : problem.severity === 'warning'
                      ? 'bg-yellow-50 dark:bg-yellow-900/20 border-yellow-200 dark:border-yellow-800'
                      : 'bg-blue-50 dark:bg-blue-900/20 border-blue-200 dark:border-blue-800'
                  }`}
                >
                  <div className="flex items-start space-x-2">
                    <span
                      className={`font-semibold ${
                        problem.severity === 'error'
                          ? 'text-red-700 dark:text-red-300'
                          : problem.severity === 'warning'
                          ? 'text-yellow-700 dark:text-yellow-300'
                          : 'text-blue-700 dark:text-blue-300'
                      }`}
                    >
                      {problem.severity.toUpperCase()}
                    </span>
                    <div className="flex-1">
                      <p
                        className={`${
                          problem.severity === 'error'
                            ? 'text-red-800 dark:text-red-200'
                            : problem.severity === 'warning'
                            ? 'text-yellow-800 dark:text-yellow-200'
                            : 'text-blue-800 dark:text-blue-200'
                        }`}
                      >
                        {problem.message}
                      </p>
                      <p className="text-gray-600 dark:text-gray-400 mt-1">
                        Line {problem.line}, Column {problem.column}
                      </p>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}

          <div className="mt-6 p-3 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded">
            <p className="text-xs text-blue-800 dark:text-blue-200">
              <strong>Basic Linting Enabled:</strong> Checks for var usage, console statements, and missing semicolons.
              For full ESLint integration, add a Tauri command that runs ESLint via Node.js.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default CodeLab;
