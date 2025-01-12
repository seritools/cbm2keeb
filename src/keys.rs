#![allow(dead_code)]

#[rustfmt::skip]
static KEYMAP: [[u8; 16]; 6] = [
    // PB0 ... PB7 → PA0 ... PA7, will be swapped to PA0 ... PA7 → PB0 ... PB7 in the inverse
    // keymap for better alignment when laying traces.

    //                                                                                                                                                            RVS,          GRAPH,
    [       KEY_F1, KEY_F2, KEY_F3, KEY_F4, KEY_F5, KEY_F6,    KEY_F7,    KEY_F8,        KEY_F9,        KEY_F10,       KEY_DOWN,        KEY_UP,   KEY_HOME,         0,        KEY_F12,   KEY_PAUSE,],
    //                                                                                                                                                   ?,        CE,
    [      KEY_ESC,  KEY_1,  KEY_2,  KEY_3,  KEY_4,  KEY_5,     KEY_7,     KEY_8,         KEY_9,          KEY_0,      KEY_EQUAL,      KEY_LEFT, KEY_INSERT,         0, KEY_KPASTERISK, KEY_KPSLASH,],
    //
    [      KEY_TAB,  KEY_Q,  KEY_W,  KEY_E,  KEY_R,  KEY_6,     KEY_U,     KEY_I,         KEY_O,      KEY_MINUS,  KEY_BACKSLASH,     KEY_RIGHT,    KEY_KP7,   KEY_KP8,        KEY_KP9, KEY_KPMINUS,],
    [            0,  KEY_A,  KEY_S,  KEY_D,  KEY_T,  KEY_Y,     KEY_J,     KEY_K,         KEY_L,          KEY_P, KEY_RIGHTBRACE, KEY_BACKSPACE,    KEY_KP4,   KEY_KP5,        KEY_KP6,  KEY_KPPLUS,],
    //                                                                                                                                      C=,
    [KEY_LEFTSHIFT,  KEY_Z,  KEY_X,  KEY_F,  KEY_G,  KEY_H,     KEY_M, KEY_COMMA, KEY_SEMICOLON,  KEY_LEFTBRACE,      KEY_ENTER, KEY_RIGHTMETA,    KEY_KP1,   KEY_KP2,        KEY_KP3, KEY_KPENTER,],
    //                                                                                                                        π,
    [ KEY_LEFTCTRL,      0,  KEY_C,  KEY_V,  KEY_B,  KEY_N, KEY_SPACE,   KEY_DOT,     KEY_SLASH, KEY_APOSTROPHE,   KEY_RIGHTALT,             0,    KEY_KP0, KEY_KPDOT,              0,           0,],
];

const fn create_inverse_keymap(keymap: [[u8; 16]; 6]) -> [(u8, u8); 256] {
    assert!(keymap.len().next_power_of_two() < (1 << crate::PINS_OUT_SHIFT));

    let mut inverse_keymap = [(0, 0); 256];

    let mut row = 0;
    while row < keymap.len() {
        let mut col = 0;
        while col < keymap[row].len() {
            let key = keymap[row][col];
            if key != 0 {
                let actual_col = if col <= 7 { col + 8 } else { col - 8 };

                inverse_keymap[key as usize] = (actual_col as u8, 1 << row);
            }

            col += 1;
        }
        row += 1;
    }

    inverse_keymap
}

/// maps hid keys to row and column bit, already shifted and negated
pub(crate) static INVERSE_KEYMAP: [(u8, u8); 256] = create_inverse_keymap(KEYMAP);

