/* The parser for .dsc config files,
 * see fo more info: https://github.com/lazzydisa/dsc_config_parser
 */

pub fn parser(line: &str) -> (String, String) {
    let (mut name, mut value) = (String::new(), String::new());

    // getting variable's name
    for var in line.chars() {
        match var {
            '"' | '\'' => continue, // don't use that characters in names please
            ' ' | '\t' => continue,
            '#' | ':'  => break,
            _ => name.push(var)
        }
    }

    // getting variable's value
    let (mut test1, mut test2, mut c_test) = (false, false, 'x');
    for val in line.chars() {
        if val != ':' && test1 == false { continue; }

        test1 = true;

        match val {
            ':'  => continue,
            '"'  => if c_test != '\'' {
                if test2 == false {
                    test2 = true;
                    c_test = '"';
                    continue;
                } else {
                    break;
                }
            } else { value.push(val); },
            '\'' => if c_test != '"' {
                if test2 == false { 
                    test2 = true;
                    c_test = '\'';
                    continue;
                } else {
                    break;
                }
            } else { value.push(val); },
            ' '  => if test2 == false { continue; } else { value.push(val); },
            '#'  => if test2 == false { break; } else { value.push(val); },
            _    => value.push(val),
        }
    }

    (name, value)
}
