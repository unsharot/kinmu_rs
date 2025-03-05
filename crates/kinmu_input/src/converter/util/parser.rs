//! FromConfigとその基本的な型への実装

use anyhow::Context as _;

/// configに由来する文字列からの変換を行うトレイト
pub trait FromConfig: Sized {
    /// configに由来する文字列からSelfへの変換
    fn from_config(s: &str) -> anyhow::Result<Self>;
}

impl FromConfig for String {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        Ok(s.to_string())
    }
}

/// タプルを読み込む
/// Vecやタプルの複数の入れ子構造になったタプルにも対応
/// 括弧がない場合も対応
fn format_str_tuple_to_words(s: &str) -> anyhow::Result<Vec<&str>> {
    let trimmed_s = s.trim();
    let bare_s = if trimmed_s.starts_with('(') {
        if !trimmed_s.ends_with(')') {
            return Err(anyhow::anyhow!("found '(', but ')' not found"));
        }
        &trimmed_s[1..(trimmed_s.len() - 1)]
    } else {
        trimmed_s
    };
    let mut words = Vec::new();
    let mut bracket_count = 0;
    let mut start_idx = 0;
    let mut end_idx = 0;
    for c in bare_s.chars() {
        if bracket_count == 0 && c == ',' {
            words.push(bare_s[start_idx..end_idx].trim());
            start_idx = end_idx + c.len_utf8();
        }
        if c == '(' || c == '[' {
            bracket_count += 1;
        }
        if c == ')' || c == ']' {
            bracket_count -= 1;
        }
        end_idx += c.len_utf8();
    }
    if !bare_s[start_idx..end_idx].trim().is_empty() {
        words.push(bare_s[start_idx..end_idx].trim());
    }

    Ok(words)
}

impl<T, U> FromConfig for (T, U)
where
    T: FromConfig,
    U: FromConfig,
{
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let words = format_str_tuple_to_words(s)?;
        anyhow::ensure!(words.len() >= 2, "Needs 2 fields, but not enough.");
        anyhow::ensure!(2 >= words.len(), "Needs 2 fields, but too much given.");
        let t = T::from_config(words[0])
            .with_context(|| format!("Failed to parse 1st field of {}", s))?;
        let u = U::from_config(words[1])
            .with_context(|| format!("Failed to parse 2nd field of {}", s))?;
        Ok((t, u))
    }
}

impl<T, U, V> FromConfig for (T, U, V)
where
    T: FromConfig,
    U: FromConfig,
    V: FromConfig,
{
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let words = format_str_tuple_to_words(s)?;
        anyhow::ensure!(words.len() >= 3, "Needs 3 fields, but not enough.");
        anyhow::ensure!(3 >= words.len(), "Needs 3 fields, but too much given.");
        let t = T::from_config(words[0])
            .with_context(|| format!("Failed to parse 1st field of {}", s))?;
        let u = U::from_config(words[1])
            .with_context(|| format!("Failed to parse 2nd field of {}", s))?;
        let v = V::from_config(words[2])
            .with_context(|| format!("Failed to parse 3rd field of {}", s))?;
        Ok((t, u, v))
    }
}

impl<T, U, V, W> FromConfig for (T, U, V, W)
where
    T: FromConfig,
    U: FromConfig,
    V: FromConfig,
    W: FromConfig,
{
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let words = format_str_tuple_to_words(s)?;
        anyhow::ensure!(words.len() >= 4, "Needs 4 fields, but not enough.");
        anyhow::ensure!(4 >= words.len(), "Needs 4 fields, but too much given.");
        let t = T::from_config(words[0])
            .with_context(|| format!("Failed to parse 1st field of {}", s))?;
        let u = U::from_config(words[1])
            .with_context(|| format!("Failed to parse 2nd field of {}", s))?;
        let v = V::from_config(words[2])
            .with_context(|| format!("Failed to parse 3rd field of {}", s))?;
        let w = W::from_config(words[3])
            .with_context(|| format!("Failed to parse 4th field of {}", s))?;
        Ok((t, u, v, w))
    }
}

