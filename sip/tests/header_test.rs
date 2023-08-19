
    use sip::header::request::*;
  /*use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };*/

    #[test]
    fn test_method(){
      assert_eq!(method("INVITE sip:bob@biloxi.com SIP/2.0"),
      Ok((" sip:bob@biloxi.com SIP/2.0",Method::Invite)));
      assert_eq!(method("%x52.45.47.49.53.54.45.52 ; sip:bob@biloxi.com SIP/2.0"),
      Ok((" sip:bob@biloxi.com SIP/2.0",Method::Register)))

      //assert_eq!(add_num(4,2), 6)
    }
    #[test]
    fun test_version(){
      assert_eq!(sip_version("INVITE sip:bob@biloxi.com SIP/2.0"),
      Ok(("INVITE sip:bob@biloxi.com ","SIP/2.0")));
    }

