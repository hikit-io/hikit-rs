
#[derive(Debug)]
pub struct Message<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub to: &'a [&'a str],
}

#[derive(Debug, PartialEq)]
pub struct PushResult<'a> {
    pub email: &'a str,
    pub success: bool,
    pub reason: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct Response<'a> {
    pub results: Vec<PushResult<'a>>,
}