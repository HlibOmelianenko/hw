const WIDTH: usize = 30;  // Ширина конверта
const HEIGHT: usize = 15; // Высота конверта

fn main() {
    let mut lines = vec![String::new(); HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let c = if y == 0 || y == HEIGHT - 1 || x == 0 || x == WIDTH - 1 {
                '*'
            } else if x == y * 2 || x == WIDTH - y * 2 - 1 {
                '*'
            } else {
                ' '
            };
            lines[y].push(c);
        }
    }

    println!("{}", lines.join("\n"));
}
