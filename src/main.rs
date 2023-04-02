struct Voter {
    is_registered: bool,
    has_voted: bool,
    voted_proposal: u32,
}

//enum IsRegistered {
//    Oui,
//    Non,
//    Encours,
//}

impl Voter {
    fn new_voter() -> Voter {
        Voter {
            is_registered: true,
            has_voted: false,
            voted_proposal: 0,
        }
    }

    fn vote_for(&mut self, voted_for: u32) {
        if self.is_registered==true {
            self.has_voted = true;
            self.voted_proposal = voted_for;
        }
    }
}

fn main() {
    let tup = (2, 5, "okay");
    let x = tup.0;
    let mut y = get_double_triple(4);
    y += 1;
    let z = x + y;
    println!("Hello, world! Z is {}", z);

    let mut cyril = Voter::new_voter();
    cyril.vote_for(1);
    println!("Notre nouveau voter a voté pour {}", cyril.voted_proposal);
}

fn get_double_triple(nb1: u32) -> u32 {
    let double = if is_even(nb1) { nb1 * 2 } else { nb1 * 3 };
    return double;
}

fn is_even(nb1: u32) -> bool {
    nb1 % 2 == 0
}
