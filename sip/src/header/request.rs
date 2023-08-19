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
Invite,
Ack,
Options,
Cancel,
Bye,
Register
}

impl From<&str> for Method{
    fn from(i:&str)->Self{
        match  i {
           "%x49.4E.56.49.54.45 ;" | "INVITE" => Method::Invite,
            "%x41.43.4B ;" | "ACK" =>Method::Ack,
            "%x4F.50.54.49.4F.4E.53 ;"|"OPTIONS" => Method::Options,
            "%x42.59.45 ;" | "BYE" => Method::Bye,
            "%x43.41.4E.43.45.4C ;" | "CANCLE" =>  Method::Cancel,
            "%x52.45.47.49.53.54.45.52 ;" |"REGISTER" => Method::Register,
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
pub fn sip_version(input:&str)->Res<&str, (&str,Option<&str>)>{
    context(
        "SIP-Version",
        separated_pair(tag("SIP"),char('/'),tag(double))
    )(input)
}
