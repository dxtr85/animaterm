#[derive(Debug, PartialEq)]
pub enum Key {
    Ctrl_a,
    Ctrl_b,
    Ctrl_c,
    Ctrl_d,
    Ctrl_e,
    Ctrl_f,
    Ctrl_g,
    Backspace,
    Tab,
    Enter, // Ctrl_j
    Ctrl_k,
    Ctrl_l,
    Ctrl_m,
    Ctrl_n,
    Ctrl_o,
    Ctrl_p,
    Ctrl_q,
    Ctrl_r,
    Ctrl_s,
    Ctrl_t,
    Ctrl_u,
    Ctrl_v,
    Ctrl_w,
    Ctrl_x,
    Ctrl_y,
    Ctrl_z,
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
    LeftBracket,  //       91
    BackSlash,    //       92
    RightBracket, //       93
    Caret,        //       94
    Underscore,   //       95
    BackTick,     //       96
    a,
    b,
    c,
    d,
    e,
    f,
    g,
    h,
    i,
    j,
    k,
    l,
    m,
    n,
    o,
    p,
    q,
    r,
    s,
    t,
    u,
    v,
    w,
    x,
    y,
    z,
    LeftBrace,  //      123
    Pipe,       //      124
    RightBrace, //      125
    Tilde,      //      126
    Delete,     //      127

