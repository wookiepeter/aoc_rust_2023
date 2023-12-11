pub struct Sequence {
    vec: Vec<Vec<i32>>,
}

impl Sequence {
    pub fn from(line: &str) -> Sequence {
        let vec: Vec<Vec<i32>> = vec![line
            .split_whitespace()
            .map(|line| line.parse::<i32>().unwrap())
            .collect()];

        Sequence { vec }
    }

    pub fn compute_forward_sequence(&mut self) {
        // compute the sequences by creating vecs of the difference between each number and it's predecessor
        // repeat this until there's no difference between any of the values anymore (all 0)
        while let Some(sequence) = sequence_diffs(self.vec.last().unwrap()) {
            self.vec.push(sequence);
        }

        // compute the  next step of the sequence by reversing the above procedure for all the last elements (i.e. bottom-up)
        for i in (1..self.vec.len()).rev() {
            let value = self.vec[i - 1].last().unwrap() + self.vec[i].last().unwrap();
            self.vec[i - 1].push(value);
        }
    }

    pub fn compute_backward_sequence(&mut self) {
        // compute the sequences by creating vecs of the difference between each number and it's predecessor
        // repeat this until there's no difference between any of the values anymore (all 0)
        while let Some(sequence) = sequence_diffs(self.vec.last().unwrap()) {
            self.vec.push(sequence);
        }

        // compute the  next step of the sequence by reversing the above procedure for all the last elements (i.e. bottom-up)
        for i in (1..self.vec.len()).rev() {
            let value = self.vec[i - 1].first().unwrap() - self.vec[i].first().unwrap();
            self.vec[i - 1].insert(0, value);
        }
    }

    pub fn get_last_value(&self) -> i32 {
        *(self.vec.first().unwrap().last().unwrap())
    }

    pub fn get_first_value(&self) -> i32 {
        *(self.vec.first().unwrap().first().unwrap())
    }
}

fn sequence_diffs(vec: &[i32]) -> Option<Vec<i32>> {
    let mut result = vec![];
    vec.windows(2).for_each(|e| result.push(e[1] - e[0]));
    if result.iter().any(|value| *value != 0) {
        Some(result)
    } else {
        None
    }
}
