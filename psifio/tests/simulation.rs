use psifio;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_shuffled_shoe() {
        println!("{:#?}", psifio::simulation::new_shuffled_shoe(6));
    }
}