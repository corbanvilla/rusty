use super::super::*;

#[allow(dead_code)]
#[repr(C)]
struct MainType {
    ret : i32,
}

#[test]
fn adds_in_result() {
    let prog = 
    "
    FUNCTION main : INT
        main := 10 + 50;
    END_FUNCTION
    ";

    let (res, _) = compile_and_run(prog.to_string(), MainType {ret : 0});
    assert_eq!(res,60)  
}