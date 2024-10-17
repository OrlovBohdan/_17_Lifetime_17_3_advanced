#[test]

/*
/* Make it work */
struct Interface<'a> {
    manager: &'a mut Manager<'a>
}

impl<'a> Interface<'a> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface(&'a mut self) -> Interface {
        Interface {
            manager: &mut self.manager
        }
    }
}

fn main() {
    let mut list = List {
        manager: Manager {
            text: "hello"
        }
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}
*/
fn main() {
    let mut list = List {
        manager: Manager {
            text: "привіт".to_string(),
        },
    };

    list.get_interface().noop();

    println!("Інтерфейс має бути видалений тут і запозичення має бути звільнене");

    use_list(&list);
}
#[allow(dead_code)]
struct Interface<'a> {
    manager: &'a mut Manager,
}

impl<'a> Interface<'a> {
    pub fn noop(self) {
        println!("інтерфейс використаний");
    }
}

struct Manager {
    text: String,
}

struct List {
    manager: Manager,
}

impl List {
    pub fn get_interface(&mut self) -> Interface {
        Interface {
            manager: &mut self.manager,
        }
    }
}



fn use_list(list: &List) {
    println!("{}", list.manager.text);
}




/*
Основна зміна в методі get_interface. Ми прибрали lifetime 'a в типі повернення,
тому запозичення обмежується лише часом виклику функції.
Таким чином, Rust може завершити запозичення після виклику методу noop, дозволяючи подальше використання змінної list.

проблему викликає інваріантність параметра 'a в структурі Interface<'a>, що означає,
що час життя Interface не може бути "скороченим" від часу життя List.
Це типовий випадок, коли Rust не може гарантувати, що запозичення живе достатньо довго.

Одним із рішень цієї проблеми є зміна структури даних так, щоб зняти обмеження на час життя,
або можна використовувати заміну на тип 'static для простішого управління часом життя.
Однак тут можна змінити підхід до часів життя, зробивши Interface менш тісно пов'язаним із List.
Один із варіантів – видалити час життя 'a з Manager та відповідних структур, зробивши так, щоб воно більше не блокувало процес.

Час життя: видалили параметр 'a з Manager, оскільки він більше не прив'язаний до часу життя,
а використовує тип String, який не потребує зовнішнього часу життя.

String замість &str: Замість запозиченого рядка (&str), тепер ми використовуємо String,
щоб менеджер володів своїми даними і не потребував часу життя 'a.
*/