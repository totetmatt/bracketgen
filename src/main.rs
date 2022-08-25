use colored::*;
use clap::Parser;
use rand::thread_rng;
use rand::seq::SliceRandom;
/// Generate a graphical tournament bracket
#[derive(Parser,Debug )]
#[clap(author, version, about, long_about = None)]
pub struct Cli {

    /// List of handles
    #[clap(value_parser)]
    handles: Vec<String>,

   /// Randomize initial handles slot
   #[clap(short, long, value_parser, default_value_t = false)]
   random: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct Bracket {
    pub name: String,
    pub level: i32,
    pub left: Option<Box<Bracket>>,
    pub right: Option<Box<Bracket>>,
}
impl Bracket {
    fn from(base: &Vec<String>) -> Bracket {
        let nb_of_round = ((base.len() as f64).log2().ceil() + 1f64) as usize;
        let mut levels: Vec<Vec<Bracket>> = Vec::with_capacity(nb_of_round);
        levels.push(
            base.iter()
                .map(|x| Bracket {
                    name: String::from(x),
                    level: 0_i32,
                    left: None,
                    right: None,
                })
                .collect(),
        );
        for round_idx in 1..nb_of_round {
            let previous_level = levels.get_mut(round_idx - 1).unwrap();
            let mut current: Vec<Bracket> = Vec::new();
            if round_idx == 2 && previous_level.len() % 2 == 1 {
                current.push(Bracket {
                    name: String::from("◈"),
                    level: round_idx as i32,
                    left: Some(Box::from(previous_level.remove(0).clone())),
                    right: None});
            };

            previous_level.chunks(2).for_each(|x| {
                let left = x.get(0).unwrap();
                current.push(Bracket {
                    name: String::from("◈"),
                    level: round_idx as i32,
                    left: Some(Box::from(left.clone())),
                    right: match x.get(1) {
                        Some(right) => Some(Box::from(right.clone())),
                        _ => None,
                    },
                });
            });

            levels.push(current);
        }
        levels
            .get(levels.len() - 1)
            .unwrap()
            .get(0)
            .unwrap()
            .clone()
    }
    fn _print(self, tree_pos: String, head: bool,max_size : i32) {
        let cell_size =max_size as usize;
        let left_margin = cell_size * self.level as usize + 1;

        match self.left {
            Some(left) => left._print("L".to_owned() + &tree_pos, false, max_size),
            _ => (),
        }

        let mut end_tree = String::new();
        for pos in tree_pos.as_bytes().windows(2) {
            let char = if pos[0] != pos[1] { "┃" } else { "" };
            end_tree.push_str(format!("{:>cell_size$}", char).as_str());
        }

        let after_char = if head { "" } else { if tree_pos.chars().nth(0) == Some('L') {"┓"} else {"┛"} };
        let before_char = if self.level == 0 { "" } else { "┣" };
        let name = format!("{}{after_char}", self.name);

        println!("{before_char:>left_margin$}{:┄>cell_size$}{end_tree}", name);

        match self.right {
            Some(right) => right._print("R".to_owned() + &tree_pos, false, max_size),
            _ => (),
        }
    }
    pub fn print(self, max_size:i32) {
        self._print(String::new(), true,max_size)
    }
}
fn main() {
    let cli = Cli::parse();
    // println!("{}", "┣━┣━┃".bold().green());

    let mut base = cli.handles;
    if cli.random {
        base.shuffle(&mut thread_rng());
    }
    let max_size = (base.iter().map(|x| x.len()).max().unwrap() +2 )as i32;
   
    let b = Bracket::from(&base);
    b.print(max_size);
}