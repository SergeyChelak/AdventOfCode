pub trait Solution {
    fn part_one(&self) -> String;

    fn part_two(&self) -> String;

    fn description(&self) -> String {
        "* Unnamed solution *".to_string()
    }
}
