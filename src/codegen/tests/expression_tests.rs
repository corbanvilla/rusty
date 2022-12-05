// Copyright (c) 2020 Ghaith Hachem and Mathias Rieder
use crate::test_utils::tests::codegen;

#[test]
fn pointers_in_function_return() {
    let result = codegen(
        r#"FUNCTION func : REF_TO INT
        END_FUNCTION"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn structs_in_function_return() {
    let result = codegen(
        r#"
        TYPE myStruct : STRUCT
            x : INT;
            END_STRUCT
        END_TYPE
        FUNCTION func : myStruct
            VAR_OUTPUT
                xxx : myStruct;
            END_VAR
        END_FUNCTION"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn strings_in_function_return() {
    let result = codegen(
        r#"
       FUNCTION func : STRING
            VAR_INPUT
                myout : REF_TO STRING;
            END_VAR

            func := 'hello';
            myout^ := 'hello';
       END_FUNCTION"#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn calling_strings_in_function_return() {
    let result = codegen(
        r#"
       FUNCTION func : STRING
            func := 'hello';
       END_FUNCTION
       
       PROGRAM main
            VAR
                x : STRING;
            END_VAR

            x := func();
       END_PROGRAM
       "#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn unary_expressions_can_be_real() {
    let result = codegen(
        r#"
            PROGRAM prg
            VAR
                a,b : REAL;
            END_VAR
                b := -2.0;
                a := -b;
            END_PROGRAM
        "#,
    );
    insta::assert_snapshot!(result);
}

#[test]
fn type_mix_in_call() {
    let result = codegen(
        "
        FUNCTION foo : INT
        VAR_INPUT
            in : INT;
        END_VAR
        END_FUNCTION
        FUNCTION baz : INT
            foo(1.5);
        END_FUNCTION
    ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn cast_pointer_to_lword() {
    let result = codegen(
        r#"
        FUNCTION baz : INT
            VAR 
                ptr_x : POINTER TO INT; 
                y : LWORD; 
            END_VAR;

            y := ptr_x;
        END_FUNCTION
    "#,
    );

    //should result in normal number-comparisons
    insta::assert_snapshot!(result);
}

#[test]
fn cast_lword_to_pointer() {
    let result = codegen(
        r#"
        FUNCTION baz : INT
            VAR 
                ptr_x : POINTER TO INT; 
                y : LWORD;
            END_VAR;

            ptr_x := y;
        END_FUNCTION
    "#,
    );

    //should result in normal number-comparisons
    insta::assert_snapshot!(result);
}

#[test]
fn cast_between_pointer_types() {
    let result = codegen(
        r#"
        PROGRAM baz
            VAR 
                ptr_x : POINTER TO BYTE; 
                y : WORD;
            END_VAR;

            ptr_x := &y;
        END_PROGRAM
    "#,
    );

    //should result in bitcast conversion when assigning to ptr_x
    insta::assert_snapshot!(result);
}

#[test]
fn unnecessary_casts_between_pointer_types() {
    let result = codegen(
        r#"
        TYPE MyByte : BYTE; END_TYPE
        
        PROGRAM baz
            VAR 
                ptr : POINTER TO BYTE; 
                b : BYTE;
                si : SINT;
                mb : MyByte;
            END_VAR;

            ptr := &b; //no cast necessary
            ptr := &si; //no cast necessary
            ptr := &mb; //no cast necessary
        END_PROGRAM
    "#,
    );

    //should not result in bitcast
    insta::assert_snapshot!(result);
}

#[test]
fn access_string_via_byte_array() {
    let result = codegen(
        r#"
        TYPE MyByte : BYTE; END_TYPE
        
        PROGRAM baz
            VAR 
                str: STRING[10];
                ptr : POINTER TO BYTE; 
                bytes : POINTER TO ARRAY[0..9] OF BYTE;
            END_VAR;

            ptr := &str; //bit-cast expected
            bytes := &str;
        END_PROGRAM
    "#,
    );

    //should result in bitcasts
    insta::assert_snapshot!(result);
}

#[test]
fn pointer_arithmetics() {
    // codegen should be successful for binary expression for pointer<->int / int<->pointer / pointer<->pointer
    let result = codegen(
        "
		PROGRAM main
		VAR
			x : INT := 10;
			y : INT := 20;
			pt : REF_TO INT;
		END_VAR
		pt := &(x);

		(* +/- *)
		pt := pt + 1;
		pt := pt + 1 + 1;
		pt := 1 + pt;
		pt := pt - y;
		pt := 1 + pt + 1;
		pt := pt - y - 1;
		pt := 1 + 1 + pt ;
		pt := y + pt - y ;
		pt := y + y + pt ;
		END_PROGRAM
		",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn pointer_arithmetics_function_call() {
    // codegen should be successful for binary expression for pointer<->int / int<->pointer / pointer<->pointer
    let result = codegen(
        "
        FUNCTION foo : LINT
        END_FUNCTION

		PROGRAM main
		VAR
			pt : REF_TO INT;
            x : INT;
		END_VAR
		pt := &(x);

		(* +/- *)
		pt := pt + foo();
		END_PROGRAM
		",
    );
    insta::assert_snapshot!(result);
}

#[test]
fn nested_call_statements() {
    // GIVEN some nested call statements
    let result = codegen(
        "
        FUNCTION foo : DINT
        VAR_INPUT
            a : DINT;
        END_VAR
        END_FUNCTION

		PROGRAM main
            foo(foo(2));
		END_PROGRAM
		",
    );
    // WHEN compiled
    // WE expect a flat sequence of calls, no regions and branching
    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_adr() {
    // GIVEN some nested call statements
    let result = codegen(
        "
		PROGRAM main
        VAR
            a : REF_TO DINT;
            b : DINT;
        END_VAR
            a := ADR(b);
		END_PROGRAM
		",
    );
    // WHEN compiled
    // We expect a direct conversion to lword and subsequent assignment (no call)
    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_ref() {
    // GIVEN some nested call statements
    let result = codegen(
        "
		PROGRAM main
        VAR
            a : REF_TO DINT;
            b : DINT;
        END_VAR
            a := REF(b);
		END_PROGRAM
		",
    );
    // WHEN compiled
    // We expect a direct conversion and subsequent assignment to pointer(no call)
    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_mux() {
    let result = codegen(
        "PROGRAM main
        VAR
            a,b,c,d,e : DINT;
        END_VAR
            a := MUX(3, b,c,d,e); //3 = d
        END_PROGRAM",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_sel() {
    let result = codegen(
        "PROGRAM main
        VAR
            a,b,c : DINT;
        END_VAR
            a := SEL(TRUE, b,c);
        END_PROGRAM",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_sel_as_expression() {
    let result = codegen(
        "PROGRAM main
        VAR
            a,b,c : DINT;
        END_VAR
            a := SEL(TRUE, b,c) + 10;
        END_PROGRAM",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_move() {
    let result = codegen(
        "PROGRAM main
        VAR
            a,b : DINT;
        END_VAR
            a := MOVE(b);
        END_PROGRAM",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn builtin_function_call_sizeof() {
    let result = codegen(
        "PROGRAM main
        VAR
            a: DINT;
            b: LINT;
        END_VAR
            a := SIZEOF(b);
        END_PROGRAM",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn test_max_int() {
    let result = codegen(
        r"
    {external}
    FUNCTION MAX<U : ANY> : U
    VAR_INPUT in : {sized} U...; END_VAR
    END_FUNCTION
    
    FUNCTION main : INT
    main := MAX(INT#5,INT#2,INT#1,INT#3,INT#4,INT#7,INT#-1);
    END_FUNCTION",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn int_to_int_expt() {
    let result = codegen(
        "
    FUNCTION main : DINT
        main := 2**3;
    END_FUNCTION
    ",
    );

    // expecting a int to uint operation because 1 is positive
    insta::assert_snapshot!(result);
}

#[test]
fn int_to_uint_expt() {
    let result = codegen(
        "
    FUNCTION main : DINT
        main := 2**UDINT#3;
    END_FUNCTION
    ",
    );

    // Expecting a int to uint operation
    insta::assert_snapshot!(result);
}

#[test]
fn int_to_negative_expt() {
    let result = codegen(
        "
    FUNCTION main : DINT
        main := 2**-1;
    END_FUNCTION
    ",
    );

    // Expecting a real to int operation
    insta::assert_snapshot!(result);
}

#[test]
fn int_to_intvar() {
    let result = codegen(
        "
    FUNCTION main : DINT
    VAR
        x,y : DINT;
    END_VAR
        main := x**y;
    END_FUNCTION
    ",
    );

    // Expecting a real to int operation
    insta::assert_snapshot!(result);
}

#[test]
fn uint_to_int_expt() {
    let result = codegen(
        "
    FUNCTION main : DINT
        main := UDINT#2**3;
    END_FUNCTION
    ",
    );

    // Expecting a udint to udint operation because 3 is positive
    insta::assert_snapshot!(result);
}

#[test]
fn int_to_real_expt() {
    let result = codegen(
        "
    FUNCTION main : REAL
        main := 2**REAL#0.5;
    END_FUNCTION
    ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn real_to_real_expt() {
    let result = codegen(
        "
    FUNCTION main : REAL
        main := REAL#2**REAL#0.1;
    END_FUNCTION
    ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn real_to_int_expt() {
    let result = codegen(
        "
    FUNCTION main : REAL
        main := REAL#3.0**2;
    END_FUNCTION
    ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn real_to_lint_expt() {
    let result = codegen(
        "
    FUNCTION main : REAL
        main := REAL#3.0**LINT#2;
    END_FUNCTION
    ",
    );

    // Expecting an lreal to lreal conversion
    insta::assert_snapshot!(result);
}

#[test]
fn lreal_to_real_expt() {
    let result = codegen(
        "
    FUNCTION main : LREAL
        main := LREAL#3**REAL#0.2;
    END_FUNCTION
    ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn real_to_lreal_expt() {
    let result = codegen(
        "
    FUNCTION main : LREAL
        main := REAL#4**LREAL#0.3;
    END_FUNCTION
    ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn lreal_to_lreal_expt() {
    let result = codegen(
        "
    FUNCTION main : LREAL
        main := LREAL#3**LREAL#0.2;
    END_FUNCTION
    ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn compare_date_time_literals() {
    let result = codegen(
        "
    PROGRAM main
    VAR_TEMP
        cmp1, cmp2, cmp3, cmp4, cmp5, cmp6, cmp7, cmp8 : BOOL;
    END_VAR
		cmp1 := TIME#2d4h6m8s10ms11us300ns < TIME#1d8h43m23s55ms;
		cmp2 := LTIME#2d4h6m8s10ms11us300ns > LTIME#1d8h43m23s55ms;

		cmp3 := TOD#23:59:59.999 < TOD#10:32:59;
		cmp4 := LTOD#23:59:59.999 > LTOD#10:32:59;

		cmp5 := DATE#2022-10-20 < DATE#1999-01-01;
		cmp6 := LDATE#2022-10-20 > LDATE#1999-01-01;

		cmp7 := DT#2022-10-20-23:59:59.999 < DT#1999-01-01-10:32;
		cmp8 := LDT#2022-10-20-23:59:59.999 > LDT#1999-01-01-10:32;
    END_PROGRAM
    ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn hardware_access_codegen() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
          x,y,z : BYTE;
        END_VAR
          x := %IB1.2;
          y := %MB1.2;
          z := %GB1.2;
          x := %IX1.2;
          y := %MD1.2;
          z := %GW1.2;
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}

#[test]
fn expt_edge_case() {
    let result = codegen(
        "
        PROGRAM prg
        VAR
            x : UDINT;
        END_VAR
          x := 1**UDINT#4_000_000_000;
        END_PROGRAM
        ",
    );

    insta::assert_snapshot!(result);
}
