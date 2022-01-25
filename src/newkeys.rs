const KEY_LEFT_CTRL: u8 = 0x80;
const KEY_LEFT_SHIFT: u8 = 0x81;
const KEY_LEFT_ALT: u8 = 0x82;
const KEY_LEFT_GUI: u8 = 0x83;
const KEY_RIGHT_CTRL: u8 = 0x84;
const KEY_RIGHT_SHIFT: u8 = 0x85;
const KEY_RIGHT_ALT: u8 = 0x86;
const KEY_RIGHT_GUI: u8 = 0x87;
const KEY_UP_ARROW: u8 = 0xDA;
const KEY_DOWN_ARROW: u8 = 0xD9;
const KEY_LEFT_ARROW: u8 = 0xD8;
const KEY_RIGHT_ARROW: u8 = 0xD7;
const KEY_BACKSPACE: u8 = 0xB2;
const KEY_TAB: u8 = 0xB3;
const KEY_RETURN: u8 = 0xB0;
const KEY_ESC: u8 = 0xB1;
const KEY_INSERT: u8 = 0xD1;
const KEY_DELETE: u8 = 0xD4;
const KEY_PAGE_UP: u8 = 0xD3;
const KEY_PAGE_DOWN: u8 = 0xD6;
const KEY_HOME: u8 = 0xD2;
const KEY_END: u8 = 0xD5;
const KEY_CAPS_LOCK: u8 = 0xC1;
const KEY_F1: u8 = 0xC2;
const KEY_F2: u8 = 0xC3;
const KEY_F3: u8 = 0xC4;
const KEY_F4: u8 = 0xC5;
const KEY_F5: u8 = 0xC6;
const KEY_F6: u8 = 0xC7;
const KEY_F7: u8 = 0xC8;
const KEY_F8: u8 = 0xC9;
const KEY_F9: u8 = 0xCA;
const KEY_F10: u8 = 0xCB;
const KEY_F11: u8 = 0xCC;
const KEY_F12: u8 = 0xCD;
const KEY_F13: u8 = 0xF0;
const KEY_F14: u8 = 0xF1;
const KEY_F15: u8 = 0xF2;
const KEY_F16: u8 = 0xF3;
const KEY_F17: u8 = 0xF4;
const KEY_F18: u8 = 0xF5;
const KEY_F19: u8 = 0xF6;
const KEY_F20: u8 = 0xF7;
const KEY_F21: u8 = 0xF8;
const KEY_F22: u8 = 0xF9;
const KEY_F23: u8 = 0xFA;
const KEY_F24: u8 = 0xFB;


