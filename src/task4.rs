#[test]

/*
/* Adding HRTB to make it work!*/
fn call_on_ref_zero<'a, F>(f: F) where F: Fn(&'a i32) {
    let zero = 0;
    f(&zero);
}

fn main() {
    println!("Success!");
}
*/
fn main() {
    // Приклад виклику `call_on_ref_zero` з замиканням, що приймає &i32
    call_on_ref_zero(|num| {
        println!("The number is: {}", num);
    });

    println!("Success!");
}
// Використовуємо HRTB з `for<'a>` для універсальності тривалостей
fn call_on_ref_zero<F>(f: F)
where
    F: for<'a> Fn(&'a i32)
{
    let zero = 0;
    f(&zero);
}




/*
HRTB (Higher-ranked trait bounds):
Оголошення параметру F, який представляє функцію. Замість того, щоб використовувати конкретну тривалість 'a,
for<'a> дозволяє функції f приймати посилання з довільною тривалістю.
Таким чином, F: for<'a> Fn(&'a i32) визначає, що f може приймати будь-яке посилання на i32, незалежно від його тривалості.

Використання HRTB:
В коді call_on_ref_zero(|num| { println!("The number is: {}", num); }) ви передаєте замикання,
що приймає посилання на i32 з будь-якою тривалістю. Оскільки HRTB дозволяє тривалості бути довільними,
функція call_on_ref_zero приймає посилання на zero, яке існує лише в рамках виклику цієї функції.
*/