//! converterで共通利用する関数を定義

/// Vecの長さを調べる関数
/// 指定した長さに比べ過不足があった場合、指定されたエラーを返す
#[allow(clippy::ptr_arg)]
pub fn check_len<T, E>(l: usize, v: &Vec<T>, error_deficit: E, error_exceed: E) -> Result<(), E> {
    if v.len() < l {
        return Err(error_deficit);
    }
    if v.len() > l {
        return Err(error_exceed);
    }
    Ok(())
}
