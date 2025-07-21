mod eval;
mod logging;
mod min;
mod counter;
mod rot13;

#[cfg(test)]
mod EvalTest {
    use crate::day2::eval::*;
    #[test]
    fn test_value() {
        assert_eq!(eval(Expression::Value(19)), Ok(19));
    }

    #[test]
    fn test_sum() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Add,
                left: Box::new(Expression::Value(10)),
                right: Box::new(Expression::Value(20)),
            }),
            Ok(30)
        );
    }

    #[test]
    fn test_recursion() {
        let term1 = Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(9)),
        };
        let term2 = Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Op {
                op: Operation::Sub,
                left: Box::new(Expression::Value(3)),
                right: Box::new(Expression::Value(4)),
            }),
            right: Box::new(Expression::Value(5)),
        };
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Add,
                left: Box::new(term1),
                right: Box::new(term2),
            }),
            Ok(85)
        );
    }

    #[test]
    fn test_error() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Div,
                left: Box::new(Expression::Value(99)),
                right: Box::new(Expression::Value(0)),
            }),
            Err(String::from("division by zero"))
        );
    }
}

#[cfg(test)]
mod LoggingTest {
    use crate::day2::logging::*;
    #[test]
    fn test_min() {
        let logger = VerbosityFilter::new(4, Box::new(StderrLogger));
        logger.log(3, Box::new("Test3"));
        logger.log(4, Box::new("Test4"));
        logger.log(5, Box::new("Test5"));
    }
}

#[cfg(test)]
mod MinTest {
    use crate::day2::min::*;
    #[test]
    fn min_test() {
        assert_eq!(min(0, 10), 0);
        assert_eq!(min(500, 123), 123);

        assert_eq!(min('a', 'z'), 'a');
        assert_eq!(min('7', '1'), '1');

        assert_eq!(min("hello", "goodbye"), "goodbye");
        assert_eq!(min("bat", "armadillo"), "armadillo");
    }
}

#[cfg(test)]
mod CounterTest {
    use crate::day2::counter::*;
    #[test]
    fn counter_test() {

        let mut ctr = Counter::new();
        ctr.count(13);
        ctr.count(14);
        ctr.count(16);
        ctr.count(14);
        ctr.count(14);
        ctr.count(11);

        for i in 10..20 {
            println!("{} 개의 {} 값을 발견했습니다.", ctr.times_seen(i), i);
        }

        let mut strctr = Counter::new();
        strctr.count("사과");
        strctr.count("오렌지");
        strctr.count("사과");
        println!("사과 {}개 받음", strctr.times_seen("사과"));
    }
}

#[cfg(test)]
mod rot13_test {
    use std::io::Read;
    use crate::day2::rot13::*;
    #[test]
    fn joke() {
        let mut rot =
            RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> { input: input.as_ref(), rot: 13 };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}