enum MessageType {
    Message(UserMessage),
    Server(ServerMessage),
}

struct UserMessage {
    message: Message,
    user: User,
}

struct ServerMessage {
    message: Message,
}

struct Message {
    content: String,
}

impl Default for Message {
    fn default() -> Self {
        Message {
            content: "".to_string(),
        }
    }
}

struct User {
    username: String,
    nickname: String,
    color: Color,
    id: i32,
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Yellow,
    Purple,
    Gray,
    White,
}

impl Default for Color {
    fn default() -> Self {
        Color::White
    }
}