    Alt_Ctrl_a,          // 27, 1
    Alt_Ctrl_b,          // 27, 2
    Alt_Ctrl_c,          // 27, 3
    Alt_Ctrl_d,          // 27, 4
    Alt_Ctrl_e,          // 27, 5
    Alt_Ctrl_f,          // 27, 6
    Alt_Ctrl_g,          // 27, 7
    Alt_Ctrl_h,          // 27, 8
    Alt_Tab,             // 27, 9
    Alt_Enter,           // 27, 10
    Alt_Ctrl_k,          // 27, 11
    Alt_Ctrl_l,          // 27, 12
    Alt_Ctrl_m,          // 27, 13
    Alt_Ctrl_n,          // 27, 14
    Alt_Ctrl_o,          // 27, 15
    Alt_Ctrl_p,          // 27, 16
    Alt_Ctrl_q,          // 27, 17
    Alt_Ctrl_r,          // 27, 18
    Alt_Ctrl_s,          // 27, 19
    Alt_Ctrl_t,          // 27, 20
    Alt_Ctrl_u,          // 27, 21
    Alt_Ctrl_v,          // 27, 22
    Alt_Ctrl_w,          // 27, 23
    Alt_Ctrl_x,          // 27, 24
    Alt_Ctrl_y,          // 27, 25
    Alt_Ctrl_z,          // 27, 26
    Alt_Escape,          // 27, 27
    Alt_FileSeparator,   // 27, 28
    Alt_GroupSeparator,  // 27, 29
    Alt_RecordSeparator, // 27, 30
    Alt_UnitSeparator,   // 27, 31
    Alt_Space,           // 27, 32
    Alt_ExclamationMark, // 27, 33
    Alt_Quote,           // 27, 34
    Alt_Hash,            // 27, 35
    Alt_Dollar,          // 27, 36
    Alt_Percent,         // 27, 37
    Alt_Ampersand,       // 27, 38
    Alt_Apostrophe,      // 27, 39
    Alt_LeftParen,       // 27, 40
    Alt_RightParen,      // 27, 41
    Alt_Star,            // 27, 42
    Alt_Plus,            // 27, 43
    Alt_Comma,           // 27, 44
    Alt_Dash,            // 27, 45
    Alt_Period,          // 27, 46
    Alt_Slash,           // 27, 47
    Alt_Zero,            // 27, 48
    Alt_One,             // 27, 49
    Alt_Two,             // 27, 50
    Alt_Three,           // 27, 51
    Alt_Four,            // 27, 52
    Alt_Five,            // 27, 53
    Alt_Six,             // 27, 54
    Alt_Seven,           // 27, 55
    Alt_Eight,           // 27, 56
    Alt_Nine,            // 27, 57
    Alt_Colon,           // 27, 58
    Alt_Semicolon,       // 27, 59
    Alt_LessThan,        // 27, 60
    Alt_Equals,          // 27, 61
    Alt_GreaterThan,     // 27, 62
    Alt_QuestionMark,    // 27, 63
    Alt_At,              // 27, 64
    Alt_A,               // 27, 65
    Alt_B,               // 27, 66
    Alt_C,               // 27, 67
    Alt_D,               // 27, 68
    Alt_E,               // 27, 69
    Alt_F,               // 27, 70
    Alt_G,               // 27, 71
    Alt_H,               // 27, 72
    Alt_I,               // 27, 73
    Alt_J,               // 27, 74
    Alt_K,               // 27, 75
    Alt_L,               // 27, 76
    Alt_M,               // 27, 77
    Alt_N,               // 27, 78
    Alt_O,               // 27, 79
    Alt_P,               // 27, 80
    Alt_Q,               // 27, 81
    Alt_R,               // 27, 82
    Alt_S,               // 27, 83
    Alt_T,               // 27, 84
    Alt_U,               // 27, 85
    Alt_V,               // 27, 86
    Alt_W,               // 27, 87
    Alt_X,               // 27, 88
    Alt_Y,               // 27, 89
    Alt_Z,               // 27, 90
    Alt_LeftBracket,     // 27, 91
    Alt_BackSlash,       // 27, 92
    Alt_RightBracket,    // 27, 93
    Alt_Caret,           // 27, 94
    Alt_Underscore,      // 27, 95
    Alt_BackTick,        // 27, 96
    Alt_a,               // 27, 97
    Alt_b,               // 27, 98
    Alt_c,               // 27, 99
    Alt_d,               // 27, 100
    Alt_e,               // 27, 101
    Alt_f,               // 27, 102
    Alt_g,               // 27, 103
    Alt_h,               // 27, 104
    Alt_i,               // 27, 105
    Alt_j,               // 27, 106
    Alt_k,               // 27, 107
    Alt_l,               // 27, 108
    Alt_m,               // 27, 109
    Alt_n,               // 27, 110
    Alt_o,               // 27, 111
    Alt_p,               // 27, 112
    Alt_q,               // 27, 113
    Alt_r,               // 27, 114
    Alt_s,               // 27, 115
    Alt_t,               // 27, 116
    Alt_u,               // 27, 117
    Alt_v,               // 27, 118
    Alt_w,               // 27, 119
    Alt_x,               // 27, 120
    Alt_y,               // 27, 121
    Alt_z,               // 27, 122
    Alt_LeftBrace,       // 27, 123
    Alt_Pipe,            // 27, 124
    Alt_RightBrace,      // 27, 125
    Alt_Tilde,           // 27, 126
    Alt_Delete,          // 27, 127

    Shift_Left,          //[27, 91, 49, 59, 50, 68]
    Alt_Left,            //[27, 91, 49, 59, 51, 68]
    Alt_Shift_Left,      //[27, 91, 49, 59, 52, 68]
    Ctrl_Left,           //[27, 91, 49, 59, 53, 68]
    Ctrl_Shift_Left,     //[27, 91, 49, 59, 54, 68]
    Alt_Ctrl_Left,       //[27, 91, 49, 59, 55, 68]
    Alt_Ctrl_Shift_Left, //[27, 91, 49, 59, 56, 68]

    Shift_Up,          //[27, 91, 49, 59, 50, 65]
    Alt_Up,            //[27, 91, 49, 59, 51, 65]
    Alt_Shift_Up,      //[27, 91, 49, 59, 52, 65]
    Ctrl_Up,           //[27, 91, 49, 59, 53, 65]
    Ctrl_Shift_Up,     //[27, 91, 49, 59, 54, 65]
    Alt_Ctrl_Up,       //[27, 91, 49, 59, 55, 65]
    Alt_Ctrl_Shift_Up, //[27, 91, 49, 59, 56, 65]

