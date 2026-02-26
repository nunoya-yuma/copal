import { ChatContainer } from './components/ChatContainer';
import { ChatInput } from './components/ChatInput';
import { TokenGate } from './components/TokenGate';
import { useAuth } from './hooks/useAuth';
import { useChat } from './hooks/useChat';
import './App.css';

function App() {
  const { token, setToken, clearToken, isAuthenticated } = useAuth();
  const { messages, currentResponse, isStreaming, errorMessage, sendMessage } = useChat(token, clearToken);

  if (!isAuthenticated) {
    return <TokenGate onSubmit={setToken} />;
  }

  return (
    <div className="app">
      <header>
        <h1>Copal</h1>
        <p>Personal Research Agent</p>
        <button onClick={clearToken} className="disconnect-button">切断</button>
      </header>
      <main>
        <ChatContainer
          messages={messages}
          currentResponse={currentResponse}
          isStreaming={isStreaming}
        />
      </main>
      {errorMessage && <div className="error-message">{errorMessage}</div>}
      <footer>
        <ChatInput onSend={sendMessage} disabled={isStreaming} />
      </footer>
    </div>
  );
}

export default App;
