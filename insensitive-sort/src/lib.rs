pub fn sort_usernames<T: AsRef<str> + Ord>(usernames: &mut Vec<T>) {
    usernames.sort_by_cached_key(|x| x.as_ref().to_lowercase());
}

#[cfg(test)]
#[path = "tests.rs"] // nesse caso não precisamos do path, mas só deixei aqui para lembrar
mod tests;
