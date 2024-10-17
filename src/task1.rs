#[test]

/*
/* Annotate struct with lifetime:
1. `r` and `s` must have different lifetimes
2. lifetime of `s` is bigger than that of 'r'
*/
struct DoubleRef<T> {
    r: &T,
    s: &T
}
fn main() {
    println!("Success!")
}
*/

fn main() {
    let value = 42;

    // Створюємо ссилки на `value`
    let r: &i32 = &value; // 'r
    let s: &i32 = &value; // 's, це може бути тією ж самою змінною

    let double_ref = DoubleRef { r, s };

    println!("Success! Values: r = {}, s = {}", double_ref.r, double_ref.s);
}

// Оголошуємо структуру з параметрами тривалості
struct DoubleRef<'r, 's, T> {
    r: &'r T, // Поле з тривалістю 'r
    s: &'s T, // Поле з тривалістю 's
}



/*
Структура DoubleRef:

Структура має два поля: r і s, кожне з яких є посиланням на тип T.
Параметри тривалості 'r і 's дозволяють кожному полю мати різні тривалості.
Функція main:

Створюємо змінну value, що містить значення 42.
Оголошуємо два посилання: r та s, які вказують на value.
Створюємо екземпляр структури DoubleRef, передаючи посилання r та s.
Виводимо значення обох полів у консоль.
*/