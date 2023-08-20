use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take,},
    character::complete::{alpha1, alphanumeric1, one_of,char},
    number::complete::{double},
    combinator::opt,
    error::{context, ErrorKind, VerboseError},
    multi::{count, many0, many1, many_m_n},
    sequence::{preceded, separated_pair, terminated, tuple},
    AsChar, Err as NomErr, IResult, InputTakeAtPosition,
};
//SIP methods
#[derive(Debug,PartialEq)]
pub enum Method{
 INVITE,
 ACK,
 OPTIONS,
CANCEL,
BYE,
REGISTER
}
//The host cant either be IP or a domain name
pub enum Host{
    HOST(String)
    IP([u8;4])
}
//sip version
type Version <'a> = (&'a str, & 'a str);
impl From<&str> for Method{
    fn from(i:&str)->Self{
        match  i {
           "%x49.4E.56.49.54.45 ;" | "INVITE" => Method::INVITE,
            "%x41.43.4B ;" | "ACK" =>Method::ACK,
            "%x4F.50.54.49.4F.4E.53 ;"|"OPTIONS" => Method::OPTIONS,
            "%x42.59.45 ;" | "BYE" => Method::BYE,
            "%x43.41.4E.43.45.4C ;" | "CANCLE" =>  Method::CANCEL,
            "%x52.45.47.49.53.54.45.52 ;" |"REGISTER" => Method::REGISTER,
            _ => unimplemented!("Unsupported method")
        }
    }
}
//Custom error handling for parsing
type Res<T, U> = IResult<T, U, VerboseError<T>>;
//parse method
pub fn method(input:&str)->Res<&str,Method>{
   context(
    "Method",
    alt((tag("INVITE"), tag("%x49.4E.56.49.54.45 ;"),tag("ACK"), tag("%x41.43.4B ;"),
        tag("OPTIONS"),tag("%x4F.50.54.49.4F.4E.53 ;"), tag("BYE"),tag("%x42.59.45 ;"),
        tag("CANCLE"), tag("%x43.41.4E.43.45.4C ;"),tag("REGISTER"), tag("%x52.45.47.49.53.54.45.52 ;" ),
    ))
   ) (input)
   .map(|(next_input,res)|
   (next_input,res.into()))
}
//parse SIP version
pub fn sip_version(input:&str)->Res<&str, (&str,&str)>{
    context(
"SIP Version",
        separated_pair(tag("SIP"), tag("/"), tag("2.0"))
    )(input)
    }

//parse domain name
