pub mod header;
use nom::*;
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
        match  i.to_uppercase().as_str(){
            "%x49.4E.56.49.54.45 ;" | "INVITE" => Method::Invite,
            "%x41.43.4B ;" | "ACK" =>Method::Ack,
            "%x4F.50.54.49.4F.4E.53 ;"|"OPTIONS" => Method::Options,
            "%x42.59.45 ;" | "BYE" => Method::Bye,
            "%x43.41.4E.43.45.4C ;" | "CANCLE" =>  Method::Cancel,
            "%x52.45.47.49.53.54.45.52 ;" | "REGISTER" => Method::Register,
            _ => Unimplemented!("Unsupported method")
        }
    }
}
//Custom error handling for parsing
type Res<T, U> = IResult<T, U, VerboseError<T>>;
//parse method
pub fn method(i:&str)->Res<&str,Method>{
   Context(
    "Method",
    alt((tag("INVITE"), tag("%x49.4E.56.49.54.45 ;"),tag("ACK"), tag("%x41.43.4B ;"),
        tag("OPTIONS"),tag("%x4F.50.54.49.4F.4E.53 ;"), tag("BYE"),tag("%x42.59.45 ;"),
        tag("CANCLE"), tag("%x43.41.4E.43.45.4C ;"),tag("REGISTER"), tag("%x52.45.47.49.53.54.45.52 ;" ),
    ))
   ) (input)
   .map(|(next_input,res)|(next_input,res)|(next_input,res)|(next_input,res)|
   (next_input,res)|(next_input,res)|(next_input,res)|(next_input,res)|
   (next_input,res.into()))
}
//unit testng