use sdl2::keyboard::Scancode;
pub fn scancode_to_key(scancode: Scancode) -> u8 {
    use Scancode::*;
    match scancode {
        A => b'a',
        B => b'b',
        C => b'c',
        D => b'd',
        E => b'e',
        F => b'f',
        G => b'g',
        H => b'h',
        I => b'i',
        J => b'j',
        K => b'k',
        L => b'l',
        M => b'm',
        N => b'n',
        O => b'o',
        P => b'p',
        Q => b'q',
        R => b'r',
        S => b's',
        T => b't',
        U => b'u',
        V => b'v',
        W => b'w',
        X => b'x',
        Y => b'y',
        Z => b'z',
        Num1 | Kp1 => b'1',
        Num2 | Kp2 => b'2',
        Num3 | Kp3 => b'3',
        Num4 | Kp4 => b'4',
        Num5 | Kp5 => b'5',
        Num6 | Kp6 => b'6',
        Num7 | Kp7 => b'7',
        Num8 | Kp8 => b'8',
        Num9 | Kp9 => b'9',
        Num0 | Kp0 => b'0',
        Return | KpEnter => KEY_RETURN,
        Escape => KEY_ESC,
        Backspace => KEY_BACKSPACE,
        Tab => KEY_TAB,
        Space => b' ',
        Minus => b'-',
        Equals => b'=',
        LeftBracket => b'{',
        RightBracket => b'{',
        Backslash => b'\\',
        //NonUsHash => HID_KEY_EUROPE_1,
        Semicolon => b';',
        Apostrophe => b'\'',
        Grave => b'`',
        Comma => b',',
        Period => b'.',
        Slash => b'/',
        CapsLock => KEY_CAPS_LOCK,
        F1 => KEY_F1,
        F2 => KEY_F2,
        F3 => KEY_F3,
        F4 => KEY_F4,
        F5 => KEY_F5,
        F6 => KEY_F6,
        F7 => KEY_F7,
        F8 => KEY_F8,
        F9 => KEY_F9,
        F10 => KEY_F10,
        F11 => KEY_F11,
        F12 => KEY_F12,
        //PrintScreen => HID_KEY_PRINT_SCREEN,
        //ScrollLock => HID_KEY_SCROLL_LOCK,
        //Pause => HID_KEY_PAUSE,
        Insert => KEY_INSERT,
        Home => KEY_HOME,
        PageUp => KEY_PAGE_UP,
        Delete => KEY_DELETE,
        End => KEY_END,
        PageDown => KEY_PAGE_DOWN,
        Right => KEY_RIGHT_ARROW,
        Left => KEY_LEFT_ARROW,
        Down => KEY_DOWN_ARROW,
        Up => KEY_UP_ARROW,
        //NumLockClear => HID_KEY_NUM_LOCK,
        KpDivide => b'/',
        KpMultiply => b'*',
        KpMinus => b'-',
        KpPlus => b'+',
        //Kp1 => HID_KEY_KEYPAD_1,
        //Kp2 => HID_KEY_KEYPAD_2,
        //Kp3 => HID_KEY_KEYPAD_3,
        //Kp4 => HID_KEY_KEYPAD_4,
        //Kp5 => HID_KEY_KEYPAD_5,
        //Kp6 => HID_KEY_KEYPAD_6,
        //Kp7 => HID_KEY_KEYPAD_7,
        //Kp8 => HID_KEY_KEYPAD_8,
        //Kp9 => HID_KEY_KEYPAD_9,
        //Kp0 => HID_KEY_KEYPAD_0,
        KpPeriod => b'.',
        //NonUsBackslash => HID_KEY_EUROPE_2,
        //Application => HID_KEY_APPLICATION,
        //Power => HID_KEY_POWER,
        //KpEquals => HID_KEY_KEYPAD_EQUAL,
        F13 => KEY_F13,
        F14 => KEY_F14,
        F15 => KEY_F15,
        F16 => KEY_F16,
        F17 => KEY_F17,
        F18 => KEY_F18,
        F19 => KEY_F19,
        F20 => KEY_F20,
        F21 => KEY_F21,
        F22 => KEY_F22,
        F23 => KEY_F23,
        F24 => KEY_F24,
        //Execute => HID_KEY_EXECUTE,
        //Help => HID_KEY_HELP,
        //Menu => HID_KEY_MENU,
        //Select => HID_KEY_SELECT,
        //Stop => HID_KEY_STOP,
        //Again => HID_KEY_AGAIN,
        //Undo => HID_KEY_UNDO,
        //Cut => HID_KEY_CUT,
        //Copy => HID_KEY_COPY,
        //Paste => HID_KEY_PASTE,
        //Find => HID_KEY_FIND,
        //Mute => HID_KEY_MUTE,
        //VolumeUp => HID_KEY_VOLUME_UP,
        //VolumeDown => HID_KEY_VOLUME_DOWN,
        //KpComma => HID_KEY_KEYPAD_COMMA,
        //KpEqualsAS400 => HID_KEY_KEYPAD_EQUAL_SIGN,
        //International1 => HID_KEY_KANJI1,
        //International2 => HID_KEY_KANJI2,
        //International3 => HID_KEY_KANJI3,
        //International4 => HID_KEY_KANJI4,
        //International5 => HID_KEY_KANJI5,
        //International6 => HID_KEY_KANJI6,
        //International7 => HID_KEY_KANJI7,
        //International8 => HID_KEY_KANJI8,
        //International9 => HID_KEY_KANJI9,
        //Lang1 => HID_KEY_LANG1,
        //Lang2 => HID_KEY_LANG2,
        //Lang3 => HID_KEY_LANG3,
        //Lang4 => HID_KEY_LANG4,
        //Lang5 => HID_KEY_LANG5,
        //Lang6 => HID_KEY_LANG6,
        //Lang7 => HID_KEY_LANG7,
        //Lang8 => HID_KEY_LANG8,
        //Lang9 => HID_KEY_LANG9,
        //AltErase => HID_KEY_ALTERNATE_ERASE,
        //SysReq => HID_KEY_SYSREQ_ATTENTION,
        //Cancel => HID_KEY_CANCEL,
        //Clear => HID_KEY_CLEAR,
        //Prior => HID_KEY_PRIOR,
        //Return2 => HID_KEY_RETURN,
        //Separator => HID_KEY_SEPARATOR,
        //Out => HID_KEY_OUT,
        //Oper => HID_KEY_OPER,
        //ClearAgain => HID_KEY_CLEAR_AGAIN,
        //CrSel => HID_KEY_CRSEL_PROPS,
        //ExSel => HID_KEY_EXSEL,

        LCtrl => KEY_LEFT_CTRL,
        LShift => KEY_LEFT_SHIFT,
        LAlt => KEY_LEFT_ALT,
        LGui => KEY_LEFT_GUI,
        RCtrl => KEY_RIGHT_CTRL,
        RShift => KEY_RIGHT_SHIFT,
        RAlt => KEY_RIGHT_ALT,
        RGui => KEY_RIGHT_GUI,

        _ => 0
    }
}