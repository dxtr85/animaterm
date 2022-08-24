#[derive(Debug, PartialEq)]
pub enum Key {
    CtrlA,
    CtrlB,
    CtrlC,
    CtrlD,
    CtrlE,
    CtrlF,
    CtrlG,
    Backspace,
    Tab,
    Enter, // Ctrl_j
    CtrlK,
    CtrlL,
    CtrlM,
    CtrlN,
    CtrlO,
    CtrlP,
    CtrlQ,
    CtrlR,
    CtrlS,
    CtrlT,
    CtrlU,
    CtrlV,
    CtrlW,
    CtrlX,
    CtrlY,
    CtrlZ,
    Escape,
    FileSeparator,
    GroupSeparator,
    RecordSeparator,
    UnitSeparator,
    Space,           // 32
    ExclamationMark, //       33
    Quote,           //       34
    Hash,            //       35
    Dollar,          //       36
    Percent,         //       37
    Ampersand,       //       38
    Apostrophe,      //       39
    LeftParen,       //       40
    RightParen,      //       41
    Star,            //       42
    Plus,            //       43
    Comma,           //       44
    Dash,            //       45
    Period,          //       46
    Slash,           //       47
    Zero,            //       48
    One,             //       49
    Two,             //       50
    Three,           //       51
    Four,            //       52
    Five,            //       53
    Six,             //       54
    Seven,           //       55
    Eight,           //       56
    Nine,            //       57
    Colon,           //       58
    Semicolon,       //       59
    LessThan,        //       60
    Equals,          //       61
    GreaterThan,     //       62
    QuestionMark,    //       63
    At,              //	      64
    ShiftA,
    ShiftB,
    ShiftC,
    ShiftD,
    ShiftE,
    ShiftF,
    ShiftG,
    ShiftH,
    ShiftI,
    ShiftJ,
    ShiftK,
    ShiftL,
    ShiftM,
    ShiftN,
    ShiftO,
    ShiftP,
    ShiftQ,
    ShiftR,
    ShiftS,
    ShiftT,
    ShiftU,
    ShiftV,
    ShiftW,
    ShiftX,
    ShiftY,
    ShiftZ,
    LeftBracket,  //       91
    BackSlash,    //       92
    RightBracket, //       93
    Caret,        //       94
    Underscore,   //       95
    BackTick,     //       96
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LeftBrace,  //      123
    Pipe,       //      124
    RightBrace, //      125
    Tilde,      //      126
    Delete,     //      127

