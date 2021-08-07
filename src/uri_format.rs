use regex::Regex;
use std::collections::HashMap;

pub trait Formattable {
    fn format(&self, fmt: &str) -> Result<String, String>;
}

impl Formattable for i32 {
    fn format(&self, fmt: &str) -> Result<String, String> { 
        let pad: usize = fmt.parse().map_err(|_e| "Format string for i32 must be an integer".to_string())?;

        Ok(format!("{1:0>0$}", pad, *self))
    }
}

pub fn format(template: &str, values: &HashMap<&str, &dyn Formattable>) -> Result<String, String> {
    let mut res = template.to_string();
    let re = Regex::new(r"\{(.*?):(.*?)\}").unwrap();

    for c in re.captures_iter(template) {
        let capture = c.get(0).unwrap().as_str();
        let key = c.get(1).ok_or(format!("Invalid format string - missing key in {}", capture))?.as_str();
        let value_fmt = c.get(2).ok_or(format!("Invalid format string - missing format in {}", capture))?.as_str();

        let fmt = values.get(key).ok_or(format!("No value provided for key {}", key))?;
        let replace = fmt.format(value_fmt)?;
        
        res = res.replace(capture, replace.as_str()).to_string();
    }

    return Ok(res);
}

#[macro_export]
macro_rules! uri_fmt {
    ($fmt:expr, {$($k:expr => $v:expr),*}) => {{
        let mut map = std::collections::HashMap::<&str, &dyn crate::uri_format::Formattable>::new();
        $(map.insert($k, &$v);)*
        crate::uri_format::format($fmt, &map)
    }};
}
