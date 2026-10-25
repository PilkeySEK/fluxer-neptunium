#[macro_export]
macro_rules! create_embed {
    (
        $(title: $title:expr,)?
        $(description: $description:expr,)?
        $(color: $color:expr,)?
        $(author: {
            name: $author_name:expr
            $(, url: $author_url:expr)?
            $(, icon_url: $author_icon_url:expr)? $(,)?
        } $(,)?)?
        $($([
            name: $field_name:expr,
            content: $field_content:expr
            $(, inline: $field_inline:expr)? $(,)?
        ])+)? $(,)?
        $(footer: {
            text: $footer_text:expr
            $(, icon_url: $footer_icon_url:expr)?
            $(, timestamp: $footer_timestamp:expr)? $(,)?
        })? $(,)?
    ) => {{
        $crate::model::channel::message::embed::RichEmbed::builder()
            $(.title($title))?
            $(.description($description))?
            $(.color($crate::model::misc::HexColor::new($color)))?
            $(.author($crate::model::channel::message::embed::EmbedAuthorInput::builder()
                .name($author_name)
                $(.url($author_url))?
                $(.icon_url($author_icon_url))?
                .build()
            ))?
            $(.fields(vec![$(
                $crate::model::channel::message::embed::EmbedFieldInput::builder()
                .name($field_name)
                .value($field_content)
                $(.inline($field_inline))?
                .build(),
            )+]))?
            $(.footer($crate::model::channel::message::embed::EmbedFooterInput::builder()
                .text($footer_text)
                $(.icon_url($footer_icon_url))?
                .build()
            ))?
            $($(.timestamp($footer_timestamp))?)?
            .build()
    }};
}

#[cfg(test)]
#[expect(clippy::unreadable_literal)]
mod tests {
    use neptunium_model::{
        channel::message::embed::{EmbedAuthorInput, EmbedFieldInput, EmbedFooterInput, RichEmbed},
        time::OffsetDateTime,
    };

    #[test]
    fn create_embed_macro() {
        let timestamp = OffsetDateTime::now_utc();
        let embed = create_embed!(
            title: "Hi!",
            description: "This is the description",
            color: 0xff0000,
            author: {
                name: "person",
                icon_url: "example.com",
            }
            [
                name: "Field 1",
                content: "Field content 1",
                inline: false
            ]
            [
                name: "Field 2",
                content: "Field content 2"
            ]
            footer: {
                text: "abc",
                timestamp: timestamp,
            }
        );

        assert_eq!(
            embed,
            RichEmbed::builder()
                .title("Hi!")
                .description("This is the description")
                .color(0xff0000)
                .author(
                    EmbedAuthorInput::builder()
                        .name("person")
                        .icon_url("example.com")
                        .build()
                )
                .fields(vec![
                    EmbedFieldInput::builder()
                        .name("Field 1")
                        .value("Field content 1")
                        .inline(false)
                        .build(),
                    EmbedFieldInput::builder()
                        .name("Field 2")
                        .value("Field content 2")
                        .build()
                ])
                .footer(EmbedFooterInput::builder().text("abc").build())
                .timestamp(timestamp)
                .build()
        );
    }
}
