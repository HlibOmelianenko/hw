fn main() {

    const HEIGHT: usize = 5;


    for i in 0..HEIGHT {
        let spaces = HEIGHT - i - 1;
        let stars = 2 * i + 1;       // Визначаємо кількість зірочок

        // Виводимо пробіли
        print!("{:width$}", "", width = spaces);
        // Виводимо зірочки
        println!("{}", "*".repeat(stars));
    }

    // Малювання середніх частин ялинки
    for _ in 0..3 {
        for i in 0..HEIGHT {
            let spaces = HEIGHT - i - 1;
            let stars = 2 * i + 1;

            print!("{:width$}", "", width = spaces);
            println!("{}", "*".repeat(stars));
        }
    }


}