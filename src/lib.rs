pub mod response {
    use serenity::builder::CreateEmbed;

    pub struct ResponseData {
        pub command: String,
        pub content: String,
    }
}