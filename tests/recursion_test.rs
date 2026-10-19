#[cfg(test)]
mod tests {
    use pseudo_interpreter::constants::error_handler::PseudoError;
    use pseudo_interpreter::{run_capture, Evaluator, Lexer, Parser, SharedBuffer};
    use std::fs;

    /// Run a `.ps` file under `tests/test_codes/functions/` and return what it
    /// printed.
    fn run_file(name: &str) -> Result<String, PseudoError> {
        let path = format!("tests/test_codes/functions/{}", name);
        let source = fs::read_to_string(&path).expect("test program should exist");
        run_capture(&source)
    }

    #[test]
    fn factorial_recurses() {
        // 5! then the two base cases.
        assert_eq!(run_file("factorial.ps").unwrap(), "120\n1\n1\n");
    }

    #[test]
    fn fibonacci_recurses() {
        // Branching recursion: each call makes two more.
        assert_eq!(run_file("fib.ps").unwrap(), "0\n1\n55\n");
    }

    #[test]
    fn mutual_recursion_works() {
        assert_eq!(run_file("mutual_recursion.ps").unwrap(), "true\ntrue\n");
    }

    #[test]
    fn locals_do_not_leak_to_caller() {
        // shadow(3) returns 6; the globals n and helper keep their values.
        assert_eq!(run_file("local_scope.ps").unwrap(), "6\n100\n7\n");
    }

    #[test]
    fn unshadowed_global_is_readable() {
        assert_eq!(run_file("read_global.ps").unwrap(), "15\n10\n");
    }

    #[test]
    fn return_exits_from_while() {
        // First value over 4, then the fallthrough when nothing qualifies.
        assert_eq!(run_file("return_in_while.ps").unwrap(), "5\n-1\n");
    }

    #[test]
    fn return_exits_from_for() {
        // The loop stops at 3 instead of running to 10.
        assert_eq!(run_file("return_in_for.ps").unwrap(), "1\n2\n3\n300\n");
    }

    #[test]
    fn missing_return_yields_null() {
        assert_eq!(run_file("no_return.ps").unwrap(), "hi \nada\nnull\nnull\n");
    }

    #[test]
    fn bare_return_does_not_swallow_next_statement() {
        // The statement after a bare `return` is unreachable, so neither its
        // side effect nor its value may escape.
        assert_eq!(
            run_file("bare_return_then_statement.ps").unwrap(),
            "start\nnull\n"
        );
    }

    #[test]
    fn arrays_declared_in_a_function_are_local() {
        assert_eq!(run_file("array_scope.ps").unwrap(), "99\n1\n");
    }

    #[test]
    fn element_assignment_cannot_reach_an_outer_array() {
        // Writing an element follows the same scoping rule as a scalar
        // assignment, so it cannot mutate the caller's array.
        let source = "\
shared[3] = [1, 2, 3]

function mutate()
    shared[0] = 99
    return 0
endfunction

print mutate()
";
        let err = run_capture(source).unwrap_err();
        assert!(
            matches!(err, PseudoError::VariableError(ref msg) if msg.contains("outer scope")),
            "expected a VariableError about scope, got {:?}",
            err
        );
    }

    #[test]
    fn return_outside_function_errors() {
        let err = run_file("return_outside.ps").unwrap_err();
        assert!(
            matches!(err, PseudoError::StatementError(ref msg) if msg.contains("return outside")),
            "expected a StatementError about return, got {:?}",
            err
        );
    }

    #[test]
    fn return_outside_function_in_loop_errors() {
        let err = run_file("return_outside_loop.ps").unwrap_err();
        assert!(
            matches!(err, PseudoError::StatementError(ref msg) if msg.contains("return outside")),
            "expected a StatementError about return, got {:?}",
            err
        );
    }

    #[test]
    fn runaway_recursion_errors_instead_of_overflowing() {
        let err = run_file("infinite_recursion.ps").unwrap_err();
        assert!(
            matches!(err, PseudoError::RuntimeError(ref msg) if msg.contains("recursion too deep")),
            "expected a RuntimeError about recursion depth, got {:?}",
            err
        );
    }

    #[test]
    fn frames_are_popped_after_each_call() {
        let source = "\
function countdown(n)
    if n == 0 then
        return 0
    endif
    return countdown(n - 1)
endfunction

print countdown(50)
print countdown(50)
";
        let tokens = Lexer::new(source).tokenize().unwrap();
        let tree = Parser::new(tokens).parse().unwrap();
        let buffer = SharedBuffer::new();
        let mut eval = Evaluator::with_writer(tree, Box::new(buffer.clone()));
        eval.evaluate().unwrap();
        assert_eq!(buffer.contents(), "0\n0\n");
        // Every pushed frame was popped, including the 50 nested ones.
        assert_eq!(eval.call_depth(), 0);
        assert!(!eval.in_function());
    }

    #[test]
    fn frames_are_popped_when_a_call_errors() {
        // The call fails partway in; its scope must not be left on the stack.
        let source = "\
function boom(n)
    return missing(n)
endfunction

print boom(1)
";
        let tokens = Lexer::new(source).tokenize().unwrap();
        let tree = Parser::new(tokens).parse().unwrap();
        let mut eval = Evaluator::with_writer(tree, Box::new(SharedBuffer::new()));
        assert!(eval.evaluate().is_err());
        assert_eq!(eval.call_depth(), 0);
    }

    #[test]
    fn recursive_calls_keep_separate_parameters() {
        // After the inner call returns, the outer call's n must be unchanged.
        let source = "\
function outer(n)
    inner = inner_call(n + 10)
    return n
endfunction

function inner_call(n)
    return n
endfunction

print outer(1)
";
        assert_eq!(run_capture(source).unwrap(), "1\n");
    }

    #[test]
    fn writer_can_be_injected_directly() {
        // The same capture, driven through the lexer/parser/evaluator by hand.
        let source = fs::read_to_string("tests/test_codes/functions/factorial.ps").unwrap();
        let tokens = Lexer::new(source.as_str()).tokenize().unwrap();
        let tree = Parser::new(tokens).parse().unwrap();
        let buffer = SharedBuffer::new();
        let mut eval = Evaluator::with_writer(tree, Box::new(buffer.clone()));
        eval.evaluate().unwrap();
        assert_eq!(buffer.contents(), "120\n1\n1\n");
    }
}
