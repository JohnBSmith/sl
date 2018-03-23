
// #![allow(non_upper_case_globals)]

static C0: &str = "\
.
.
.xxx
x...x
x...x
x...x
x...x
x...x
.xxx";

static C1: &str = "\
.
.
..x
.xx
x.x
..x
..x
..x
xxxxx";

static C2: &str = "\
.
.
.xxx
x...x
....x
...x
..x
.x
xxxxx";

static C3: &str = "\
.
.
.xxx
....x
....x
.xxx
....x
....x
.xxx";

static C4: &str = "\
.
.
x..x
x..x
x..x
xxxxx
...x
...x
...x";

static C5: &str = "\
.
.
xxxxx
x
x
xxxx
....x
....x
xxxx";

static C6: &str = "\
.
.
.xxx
x
x
xxxx
x...x
x...x
.xxx";

static C7: &str = "\
.
.
xxxxx
....x
...x
..x
..x
..x
..x";

static C8: &str = "\
.
.
.xxx
x...x
x...x
.xxx
x...x
x...x
.xxx";

static C9: &str = "\
.
.
.xxx
x...x
x...x
.xxxx
....x
....x
.xxx";

static COMMA: &str = "\
.
.
.
.
.
.
.
.x
.x
x";

static SEMICOLON: &str = "\
.
.
.
.
.x
.x
.
.x
.x
x";

static DOT: &str = "\
.
.
.
.
.
.
.
.xx
.xx";

static COLON: &str = "\
.
.
.
.
.xx
.xx
.
.xx
.xx";

static PLUS: &str = "\
.
.
.
..x
..x
xxxxx
..x
..x
.";

static MINUS: &str = "\
.
.
.
.
.
xxxxx
.
.
.";

static CA: &str = "\
.
.
.xxx.
x...x
x...x
xxxxx
x...x
x...x
x...x";

static CB: &str = "\
.
.
xxxx
x...x
x...x
xxxx
x...x
x...x
xxxx";

static CC: &str = "\
.
.
.xxxx
x
x
x
x
x
.xxxx";

static CD: &str = "\
.
.
xxxx
x...x
x...x
x...x
x...x
x...x
xxxx";

static CF: &str = "\
.
.
xxxxx
x
x
xxxxx
x
x
xxxxx";

static CE: &str = "\
.
.
xxxxx
x
x
xxxxx
x
x
x";

pub fn pixelmap(c: char) -> &'static str {
  return match c {
    '0' => C0,
    '1' => C1,
    '2' => C2,
    '3' => C3,
    '4' => C4,
    '5' => C5,
    '6' => C6,
    '7' => C7,
    '8' => C8,
    '9' => C9,

    ',' => COMMA,
    ';' => SEMICOLON,
    '.' => DOT,
    ':' => COLON,
    '+' => PLUS,
    '-' => MINUS,

    'A' => CA,
    'B' => CB,
    'C' => CC,
    'D' => CD,
    'E' => CE,
    'F' => CF,
    _ => ""
  };
}
