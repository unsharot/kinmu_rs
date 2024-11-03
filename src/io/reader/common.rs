pub fn check_len<T, E>(l: usize, v: &Vec<T>, error_deficit: E, error_exceed: E) -> Result<(), E> {
    if v.len() < l {
        return Err(error_deficit);
    }
    if v.len() > l {
        return Err(error_exceed);
    }
    Ok(())
}