    AltCtrlA,           // 27, 1
    AltCtrlB,           // 27, 2
    AltCtrlC,           // 27, 3
    AltCtrlD,           // 27, 4
    AltCtrlE,           // 27, 5
    AltCtrlF,           // 27, 6
    AltCtrlG,           // 27, 7
    AltCtrlH,           // 27, 8
    AltTab,             // 27, 9
    AltEnter,           // 27, 10
    AltCtrlK,           // 27, 11
    AltCtrlL,           // 27, 12
    AltCtrlM,           // 27, 13
    AltCtrlN,           // 27, 14
    AltCtrlO,           // 27, 15
    AltCtrlP,           // 27, 16
    AltCtrlQ,           // 27, 17
    AltCtrlR,           // 27, 18
    AltCtrlS,           // 27, 19
    AltCtrlT,           // 27, 20
    AltCtrlU,           // 27, 21
    AltCtrlV,           // 27, 22
    AltCtrlW,           // 27, 23
    AltCtrlX,           // 27, 24
    AltCtrlY,           // 27, 25
    AltCtrlZ,           // 27, 26
    AltEscape,          // 27, 27
    AltFileSeparator,   // 27, 28
    AltGroupSeparator,  // 27, 29
    AltRecordSeparator, // 27, 30
    AltUnitSeparator,   // 27, 31
    AltSpace,           // 27, 32
    AltExclamationMark, // 27, 33
    AltQuote,           // 27, 34
    AltHash,            // 27, 35
    AltDollar,          // 27, 36
    AltPercent,         // 27, 37
    AltAmpersand,       // 27, 38
    AltApostrophe,      // 27, 39
    AltLeftParen,       // 27, 40
    AltRightParen,      // 27, 41
    AltStar,            // 27, 42
    AltPlus,            // 27, 43
    AltComma,           // 27, 44
    AltDash,            // 27, 45
    AltPeriod,          // 27, 46
    AltSlash,           // 27, 47
    AltZero,            // 27, 48
    AltOne,             // 27, 49
    AltTwo,             // 27, 50
    AltThree,           // 27, 51
    AltFour,            // 27, 52
    AltFive,            // 27, 53
    AltSix,             // 27, 54
    AltSeven,           // 27, 55
    AltEight,           // 27, 56
    AltNine,            // 27, 57
    AltColon,           // 27, 58
    AltSemicolon,       // 27, 59
    AltLessThan,        // 27, 60
    AltEquals,          // 27, 61
    AltGreaterThan,     // 27, 62
    AltQuestionMark,    // 27, 63
    AltAt,              // 27, 64
    AltShiftA,          // 27, 65
    AltShiftB,          // 27, 66
    AltShiftC,          // 27, 67
    AltShiftD,          // 27, 68
    AltShiftE,          // 27, 69
    AltShiftF,          // 27, 70
    AltShiftG,          // 27, 71
    AltShiftH,          // 27, 72
    AltShiftI,          // 27, 73
    AltShiftJ,          // 27, 74
    AltShiftK,          // 27, 75
    AltShiftL,          // 27, 76
    AltShiftM,          // 27, 77
    AltShiftN,          // 27, 78
    AltShiftO,          // 27, 79
    AltShiftP,          // 27, 80
    AltShiftQ,          // 27, 81
    AltShiftR,          // 27, 82
    AltShiftS,          // 27, 83
    AltShiftT,          // 27, 84
    AltShiftU,          // 27, 85
    AltShiftV,          // 27, 86
    AltShiftW,          // 27, 87
    AltShiftX,          // 27, 88
    AltShiftY,          // 27, 89
    AltShiftZ,          // 27, 90
    AltLeftBracket,     // 27, 91
    AltBackSlash,       // 27, 92
    AltRightBracket,    // 27, 93
    AltCaret,           // 27, 94
    AltUnderscore,      // 27, 95
    AltBackTick,        // 27, 96
    AltA,               // 27, 97
    AltB,               // 27, 98
    AltC,               // 27, 99
    AltD,               // 27, 100
    AltE,               // 27, 101
    AltF,               // 27, 102
    AltG,               // 27, 103
    AltH,               // 27, 104
    AltI,               // 27, 105
    AltJ,               // 27, 106
    AltK,               // 27, 107
    AltL,               // 27, 108
    AltM,               // 27, 109
    AltN,               // 27, 110
    AltO,               // 27, 111
    AltP,               // 27, 112
    AltQ,               // 27, 113
    AltR,               // 27, 114
    AltS,               // 27, 115
    AltT,               // 27, 116
    AltU,               // 27, 117
    AltV,               // 27, 118
    AltW,               // 27, 119
    AltX,               // 27, 120
    AltY,               // 27, 121
    AltZ,               // 27, 122
    AltLeftBrace,       // 27, 123
    AltPipe,            // 27, 124
    AltRightBrace,      // 27, 125
    AltTilde,           // 27, 126
    AltDelete,          // 27, 127

    ShiftLeft,        //[27, 91, 49, 59, 50, 68]
    AltLeft,          //[27, 91, 49, 59, 51, 68]
    AltShiftLeft,     //[27, 91, 49, 59, 52, 68]
    CtrlLeft,         //[27, 91, 49, 59, 53, 68]
    CtrlShiftLeft,    //[27, 91, 49, 59, 54, 68]
    AltCtrlLeft,      //[27, 91, 49, 59, 55, 68]
    AltCtrlShiftLeft, //[27, 91, 49, 59, 56, 68]

