#![allow(non_snake_case)]

use core::convert::TryInto;
use test_winrt_signatures::*;
use windows::core::*;
use Component::Signatures::*;

#[implement(Component::Signatures::ITestDouble)]
struct RustTest();

impl RustTest {
    fn SignatureDouble(&self, a: f64, b: &mut f64) -> Result<f64> {
        *b = a;
        Ok(a)
    }
    fn ArraySignatureDouble(&self, a: &[f64], b: &mut [f64], c: &mut Array<f64>) -> Result<Array<f64>> {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }
    fn CallSignatureDouble(&self, handler: &Option<SignatureDouble>) -> Result<()> {
        let a = 123.0;
        let mut b = 0.0;
        // TODO: this seems rather verbose...
        let c = handler.as_ref().unwrap().Invoke(a, &mut b)?;
        assert!(a == b);
        assert!(a == c);
        Ok(())
    }
    fn CallArraySignatureDouble(&self, handler: &Option<ArraySignatureDouble>) -> Result<()> {
        let a = [1.0, 2.0, 3.0];
        let mut b = [0.0; 3];
        let mut c = Array::new();
        let d = handler.as_ref().unwrap().Invoke(&a, &mut b, &mut c)?;

        assert!(a == b);
        // TODO: should `a == c` be sufficient? Does that work for Vec?
        assert!(a == c[..]);
        assert!(a == d[..]);

        Ok(())
    }
}

fn test_interface(test: &ITestDouble) -> Result<()> {
    let a = 123.0;
    let mut b = 0.0;
    let c = test.SignatureDouble(a, &mut b)?;

    assert!(a == b);
    assert!(a == c);

    test.CallSignatureDouble(SignatureDouble::new(|a, b| {
        *b = a;
        Ok(a)
    }))?;

    let a = [4.0, 5.0, 6.0];
    let mut b = [0.0; 3];
    let mut c = Array::new();
    let d = test.ArraySignatureDouble(&a, &mut b, &mut c)?;

    assert!(a == b);
    assert!(a == c[..]);
    assert!(a == d[..]);

    test.CallArraySignatureDouble(ArraySignatureDouble::new(|a, b, c| {
        assert!(a.len() == b.len());
        assert!(c.is_empty());
        b.copy_from_slice(a);
        *c = Array::from_slice(a);
        Ok(Array::from_slice(a))
    }))?;

    Ok(())
}

#[test]
fn test() -> Result<()> {
    test_interface(&Test::new()?.try_into()?)?;
    test_interface(&RustTest().into())?;
    Ok(())
}
