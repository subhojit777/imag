//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use filters::filter::Filter;

use libimagstore::storeid::StoreId;

pub struct IsInCollectionsFilter<'a, A>(Option<A>, ::std::marker::PhantomData<&'a str>)
    where A: AsRef<[&'a str]>;

impl<'a, A> IsInCollectionsFilter<'a, A>
    where A: AsRef<[&'a str]>
{
    pub fn new(collections: Option<A>) -> Self {
        IsInCollectionsFilter(collections, ::std::marker::PhantomData)
    }
}

impl<'a, A> Filter<StoreId> for IsInCollectionsFilter<'a, A>
    where A: AsRef<[&'a str]> + 'a
{
    fn filter(&self, sid: &StoreId) -> bool {
        match self.0 {
            Some(ref colls) => sid.is_in_collection(colls),
            None => true,
        }
    }
}

/// Language definition for the header-filter language
pub mod header_filter_lang {
    use std::str;
    use std::str::FromStr;
    use std::process::exit;

    use nom::digit;
    use nom::multispace;

    use libimagstore::store::Entry;
    use libimagerror::trace::MapErrTrace;

    #[derive(Debug, PartialEq, Eq)]
    enum Unary {
        Not
    }

    named!(unary_operator<Unary>, alt_complete!(
        tag!("not") => { |_| { trace!("Unary::Not"); Unary::Not }}
    ));

    #[derive(Debug, PartialEq, Eq)]
    enum CompareOp {
        OpIs,
        OpIn,
        OpEq,
        OpNeq,
        OpGte, // >=
        OpLte, // <=
        OpLt,  // <
        OpGt,  // >
    }

    named!(compare_op<CompareOp>, alt_complete!(
        tag!("is" ) => { |_| { trace!("CompareOp::OpIs");  CompareOp::OpIs }} |
        tag!("in" ) => { |_| { trace!("CompareOp::OpIn");  CompareOp::OpIn }} |
        tag!("==" ) => { |_| { trace!("CompareOp::OpEq");  CompareOp::OpEq }} |
        tag!("eq" ) => { |_| { trace!("CompareOp::OpEq");  CompareOp::OpEq }} |
        tag!("!=" ) => { |_| { trace!("CompareOp::OpNeq"); CompareOp::OpNeq }} |
        tag!("neq") => { |_| { trace!("CompareOp::OpNeq"); CompareOp::OpNeq }} |
        tag!(">=" ) => { |_| { trace!("CompareOp::OpGte"); CompareOp::OpGte }} |
        tag!("<=" ) => { |_| { trace!("CompareOp::OpLte"); CompareOp::OpLte }} |
        tag!("<"  ) => { |_| { trace!("CompareOp::OpLt");  CompareOp::OpLt }}  |
        tag!(">"  ) => { |_| { trace!("CompareOp::OpGt");  CompareOp::OpGt }}
    ));

    #[derive(Debug, PartialEq, Eq)]
    enum Operator {
        Or,
        And,
        Xor,
    }

    named!(operator<Operator>, alt_complete!(
        tag!("or")      => { |_| { trace!("Operator::Or");  Operator::Or  }} |
        tag!("and")     => { |_| { trace!("Operator::And"); Operator::And }} |
        tag!("xor")     => { |_| { trace!("Operator::Xor"); Operator::Xor }}
    ));

    #[derive(Debug, PartialEq, Eq)]
    enum Function {
        Length,
        Keys,
        Values,
    }

    named!(function<Function>, alt_complete!(
        tag!("length") => { |_| { trace!("Function::Length"); Function::Length }} |
        tag!("keys")   => { |_| { trace!("Function::Keys");   Function::Keys   }} |
        tag!("values") => { |_| { trace!("Function::Values"); Function::Values }}
    ));

    #[derive(Debug, PartialEq, Eq)]
    enum Value {
        Boolean(bool),
        Integer(i64),
        String(String),
    }

    named!(int64<i64>, map!(digit, |r: &[u8]| {
        let val = str::from_utf8(r).unwrap_or_else(|e| {
            error!("Error = '{:?}'", e);
            ::std::process::exit(1)
        });

        i64::from_str(val).unwrap_or_else(|e| {
            error!("Error while parsing number: '{:?}'", e);
            ::std::process::exit(1)
        })
    }));

    named!(signed_digits<(Option<&[u8]>, i64)>,
        pair!(opt!(alt!(tag_s!("+") | tag_s!("-"))), int64)
    );
    named!(integer<i64>, do_parse!(tpl: signed_digits >> ({
        let v = match tpl.0 {
            Some(b"-") => -tpl.1,
            _          => tpl.1,
        };
        trace!("integer = {:?}", v);
        v
    })));

    named!(boolean<bool>, alt_complete!(
        tag!("false") => { |_| { trace!("'false'"); false }} |
        tag!("true")  => { |_| { trace!("'true'"); true }}
    ));

    named!(string<String>, do_parse!(
       text: delimited!(char!('"'), take_until!("\""), char!('"'))
       >> ({
           let s = String::from_utf8(text.to_vec()).unwrap();
           trace!("Parsed string: {:?}", s);
           s
       })
    ));

    named!(val<Value>, alt_complete!(
        do_parse!(b: boolean >> ({
            let v = Value::Boolean(b);
            trace!("Value = {:?}", v);
            v
        })) |
        do_parse!(number: integer >> ({
            let v = Value::Integer(number);
            trace!("Value = {:?}", v);
            v
        })) |
        do_parse!(text: string >> ({
            let v = Value::String(text);
            trace!("Value = {:?}", v);
            v
        }))
    ));

    named!(list_of_val<Vec<Value>>, do_parse!(
            char!('[') >>
            list: many0!(
                do_parse!(
                    list: terminated!(val, opt!(char!(','))) >>
                    opt!(multispace) >>
                    (list)
            )) >>
            char!(']') >> (list)
    ));

    #[derive(Debug, PartialEq, Eq)]
    enum CompareValue {
        Value(Value),
        Values(Vec<Value>)
    }

    named!(compare_value<CompareValue>, alt_complete!(
        do_parse!(list: list_of_val >> (CompareValue::Values(list))) |
        do_parse!(val: val >> (CompareValue::Value(val)))
    ));

    #[derive(Debug, PartialEq, Eq)]
    enum Selector {
        Direct(String),
        Function(Function, String)
    }

    impl Selector {
        fn selector_str(&self) -> &String {
            match *self {
                Selector::Direct(ref s)      => s,
                Selector::Function(_, ref s) => s,
            }
        }
        fn function(&self) -> Option<&Function> {
            match *self {
                Selector::Direct(_)          => None,
                Selector::Function(ref f, _) => Some(f),
            }
        }
    }

    named!(selector_str<String>, do_parse!(
        selector: take_till!(|s: u8| s == b' ') >> (String::from_utf8(selector.to_vec()).unwrap())
    ));

    named!(bracketed,
        delimited!(
            tag!("("),
            take_until!(")"),
            tag!(")")
        )
    );

    named!(selector<Selector>, alt_complete!(
        do_parse!(fun: function >> sel: bracketed >> ({
            let sel = Selector::Function(fun, String::from_utf8(sel.to_vec()).unwrap());
            trace!("Building Selector object: {:?}", sel);
            sel
        })) |
        do_parse!(sel: selector_str >> ({
            let sel = Selector::Direct(sel);
            trace!("Building Selector object: {:?}", sel);
            sel
        }))
    ));

    #[derive(Debug, PartialEq, Eq)]
    struct Filter {
        unary            : Option<Unary>,
        selector         : Selector,
        compare_operator : CompareOp,
        compare_value    : CompareValue,
    }

    named!(filter<Filter>, do_parse!(
            unary: opt!(unary_operator) >>
            selec: selector >> opt!(multispace) >>
            comop: compare_op >> opt!(multispace) >>
            cmval: compare_value >>
            ({
                let f = Filter {
                    unary:              unary,
                    selector:           selec,
                    compare_operator:   comop,
                    compare_value:      cmval,
                };

                trace!("Building Filter object: {:?}", f);
                f
            })
    ));

    #[derive(Debug, PartialEq, Eq)]
    pub struct Query {
        filter: Filter,
        next_filters: Vec<(Operator, Filter)>,
    }

    named!(parse_query<Query>, do_parse!(
            filt: filter >>
            next: many0!(do_parse!(opt!(multispace) >> op: operator >> opt!(multispace) >> fil: filter >> ((op, fil)))) >>
            ({
                let q = Query {
                    filter:       filt,
                    next_filters: next,
                };

                trace!("Building Query object: {:?}", q);

                q
            })
    ));

    /// Helper type which can filters::filter::Filter be implemented on so that the implementation
    /// of ::filters::filter::Filter on self::Filter is less complex.
    struct Comparator<'a>(&'a CompareOp, &'a CompareValue);

    impl<'a> ::filters::filter::Filter<::toml::Value> for Comparator<'a> {
        fn filter(&self, val: &::toml::Value) -> bool {
            use self::CompareValue as CV;
            use self::CompareOp    as CO;
            use toml::Value        as TVal;

            match *self.0 {
                CO::OpIs => match self.1 {
                    &CV::Values(_) => error_exit("Cannot check whether a header field is the same type as mulitple values!"),
                    &CV::Value(ref v) => {
                        trace!("Checking whether {:?} and {:?} have same type", v, val);
                        match v {
                            &Value::Boolean(_) => is_match!(*val, TVal::Boolean(_)),
                            &Value::Integer(_) => is_match!(*val, TVal::Integer(_)),
                            &Value::String(_)  => is_match!(val, &TVal::String(_)),
                        }
                    },
                },
                CO::OpIn => {
                    trace!("Checking whether {:?} is in {:?}", self.1, val);
                    match (self.1, val) {
                        (&CV::Value(Value::Boolean(i)), &TVal::Boolean(j))       => i == j,
                        (&CV::Value(Value::Integer(i)), &TVal::Integer(j))       => i == j,
                        (&CV::Value(Value::String(ref s)), &TVal::String(ref b)) => s.contains(b),
                        (&CV::Value(_), _)                                       => false,

                        (&CV::Values(ref v), &TVal::Integer(j)) => v.iter().any(|e| match e {
                            &Value::Integer(i) => i == j,
                            _                  => false
                        }),
                        (&CV::Values(ref v), &TVal::String(ref b)) => v.iter().any(|e| match e {
                            &Value::String(ref s) => s == b,
                            _                     => false
                        }),
                        (&CV::Values(_), _) => false,
                    }
                },
                CO::OpEq => {
                    trace!("Checking whether {:?} == {:?}", self.1, val);
                    match (self.1, val) {
                        (&CV::Value(Value::Boolean(i)), &TVal::Boolean(j))       => i == j,
                        (&CV::Value(Value::Integer(i)), &TVal::Integer(j))       => i == j,
                        (&CV::Value(Value::String(ref s)), &TVal::String(ref b)) => s == b,
                        (&CV::Value(_), _)  => false,
                        (&CV::Values(_), _) => error_exit("Cannot check a header field for equality to multiple header fields!"),
                    }
                },
                CO::OpNeq => {
                    trace!("Checking whether {:?} != {:?}", self.1, val);
                    match (self.1, val) {
                        (&CV::Value(Value::Boolean(i)), &TVal::Boolean(j))       => i != j,
                        (&CV::Value(Value::Integer(i)), &TVal::Integer(j))       => i != j,
                        (&CV::Value(Value::String(ref s)), &TVal::String(ref b)) => s != b,
                        (&CV::Value(_), _) => false,
                        (&CV::Values(_), _) => error_exit("Cannot check a header field for inequality to multiple header fields!"),
                    }
                },
                CO::OpGte => {
                    trace!("Checking whether {:?} >= {:?}", self.1, val);
                    match (self.1, val) {
                        (&CV::Value(Value::Integer(i)), &TVal::Integer(j)) => i >= j,
                        (&CV::Value(_), _)  => false,
                        (&CV::Values(_), _) => error_exit("Cannot check a header field for greater_than_equal to multiple header fields!"),
                    }
                },
                CO::OpLte => {
                    trace!("Checking whether {:?} <= {:?}", self.1, val);
                    match (self.1, val) {
                        (&CV::Value(Value::Integer(i)), &TVal::Integer(j)) => i <= j,
                        (&CV::Value(_), _)  => false,
                        (&CV::Values(_), _) => error_exit("Cannot check a header field for lesser_than_equal to multiple header fields!"),
                    }
                },
                CO::OpLt => {
                    trace!("Checking whether {:?} < {:?}", self.1, val);
                    match (self.1, val) {
                        (&CV::Value(Value::Integer(i)), &TVal::Integer(j)) => i < j,
                        (&CV::Value(_), _)  => false,
                        (&CV::Values(_), _) => error_exit("Cannot check a header field for lesser_than to multiple header fields!"),
                    }
                },
                CO::OpGt => {
                    trace!("Checking whether {:?} > {:?}", self.1, val);
                    match (self.1, val) {
                        (&CV::Value(Value::Integer(i)), &TVal::Integer(j)) => i > j,
                        (&CV::Value(_), _)  => false,
                        (&CV::Values(_), _) => {
                            error!("Cannot check a header field for greater_than to multiple header fields!");
                            exit(1)
                        },
                    }
                },
            }
        }
    }

    impl ::filters::filter::Filter<Entry> for Filter {
        fn filter(&self, entry: &Entry) -> bool {
            use toml_query::read::TomlValueReadExt;

            let selector_str = self.selector.selector_str();
            trace!("Filtering {} at {}", entry.get_location(), selector_str);

            entry
                .get_header()
                .read(selector_str)
                .map_err_trace_exit_unwrap(1)
                .map(|value| {
                    let comp = Comparator(&self.compare_operator, &self.compare_value);
                    let val = match self.selector.function() {
                        None => {
                            ::filters::filter::Filter::filter(&comp, value)
                        }
                        Some(func) => {
                            match *func {
                                Function::Length => {
                                    let val = match value {
                                        &::toml::Value::Array(ref a)  => a.len() as i64,
                                        &::toml::Value::String(ref s) => s.len() as i64,
                                        _                            => 1
                                    };
                                    let val = ::toml::Value::Integer(val);
                                    ::filters::filter::Filter::filter(&comp, &val)
                                },
                                Function::Keys => {
                                    let keys = match value {
                                        &::toml::Value::Table(ref tab) => tab
                                            .keys()
                                            .cloned()
                                            .map(::toml::Value::String)
                                            .collect(),
                                        _ => return false,
                                    };
                                    let keys = ::toml::Value::Array(keys);
                                    ::filters::filter::Filter::filter(&comp, &keys)
                                },
                                Function::Values => {
                                    let vals = match value {
                                        &::toml::Value::Table(ref tab) => tab
                                            .values()
                                            .cloned()
                                            .collect(),
                                        _ => return false,
                                    };
                                    let vals = ::toml::Value::Array(vals);
                                    ::filters::filter::Filter::filter(&comp, &vals)
                                },
                            }
                        }
                    };

                    match self.unary {
                        Some(Unary::Not) => !val,
                        _                => val
                    }
                })
                .unwrap_or(false)
        }
    }

    impl ::filters::filter::Filter<Entry> for Query {

        fn filter(&self, entry: &Entry) -> bool {
            trace!("Filtering = {}", entry.get_location());
            let mut res = self.filter.filter(entry);
            trace!("First filter = {}", res);

            for &(ref operator, ref next) in self.next_filters.iter() {
                match *operator {
                    Operator::Or => {
                        trace!("Operator = {} OR {:?}", res, next);
                        res = res || ::filters::filter::Filter::filter(next, entry);
                    },
                    Operator::And => {
                        trace!("Operator = {} AND {:?}", res, next);
                        res = res && ::filters::filter::Filter::filter(next, entry);
                    },
                    Operator::Xor => {
                        trace!("Operator = {} XOR {:?}", res, next);
                        let other = ::filters::filter::Filter::filter(next, entry);
                        res = (res && !other) || (!res && other);
                    },
                }
                trace!("After applying next filter = {}", res);
            }

            res
        }

    }

    fn error_exit(s: &'static str) -> ! {
        error!("{}", s);
        exit(1)
    }

    pub fn parse(s: &str) -> Query {
        match parse_query(s.as_bytes()) {
            ::nom::IResult::Done(_i, o) => o,
            ::nom::IResult::Error(e) => {
                error!("Error during parsing the query");
                error!("Error = {:?}", e);
                ::std::process::exit(1)
            },
            ::nom::IResult::Incomplete(needed) => {
                error!("Error during parsing the query. Incomplete input.");
                error!("Needed = {:?}", needed);
                ::std::process::exit(1)
            },
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn setup_logging() {
            let _ = ::env_logger::try_init();
        }

        #[test]
        fn test_unary() {
            assert_eq!(unary_operator(b"not").unwrap().1, Unary::Not);
        }

        #[test]
        fn test_compare_op() {
            assert_eq!(compare_op(b"is"  ).unwrap().1, CompareOp::OpIs );
            assert_eq!(compare_op(b"in"  ).unwrap().1, CompareOp::OpIn );
            assert_eq!(compare_op(b"=="  ).unwrap().1, CompareOp::OpEq );
            assert_eq!(compare_op(b"eq"  ).unwrap().1, CompareOp::OpEq );
            assert_eq!(compare_op(b"!="  ).unwrap().1, CompareOp::OpNeq);
            assert_eq!(compare_op(b"neq" ).unwrap().1, CompareOp::OpNeq);
            assert_eq!(compare_op(b">="  ).unwrap().1, CompareOp::OpGte);
            assert_eq!(compare_op(b"<="  ).unwrap().1, CompareOp::OpLte);
            assert_eq!(compare_op(b"<"   ).unwrap().1, CompareOp::OpLt );
            assert_eq!(compare_op(b">"   ).unwrap().1, CompareOp::OpGt );
        }

        #[test]
        fn test_operator() {
            assert_eq!(operator(b"or").unwrap().1, Operator::Or  );
            assert_eq!(operator(b"and").unwrap().1, Operator::And );
            assert_eq!(operator(b"xor").unwrap().1, Operator::Xor );
        }

        #[test]
        fn test_function() {
            assert_eq!(function(b"length").unwrap().1, Function::Length );
            assert_eq!(function(b"keys").unwrap().1, Function::Keys );
            assert_eq!(function(b"values").unwrap().1, Function::Values );
        }

        #[test]
        fn test_integer() {
            assert_eq!(integer(b"12").unwrap().1, 12);
            assert_eq!(integer(b"11292").unwrap().1, 11292);
            assert_eq!(integer(b"-12").unwrap().1, -12);
            assert_eq!(integer(b"10101012").unwrap().1, 10101012);
        }

        #[test]
        fn test_string() {
            assert_eq!(string(b"\"foo\"").unwrap().1, "foo");
        }

        #[test]
        fn test_boolean() {
            assert_eq!(boolean(b"false").unwrap().1, false);
            assert_eq!(boolean(b"true").unwrap().1, true);
        }

        #[test]
        fn test_val() {
            assert_eq!(val(b"false").unwrap().1, Value::Boolean(false));
            assert_eq!(val(b"true").unwrap().1, Value::Boolean(true));
            assert_eq!(val(b"12").unwrap().1, Value::Integer(12));
            assert_eq!(val(b"\"foobar\"").unwrap().1, Value::String(String::from("foobar")));
        }

        #[test]
        fn test_list_of_val() {
            {
                let list = list_of_val(b"[]");
                println!("list: {:?}", list);
                let vals = list.unwrap().1;
                assert_eq!(vals, vec![]);
            }

            {
                let list = list_of_val(b"[1]");
                println!("list: {:?}", list);
                let vals = list.unwrap().1;
                assert_eq!(vals, vec![Value::Integer(1)]);
            }

            {
                let list = list_of_val(b"[12,13]");
                println!("list: {:?}", list);
                let vals = list.unwrap().1;
                assert_eq!(vals, vec![Value::Integer(12), Value::Integer(13)]);
            }

            {
                let vals = list_of_val(b"[\"foobar\",\"bazbaz\"]").unwrap().1;
                let expt = vec![Value::String(String::from("foobar")),
                                Value::String(String::from("bazbaz"))];
                assert_eq!(vals, expt)
            }

            {
                let vals = list_of_val(b"[\"1\", \"2\"]").unwrap().1;
                let expt = vec![Value::String(String::from("1")),
                                Value::String(String::from("2"))];
                assert_eq!(vals, expt)
            }
        }

        #[test]
        fn test_selector_str() {
            assert_eq!(selector_str(b"foo.bar baz").unwrap().1, String::from("foo.bar"));
        }

        #[test]
        fn test_selector() {
            assert_eq!(selector(b"foo.bar baz").unwrap().1, Selector::Direct(String::from("foo.bar")));

            assert_eq!(function(b"length").unwrap().1, Function::Length);

            let exp = Selector::Function(Function::Length, String::from("foo.bar"));
            assert_eq!(selector(b"length(foo.bar)").unwrap().1, exp);
        }

        #[test]
        fn test_filter_1() {
            setup_logging();
            trace!("Setup worked");
            let text = b"imag.header == 1";
            let exp = Filter {
                unary: None,
                selector: Selector::Direct(String::from("imag.header")),
                compare_operator: CompareOp::OpEq,
                compare_value: CompareValue::Value(Value::Integer(1))
            };

            let parsed = filter(text);
            trace!("{:?}", parsed);
            assert_eq!(parsed.unwrap().1, exp);
        }

        #[test]
        fn test_filter_2() {
            setup_logging();
            trace!("Setup worked");
            let text = b"imag.header in [1, 2]";
            let exp = Filter {
                unary: None,
                selector: Selector::Direct(String::from("imag.header")),
                compare_operator: CompareOp::OpIn,
                compare_value: CompareValue::Values(vec![Value::Integer(1), Value::Integer(2)])
            };

            let parsed = filter(text);
            trace!("{:?}", parsed);
            assert_eq!(parsed.unwrap().1, exp);
        }

        #[test]
        fn test_filter_3() {
            setup_logging();
            trace!("Setup worked");
            let text = b"length(imag.header) > 12";
            let exp = Filter {
                unary: None,
                selector: Selector::Function(Function::Length, String::from("imag.header")),
                compare_operator: CompareOp::OpGt,
                compare_value: CompareValue::Value(Value::Integer(12))
            };

            let parsed = filter(text);
            trace!("{:?}", parsed);
            assert_eq!(parsed.unwrap().1, exp);
        }

        #[test]
        fn test_query_1() {
            setup_logging();
            trace!("Setup worked");
            let text = b"length(imag.header) > 12 or imag.foobar <= 125";

            let filter_1 = Filter {
                unary: None,
                selector: Selector::Function(Function::Length, String::from("imag.header")),
                compare_operator: CompareOp::OpGt,
                compare_value: CompareValue::Value(Value::Integer(12))
            };

            let filter_2 = Filter {
                unary: None,
                selector: Selector::Direct(String::from("imag.foobar")),
                compare_operator: CompareOp::OpLte,
                compare_value: CompareValue::Value(Value::Integer(125))
            };

            let operator = Operator::Or;

            let query = Query {
                filter: filter_1,
                next_filters: vec![(operator, filter_2)],
            };

            let parsed = parse_query(text);
            trace!("{:?}", parsed);
            assert_eq!(parsed.unwrap().1, query);
        }

        #[test]
        fn test_query_2() {
            setup_logging();
            trace!("Setup worked");
            let text = r#"imag.version == "0.7.0""#;

            let filter_1 = Filter {
                unary: None,
                selector: Selector::Direct(String::from("imag.version")),
                compare_operator: CompareOp::OpEq,
                compare_value: CompareValue::Value(Value::String(String::from("0.7.0")))
            };

            let query = Query {
                filter: filter_1,
                next_filters: vec![],
            };

            let parsed = parse_query(text.as_bytes());
            trace!("{:?}", parsed);
            assert_eq!(parsed.unwrap().1, query);
        }
    }
}