    Shift_Right,          //[27, 91, 49, 59, 50, 67]
    Alt_Right,            //[27, 91, 49, 59, 51, 67]
    Alt_Shift_Right,      //[27, 91, 49, 59, 52, 67]
    Ctrl_Right,           //[27, 91, 49, 59, 53, 67]
    Ctrl_Shift_Right,     //[27, 91, 49, 59, 54, 67]
    Alt_Ctrl_Right,       //[27, 91, 49, 59, 55, 67]
    Alt_Ctrl_Shift_Right, //[27, 91, 49, 59, 56, 67]

    Shift_Down,          //[27, 91, 49, 59, 50, 66]
    Alt_Down,            //[27, 91, 49, 59, 51, 66]
    Alt_Shift_Down,      //[27, 91, 49, 59, 52, 66]
    Ctrl_Down,           //[27, 91, 49, 59, 53, 66]
    Ctrl_Shift_Down,     //[27, 91, 49, 59, 54, 66]
    Alt_Ctrl_Down,       //[27, 91, 49, 59, 55, 66]
    Alt_Ctrl_Shift_Down, //[27, 91, 49, 59, 56, 66]

    F1,                 //      27,79,80
    Shift_F1,           //      27,91,49,59,50,80
    Alt_F1,             //      27,91,49,59,51,80
    Alt_Shift_F1,       //      27,91,49,59,52,80
    Ctrl_F1,            //      27,91,49,59,53,80
    Ctrl_Shift_F1,      //      27,91,49,59,54,80
    Alt_Ctrl_F1,        //      27,91,49,59,55,80
    Alt_Ctrl_Shift_F1,  //      27,91,49,59,56,80
    F2,                 //      27,79,81
    Shift_F2,           //      27,91,49,59,50,81
    Alt_F2,             //      27,91,49,59,51,81
    Alt_Shift_F2,       //      27,91,49,59,52,81
    Ctrl_F2,            //      27,91,49,59,53,81
    Ctrl_Shift_F2,      //      27,91,49,59,54,81
    Alt_Ctrl_F2,        //      27,91,49,59,55,81
    Alt_Ctrl_Shift_F2,  //      27,91,49,59,56,81
    F3,                 //      27,79,82
    Shift_F3,           //      27,91,49,59,50,82
    Alt_F3,             //      27,91,49,59,51,82
    Alt_Shift_F3,       //      27,91,49,59,52,82
    Ctrl_F3,            //      27,91,49,59,53,82
    Ctrl_Shift_F3,      //      27,91,49,59,54,82
    Alt_Ctrl_F3,        //      27,91,49,59,55,82
    Alt_Ctrl_Shift_F3,  //      27,91,49,59,56,82
    F4,                 //      27,79,83
    Shift_F4,           //      27,91,49,59,50,83
    Alt_F4,             //      27,91,49,59,51,83
    Alt_Shift_F4,       //      27,91,49,59,52,83
    Ctrl_F4,            //      27,91,49,59,53,83
    Ctrl_Shift_F4,      //      27,91,49,59,54,83
    Alt_Ctrl_F4,        //      27,91,49,59,55,83
    Alt_Ctrl_Shift_F4,  //      27,91,49,59,56,83
    F5,                 //      27,91,49,53,126
    Shift_F5,           //      27,91,49,53,59,50,126
    Alt_F5,             //      27,91,49,53,59,51,126
    Alt_Shift_F5,       //      27,91,49,53,59,52,126
    Ctrl_F5,            //      27,91,49,53,59,53,126
    Ctrl_Shift_F5,      //      27,91,49,53,59,54,126
    Alt_Ctrl_F5,        //      27,91,49,53,59,55,126
    Alt_Ctrl_Shift_F5,  //      27,91,49,53,59,56,126
    F6,                 //      27,91,49,55,126
    Shift_F6,           //      27,91,49,55,59,50,126
    Alt_F6,             //      27,91,49,55,59,51,126
    Alt_Shift_F6,       //      27,91,49,55,59,52,126
    Ctrl_F6,            //      27,91,49,55,59,53,126
    Ctrl_Shift_F6,      //      27,91,49,55,59,54,126
    Alt_Ctrl_F6,        //      27,91,49,55,59,55,126
    Alt_Ctrl_Shift_F6,  //      27,91,49,55,59,56,126
    F7,                 //      27,91,49,56,126
    Shift_F7,           //      27,91,49,56,59,50,126
    Alt_F7,             //      27,91,49,56,59,51,126
    Alt_Shift_F7,       //      27,91,49,56,59,52,126
    Ctrl_F7,            //      27,91,49,56,59,53,126
    Ctrl_Shift_F7,      //      27,91,49,56,59,54,126
    Alt_Ctrl_F7,        //      27,91,49,56,59,55,126
    Alt_Ctrl_Shift_F7,  //      27,91,49,56,59,56,126
    F8,                 //      27,91,49,57,126
    Shift_F8,           //      27,91,49,57,59,50,126
    Alt_F8,             //      27,91,49,57,59,51,126
    Alt_Shift_F8,       //      27,91,49,57,59,52,126
    Ctrl_F8,            //      27,91,49,57,59,53,126
    Ctrl_Shift_F8,      //      27,91,49,57,59,54,126
    Alt_Ctrl_F8,        //      27,91,49,57,59,55,126
    Alt_Ctrl_Shift_F8,  //      27,91,49,57,59,56,126
    F9,                 //      27,91,50,48,126
    Shift_F9,           //      27,91,49,48,59,50,126
    Alt_F9,             //      27,91,49,48,59,51,126
    Alt_Shift_F9,       //      27,91,49,48,59,52,126
    Ctrl_F9,            //      27,91,49,48,59,53,126
    Ctrl_Shift_F9,      //      27,91,49,48,59,54,126
    Alt_Ctrl_F9,        //      27,91,49,48,59,55,126
    Alt_Ctrl_Shift_F9,  //      27,91,49,48,59,56,126
    F10,                //      27,91,50,49,126
    Shift_F10,          //      27,91,49,49,59,50,126
    Alt_F10,            //      27,91,49,49,59,51,126
    Alt_Shift_F10,      //      27,91,49,49,59,52,126
    Ctrl_F10,           //      27,91,49,49,59,53,126
    Ctrl_Shift_F10,     //      27,91,49,49,59,54,126
    Alt_Ctrl_F10,       //      27,91,49,49,59,55,126
    Alt_Ctrl_Shift_F10, //      27,91,49,49,59,56,126
    F11,                //      27,91,50,51,126
    Shift_F11,          //      27,91,49,51,59,50,126
    Alt_F11,            //      27,91,49,51,59,51,126
    Alt_Shift_F11,      //      27,91,49,51,59,52,126
    Ctrl_F11,           //      27,91,49,51,59,53,126
    Ctrl_Shift_F11,     //      27,91,49,51,59,54,126
    Alt_Ctrl_F11,       //      27,91,49,51,59,55,126
    Alt_Ctrl_Shift_F11, //      27,91,49,51,59,56,126
    F12,                //      27,91,50,52,126
    Shift_F12,          //      27,91,49,52,59,50,126
    Alt_F12,            //      27,91,49,52,59,51,126
    Alt_Shift_F12,      //      27,91,49,52,59,52,126
    Ctrl_F12,           //      27,91,49,52,59,53,126
    Ctrl_Shift_F12,     //      27,91,49,52,59,54,126
    Alt_Ctrl_F12,       //      27,91,49,52,59,55,126
    Alt_Ctrl_Shift_F12, //      27,91,49,52,59,56,126

    Up,    // 27,91,65
    Down,  // 27,91,66
    Left,  // 27,91,68
    Right, // 27,91,67
    //Delete, // 27,91,51,126
    //Prt_Scrn,
    //Scroll_Lock,
    //Pause,
    //Break,
    Insert,    // 27,91,50,126
    Home,      // 27,91,72
    PgUp,      // 27,91,53,126
    PgDn,      // 27.91.54.126
    End,       // 27,91,70
    Menu,      // 27, 91, 50, 57, 126
    Shift_Tab, //27,91,90

    Alt_Unicode(Vec<u8>),
    Unicode(Vec<u8>),
}
