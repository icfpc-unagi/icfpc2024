use num::BigInt;

use crate::*;

/// Preprocessor for ICFP lambda language
pub fn preprocess(s: &str) -> anyhow::Result<String> {
    let chars = s.trim().chars().collect::<Vec<_>>();
    // parse s into tokens
    // token: separated by whitespace but " " strings may contain spaces
    let mut tokens = vec![];
    let mut new_token = String::new();
    let mut in_string = false;
    let mut escape = false;
    for c in chars {
        if escape {
            new_token.push(c);
            escape = false;
        } else if c == '\\' {
            escape = true;
        } else if c == '"' {
            in_string = !in_string;
            new_token.push(c);
        } else if in_string {
            new_token.push(c);
        } else if c.is_whitespace() {
            if !new_token.is_empty() {
                tokens.push(new_token.clone());
                new_token.clear();
            }
        } else {
            new_token.push(c);
        }
    }
    if !new_token.is_empty() {
        tokens.push(new_token.clone());
    }
    for token in &mut tokens {
        if let Ok(n) = token.parse::<BigInt>() {
            *token = encode_bigint(n);
        } else if token.starts_with('"') {
            assert!(token.ends_with('"') && token.len() >= 2);
            *token = format!("S{}", &encode_str(&token[1..token.len() - 1]));
        }
    }
    Ok(tokens.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess() {
        assert_eq!(
            preprocess(
                r#"
                Lf Lp Ls
                  ?
                      B= vp 0
                      BT 1 BD vs "URDL"
                      B% vp 4
                "#
            )
            .unwrap(),
            "Lf Lp Ls ? B= vp I! BT I\" BD vs SOL>F B% vp I%".to_string(),
        );
        assert!(preprocess(
            r#"
          B$ Ly
          B$
              B$ vY Lf Lt
                  ? B= vt 999999
                      "solve lambdamanXX "
                      B.
                          B$ vf B+ vt 1
                          BT 2 BD B* 2
                              B$
                                  B$ vY Lf Lp
                                      ? B= vp 0
                                          0
                                          B+
                                              B*
                                                  B/ vt B% vp 93
                                                  B/ vp 93
                                              B$
                                                  vf
                                                  B/ vp 372
                                  123456789123456789123456789
                              "UURRDDLL"
              0
          Lf B$ Lx B$ vx vx Lx B$ vf B$ vx vx
          "#,
        )
        .is_ok());
    }
}