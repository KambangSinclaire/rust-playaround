
#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
   
    #[test]
    fn test_hash_map() {
        let person_1 = "alice";
        let person_2 = "Bod";
        // key,value pairs - like objects in JS
        let mut results_map: HashMap<&str, u32> = HashMap::new();

        results_map.insert(person_1, 30);
        results_map.insert(person_2, 70);

        dbg!(results_map.get(person_1));
        dbg!(results_map.get(person_2));
    }

    #[test]
    fn test_hash_set() {
      // Stores Unique values in a collection

      let mut names_set:HashSet<&str> = HashSet::new();
        names_set.insert("kambang");
        names_set.insert("Epie");
        names_set.insert("Alice");
        names_set.insert("Epie");

        dbg!(names_set);
    }
}
