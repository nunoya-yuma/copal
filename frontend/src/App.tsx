import { ChatContainer } from './components/ChatContainer';
import { ChatInput } from './components/ChatInput';
import { ResearchPhaseIndicator } from './components/ResearchPhaseIndicator';
import { TokenGate } from './components/TokenGate';
import { useAuth } from './hooks/useAuth';
import { useChat } from './hooks/useChat';
import { exportToMarkdown } from './utils/export';
import './App.css';

function App() {
  const { token, setToken, clearToken, isAuthenticated } = useAuth();
  const { messages, currentResponse, isStreaming, errorMessage, currentPhase, sendMessage, stopGeneration } = useChat(token, clearToken);

  const handleExport = () => {
    const markdown = exportToMarkdown(messages);
    const blob = new Blob([markdown], { type: 'text/markdown' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `copal-session-${Date.now()}.md`;
    a.click();
    URL.revokeObjectURL(url);
  };

  if (!isAuthenticated) {
    return <TokenGate onSubmit={setToken} />;
  }

  return (
    <div className="app">
      <header>
        <h1>Copal</h1>
        <p>Personal Research Agent</p>
        <div className="header-actions">
          {messages.length > 0 && (
            <button onClick={handleExport} className="export-button">📥 エクスポート</button>
          )}
          <button onClick={clearToken} className="disconnect-button">切断</button>
        </div>
      </header>
      <main>
        <ChatContainer
          messages={messages}
          currentResponse={currentResponse}
          isStreaming={isStreaming}
        />
        <ResearchPhaseIndicator currentPhase={currentPhase} />
      </main>
      {errorMessage && <div className="error-message">{errorMessage}</div>}
      <footer>
        <ChatInput onSend={sendMessage} onStop={stopGeneration} disabled={isStreaming} isStreaming={isStreaming} />
      </footer>
    </div>
  );
}

export default App;
