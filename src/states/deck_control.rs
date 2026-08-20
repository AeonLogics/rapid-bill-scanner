use crate::decks::{ArchiveDeck, TelemetryDeck};
use gpui::{
    App, AppContext, Context, Entity, Global, IntoElement, ParentElement, Render, Styled, Window,
    div,
};
use gpui_component::IconName;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Deck {
    #[default]
    Telemetry,
    Archive,
}

impl Deck {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Telemetry => "Telemetry",
            Self::Archive => "Archive",
        }
    }

    pub fn icon(&self) -> IconName {
        match self {
            Self::Telemetry => IconName::Bot,
            Self::Archive => IconName::MemoryStick,
        }
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Deck::Telemetry => write!(f, "Telemetry"),
            Deck::Archive => write!(f, "Archive"),
        }
    }
}

pub struct DeckControl {
    pub active: Deck,
    pub telemetry: Entity<TelemetryDeck>,
    pub archive: Entity<ArchiveDeck>,
}

impl DeckControl {
    pub fn new(cx: &mut App) -> Self {
        Self {
            active: Deck::default(),
            telemetry: TelemetryDeck::build(cx),
            archive: ArchiveDeck::build(cx),
        }
    }

    pub fn set(&mut self, deck: Deck, cx: &mut Context<Self>) {
        if self.active != deck {
            self.active = deck;
            cx.notify();
        }
    }
}

impl Render for DeckControl {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(match self.active {
            Deck::Telemetry => self.telemetry.clone().into_any_element(),
            Deck::Archive => self.archive.clone().into_any_element(),
        })
    }
}

pub struct GlobalDeck(pub Entity<DeckControl>);
impl Global for GlobalDeck {}

impl GlobalDeck {
    pub fn init(cx: &mut App) {
        let handle = cx.new(|cx| DeckControl::new(cx));
        cx.set_global(Self(handle));
    }

    pub fn set(cx: &mut App, deck: Deck) {
        if cx.has_global::<Self>() {
            let handle = cx.global::<Self>().0.clone();
            handle.update(cx, |control, cx| control.set(deck, cx));
        }
    }

    pub fn active(cx: &App) -> Deck {
        cx.global::<Self>().0.read(cx).active
    }

    pub fn handle(cx: &App) -> Entity<DeckControl> {
        cx.global::<Self>().0.clone()
    }
}
