use anthropic_sdk::clients::AnthropicClient;
use anthropic_sdk::types::message::{
    CountMessageTokensParams, Message, MessageClient, MessageError, Role,
};
use std::env;
use tracing::{error, info};

/// cd anthropic-sdk-rs/examples/messages
/// cargo run

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(false)
        .with_level(true)
        .try_init()
        .expect("Failed to initialize logger");

    let api_key = env::var("ANTHROPIC_API_KEY").unwrap();
    let api_version = env::var("ANTHROPIC_API_VERSION").unwrap_or("2023-06-01".to_string());

    let client = AnthropicClient::new::<MessageError>(api_key, api_version).unwrap();

    let body = CountMessageTokensParams {
        model: "claude-3-5-sonnet-20240620".to_string(),
        messages: vec![Message::new_text(Role::User, "Hello, Claude")],
    };

    info!("body: {:?}", body);

    match client.count_tokens(Some(&body)).await {
        Ok(message) => {
            info!("Successfully created message: {:?}", message.content);
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    }
}

//// Simple text message
//let text_message = Message::new_text(Role::User, "Hello, Claude");
//
//// Message with text block
//let text_block_message = Message::new_blocks(
//    Role::User,
//    vec![ContentBlock::text("Hello, Claude")],
//);
//
//// Message with image and text
//let mixed_message = Message::new_blocks(
//    Role::User,
//    vec![
//        ContentBlock::image(
//            "base64",
//            "image/jpeg",
//            "/9j/4AAQSkZJRg...",
//        ),
//        ContentBlock::text("What is in this image?"),
//    ],
//);
//
//// Using in CreateMessageParams
//let params = CreateMessageParams {
//    model: "claude-3-5-sonnet-20240620".to_string(),
//    messages: vec![text_message],
//    max_tokens: 1024,
//    // ... other fields
//};
