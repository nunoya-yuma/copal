use futures::StreamExt;
use log::{error, warn};
use rig::agent::Agent;
use rig::agent::MultiTurnStreamItem;
use rig::completion::{CompletionModel, GetTokenUsage};
use rig::streaming::StreamedAssistantContent;
use rig::streaming::StreamingChat;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::io::{self, Write};

use super::render::{render_markdown, try_clear_lines};
use crate::cli::ConversationHistory;
use crate::cli::DEFAULT_MAX_TURNS;

const PROMPT: &str = "> ";
const HISTORY_FILE: &str = ".copal_history";

pub async fn run_interactive<M>(agent: Agent<M>)
where
    M: CompletionModel + 'static,
    M::StreamingResponse: GetTokenUsage,
{
    println!("Copal Interactive Mode");
    println!("Type 'exit' or 'quit' to exit, Ctrl+D to quit\n");

    let mut rl = DefaultEditor::new().expect("Failed to create editor");

    // Load history from previous sessions
    _ = rl.load_history(HISTORY_FILE);

    // Conversation history for multi-turn context
    let mut conversation_history = ConversationHistory::new(DEFAULT_MAX_TURNS);

    loop {
        let input = match rl.readline(PROMPT) {
            Ok(line) => line.trim().to_string(),
            Err(ReadlineError::Interrupted) => {
                /* Nothing to do */
                continue;
            }
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                warn!("Readline error: {}", err);
                break;
            }
        };

        if input.is_empty() {
            continue;
        }
        if input == "exit" || input == "quit" {
            break;
        }

        // Add input to history
        _ = rl.add_history_entry(&input);

        conversation_history.add_user(&input);

        // Stream with conversation history
        let mut stream = agent
            .stream_chat(&input, conversation_history.to_vec())
            .await;

        let mut response_text = String::new();

        while let Some(result) = stream.next().await {
            match result {
                Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Text(
                    text,
                ))) => {
                    print!("{}", text.text);
                    response_text.push_str(&text.text);
                    io::stdout().flush().unwrap();
                }
                Ok(MultiTurnStreamItem::FinalResponse(_)) => {
                    // Final response from LLM
                }
                Err(e) => {
                    error!("Stream error: {}", e);
                    break;
                }
                _ => {} // Others(tool call etc.)
            }
        }
        // Replace raw streamed text with rendered markdown
        if !response_text.is_empty() {
            if !try_clear_lines(&response_text) {
                // Text was too long to clear; add separator before rendered output
                println!("\n─────────────────────────────────────────");
            }
            render_markdown(&response_text);
        }
        conversation_history.add_assistant(&response_text);
    }

    // Save history for next session
    _ = rl.save_history(HISTORY_FILE);
}
