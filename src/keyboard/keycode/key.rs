use super::macros::{normal, with_consts};

with_consts!(
    Key,
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub enum Key {
        A = 0x04,
        B = 0x05,
        C = 0x06,
        D = 0x07,
        E = 0x08,
        F = 0x09,
        G = 0x0A,
        H = 0x0B,
        I = 0x0C,
        J = 0x0D,
        K = 0x0E,
        L = 0x0F,
        M = 0x10,
        N = 0x11,
        O = 0x12,
        P = 0x13,
        Q = 0x14,
        R = 0x15,
        S = 0x16,
        T = 0x17,
        U = 0x18,
        V = 0x19,
        W = 0x1A,
        X = 0x1B,
        Y = 0x1C,
        Z = 0x1D,
        D1 = 0x1E,
        D2 = 0x1F,
        D3 = 0x20,
        D4 = 0x21,
        D5 = 0x22,
        D6 = 0x23,
        D7 = 0x24,
        D8 = 0x25,
        D9 = 0x26,
        D0 = 0x27,
        Enter = 0x28,
        Escape = 0x29,
        Backspace = 0x2A,
        Tab = 0x2B,
        Space = 0x2C,
        Minus = 0x2D,
        Equal = 0x2E,
        LeftBracket = 0x2F,
        RightBracket = 0x30,
        Backslash = 0x31,
        NonUsHash = 0x32,
        Semicolon = 0x33,
        Quote = 0x34,
        Grave = 0x35,
        Comma = 0x36,
        Dot = 0x37,
        Slash = 0x38,
        CapsLock = 0x39,
        F1 = 0x3A,
        F2 = 0x3B,
        F3 = 0x3C,
        F4 = 0x3D,
        F5 = 0x3E,
        F6 = 0x3F,
        F7 = 0x40,
        F8 = 0x41,
        F9 = 0x42,
        F10 = 0x43,
        F11 = 0x44,
        F12 = 0x45,
        PrintScreen = 0x46,
        ScrollLock = 0x47,
        Pause = 0x48,
        Insert = 0x49,
        Home = 0x4A,
        PageUp = 0x4B,
        Delete = 0x4C,
        End = 0x4D,
        PageDown = 0x4E,
        Right = 0x4F,
        Left = 0x50,
        Down = 0x51,
        Up = 0x52,
        NumLock = 0x53,
        KpSlash = 0x54,
        KpAsterisk = 0x55,
        KpMinus = 0x56,
        KpPlus = 0x57,
        KpEnter = 0x58,
        Kp1 = 0x59,
        Kp2 = 0x5A,
        Kp3 = 0x5B,
        Kp4 = 0x5C,
        Kp5 = 0x5D,
        Kp6 = 0x5E,
        Kp7 = 0x5F,
        Kp8 = 0x60,
        Kp9 = 0x61,
        Kp0 = 0x62,
        KpDot = 0x63,
        NonUsBackslash = 0x64,
        Application = 0x65,
        KbPower = 0x66,
        KpEqual = 0x67,
        F13 = 0x68,
        F14 = 0x69,
        F15 = 0x6A,
        F16 = 0x6B,
        F17 = 0x6C,
        F18 = 0x6D,
        F19 = 0x6E,
        F20 = 0x6F,
        F21 = 0x70,
        F22 = 0x71,
        F23 = 0x72,
        F24 = 0x73,
        Execute = 0x74,
        Help = 0x75,
        Menu = 0x76,
        Select = 0x77,
        Stop = 0x78,
        Again = 0x79,
        Undo = 0x7A,
        Cut = 0x7B,
        Copy = 0x7C,
        Paste = 0x7D,
        Find = 0x7E,
        Mute = 0x7F,
        VolumeUp = 0x80,
        VolumeDown = 0x81,
        LockingCapsLock = 0x82,
        LockingNumLock = 0x83,
        LockingScrollLock = 0x84,
        KpComma = 0x85,
        KpEqualSign = 0x86,
        International1 = 0x87,
        International2 = 0x88,
        International3 = 0x89,
        International4 = 0x8A,
        International5 = 0x8B,
        International6 = 0x8C,
        International7 = 0x8D,
        International8 = 0x8E,
        International9 = 0x8F,
        Lang1 = 0x90,
        Lang2 = 0x91,
        Lang3 = 0x92,
        Lang4 = 0x93,
        Lang5 = 0x94,
        Lang6 = 0x95,
        Lang7 = 0x96,
        Lang8 = 0x97,
        Lang9 = 0x98,
        AltErase = 0x99,
        SysReq = 0x9A,
        Cancel = 0x9B,
        Clear = 0x9C,
        Prior = 0x9D,
        Return = 0x9E,
        Separator = 0x9F,
        Out = 0xA0,
        Oper = 0xA1,
        ClearAgain = 0xA2,
        CrSel = 0xA3,
        ExSel = 0xA4,
        Kp00 = 0xB0,
        Kp000 = 0xB1,
        ThousandsSeparator = 0xB2,
        DecimalSeparator = 0xB3,
        CurrencyUnit = 0xB4,
        CurrencySubUnit = 0xB5,
        KpLeftParen = 0xB6,
        KpRightParen = 0xB7,
        KpLeftBrace = 0xB8,
        KpRightBrace = 0xB9,
        KpTab = 0xBA,
        KpBackspace = 0xBB,
        KpA = 0xBC,
        KpB = 0xBD,
        KpC = 0xBE,
        KpD = 0xBF,
        KpE = 0xC0,
        KpF = 0xC1,
        KpXor = 0xC2,
        KpPower = 0xC3,
        KpPercent = 0xC4,
        KpLess = 0xC5,
        KpGreater = 0xC6,
        KpAmpersand = 0xC7,
        KpDblAmpersand = 0xC8,
        KpVerticalBar = 0xC9,
        KpDblVerticalBar = 0xCA,
        KpColon = 0xCB,
        KpHash = 0xCC,
        KpSpace = 0xCD,
        KpAt = 0xCE,
        KpExclam = 0xCF,
        KpMemStore = 0xD0,
        KpMemRecall = 0xD1,
        KpMemClear = 0xD2,
        KpMemAdd = 0xD3,
        KpMemSubtract = 0xD4,
        KpMemMultiply = 0xD5,
        KpMemDivide = 0xD6,
        KpPlusMinus = 0xD7,
        KpClear = 0xD8,
        KpClearEntry = 0xD9,
        KpBinary = 0xDA,
        KpOctal = 0xDB,
        KpDecimal = 0xDC,
        KpHexadecimal = 0xDD,
        // LeftCtrl = 0xE0,
        // LeftShift = 0xE1,
        // LeftAlt = 0xE2,
        // LeftGui = 0xE3,
        // RightCtrl = 0xE4,
        // RightShift = 0xE5,
        // RightAlt = 0xE6,
        // RightGui = 0xE7,
    }
);

// aliases
normal!(BS, Key, Backspace);
normal!(DEL, Key, Delete);
normal!(ESC, Key, Escape);
normal!(SEMI, Key, Semicolon);
normal!(COMM, Key, Comma);
normal!(LBRC, Key, LeftBracket);
normal!(RBRC, Key, RightBracket);
normal!(PRTSC, Key, PrintScreen);
normal!(BSLSH, Key, Backslash);
normal!(SCLN, Key, Semicolon);
normal!(PGUP, Key, PageUp);
normal!(PGDN, Key, PageDown);

// Japanese keys
normal!(JCOLN, Key, Quote);
normal!(JBSLSH, Key, International1);
normal!(JBSLSH2, Key, International3);
normal!(JAT, Key, LeftBracket);
normal!(JCARET, Key, Equal);
normal!(JZNHN, Key, Grave);
normal!(JHNKN, Key, International4);
normal!(JMHNKN, Key, International5);
normal!(JLBRC, Key, RightBracket);
normal!(JRBRC, Key, NonUsHash);