pub(crate) const KEY_NONE: u8 = 0x00; // No key pressed
pub(crate) const KEY_ERR_OVF: u8 = 0x01; //  Keyboard Error Roll Over - used for all slots if too many keys are pressed ("Phantom key")
pub(crate) const KEY_A: u8 = 0x04; // Keyboard a and A
pub(crate) const KEY_B: u8 = 0x05; // Keyboard b and B
pub(crate) const KEY_C: u8 = 0x06; // Keyboard c and C
pub(crate) const KEY_D: u8 = 0x07; // Keyboard d and D
pub(crate) const KEY_E: u8 = 0x08; // Keyboard e and E
pub(crate) const KEY_F: u8 = 0x09; // Keyboard f and F
pub(crate) const KEY_G: u8 = 0x0a; // Keyboard g and G
pub(crate) const KEY_H: u8 = 0x0b; // Keyboard h and H
pub(crate) const KEY_I: u8 = 0x0c; // Keyboard i and I
pub(crate) const KEY_J: u8 = 0x0d; // Keyboard j and J
pub(crate) const KEY_K: u8 = 0x0e; // Keyboard k and K
pub(crate) const KEY_L: u8 = 0x0f; // Keyboard l and L
pub(crate) const KEY_M: u8 = 0x10; // Keyboard m and M
pub(crate) const KEY_N: u8 = 0x11; // Keyboard n and N
pub(crate) const KEY_O: u8 = 0x12; // Keyboard o and O
pub(crate) const KEY_P: u8 = 0x13; // Keyboard p and P
pub(crate) const KEY_Q: u8 = 0x14; // Keyboard q and Q
pub(crate) const KEY_R: u8 = 0x15; // Keyboard r and R
pub(crate) const KEY_S: u8 = 0x16; // Keyboard s and S
pub(crate) const KEY_T: u8 = 0x17; // Keyboard t and T
pub(crate) const KEY_U: u8 = 0x18; // Keyboard u and U
pub(crate) const KEY_V: u8 = 0x19; // Keyboard v and V
pub(crate) const KEY_W: u8 = 0x1a; // Keyboard w and W
pub(crate) const KEY_X: u8 = 0x1b; // Keyboard x and X
pub(crate) const KEY_Y: u8 = 0x1c; // Keyboard y and Y
pub(crate) const KEY_Z: u8 = 0x1d; // Keyboard z and Z
pub(crate) const KEY_1: u8 = 0x1e; // Keyboard 1 and !
pub(crate) const KEY_2: u8 = 0x1f; // Keyboard 2 and @
pub(crate) const KEY_3: u8 = 0x20; // Keyboard 3 and #
pub(crate) const KEY_4: u8 = 0x21; // Keyboard 4 and $
pub(crate) const KEY_5: u8 = 0x22; // Keyboard 5 and %
pub(crate) const KEY_6: u8 = 0x23; // Keyboard 6 and ^
pub(crate) const KEY_7: u8 = 0x24; // Keyboard 7 and &
pub(crate) const KEY_8: u8 = 0x25; // Keyboard 8 and *
pub(crate) const KEY_9: u8 = 0x26; // Keyboard 9 and (
pub(crate) const KEY_0: u8 = 0x27; // Keyboard 0 and )
pub(crate) const KEY_ENTER: u8 = 0x28; // Keyboard Return (ENTER)
pub(crate) const KEY_ESC: u8 = 0x29; // Keyboard ESCAPE
pub(crate) const KEY_BACKSPACE: u8 = 0x2a; // Keyboard DELETE (Backspace)
pub(crate) const KEY_TAB: u8 = 0x2b; // Keyboard Tab
pub(crate) const KEY_SPACE: u8 = 0x2c; // Keyboard Spacebar
pub(crate) const KEY_MINUS: u8 = 0x2d; // Keyboard - and _
pub(crate) const KEY_EQUAL: u8 = 0x2e; // Keyboard = and +
pub(crate) const KEY_LEFTBRACE: u8 = 0x2f; // Keyboard [ and {
pub(crate) const KEY_RIGHTBRACE: u8 = 0x30; // Keyboard ] and }
pub(crate) const KEY_BACKSLASH: u8 = 0x31; // Keyboard \ and |
pub(crate) const KEY_HASHTILDE: u8 = 0x32; // Keyboard Non-US # and ~
pub(crate) const KEY_SEMICOLON: u8 = 0x33; // Keyboard ; and :
pub(crate) const KEY_APOSTROPHE: u8 = 0x34; // Keyboard ' and "
pub(crate) const KEY_GRAVE: u8 = 0x35; // Keyboard ` and ~
pub(crate) const KEY_COMMA: u8 = 0x36; // Keyboard , and <
pub(crate) const KEY_DOT: u8 = 0x37; // Keyboard . and >
pub(crate) const KEY_SLASH: u8 = 0x38; // Keyboard / and ?
pub(crate) const KEY_CAPSLOCK: u8 = 0x39; // Keyboard Caps Lock
pub(crate) const KEY_F1: u8 = 0x3a; // Keyboard F1
pub(crate) const KEY_F2: u8 = 0x3b; // Keyboard F2
pub(crate) const KEY_F3: u8 = 0x3c; // Keyboard F3
pub(crate) const KEY_F4: u8 = 0x3d; // Keyboard F4
pub(crate) const KEY_F5: u8 = 0x3e; // Keyboard F5
pub(crate) const KEY_F6: u8 = 0x3f; // Keyboard F6
pub(crate) const KEY_F7: u8 = 0x40; // Keyboard F7
pub(crate) const KEY_F8: u8 = 0x41; // Keyboard F8
pub(crate) const KEY_F9: u8 = 0x42; // Keyboard F9
pub(crate) const KEY_F10: u8 = 0x43; // Keyboard F10
pub(crate) const KEY_F11: u8 = 0x44; // Keyboard F11
pub(crate) const KEY_F12: u8 = 0x45; // Keyboard F12
pub(crate) const KEY_SYSRQ: u8 = 0x46; // Keyboard Print Screen
pub(crate) const KEY_SCROLLLOCK: u8 = 0x47; // Keyboard Scroll Lock
pub(crate) const KEY_PAUSE: u8 = 0x48; // Keyboard Pause
pub(crate) const KEY_INSERT: u8 = 0x49; // Keyboard Insert
pub(crate) const KEY_HOME: u8 = 0x4a; // Keyboard Home
pub(crate) const KEY_PAGEUP: u8 = 0x4b; // Keyboard Page Up
pub(crate) const KEY_DELETE: u8 = 0x4c; // Keyboard Delete Forward
pub(crate) const KEY_END: u8 = 0x4d; // Keyboard End
pub(crate) const KEY_PAGEDOWN: u8 = 0x4e; // Keyboard Page Down
pub(crate) const KEY_RIGHT: u8 = 0x4f; // Keyboard Right Arrow
pub(crate) const KEY_LEFT: u8 = 0x50; // Keyboard Left Arrow
pub(crate) const KEY_DOWN: u8 = 0x51; // Keyboard Down Arrow
pub(crate) const KEY_UP: u8 = 0x52; // Keyboard Up Arrow
pub(crate) const KEY_NUMLOCK: u8 = 0x53; // Keyboard Num Lock and Clear
pub(crate) const KEY_KPSLASH: u8 = 0x54; // Keypad /
pub(crate) const KEY_KPASTERISK: u8 = 0x55; // Keypad *
pub(crate) const KEY_KPMINUS: u8 = 0x56; // Keypad -
pub(crate) const KEY_KPPLUS: u8 = 0x57; // Keypad +
pub(crate) const KEY_KPENTER: u8 = 0x58; // Keypad ENTER
pub(crate) const KEY_KP1: u8 = 0x59; // Keypad 1 and End
pub(crate) const KEY_KP2: u8 = 0x5a; // Keypad 2 and Down Arrow
pub(crate) const KEY_KP3: u8 = 0x5b; // Keypad 3 and PageDn
pub(crate) const KEY_KP4: u8 = 0x5c; // Keypad 4 and Left Arrow
pub(crate) const KEY_KP5: u8 = 0x5d; // Keypad 5
pub(crate) const KEY_KP6: u8 = 0x5e; // Keypad 6 and Right Arrow
pub(crate) const KEY_KP7: u8 = 0x5f; // Keypad 7 and Home
pub(crate) const KEY_KP8: u8 = 0x60; // Keypad 8 and Up Arrow
pub(crate) const KEY_KP9: u8 = 0x61; // Keypad 9 and Page Up
pub(crate) const KEY_KP0: u8 = 0x62; // Keypad 0 and Insert
pub(crate) const KEY_KPDOT: u8 = 0x63; // Keypad . and Delete
pub(crate) const KEY_102ND: u8 = 0x64; // Keyboard Non-US \ and |
pub(crate) const KEY_COMPOSE: u8 = 0x65; // Keyboard Application
pub(crate) const KEY_POWER: u8 = 0x66; // Keyboard Power
pub(crate) const KEY_KPEQUAL: u8 = 0x67; // Keypad =
pub(crate) const KEY_F13: u8 = 0x68; // Keyboard F13
pub(crate) const KEY_F14: u8 = 0x69; // Keyboard F14
pub(crate) const KEY_F15: u8 = 0x6a; // Keyboard F15
pub(crate) const KEY_F16: u8 = 0x6b; // Keyboard F16
pub(crate) const KEY_F17: u8 = 0x6c; // Keyboard F17
pub(crate) const KEY_F18: u8 = 0x6d; // Keyboard F18
pub(crate) const KEY_F19: u8 = 0x6e; // Keyboard F19
pub(crate) const KEY_F20: u8 = 0x6f; // Keyboard F20
pub(crate) const KEY_F21: u8 = 0x70; // Keyboard F21
pub(crate) const KEY_F22: u8 = 0x71; // Keyboard F22
pub(crate) const KEY_F23: u8 = 0x72; // Keyboard F23
pub(crate) const KEY_F24: u8 = 0x73; // Keyboard F24
pub(crate) const KEY_OPEN: u8 = 0x74; // Keyboard Execute
pub(crate) const KEY_HELP: u8 = 0x75; // Keyboard Help
pub(crate) const KEY_PROPS: u8 = 0x76; // Keyboard Menu
pub(crate) const KEY_FRONT: u8 = 0x77; // Keyboard Select
pub(crate) const KEY_STOP: u8 = 0x78; // Keyboard Stop
pub(crate) const KEY_AGAIN: u8 = 0x79; // Keyboard Again
pub(crate) const KEY_UNDO: u8 = 0x7a; // Keyboard Undo
pub(crate) const KEY_CUT: u8 = 0x7b; // Keyboard Cut
pub(crate) const KEY_COPY: u8 = 0x7c; // Keyboard Copy
pub(crate) const KEY_PASTE: u8 = 0x7d; // Keyboard Paste
pub(crate) const KEY_FIND: u8 = 0x7e; // Keyboard Find
pub(crate) const KEY_MUTE: u8 = 0x7f; // Keyboard Mute
pub(crate) const KEY_VOLUMEUP: u8 = 0x80; // Keyboard Volume Up
pub(crate) const KEY_VOLUMEDOWN: u8 = 0x81; // Keyboard Volume Down
pub(crate) const KEY_KPCOMMA: u8 = 0x85; // Keypad Comma
pub(crate) const KEY_RO: u8 = 0x87; // Keyboard International1
pub(crate) const KEY_KATAKANAHIRAGANA: u8 = 0x88; // Keyboard International2
pub(crate) const KEY_YEN: u8 = 0x89; // Keyboard International3
pub(crate) const KEY_HENKAN: u8 = 0x8a; // Keyboard International4
pub(crate) const KEY_MUHENKAN: u8 = 0x8b; // Keyboard International5
pub(crate) const KEY_KPJPCOMMA: u8 = 0x8c; // Keyboard International6
pub(crate) const KEY_HANGEUL: u8 = 0x90; // Keyboard LANG1
pub(crate) const KEY_HANJA: u8 = 0x91; // Keyboard LANG2
pub(crate) const KEY_KATAKANA: u8 = 0x92; // Keyboard LANG3
pub(crate) const KEY_HIRAGANA: u8 = 0x93; // Keyboard LANG4
pub(crate) const KEY_ZENKAKUHANKAKU: u8 = 0x94; // Keyboard LANG5
pub(crate) const KEY_KPLEFTPAREN: u8 = 0xb6; // Keypad (
pub(crate) const KEY_KPRIGHTPAREN: u8 = 0xb7; // Keypad )
pub(crate) const KEY_LEFTCTRL: u8 = 0xe0; // Keyboard Left Control
pub(crate) const KEY_LEFTSHIFT: u8 = 0xe1; // Keyboard Left Shift
pub(crate) const KEY_LEFTALT: u8 = 0xe2; // Keyboard Left Alt
pub(crate) const KEY_LEFTMETA: u8 = 0xe3; // Keyboard Left GUI
pub(crate) const KEY_RIGHTCTRL: u8 = 0xe4; // Keyboard Right Control
pub(crate) const KEY_RIGHTSHIFT: u8 = 0xe5; // Keyboard Right Shift
pub(crate) const KEY_RIGHTALT: u8 = 0xe6; // Keyboard Right Alt
pub(crate) const KEY_RIGHTMETA: u8 = 0xe7; // Keyboard Right GUI
