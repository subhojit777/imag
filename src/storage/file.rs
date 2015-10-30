use regex::Regex;

pub struct ParserError {
    summary: String,
    parsertext: String,
    index: i32,
    explanation: Option<String>,
}

impl ParserError {
    fn new(sum: &'static str, text: String, idx: i32, expl: &'static str) -> ParserError {
        ParserError {
            summary: String::from(sum),
            parsertext: text,
            index: idx,
            explanation: Some(String::from(expl)),
        }
    }

    fn short(sum: &'static str, text: String, idx: i32) -> ParserError {
        ParserError {
            summary: String::from(sum),
            parsertext: text,
            index: idx,
            explanation: None
        }
    }
}

pub enum FileHeaderSpec {
    Null,
    Bool,
    Integer,
    UInteger,
    Float,
    Text,
    Key { name: String, value_type: Box<FileHeaderSpec> },
    Array { allowed_types: Box<Vec<FileHeaderSpec>> },
}

pub enum FileHeaderData {
    Null,
    Bool(bool),
    Integer(i64),
    UInteger(u64),
    Float(f64),
    Text(String),
    Key { name: String, value: Box<FileHeaderData> },
    Array { values: Box<Vec<FileHeaderData>> },
}

pub trait FileHeaderParser : Sized {
    fn new(spec: &FileHeaderSpec) -> Self;
    fn read(&self, string: Option<String>) -> Result<FileHeaderData, ParserError>;
    fn write(&self, data: &FileHeaderData) -> Result<String, ParserError>;
}

pub trait FileData : Sized {
    fn get_fulltext(&self) -> String;
    fn get_abbrev(&self) -> String;
}

pub trait FileDataParser<FD: FileData + Sized> : Sized {
    fn new() -> Self;
    fn read(&self, string: Option<String>) -> Result<FD, ParserError>;
    fn write(&self, data: &FD) -> Result<String, ParserError>;
}

type TextTpl = (Option<String>, Option<String>);

pub struct Parser<HP, DP>
{
    headerp : HP,
    datap : DP,
}

impl<HP, DP> Parser<HP, DP> where
    HP: FileHeaderParser,
{

    fn new(headerp: HP, datap: DP) -> Parser<HP, DP> {
        Parser {
            headerp: headerp,
            datap: datap,
        }
    }

    fn read<FD>(&self, s: String) -> Result<(FileHeaderData, FD), ParserError>
        where FD: FileData + Sized,
              DP: FileDataParser<FD>
    {
        let divided = divide_text(&s);

        if divided.is_err() {
            return Err(divided.err().unwrap());
        }

        let (header, data) = divided.ok().unwrap();

        let h_parseres = self.headerp.read(header);
        let d_parseres = self.datap.read(data);

        if h_parseres.is_err() {
            return Err(h_parseres.err().unwrap());
        }

        if d_parseres.is_err() {
            return Err(d_parseres.err().unwrap());
        }

        Ok((h_parseres.ok().unwrap(), d_parseres.ok().unwrap()))
    }

    fn write<FD>(&self, tpl : (FileHeaderData, FD)) -> Result<String, ParserError>
        where FD: FileData + Sized,
              DP: FileDataParser<FD>
    {
        let (header, data) = tpl;
        let h_text = self.headerp.write(&header);
        let d_text = self.datap.write(&data);

        if h_text.is_err() {
            return Err(h_text.err().unwrap());
        }

        if d_text.is_err() {
            return Err(d_text.err().unwrap());
        }

        Ok(h_text.ok().unwrap() + &d_text.ok().unwrap()[..])
    }
}

fn divide_text(text: &String) -> Result<TextTpl, ParserError> {
    let re = Regex::new(r"(?m)^\-\-\-$\n(.*)^\-\-\-$\n(.*)").unwrap();

    let captures = re.captures(&text[..]).unwrap_or(
        return Err(ParserError::new("Cannot run regex on text",
                                    text.clone(), 0,
                                    "Cannot run regex on text to divide it into header and content."))
    );

    if captures.len() != 2 {
        return Err(ParserError::new("Unexpected Regex output",
                                    text.clone(), 0,
                                    "The regex to divide text into header and content had an unexpected output."))
    }

    let header  = captures.at(0).map(|s| String::from(s));
    let content = captures.at(1).map(|s| String::from(s));
    Ok((header, content))
}
