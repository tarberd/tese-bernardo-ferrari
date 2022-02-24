fn parse_Expr(p: Parser) -> Parse {
  parse_Expr_binding_power(p, 0)
}
fn parse_Expr_binding_power(p: Parser, min_bp: Int) -> Parse {
  let left = if p.at("id") {
    let token = p.eat_token();
    Id(token)
  } else if p.at("number") {
    let token = p.eat_token();
    Number(token)
  } else if p.at("(") {
    p.eat_token();
    let expr = p.parse_Expr();
    p.expect(")");
    expr
  } else {
    panic!("Parse error.");
  };
  loop {
    let op = if p.at("+") { Sum }
      else if p.at("-") { Minus }
      else if p.at("*") { Times }
      else if p.at("/") { Div }
      else { break; }
    let (left_bp, right_bp) = binding_power(op);
    if left_bp < min_bp {
      break;
    }
    p.eat_token();
    let right = parse_Expr_binding_power(p, right_bp);
    left = Expr(left, op, right);
  }
  left
}
fn parse_BinOp(p: Parser) -> BinOp {
}
fn binding_power(op: BinOp) -> (Int, Int) {
  match op {
    Sum | Minus => (1, 2),
    Times | Div => (3, 4),
  }
}
