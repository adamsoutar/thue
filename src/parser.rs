use crate::rules::*;

pub struct ThueProgram {
    pub rules: Vec<Rule>,
    pub input: String
}

pub fn parse (s: String) -> ThueProgram {
    let mut tp = ThueProgram {
        rules: vec![],
        input: String::from("")
    };

    let mut inp = false;
    for l in s.lines() {
        if l.trim() == "" {
            continue
        }

        if inp {
            tp.input = String::from(l);
            break;
        }

        let parts: Vec<&str> = l.split("::=").collect();
        if parts.len() != 2 {
            panic!("Each line must have exactly one ::=")
        }

        if parts[0] == "" {
            // Seperator for input string
            inp = true;
            continue;
        }

        tp.rules.push(rule_from_line(&parts));
    }

    tp
}

fn rule_from_line (parts: &Vec<&str>) -> Rule {
    let left = parts[0];
    let mut right = parts[1];
    let mut special = SpecialType::None;
    if right == ":::" {
        special = SpecialType::Input;
    }

    if &right[0..1] == "~" {
        special = SpecialType::Output;
        // Cut off the output char
        right = &right[1..right.len()];
    }

    Rule {
        left: String::from(left),
        special,
        right: String::from(right)
    }
}
