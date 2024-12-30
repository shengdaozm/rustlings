// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.


#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        // 没有处理错误的情况？我直接修改？
        if value < 0 {
            return Err(CreationError::Negative);
        } else if value == 0 {
            return Err(CreationError::Zero);
        }
        return Ok(PositiveNonzeroInteger(value as u64));
    }
}
fn main() {}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