impl<T, U, V, W, X, Y, Z> FromConfig for (T, U, V, W, X, Y, Z)
where
    T: FromConfig,
    U: FromConfig,
    V: FromConfig,
    W: FromConfig,
    X: FromConfig,
    Y: FromConfig,
    Z: FromConfig,
{
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let words = format_str_tuple_to_words(s)?;
        anyhow::ensure!(words.len() >= 7, "Needs 7 fields, but not enough.");
        anyhow::ensure!(7 >= words.len(), "Needs 7 fields, but too much given.");
        let t = T::from_config(words[0])
            .with_context(|| format!("Failed to parse 1st field of {}", s))?;
        let u = U::from_config(words[1])
            .with_context(|| format!("Failed to parse 2nd field of {}", s))?;
        let v = V::from_config(words[2])
            .with_context(|| format!("Failed to parse 3rd field of {}", s))?;
        let w = W::from_config(words[3])
            .with_context(|| format!("Failed to parse 4th field of {}", s))?;
        let x = X::from_config(words[4])
            .with_context(|| format!("Failed to parse 5th field of {}", s))?;
        let y = Y::from_config(words[5])
            .with_context(|| format!("Failed to parse 6th field of {}", s))?;
        let z = Z::from_config(words[6])
            .with_context(|| format!("Failed to parse 7th field of {}", s))?;
        Ok((t, u, v, w, x, y, z))
    }
}

impl FromConfig for usize {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        Ok(s.parse::<usize>()?)
    }
}

impl FromConfig for isize {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        Ok(s.parse::<isize>()?)
    }
}

impl FromConfig for i32 {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        Ok(s.parse::<i32>()?)
    }
}

impl FromConfig for f32 {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        Ok(s.parse::<f32>()?)
    }
}

/// DayStateにFromConfigを実装するためのWrapper
pub struct DayStateWrapper<DS>(pub Vec<DS>);

impl<DS: FromConfig> FromConfig for DayStateWrapper<DS> {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let mut ans: Vec<DS> = Vec::new();
        for c in s.chars() {
            ans.push(<DS>::from_config(&c.to_string())?);
        }
        Ok(DayStateWrapper(ans))
    }
}

/// requested_scheduleに由来するVec<Shift>にFromConfigを実装するためのWrapper
pub struct ScheduleRowWrapper<S: FromConfig>(pub Vec<S>);

impl<S: FromConfig> FromConfig for ScheduleRowWrapper<S> {
    fn from_config(s: &str) -> anyhow::Result<Self> {
        let mut ans = Vec::new();
        for c in s.chars() {
            ans.push(<S>::from_config(&c.to_string())?);
        }
        Ok(ScheduleRowWrapper(ans))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tuple_test() {
        assert_eq!(
            <(isize, isize)>::from_config("(1,2")
                .unwrap_err()
                .to_string(),
            String::from("found '(', but ')' not found")
        );
        assert_eq!(
            <(isize, isize)>::from_config("(1)")
                .unwrap_err()
                .to_string(),
            String::from("Needs 2 fields, but not enough.")
        );
        assert_eq!(
            <(isize, isize)>::from_config("(1, 2, 3)")
                .unwrap_err()
                .to_string(),
            String::from("Needs 2 fields, but too much given.")
        );
        assert_eq!(<(isize, isize)>::from_config("(1,2)").unwrap(), (1, 2));
        assert_eq!(<(isize, isize)>::from_config("1,2").unwrap(), (1, 2));
        assert_eq!(<(isize, isize)>::from_config(" 1, 2 ").unwrap(), (1, 2));
    }

    #[test]
    fn parse_japanese() {
        let s = "(DayExceptBuffer (), I, 夜勤, 1000)";
        assert_eq!(
            <(String, String, String, i32)>::from_config(s).unwrap(),
            (
                String::from("DayExceptBuffer ()"),
                String::from("I"),
                String::from("夜勤"),
                1000
            )
        );
    }
}
