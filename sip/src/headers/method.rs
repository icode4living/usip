
use nom;
/**SIP methods */
#[derive(Debug, PartialEq,Eq)]
pub enum Method{
    INVITE(&str),
    ACK(&str),
    BYE(&str),
    CANCEL(&str),
    REGISTER(&str),
    Opti(&str),
    Info(&str,)
}
pub enum Host {
    HOST(String),
    IP([u8; 4]),
}
impl From<&str> for Method{
    fn from(m:&str)->Self{
      match  m.as_tr(){
       "%x49.4E.56.49.54.45 ;" |"INVITE" => Method::Invite,
       "%x41.43.4B ;" |"ACK" => Method::Ack,
        "%x42.59.45 ;"|"BYE" => Method::Bye,
       "%x43.41.4E.43.45.4C ;" |"CANCEL" => Method::Cancel,
       "%x52.45.47.49.53.54.45.52 ;" | "REGISTER" => Method::Register,
        "%x4F.50.54.49.4F.4E.53 ;"|"OPTIONS" => Method::Options,
      }
    }
    
}
//Parse Uri 
#[doc(hidden)]
fn alphanumerichyphen1<T>(i: T) -> Res<T, T>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    i.split_at_position1_complete(
        |item| {
            let char_item = item.as_char();
            !(char_item == '-') && !char_item.is_alphanum()
        },
        ErrorKind::AlphaNumeric,
    )
}
#[doc(hidden)]
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
#[doc(hidden)]
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