use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take,},
    character::complete::{alpha1, alphanumeric1, one_of,char},
    number::complete::{double},
    combinator::opt,
    error::{context, ErrorKind, VerboseError},
    multi::{count, many0, many1, many_m_n},
    sequence::{preceded, separated_pair, terminated, tuple, delimited},
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
#[derive(Debug,PartialEq)]
pub enum Host{
    HOST(String),
    IP([u8;4])
}
//sip uri

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

//parse host name
fn host(input: &str) -> Res<&str, Host> {
    context(
        "host",
        alt((
            tuple((many1(terminated(alphanumerichyphen1, tag("."))), alpha1)),
            tuple((many_m_n(1, 1, alphanumerichyphen1), take(0 as usize))),
        )),
    )(input)
    .map(|(next_input, mut res)| {
        if !res.1.is_empty() {
            res.0.push(res.1);
        }
        (next_input, Host::HOST(res.0.join(".")))
    })
}
fn alphanumerichyphen1<T>(i: T) -> Res<T, T>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    i.split_at_position1_complete(
        |item| {
            let char_item = item.as_char();
           // !(char_item == '_') && !char_item.is_alphanum();
              !(char_item == '-') && !char_item.is_alphanum()
        },
        ErrorKind::AlphaNumeric,
    )
}
//parse IP number
fn ip_num(input: &str) -> Res<&str, u8> {
    context("ip number", n_to_m_digits(1, 3))(input).and_then(|(next_input, result)| {
        match result.parse::<u8>() {
            Ok(n) => Ok((next_input, n)),
            Err(_) => Err(NomErr::Error(VerboseError { errors: vec![] })),
        }
    })
}

fn n_to_m_digits<'a>(n: usize, m: usize) -> impl FnMut(&'a str) -> Res<&str, String> {
    move |input| {
        many_m_n(n, m, one_of("0123456789"))(input)
            .map(|(next_input, result)| (next_input, result.into_iter().collect()))
    }
}

fn ip(input: &str) -> Res<&str, Host> {
    context(
        "ip",
        tuple((count(terminated(ip_num, tag(".")), 3), ip_num)),
    )(input)
    .map(|(next_input, res)| {
        let mut result: [u8; 4] = [0, 0, 0, 0];
        res.0
            .into_iter()
            .enumerate()
            .for_each(|(i, v)| result[i] = v);
        result[3] = res.1;
        (next_input, Host::IP(result))
    })
}
//ip or host name
pub fn ip_or_host(input: &str) -> Res<&str, Host> {
    context("ip or host", alt((ip, host)))(input)
}
//parse SIP URI e.g sip:bob@biloxi.com
pub fn sip_uri(input: &str) ->Res<&str, &str>{
    context("sip uri",
   
 )
  
    (input)
}