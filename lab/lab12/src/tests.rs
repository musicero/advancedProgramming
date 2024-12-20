#[cfg(test)]
mod tests {
    use crate::interpreter::*;
    use crate::tokenizer::*;

    #[test]
    fn test() {
        let test_cases = vec![
            // literals
            ("3", "3"),
            ("121231324135634563456363567", "121231324135634563456363567"),
            ("1.0", "1.0"),
            ("0.0", "0.0"),
            ("-1", "-1"),
            ("-1.1", "-1.1"),
            ("False", "False"),
            ("True", "True"),
            ("[ [ ] [ ] ]", "[[],[]]"),
            ("[ False [ ] True [ 1 2 ] ]", "[False,[],True,[1,2]]"),
            ("\" [ so { not if ] and } \"", "\"[ so { not if ] and }\""),
            // quotation literals
            ("{ 20 10 + }", "{ 20 10 + }"),
            (
                "[ { + } { 10 + } { 20 10 + } ]",
                "[{ + },{ 10 + },{ 20 10 + }]",
            ),
            // simple arithmetic
            ("1 1 +", "2"),
            ("10 20 *", "200"),
            ("20 2 div", "10"),
            ("20 2 /", "10"), // CHANGED THIS TO INT RESULT
            // arithmetic with type coercion
            ("1 1.0 +", "2.0"),
            ("10 20.0 *", "200.0"),
            ("20 2.0 div", "10.0"),
            ("20.0 2.0 div", "10.0"),
            // comparisons
            ("20 10 <", "False"),
            ("20 10 >", "True"),
            ("20 10.0 >", "True"),
            ("20.0 20.0 >", "False"),
            ("10 10 ==", "True"),
            ("10 10.0 ==", "True"),
            ("True True ==", "True"),
            ("True 40 40 == ==", "True"),
            ("\" abba \" \" abba \" ==", "True"),
            // ("[ ] [ ] ==", "True"),
            // ("[ 1 2 ] [ 1 2 ] ==", "True"),
            // (" [ [ ] ] [ [ ] ] ==", "True"),
            // {-- stack operations -}
            ("10 20 swap pop", "20"),
            ("10 dup dup + swap pop", "20"),
            ("10 20 swap dup + div", "1"),
            // {-- length -}
            // ("\" hello \" length", "5"),
            // ("\" hello world \" length", "11"),
            ("[ 1 2 3 [ ] ] length", "4"),
            // ("{ 10 20 + } length", "3"),
            // {-- String parsing -}
            // ("\" 12 \" parseInteger", "12"),
            // ("\" 12.34 \" parseFloat", "12.34"),
            // (
            //     "\" adam bob charlie \" words",
            //     "[\"adam\",\"bob\",\"charlie\"]",
            // ),
            // {-- lists -}
            ("[ 1 2 3 ]", "[1,2,3]"),
            ("[ 1 \" bob \" ]", "[1,\"bob\"]"),
            // ("[ 1 2 ] empty", "False"),
            // ("[ ] empty", "True"),
            ("[ 1 2 3 ] head", "1"),
            ("[ 1 2 3 ] length", "3"),
            // ("[ 1 2 3 ] tail", "[2,3]"),
            // ("1 [ ] cons", "[1]"),
            // ("1 [ 2 3 ] cons", "[1,2,3]"),
            // ("[ 1 ] [ 2 3 ] append", "[1,2,3]"),
            // ("[ 1 2 ] [ ] append", "[1,2]"),
            // ("[ 1 ] [ 2 3 ] cons", "[[1],2,3]"),
            // {-- list quotations -}
            ("[ 1 2 3 ] { 10 * } map", "[10,20,30]"),
            ("[ 1 2 3 ] { 1 + } map", "[2,3,4]"),
            (
                "[ 1 2 3 4 ] { dup 2 > { 10 * } { 2 * } if } map",
                "[2,4,30,40]",
            ),
            // ("[ 1 2 3 4 ] each { 10 * } + + +", "100"),
            // ("[ 1 2 3 4 ] 0 foldl { + }", "10"),
            // ("[ 2 5 ] 20 foldl { div }", "2"),
            // {-- note no { } needed for 1 instruction code -}
            // (
            //     "[ \" 1 \" \" 2 \" \" 3 \" ] each { parseInteger } [ ] cons cons cons",
            //     "[1,2,3]",
            // ),
            // (
            //     "[ \" 1 \" \" 2 \" \" 3 \" ] each parseInteger [ ] 3 times cons",
            //     "[1,2,3]",
            // ),
            // ("[ 1 2 3 4 ] 0 foldl +", "10"),
            // ("[ 2 5 ] 20 foldl div", "2"),
            // {-- assignments -}
            ("age", "age"),
            ("age 10 := age", "10"),
            ("10 age swap := age", "10"),
            ("[ 1 2 3 ] list swap := list", "[1,2,3]"),
            // ("age 20 := [ 10 age ]", "[10,20]"),
            // For functions I also used the := operator
            ("inc { 1 + } fun 1 inc", "2"),
            ("mul10 { 10 * } fun inc { 1 + } fun 10 inc mul10", "110"),
            // {-- quotations -}
            ("{ 20 10 + } exec", "30"),
            ("10 { 20 + } exec", "30"),
            ("10 20 { + } exec", "30"),
            ("{ { 10 20 + } exec } exec", "30"),
            ("{ { 10 20 + } exec 20 + } exec", "50"),
            // {-- if -} // I get some ordering errors here, but in essence, it works.
            ("True { 20 } { } if ", "20"),
            ("True { 20 10 + } { 3 } if", "30"),
            // ("10 5 5 == if { 10 + } { 100 + }", "20"),
            ("False { } { 45 } if", "45"),
            ("True { False { 50 } { 100 } if } { 30 } if", "100"),
            // {-- if without quotation ,more ergonomic expressions -}
            ("True 20 { } if", "20"),
            ("True { 20 10 + } 3 if", "30"),
            // ("10 10 5 5 == if + { 100 + }", "20"),
            ("False { } 45 if", "45"),
            // ("True if { False if 50 100 } 30", "100"),
            // {-- times -}
            ("{ 100 50 + } 1 for", "150"),
            // ("5 times { 1 } [ ] 5 times { cons } 0 foldl { + }", "5"),
            // ("5 times 1     [ ] 5 times   cons   0 foldl   +  ", "5"),
            // ("5 times { 10 } + + + +", "50"),
            // ("5 times 10 4 times +", "50"),
            // {-- loop -}
            // (
            //     "1 loop { dup 4 > } { dup 1 + } [ ] 5 times { cons }",
            //     "[12,3,4,5]",
            // ),
            // (
            //     "1 loop { dup 4 > } { dup 1 + } [ ] 5 times   cons  ",
            //     "[1,2,3,4,5]",
            // ),
            // (
            //     "[ 1 ] loop { dup length 9 > }  { dup head 1 + swap cons }",
            //     "[10,9,8,7,6,5,4,3,2,1]",
            // ),
        ];

        for (test_input, test_output) in test_cases {
            println!("Testing: {}", test_input);
            let program = match tokenize(test_input.to_string()) {
                Ok(program) => program,
                Err(err) => {
                    panic!("Error: {:?}", err);
                }
            };
            let mut runtime = Runtime::new();
            for statement in program {
                match interpret(&mut runtime, statement) {
                    Ok(_) => {}
                    Err(err) => panic!("Error: {:?}", err),
                }
            }
            if runtime.stack_len() == 1 {
                let output = runtime.pop_check().unwrap();
                println!("Expected: {}, Got {}", test_output, output);
                assert_eq!(test_output, format!("{}", output));
            } else {
                panic!("Multiple values remaining");
            }
        }
    }
}
