use super::*;

macro_rules! eval {
    ($src:expr, $expected:expr) => {
        fn test(args: &[f64]) {
            assert_eq!(args[0], $expected);
        }
        Runtime::initialize();
        let mut runtime = Runtime::new().with_host_function("test", test);
        let module = runtime.compile_script($src.as_ref()).unwrap();
        runtime.eval(module);
    };
}

#[test]
fn test_eval_number() {
    const A: f64 = 1.0;
    eval!(format!("test({A})"), A);
}

#[test]
fn test_eval_addition_expression() {
    const A: f64 = 1.2;
    const B: f64 = 3.4;
    eval!(format!("test({A} + {B})"), A + B);
}

#[test]
fn test_eval_subtraction_expression() {
    const A: f64 = 1.2;
    const B: f64 = 3.4;
    eval!(format!("test({A} - {B})"), A - B);
}

#[test]
fn test_eval_multiplication_expression() {
    const A: f64 = 1.2;
    const B: f64 = 3.4;
    eval!(format!("test({A} * {B})"), A * B);
}

#[test]
fn test_eval_division_expression() {
    const A: f64 = 1.2;
    const B: f64 = 3.4;
    eval!(format!("test({A} / {B})"), A / B);
}

#[test]
fn test_eval_remainder_expression() {
    const A: f64 = 1.0;
    const B: f64 = 3.0;
    eval!(format!("test({A} % {B})"), A % B);
}

#[test]
fn test_eval_group_expression() {
    const A: f64 = 1.2;
    const B: f64 = 3.4;
    const C: f64 = 5.6;
    eval!(format!("test({A} * ({B} + {C}))"), A * (B + C));
}

#[test]
fn test_eval_call_with_no_argument() {
    const A: f64 = 1.2;
    eval!(format!("function a() {{ return {A}; }} test(a());"), A);
}

#[test]
fn test_eval_call_with_no_argument_hoistable_declaration() {
    const A: f64 = 1.2;
    eval!(format!("test(a()); function a() {{ return {A}; }}"), A);
}

#[test]
fn test_eval_const_declaration() {
    const A: f64 = 1.2;
    const B: f64 = 3.4;
    eval!(format!("const a = {A}, b = {B}; test(a);"), A);
}

#[test]
fn test_eval_let_declaration() {
    const A: f64 = 1.2;
    const B: f64 = 3.4;
    eval!(format!("let a, b = {B}; a = {A}; test(a);"), A);
}

#[test]
fn test_eval_arithmetic_operations_with_variables() {
    const A: f64 = 1.2;
    const B: f64 = 3.4;
    const C: f64 = 5.6;
    eval!(
        format!("const a = {A}, b = {B}, c = {C}; test(a + b * c);"),
        A + B * C
    );
}

#[test]
fn test_eval_conditional_expression() {
    eval!("test(1 > 0 ? 2 : 3)", 2.);
}

#[test]
fn test_eval_nested_conditional_expression() {
    eval!("test(1 > 0 ? 1 > 0 ? 2 : 3 : 1 > 0 ? 4 : 5)", 2.);
}

#[test]
fn test_eval_if_statement() {
    eval!("let a = 1; if (1) { a = 2; } test(a);", 2.);
}

#[test]
fn test_eval_if_else_statement() {
    eval!("let a = 1; if (0) { a = 2; } else { a = 3; } test(a);", 3.);
}

#[test]
fn test_eval_block_statement() {
    eval!("let a = 1; { let a = 2; } test(a);", 1.);
}

#[test]
fn test_eval_return_statement_in_block() {
    eval!("test(a()); function a() { let a = 1; { return a; } }", 1.);
}

#[test]
fn test_eval_terminated_basic_block() {
    eval!(
        "test(a()); function a() { if (1) { return 1; } return 2; }",
        1.
    );
}

#[test]
fn test_eval_function_single_name_binding() {
    eval!("test(a(1)); function a(x) { return x; }", 1.);
}

#[test]
fn test_eval_fibonacci() {
    eval!(
        "test(fib(10)); function fib(n) { if (n < 2) return n; return fib(n - 1) + fib(n - 2); }",
        55.
    );
}
