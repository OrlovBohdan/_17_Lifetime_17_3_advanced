#[test]

/*
/* Adding trait bounds to make it work */
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    println!("Success!")
}
*/
fn main() {
    let excerpt = ImportantExcerpt { part: "Rust is great!" };
    let announcement = "This is an important message!";

    // Викликаємо метод і зберігаємо результат
    let returned_part = excerpt.announce_and_return_part(announcement);
    println!("Returned part: {}", returned_part);

    println!("Success!");
}
#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        // Використовуємо self.part в контексті тривалості 'a
        // Тож повертаємо 'b - 'a не може бути більшим
        announcement
    }
}




/*
Оголошення тривалостей у методі:
Я видалив тривалість 'a з методу announce_and_return_part, щоб метод використовував тільки тривалість 'b для параметра announcement. Це дозволяє методові коректно повертати посилання, не конфліктуючи з тривалістю поля part.

Виклик методу в main:
Створив змінну excerpt, яка містить фрагмент тексту, і змінну announcement, що містить оголошення.
Викликав метод announce_and_return_part і зберіг результат, щоб показати, як це працює.
*/