macro_rules! query {
  (from $v:ident; in $c:ident; $(where $mv:expr;)* select $ms:expr;) =>
  { $c.filter(|$v| (true $(&& $mv)*) ).map(|$v| $ms) };
}

#[test]
fn query() {
  let x = 1..100;
  let y: Vec<i32> = x.clone().filter(|p| p % 2 == 0).map(|p| p * 2).collect();
  let e: Vec<i32> = query!(from p; in x; where p % 2 == 0; select p * 2;).collect();
  assert_eq!(e, y);
}
