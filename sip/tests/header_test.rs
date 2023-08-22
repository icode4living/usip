
    use sip::header::request::*;
  /*use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };*/

    #[test]
    fn test_method(){
      assert_eq!(method("INVITE sip:bob@biloxi.com SIP/2.0"),
      Ok((" sip:bob@biloxi.com SIP/2.0",Method::INVITE)));
      assert_eq!(method("%x52.45.47.49.53.54.45.52 ; sip:bob@biloxi.com SIP/2.0"),
      Ok((" sip:bob@biloxi.com SIP/2.0",Method::REGISTER)))

      //assert_eq!(add_num(4,2), 6)
    }
    #[test]
    fn test_version(){
      assert_eq!(sip_version("SIP/2.0"),
      Ok(("",("SIP","2.0"))));
    }

#[test]
fn test_domain(){
 assert_eq!(ip_or_host("example.com:8000"),Ok((":8000", Host::HOST("example.com".to_string()))));
 assert_eq!(ip_or_host("197.110.115.179:8000"),Ok((":8000", Host::IP([197,110,115,179]))));

}
#[test]
fn sip_uri(){
  assert_eq!(sip_uri("INVITE sip:bob@biloxi.com"),Ok(("INVITE ", "sip:bob@biloxi.com")))
}