    ShiftUp,        //[27, 91, 49, 59, 50, 65]
    AltUp,          //[27, 91, 49, 59, 51, 65]
    AltShiftUp,     //[27, 91, 49, 59, 52, 65]
    CtrlUp,         //[27, 91, 49, 59, 53, 65]
    CtrlShiftUp,    //[27, 91, 49, 59, 54, 65]
    AltCtrlUp,      //[27, 91, 49, 59, 55, 65]
    AltCtrlShiftUp, //[27, 91, 49, 59, 56, 65]

    ShiftRight,        //[27, 91, 49, 59, 50, 67]
    AltRight,          //[27, 91, 49, 59, 51, 67]
    AltShiftRight,     //[27, 91, 49, 59, 52, 67]
    CtrlRight,         //[27, 91, 49, 59, 53, 67]
    CtrlShiftRight,    //[27, 91, 49, 59, 54, 67]
    AltCtrlRight,      //[27, 91, 49, 59, 55, 67]
    AltCtrlShiftRight, //[27, 91, 49, 59, 56, 67]

    ShiftDown,        //[27, 91, 49, 59, 50, 66]
    AltDown,          //[27, 91, 49, 59, 51, 66]
    AltShiftDown,     //[27, 91, 49, 59, 52, 66]
    CtrlDown,         //[27, 91, 49, 59, 53, 66]
    CtrlShiftDown,    //[27, 91, 49, 59, 54, 66]
    AltCtrlDown,      //[27, 91, 49, 59, 55, 66]
    AltCtrlShiftDown, //[27, 91, 49, 59, 56, 66]

