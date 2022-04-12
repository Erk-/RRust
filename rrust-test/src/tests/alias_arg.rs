use rrust::rfn;

rfn!(AliasArg, (x1: &mut i32, x2: &mut i32), {
    *x1 += 1;
    *x2 += 1;
});

fn main() {
    let mut x = 0;

    AliasArg::forward(&mut x, &mut x);
    AliasArg::backwards(&mut x, &mut x);
}
