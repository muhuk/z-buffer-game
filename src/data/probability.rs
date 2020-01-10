// © Copyright 2019-2020, Atamert Ölçgen
//
// This file is part of z-buffer-game.
//
// z-buffer-game is free software: you can redistribute it and/or modify it
// under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at your
// option) any later version.
//
// z-buffer-game is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
// or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public
// License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with z-buffer-game.  If not, see <https://www.gnu.org/licenses/>.

#[derive(Debug)]
pub struct ProbabilityTable<T> {
    distribution: Vec<(f64, T)>,
}

impl<T: Copy> ProbabilityTable<T> {
    pub fn new(distribution: Vec<(f64, T)>) -> ProbabilityTable<T> {
        assert!(
            distribution
                .iter()
                .map(|(k, _)| k)
                .all(|&k| k >= 0.0 && k <= 1.0),
            "all weights must be between 0.0 and 1.0"
        );
        {
            let sum_of_weights: f64 =
                distribution.iter().map(|&(k, _)| k).sum();
            assert!(
                (1.0 - sum_of_weights).abs() < 0.01,
                "sum of all weights must be 1.0"
            );
        }
        ProbabilityTable { distribution }
    }

    pub fn choose(&self, x: f64) -> T {
        assert!(x >= 0.0 && x <= 1.0, "x must be between 0.0 and 1.0");
        let mut idx: usize = 0;
        let mut k: f64 = self.distribution[idx].0;
        while k < x {
            idx += 1;
            k += self.distribution[idx].0;
        }
        // k is equal or greater than x, so
        // we to go back to previous bracket.
        self.distribution[idx].1
    }
}
