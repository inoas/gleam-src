use crate::assert_js;

#[test]
fn list_literals() {
    assert_js!(
        r#"
fn go(x) {
    []
    [1]
    [1, 2]
    [1, 2, ..x]
}
"#,
        r#""use strict";

function go(x) {
  [];
  [1, []];
  [1, [2, []]];
  return [1, [2, x]];
}
"#
    );
}

#[test]
fn long_list_literals() {
    assert_js!(
        r#"
fn go() {
    [111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111]
    [11111111111111111111111111111111111111111111, 11111111111111111111111111111111111111111111111111111111111111111111111111111111]
}
"#,
        r#""use strict";

function go() {
  [111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111,
    []];
  return [11111111111111111111111111111111111111111111,
    [11111111111111111111111111111111111111111111111111111111111111111111111111111111,
      []]];
}
"#
    );
}

#[test]
fn multi_line_list_literals() {
    assert_js!(
        r#"
fn go(x) {
    [{True; 1}]
}
"#,
        r#""use strict";

function go(x) {
  return [(() => {
      true;
      return 1;
    })(),
    []];
}
"#
    );
}

#[test]
fn list_constants() {
    assert_js!(
        r#"
const a = []
const b = [1, 2, 3]
"#,
        r#""use strict";

const a = [];

const b = [1, [2, [3, []]]];
"#
    );
}
