#[cfg(test)]
use rrust::{delocal, rfn, rif, rloop};

#[test]
fn test_addone() {
    rfn!(AddOne, (a: &mut i32, b: &mut i32), {
        *a += 1;
        *b += 1;
    });

    let mut a = 1;
    let mut b = 2;

    AddOne::forward(&mut a, &mut b);

    assert_eq!(a, 2);
    assert_eq!(b, 3);

    AddOne::backwards(&mut a, &mut b);

    assert_eq!(a, 1);
    assert_eq!(b, 2);
}

#[test]
fn test_block() {
    rfn!(AddOne, (a: &mut i32, b: &mut i32), {
        {
            *a += 1;
            *b += 1;
        }
    });

    let mut a = 1;
    let mut b = 2;

    AddOne::forward(&mut a, &mut b);

    assert_eq!(a, 2);
    assert_eq!(b, 3);

    AddOne::backwards(&mut a, &mut b);

    assert_eq!(a, 1);
    assert_eq!(b, 2);
}

#[test]
fn test_block_block() {
    rfn!(AddOne, (a: &mut i32, b: &mut i32), {
        {
            {
                *a += 1;
                *b += 1;
            }
        }
    });

    let mut a = 1;
    let mut b = 2;

    AddOne::forward(&mut a, &mut b);

    assert_eq!(a, 2);
    assert_eq!(b, 3);

    AddOne::backwards(&mut a, &mut b);

    assert_eq!(a, 1);
    assert_eq!(b, 2);
}

#[test]
fn test_blocks() {
    rfn!(AddOther, (a: &mut i32, b: &mut i32), {
        {
            {
                *a += 1;
            }
            {
                *b += 1;
            }
        }
    });

    let mut a = 1;
    let mut b = 2;

    AddOther::forward(&mut a, &mut b);

    assert_eq!(a, 2);
    assert_eq!(b, 3);

    AddOther::backwards(&mut a, &mut b);

    assert_eq!(a, 1);
    assert_eq!(b, 2);
}

#[test]
fn test_fib() {
    rfn!(Fib, (x1: &mut i32, x2: &mut i32, n: &mut i32), {
        rif!(
            *n == 0,
            {
                *x1 += 1;
                *x2 += 1;
            },
            {
                *n -= 1;
                Fib::forward(x1, x2, n);
                *x1 += *x2;
                std::mem::swap(x1, x2);
            },
            *x1 == *x2
        );
    });

    let mut x1 = 0;
    let mut x2 = 0;
    let mut n = 10;

    Fib::forward(&mut x1, &mut x2, &mut n);

    assert_eq!(x1, 89);
    assert_eq!(x2, 144);
    assert_eq!(n, 0);

    Fib::backwards(&mut x1, &mut x2, &mut n);

    assert_eq!(x1, 0);
    assert_eq!(x2, 0);
    assert_eq!(n, 10);
}

#[test]
fn test_scary_correct() {
    rfn!(Scary, (arr: &mut [i32], payload: &mut [i32]), {
        let mut i = 0;
        rloop!(
            i == 0,
            {
                arr[i] += payload[i];
                i += 1;
            },
            i == 2048
        );
        delocal!(i, 2048);
    });

    let mut arr = [0; 2048];
    let mut payload = [42_i32; 2048];

    Scary::forward(&mut arr[..], &mut payload[..]);

    assert_eq!(arr, payload);

    Scary::backwards(&mut arr[..], &mut payload[..]);

    assert_eq!(arr, [0; 2048]);
}

#[test]
#[should_panic]
fn test_scary_incorrect() {
    rfn!(Scary, (arr: &mut [i32], payload: &mut [i32]), {
        let mut i = 0;
        rloop!(
            i == 0,
            {
                arr[i] += payload[i];
                i += 1;
            },
            i == 2048
        );
        delocal!(i, 2048);
    });

    let mut arr = [0; 1024];
    let mut payload = [42_i32; 2048];

    Scary::forward(&mut arr[..], &mut payload[..]);
    Scary::backwards(&mut arr[..], &mut payload[..]);
}

#[test]
fn test_delocal_block() {
    rfn!(Alias, (arr: &mut [i32]), {
        {
            let i = 42;
            delocal!(i, 42);
        }
    });
}

#[test]
#[should_panic]
fn test_alias_arr() {
    rfn!(Alias, (arr: &mut [i32]), {
        let i = 42;
        arr[42] -= arr[i];
        delocal!(i, 42);
    });

    let mut arr = [10; 100];

    Alias::forward(&mut arr[..]);
    Alias::backwards(&mut arr[..]);
}

#[test]
#[should_panic]
fn test_alias_var() {
    rfn!(Alias, (x: &mut i32), {
        *x -= *x;
    });

    let mut var = 5;

    Alias::forward(&mut var);
    Alias::backwards(&mut var);
}

#[test]
fn test_alias_arg() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/tests/alias_arg.rs");
}

#[test]
fn test_no_delocal() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/tests/no_delocal.rs");
}

#[test]
fn test_factor() {
    rfn!(Factor, (num: &mut usize, fact: &mut [usize; 20]), {
        let mut tryf = 0;
        let mut i = 0;
        rloop!(
            tryf == 0 && *num > 1,
            {
                NextTry::forward(&mut tryf);
                rloop!(
                    fact[i] != tryf,
                    {
                        i += 1;
                        fact[i] += tryf;
                        let mut z = *num / tryf;
                        std::mem::swap(&mut z, num);
                        delocal!(z, *num * tryf);
                    },
                    *num % tryf != 0
                );
            },
            tryf * tryf > *num
        );

        rif!(
            *num != 1,
            {
                i += 1;
                fact[i] ^= *num;
                *num ^= fact[i];
                fact[i] ^= *num;
            },
            {
                *num -= 1;
            },
            fact[i] != fact[i - 1]
        );

        rif!(
            (fact[i - 1] * fact[i - 1]) < fact[i],
            {
                rloop!(
                    tryf * tryf > fact[i],
                    {
                        NextTry::backwards(&mut tryf);
                    },
                    tryf == 0
                );
            },
            {
                tryf -= fact[i - 1];
            },
            (fact[i - 1] * fact[i - 1]) < fact[i]
        );

        ZeroI::forward(&mut i, fact);
        delocal!(i, 0);
        delocal!(tryf, 0);
    });

    rfn!(ZeroI, (i: &mut usize, fact: &mut [usize; 20]), {
        rloop!(
            fact[*i + 1] == 0,
            {
                *i -= 1;
            },
            *i == 0
        );
    });

    rfn!(NextTry, (tryf: &mut usize), {
        *tryf += 2;
        rif!(
            *tryf == 4,
            {
                *tryf -= 1;
            },
            *tryf == 3
        );
    });

    let mut num = 840;
    let mut fact = [0; 20];

    Factor::forward(&mut num, &mut fact);
    print!("Num: {}, Factors: ", num);
    for i in 1u64..=6 {
        print!("{}: {}", i, fact[i as usize]);
        if i != 6 {
            print!(", ");
        } else {
            println!(".");
        }
    }
    Factor::backwards(&mut num, &mut fact);
    print!("Num: {}, Factors: ", num);
    for i in 1u64..=6 {
        print!("{}: {}", i, fact[i as usize]);
        if i != 6 {
            print!(", ");
        } else {
            println!(".");
        }
    }
}
