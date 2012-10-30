#[link (name = "language", vers = "0.0")];

type Program <T> = ~ [ScDefn <T>];

type ScDefn <T> = (Name, ~ [T], @ Expr <T>); // name, formals, body

type Name = ~ str;

enum Expr <T> {
  EVar (Name),
  ENum (int),
  EConstr (int, int),                      // tag, arity
  EAp (@ Expr <T>, @ Expr <T>),            // function, actuals
  ELet (bool, ~ [Equate <T>], @ Expr <T>), // letrec?, equations, body
  ECase (@ Expr <T>, ~ [Alter <T>]),       // expression, alternatives
  ELam (int, ~ [T], @ Expr <T>)            // tag, formals, body
}

type Equate <T> = (T, @ Expr <T>); // name, rhs

type Alter <T> = (int, ~ [T], @ Expr <T>); // tag, vars, body

pub pure fn is_atomic_expr <T> (expr : & Expr <T>) -> bool {
  match * expr {
    EVar (_) | ENum (_) => true,
    _ => false
  }
}

pub pure fn binders_of <T> (defns: ~ [Equate <T>]) -> ~ [T] {
  let (res, _) = vec :: unzip (defns);
  res
}

pub pure fn rhss_of <T> (defns: ~ [Equate <T>]) -> ~ [@ Expr <T>] {
  let (_, res) = vec :: unzip (defns);
  res
}

pub type CoreProgram = Program <Name>;
pub type CoreScDefn = ScDefn <Name>;
pub type CoreExpr = Expr <Name>;
pub type CoreEquate = Equate <Name>;
pub type CoreAlt = Alter <Name>;
