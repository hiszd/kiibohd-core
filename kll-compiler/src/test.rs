#[cfg(test)]
mod trivial {
    use crate::types::KllFile;

    #[test]
    fn define() {
        let result = dbg!(KllFile::from_str("myDefine => myCDefine;\n"));
        assert!(result.is_ok());
    }

    #[test]
    fn quoted() {
        let result = dbg!(KllFile::from_str("\"Foo Bar\" = \"Baz Cubed\";\n"));
        assert!(result.is_ok());
    }

    #[test]
    fn array() {
        let result = dbg!(KllFile::from_str("Name_Foo[0] = myKeymapFile;\n"));
        assert!(result.is_ok());
    }

    #[test]
    fn capability() {
        let result = dbg!(KllFile::from_str(
            "myCapability => myCFunction(arg1:1, arg2:2);\n"
        ));
        assert!(result.is_ok());
    }

    #[test]
    fn scancode() {
        let result = dbg!(KllFile::from_str("S100 : U\"A\";\n"));
        assert!(result.is_ok());
    }

    #[test]
    fn pixelmap() {
        let result = dbg!(KllFile::from_str("P[5](30:8) : S13;\n"));
        assert!(result.is_ok());
    }

    #[test]
    fn position() {
        let result = dbg!(KllFile::from_str("P[30] <= x:20,rx:15;\n"));
        assert!(result.is_ok());
    }

    #[test]
    fn animation() {
        let result = dbg!(KllFile::from_str(
            "A[MyEyesAreBleeding] <= start, loop:3;\n"
        ));
        assert!(result.is_ok());
    }

    #[test]
    fn frame() {
        let result = dbg!(KllFile::from_str("A[Bleeed, 5] <= P[2](255,255,255);\n"));
        assert!(result.is_ok());
    }

    #[test]
    fn result() {
        let result = dbg!(KllFile::from_str("S100 : P[23](+43,+21,-40);\n"));
        assert!(result.is_ok());
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod examples {
    use crate::types::KllFile;
    use std::fs;

    #[test]
    fn assignment() {
        let test = fs::read_to_string("examples/assignment.kll").unwrap();
        let result = dbg!(KllFile::from_str(&test));
        assert!(result.is_ok());
    }

    #[test]
    fn capabilities() {
        let test = fs::read_to_string("examples/capabilitiesExample.kll").unwrap();
        let result = dbg!(KllFile::from_str(&test));
        assert!(result.is_ok());
    }

    #[test]
    fn colemak() {
        let test = fs::read_to_string("examples/colemak.kll").unwrap();
        let result = dbg!(KllFile::from_str(&test));
        assert!(result.is_ok());
    }

    #[test]
    fn defaultmap() {
        let test = fs::read_to_string("examples/defaultMapExample.kll").unwrap();
        let result = dbg!(KllFile::from_str(&test));
        assert!(result.is_ok());
    }

    #[test]
    fn example() {
        let test = fs::read_to_string("examples/example.kll").unwrap();
        let result = dbg!(KllFile::from_str(&test));
        assert!(result.is_ok());
    }

    #[test]
    fn example2() {
        let test = fs::read_to_string("examples/example2.kll").unwrap();
        let result = dbg!(KllFile::from_str(&test));
        assert!(result.is_ok());
    }

    #[test]
    fn hhkbpro2() {
        let test = fs::read_to_string("examples/hhkbpro2.kll").unwrap();
        let result = dbg!(KllFile::from_str(&test));
        assert!(result.is_ok());
    }

    /* TODO
    #[test]
    fn leds() {
        let test = fs::read_to_string("examples/leds.kll").unwrap();
        let result = dbg!(KllFile::from_str(&test));
        assert!(result.is_ok());
    }

    #[test]
    fn leds2() {
        let test = fs::read_to_string("examples/leds2.kll").unwrap();
        let result = dbg!(KllFile::from_str(&test));
        assert!(result.is_ok());
    }
    */

    #[test]
    fn mapping() {
        let test = fs::read_to_string("examples/mapping.kll").unwrap();
        let result = dbg!(KllFile::from_str(&test));
        assert!(result.is_ok());
    }

    #[test]
    fn md1map() {
        let test = fs::read_to_string("examples/md1Map.kll").unwrap();
        let result = dbg!(KllFile::from_str(&test));
        assert!(result.is_ok());
    }

    #[test]
    fn nonetest() {
        let test = fs::read_to_string("examples/nonetest.kll").unwrap();
        let result = dbg!(KllFile::from_str(&test));
        assert!(result.is_ok());
    }

    #[test]
    fn simple1() {
        let test = fs::read_to_string("examples/simple1.kll").unwrap();
        let result = dbg!(KllFile::from_str(&test));
        assert!(result.is_ok());
    }

    /* TODO
    #[test]
    fn simple2() {
        let test = fs::read_to_string("examples/simple2.kll").unwrap();
        let result = dbg!(KllFile::from_str(&test));
        assert!(result.is_ok());
    }
    */

    #[test]
    fn simpleExample() {
        let test = fs::read_to_string("examples/simpleExample.kll").unwrap();
        let result = dbg!(KllFile::from_str(&test));
        assert!(result.is_ok());
    }

    /* TODO
    #[test]
    fn state_scheduling() {
        let test = fs::read_to_string("examples/state_scheduling.kll").unwrap();
        let result = dbg!(KllFile::from_str(&test));
        assert!(result.is_ok());
    }
    */

    /* TODO
    #[test]
    fn triggers() {
        let test = fs::read_to_string("examples/triggers.kll").unwrap();
        let result = dbg!(KllFile::from_str(&test));
        assert!(result.is_ok());
    }
    */

    #[test]
    fn utf8() {
        let test = fs::read_to_string("examples/utf8.kll").unwrap();
        let result = dbg!(KllFile::from_str(&test));
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod processing {
    use crate::types::{KllFile, Statement};

    #[test]
    fn scancode_implied_state() {
        let result = dbg!(KllFile::from_str("S100 : U\"A\";\n"));
        assert!(result.is_ok());
        if let Statement::Keymap(mapping) = &result.unwrap().statements[0] {
            dbg!(mapping.implied_state());
        }
    }
}
