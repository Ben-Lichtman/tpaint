use bitflags::bitflags;

bitflags! {
	pub struct BoxFlags: u8 {
		const NONE = 0b0000;
		const UP = 0b0001;
		const DOWN = 0b0010;
		const LEFT = 0b0100;
		const RIGHT = 0b1000;
	}
}

impl BoxFlags {
	pub fn from_char(c: char, ascii_mode: bool) -> Self {
		match ascii_mode {
			false => match c {
				'╋' => Self::UP | Self::DOWN | Self::LEFT | Self::RIGHT,

				'┳' => Self::DOWN | Self::LEFT | Self::RIGHT,
				'┻' => Self::UP | Self::LEFT | Self::RIGHT,
				'┣' => Self::UP | Self::DOWN | Self::RIGHT,
				'┫' => Self::UP | Self::DOWN | Self::LEFT,

				'┛' => Self::UP | Self::LEFT,
				'┗' => Self::UP | Self::RIGHT,
				'┓' => Self::DOWN | Self::LEFT,
				'┏' => Self::DOWN | Self::RIGHT,

				'┃' => Self::UP | Self::DOWN,
				'━' => Self::LEFT | Self::RIGHT,

				'╹' => Self::UP,
				'╻' => Self::DOWN,
				'╸' => Self::LEFT,
				'╺' => Self::RIGHT,

				_ => Self::NONE,
			},
			true => match c {
				'+' => Self::UP | Self::DOWN | Self::LEFT | Self::RIGHT,
				'|' => Self::UP | Self::DOWN,
				'-' => Self::LEFT | Self::RIGHT,
				_ => Self::NONE,
			},
		}
	}

	pub fn to_char(self, ascii_mode: bool) -> char {
		match ascii_mode {
			false => match (
				self.contains(Self::UP),
				self.contains(Self::DOWN),
				self.contains(Self::LEFT),
				self.contains(Self::RIGHT),
			) {
				(true, true, true, true) => '╋',

				(false, true, true, true) => '┳',
				(true, false, true, true) => '┻',
				(true, true, false, true) => '┣',
				(true, true, true, false) => '┫',

				(true, false, true, false) => '┛',
				(true, false, false, true) => '┗',
				(false, true, true, false) => '┓',
				(false, true, false, true) => '┏',

				(true, true, false, false) => '┃',
				(false, false, true, true) => '━',

				(true, false, false, false) => '╹',
				(false, true, false, false) => '╻',
				(false, false, true, false) => '╸',
				(false, false, false, true) => '╺',

				_ => ' ',
			},
			true => match (
				self.contains(Self::UP),
				self.contains(Self::DOWN),
				self.contains(Self::LEFT),
				self.contains(Self::RIGHT),
			) {
				(true, true, true, false) => ' ',
				(true, true, false, false) => '|',
				(false, false, true, true) => '-',

				(true, false, false, false) => '|',
				(false, true, false, false) => '|',
				(false, false, true, false) => '-',
				(false, false, false, true) => '-',

				_ => '+',
			},
		}
	}
}
