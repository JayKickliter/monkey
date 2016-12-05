use copperline;
use monkey;

pub fn repl() {
    let mut cl = copperline::Copperline::new();
    while let Ok(line) = cl.read_line_default(">> ") {
        cl.add_history(line.clone());
        let mut lexer = monkey::lexer::Lexer::new(&line);
        while let Some(tok) = lexer.next_token() {
            println!("{:?}", tok);
        }
    }
}
