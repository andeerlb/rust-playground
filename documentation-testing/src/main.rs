/// Soma dois números.
///
/// # Exemplo
///
/// ```
/// let result = meu_modulo::soma(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn soma(a: i32, b: i32) -> i32 {
    a + b
}

/// ```no_run
/// let x = meu_modulo::requisicao_http("https://exemplo.com");
/// ```
pub fn requisicao_http(url: &str) -> String {
    // Implementação fictícia de uma requisição HTTP
    format!("Conteúdo de {}", url)
}

/// ```ignore
/// let x = meu_modulo::requisicao_http("https://exemplo.com");
/// ```
pub fn ignore_requisicao_http(url: &str) -> String {
    // Implementação fictícia de uma requisição HTTP
    format!("Conteúdo de {}", url)
}   

/// ```should_panic
/// panic!("Este código deve causar pânico");
/// ```
pub fn funcao_que_panica() {
    panic!("Este código deve causar pânico");
}

fn main() {
    println!("Hello, world!");
}