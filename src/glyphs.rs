use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Glyph {
    icon: &'static str,
    pos: (u16, u16),
}

impl Glyph {
    pub const fn new(icon: &'static str, pos: (u16, u16)) -> Glyph {
        Glyph { icon, pos }
    }

    pub const fn icon(&self) -> &str {
        self.icon
    }

    pub const fn pos(&self) -> (u16, u16) {
        self.pos
    }

    pub const fn blanked(&self) -> Glyph {
        Glyph { icon: " ", ..*self }
    }
}

impl Display for Glyph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.icon)
    }
}

macro_rules! glyphs {
    ($($name:tt, $icon:literal, $pos:expr),*) => {
        $(
            pub const $name: Glyph = Glyph::new($icon, $pos);
        )*
    };
}

macro_rules! pet_impl {
    ($($name:tt, $icon:literal),*) => {
        #[derive(Debug, Clone, Copy)]
        pub struct Pet;

        impl Pet {
            $(
            pub const fn $name() -> &'static str {
                $icon
            }
            )*

            pub const fn pos() -> (u16, u16) {
                (10, 7)
            }
        }
    };
}

glyphs! {
    // Status indicators
    TOILET, "🚽", (0, 5),
    LETTER_T, "🇹", (0, 6),
    POOP, "💩",  (9, 9),
    LETTER_C, "🇨",  (9, 10),

    // Moods
    SMILEY, "🙂", (0, 3),
    WEARY, "😩", (0, 3),
    SICK, "🤕", (0, 3),

    // Actions
    MEAL, "🍔", (6, 12),
    SNACK, "🥨", (11, 12),
    BALL, "⚽", (16, 12),
    SCOLD_FINGER, "👉", (21, 12),

    // Action buttons
    DIGIT_1, "1️⃣", (6, 13),
    DIGIT_2, "2️⃣", (11, 13),
    DIGIT_3, "3️⃣", (16, 13),
    DIGIT_4, "3️⃣", (21, 13)
}

pet_impl! {
    neutral, "(\\_/)\n( •,•)\n(\")_(\")",
    neutral_blink, "(\\_/)\n( -,-)\n(\")_(\")",
    sad, "(\\(\\)\n( ..)\n((‘)(’)",
    sick, "(\\(\\)\n(– -)\n((‘)(’)",
    dead, "(\\(\\)\n(x x)\n((‘)(’)"
}
