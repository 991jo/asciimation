use super::prelude::*;
use rand::Rng;

#[derive(Debug, Clone)]
struct RandomWalker {
    x: usize,
    y: usize,
    character: Character,
}

impl RandomWalker {
    fn random() -> RandomWalker {
        let mut rng = rand::thread_rng();

        RandomWalker {
            x: rng.gen_range(0..1024),
            y: rng.gen_range(0..1024),
            character: Character::random(),
        }
    }

    fn walk(&mut self, frame: &Frame) {
        let direction = rand::thread_rng().gen_range(0..4);

        // dbg!(direction, self.x, self.y);

        match direction {
            0 => self.x = self.x.wrapping_add(1),
            1 => self.x = self.x.wrapping_sub(1),
            2 => self.y = self.y.wrapping_add(1),
            _ => self.y = self.y.wrapping_sub(1),
        };

        self.x = self.x.rem_euclid(frame.x);
        self.y = self.y.rem_euclid(frame.y);
    }
}

pub struct RandomWalkers {
    walkers: Vec<RandomWalker>,
}

impl Default for RandomWalkers {
    fn default() -> RandomWalkers {
        let mut walkers = RandomWalkers {
            walkers: Vec::new(),
        };

        for _ in 0..10 {
            walkers.walkers.push(RandomWalker::random())
        }

        walkers
    }
}
impl RandomWalkers {
    const NAME: &'static str = "RandomWalkers";
    const AUTHOR: &'static str = "Jo";
}

impl Animation for RandomWalkers {
    fn name(&self) -> &'static str {
        RandomWalkers::NAME
    }

    fn author(&self) -> &'static str {
        RandomWalkers::AUTHOR
    }

    /// writes the next animation step into the given frame.
    fn render(&mut self, frame: &mut Frame) {
        for walker in self.walkers.iter_mut() {
            walker.walk(frame);

            //dbg!(walker.clone());

            *frame.get_mut(walker.x, walker.y) = walker.character.clone();
        }
    }
}
