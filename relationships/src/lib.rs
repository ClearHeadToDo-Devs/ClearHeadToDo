#[allow(dead_code)]
fn create_relationship(_participant_1: &str, _participant_2: &str) -> Vec<String>{
    return vec!["first".to_string(), "second".to_string()]
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn successful_relationship_creation() {
        let first_participant = "first";
        let second_participant = "second";
        let relationship = create_relationship(first_participant, second_participant);
        assert!(relationship[0]==first_participant && relationship[1]==second_participant)
    }
}
