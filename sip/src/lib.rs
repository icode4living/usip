
pub mod header;
#[cfg(test)]
mod tests{
    use header::*;
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };

    #[test]
    fn test_method(){
        assert_eq!(method("INVITE sip:bob@biloxi.com SIP/2.0"),Ok(("INVITE",Method::Invite)))
    }
}
