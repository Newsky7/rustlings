// errors4.rs
// Make this test pass! Execute `rustlings hint errors4` for hints :)


#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        
        if value > 0 {
            return Ok(PositiveNonzeroInteger(value as u64)); 
        }else if value == 0{
            return Err(CreationError::Zero);
        }else{
            return Err(CreationError::Negative);
        }
        /*
        match: (value as u64) {
            Ok(thing) => PositiveNonzeroInteger(thing),
            Err(thing) => Err(thing),
        }
        */
        //Ok(PositiveNonzeroInteger(value as u64))
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
