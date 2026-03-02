pub(crate) mod data;
mod listener;

pub use data::*;
pub use listener::{Context, EventListener};

use crate::ClientInfo;

macro_rules! call_event_listeners {
    (
        $self:expr;
        $event:expr;
        $ctx:expr;
        $(
            $enum_name:ident => $fn_name:ident ;
        )*
        $(
            !custom
            $($custom_enum_name:ident => $custom_fn_name:ident ($apply:expr) ;)+
        )?
    ) => {
        match $event {
            $(
                $crate::events::Event::$enum_name(data) => {
                    for listener in &mut $self.listeners {
                        listener.$fn_name(&$ctx, data.clone()).await;
                    }
                }
            )*
            $(
                $(
                    $crate::events::Event::$custom_enum_name(data) => {
                        for listener in &mut $self.listeners {
                            listener.$custom_fn_name(&$ctx, $apply(data.clone())).await;
                        }
                    }
                )+
            )?
        }
    };
}

#[expect(unused)]
pub(super) enum Event {
    Ready(Box<ReadyEventData>),
    MessageCreate(Box<MessageCreateEventData>),
    GuildDelete(GuildDeleteEventData),
    GuildCreate(GuildCreateEventData),
}

pub(super) struct EventBus {
    listeners: Vec<Box<dyn EventListener + Send>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }

    pub fn register(&mut self, listener: Box<dyn EventListener + Send>) {
        self.listeners.push(listener);
    }

    pub async fn emit(&mut self, event: Event, client_info: ClientInfo) {
        let context = listener::Context { client_info };
        call_event_listeners! {
            self;
            event;
            context;
            GuildCreate => guild_create;
            GuildDelete => guild_delete;
            !custom
            Ready => ready (|data: Box<ReadyEventData>| { *data });
            MessageCreate => message_create (|data: Box<MessageCreateEventData>| { *data });
        }
    }
}
