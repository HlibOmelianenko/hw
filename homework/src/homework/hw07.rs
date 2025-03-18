fn main() {
    let input = "HeLLo WoRLd";

    // Міняємо регістр з верхнього на нижній та з нижнього на верхній
    let changed_case: String = input.chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect();

    println!("Original: {}", input);
    println!("Changed: {}", changed_case);
}
