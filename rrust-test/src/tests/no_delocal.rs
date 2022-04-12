use rrust::rfn;

rfn!(NoDelocal, (x1: &mut i32), {
    let i = 1
    *x1 += i;
});

fn main() {
    let mut x = 0;

    NoDelocal::forward(&mut x);
    NoDelocal::backwards(&mut x);
}
