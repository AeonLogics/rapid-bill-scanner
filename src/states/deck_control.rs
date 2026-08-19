use crate::decks::TelemetryDeck;
use gpui::{App, Global, IntoElement, ParentElement, RenderOnce, Styled, Window, div};
use gpui_component::IconName;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, IntoElement)]
pub enum Deck {
    #[default]
    Telemetry,
    // Archive,
    // System,
}

impl Global for Deck {}

impl Display for Deck {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Deck::Telemetry => write!(f, "Telemetry"),
        }
    }
}

impl Deck {
    pub fn init(cx: &mut App) {
        cx.set_global(Self::default());
    }

    pub fn set(cx: &mut App, new_deck: Deck) {
        cx.set_global(new_deck);
    }

    pub fn icon(&self) -> IconName {
        match self {
            Deck::Telemetry => IconName::SquareTerminal,
        }
    }
}

impl RenderOnce for Deck {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let active = if cx.has_global::<Self>() {
            *cx.global::<Self>()
        } else {
            Self::default()
        };

        div().size_full().child(match active {
            Deck::Telemetry => TelemetryDeck.into_element(),
            // Deck::Archive => ArchiveDeck.into_element(),
            // Deck::System => SystemDeck.into_element(),
        })
    }
}