    F1,              //      27,79,80
    ShiftF1,         //      27,91,49,59,50,80
    AltF1,           //      27,91,49,59,51,80
    AltShiftF1,      //      27,91,49,59,52,80
    CtrlF1,          //      27,91,49,59,53,80
    CtrlShiftF1,     //      27,91,49,59,54,80
    AltCtrlF1,       //      27,91,49,59,55,80
    AltCtrlShiftF1,  //      27,91,49,59,56,80
    F2,              //      27,79,81
    ShiftF2,         //      27,91,49,59,50,81
    AltF2,           //      27,91,49,59,51,81
    AltShiftF2,      //      27,91,49,59,52,81
    CtrlF2,          //      27,91,49,59,53,81
    CtrlShiftF2,     //      27,91,49,59,54,81
    AltCtrlF2,       //      27,91,49,59,55,81
    AltCtrlShiftF2,  //      27,91,49,59,56,81
    F3,              //      27,79,82
    ShiftF3,         //      27,91,49,59,50,82
    AltF3,           //      27,91,49,59,51,82
    AltShiftF3,      //      27,91,49,59,52,82
    CtrlF3,          //      27,91,49,59,53,82
    CtrlShiftF3,     //      27,91,49,59,54,82
    AltCtrlF3,       //      27,91,49,59,55,82
    AltCtrlShiftF3,  //      27,91,49,59,56,82
    F4,              //      27,79,83
    ShiftF4,         //      27,91,49,59,50,83
    AltF4,           //      27,91,49,59,51,83
    AltShiftF4,      //      27,91,49,59,52,83
    CtrlF4,          //      27,91,49,59,53,83
    CtrlShiftF4,     //      27,91,49,59,54,83
    AltCtrlF4,       //      27,91,49,59,55,83
    AltCtrlShiftF4,  //      27,91,49,59,56,83
    F5,              //      27,91,49,53,126
    ShiftF5,         //      27,91,49,53,59,50,126
    AltF5,           //      27,91,49,53,59,51,126
    AltShiftF5,      //      27,91,49,53,59,52,126
    CtrlF5,          //      27,91,49,53,59,53,126
    CtrlShiftF5,     //      27,91,49,53,59,54,126
    AltCtrlF5,       //      27,91,49,53,59,55,126
    AltCtrlShiftF5,  //      27,91,49,53,59,56,126
    F6,              //      27,91,49,55,126
    ShiftF6,         //      27,91,49,55,59,50,126
    AltF6,           //      27,91,49,55,59,51,126
    AltShiftF6,      //      27,91,49,55,59,52,126
    CtrlF6,          //      27,91,49,55,59,53,126
    CtrlShiftF6,     //      27,91,49,55,59,54,126
    AltCtrlF6,       //      27,91,49,55,59,55,126
    AltCtrlShiftF6,  //      27,91,49,55,59,56,126
    F7,              //      27,91,49,56,126
    ShiftF7,         //      27,91,49,56,59,50,126
    AltF7,           //      27,91,49,56,59,51,126
    AltShiftF7,      //      27,91,49,56,59,52,126
    CtrlF7,          //      27,91,49,56,59,53,126
    CtrlShiftF7,     //      27,91,49,56,59,54,126
    AltCtrlF7,       //      27,91,49,56,59,55,126
    AltCtrlShiftF7,  //      27,91,49,56,59,56,126
    F8,              //      27,91,49,57,126
    ShiftF8,         //      27,91,49,57,59,50,126
    AltF8,           //      27,91,49,57,59,51,126
    AltShiftF8,      //      27,91,49,57,59,52,126
    CtrlF8,          //      27,91,49,57,59,53,126
    CtrlShiftF8,     //      27,91,49,57,59,54,126
    AltCtrlF8,       //      27,91,49,57,59,55,126
    AltCtrlShiftF8,  //      27,91,49,57,59,56,126
    F9,              //      27,91,50,48,126
    ShiftF9,         //      27,91,49,48,59,50,126
    AltF9,           //      27,91,49,48,59,51,126
    AltShiftF9,      //      27,91,49,48,59,52,126
    CtrlF9,          //      27,91,49,48,59,53,126
    CtrlShiftF9,     //      27,91,49,48,59,54,126
    AltCtrlF9,       //      27,91,49,48,59,55,126
    AltCtrlShiftF9,  //      27,91,49,48,59,56,126
    F10,             //      27,91,50,49,126
    ShiftF10,        //      27,91,49,49,59,50,126
    AltF10,          //      27,91,49,49,59,51,126
    AltShiftF10,     //      27,91,49,49,59,52,126
    CtrlF10,         //      27,91,49,49,59,53,126
    CtrlShiftF10,    //      27,91,49,49,59,54,126
    AltCtrlF10,      //      27,91,49,49,59,55,126
    AltCtrlShiftF10, //      27,91,49,49,59,56,126
    F11,             //      27,91,50,51,126
    ShiftF11,        //      27,91,49,51,59,50,126
    AltF11,          //      27,91,49,51,59,51,126
    AltShiftF11,     //      27,91,49,51,59,52,126
    CtrlF11,         //      27,91,49,51,59,53,126
    CtrlShiftF11,    //      27,91,49,51,59,54,126
    AltCtrlF11,      //      27,91,49,51,59,55,126
    AltCtrlShiftF11, //      27,91,49,51,59,56,126
    F12,             //      27,91,50,52,126
    ShiftF12,        //      27,91,49,52,59,50,126
    AltF12,          //      27,91,49,52,59,51,126
    AltShiftF12,     //      27,91,49,52,59,52,126
    CtrlF12,         //      27,91,49,52,59,53,126
    CtrlShiftF12,    //      27,91,49,52,59,54,126
    AltCtrlF12,      //      27,91,49,52,59,55,126
    AltCtrlShiftF12, //      27,91,49,52,59,56,126

    Up,    // 27,91,65
    Down,  // 27,91,66
    Left,  // 27,91,68
    Right, // 27,91,67
    //Delete, // 27,91,51,126
    //Prt_Scrn,
    //Scroll_Lock,
    //Pause,
    //Break,
    Insert,   // 27,91,50,126
    Home,     // 27,91,72
    PgUp,     // 27,91,53,126
    PgDn,     // 27.91.54.126
    End,      // 27,91,70
    Menu,     // 27, 91, 50, 57, 126
    ShiftTab, //27,91,90

    AltUnicode(Vec<u8>),
    Unicode(Vec<u8>),
}
