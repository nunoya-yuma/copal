import type { Message } from '../types';

export function exportToMarkdown(messages: Message[]): string {
  let markdownTxt = "# Copal Session";

  if (messages.length === 0) return markdownTxt;

  markdownTxt += '\n\n' + messages.map((message: Message) => {
    let insertMessage = '';
    switch (message.role) {
      case 'user':
        insertMessage = '## User\n' + message.content;
        break;
      case 'assistant':
        insertMessage = '## Assistant\n' + message.content;
        break;
      default:
        console.error("Unexpected message role: " + message.role);
        break;
    }
    return insertMessage;
  }).join('\n\n');
  return markdownTxt;
}
