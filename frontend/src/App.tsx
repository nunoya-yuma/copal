import { ChatContainer } from './components/ChatContainer';
import { ChatInput } from './components/ChatInput';
import { useChat } from './hooks/useChat';
import './App.css';

function App() {
  const { messages, currentResponse, isStreaming, sendMessage } = useChat();

  return (
    <div className="app">
      <header>
        <h1>Copal</h1>
        <p>Personal Research Agent</p>
      </header>
      <main>
        <ChatContainer
          messages={messages}
          currentResponse={currentResponse}
          isStreaming={isStreaming}
        />
      </main>
      <footer>
        <ChatInput onSend={sendMessage} disabled={isStreaming} />
      </footer>
    </div>
  );
}

export default App;
