extern mod language;

use language :: *;

fn main () {
  let exp = make_exps ();
  test_atomicity (exp);
  test_let_bindings (exp);
}

fn make_exps () -> ~ [CoreExpr] {
  ~ [EVar (~ "x"), ENum (7), 
     ELet (false, ~ [(~ "y", @ ENum (13)), (~ "z", @ ENum (33))],
           @ EVar (~ "y"))]
}

fn test_atomicity (exp : & [CoreExpr]) {
  assert is_atomic_expr (& exp [0]);
  assert is_atomic_expr (& exp [1]);
  assert ! is_atomic_expr (& exp [2]);
  io :: println ("is_atomic_expr: passed");
}

fn test_let_bindings (exp : & [CoreExpr]) {
  match exp [2] {
    ELet (_, bindings, _) => {
      test_bound_names (copy bindings);
      test_bound_vals (copy bindings);
    },
    _ => fail
  };
}  

fn test_bound_names (bindings : ~ [CoreEquate]) {
  let names : ~ [~ str] = binders_of (bindings);
  assert names . len () == 2;
  assert names . all (|n| {* n == ~ "y" || * n == ~ "z"});
  io :: println ("binders_of: passed");
}

fn test_bound_vals (bindings : ~ [CoreEquate]) {
  let vals = rhss_of (bindings);
  assert vals . len () == 2;
  let nums = 
    vals . map (|v| match * * v {ENum (n) => n, _ => fail});
  assert nums [0] + nums [1] == 13 + 33;
  io :: println ("rhss_of: passed");
}
