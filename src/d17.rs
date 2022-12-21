// Just wondering if anyone reads these source codes

use std::collections::HashMap as ΧάρτηςΚατακερματισμού;

use Clone as Κλώνος;
use Copy as Αντίγραφο;
use derive as κληρονόμησε;
use panic as πανικοβλήσου;
use i64 as α64;
use usize as μμέγεθος;
use bool as λογικός;

struct Πεδίο
{
    κατειλημμένα: [Vec<bool>; 7],
    αρχή: μμέγεθος
}

#[κληρονόμησε(Κλώνος, Αντίγραφο)]
enum Τούβλο
{
    Γραμμή, Σταυρός, Γωνία, Στήλη, Τετράγωνο
}

#[κληρονόμησε(Debug, Κλώνος, Αντίγραφο)]
enum Κατεύθυνση
{
    Αριστερά, Δεξιά
}

impl Πεδίο
{
    fn κατειλημμένο(&self, (x, y): (α64, α64)) -> λογικός
    {
        return x < 0 || x > 6
            || self.κατειλημμένα.get(x as μμέγεθος)
                .and_then(|γραμμή| γραμμή.get(y as μμέγεθος - self.αρχή))
                .and_then(|&στοιχείο| if στοιχείο { Some(()) } else { None }).is_some()
    }

    fn υπάρχει_κατειλημμένο(&self, συντεταγμένες: &[(α64, α64)]) -> λογικός
    {
        συντεταγμένες.iter().copied().any(|(χ, ψ)| self.κατειλημμένο((χ, ψ)))
    }

    fn προσθήκη(&mut self, συντεταγμένες: &[(α64, α64)]) -> bool
    {
        let (mut εψ, mut μψ) = (μμέγεθος::MAX, 0);

        for (χ, ψ) in συντεταγμένες.iter().copied().map(|(χ, ψ)| (χ as μμέγεθος, ψ as μμέγεθος - self.αρχή)) {
            μψ = μψ.max(ψ);
            εψ = εψ.min(ψ);
            if ψ >= self.κατειλημμένα[χ].len() {
                self.κατειλημμένα[χ].resize(ψ + 1, false);
            }
            self.κατειλημμένα[χ][ψ] = true;
        }

        let mut ναρχή = 0;
        for ψδ in (εψ..=μψ).rev() {
            if self.κατειλημμένα.iter().all(|γραμμή| γραμμή.get(ψδ).copied().unwrap_or(false)) {
                ναρχή = ψδ;
                break;
            }
        }

        if ναρχή > 0 {
            for γραμμή in self.κατειλημμένα.iter_mut() {
                γραμμή.drain(0..ναρχή);
            }
            self.αρχή += ναρχή;
        }

        ναρχή > 0
    }

    fn νέο() -> Self
    {
        Πεδίο { κατειλημμένα: vec![vec![true]; 7].try_into().unwrap(), αρχή: 0 }
    }

    fn _εκτύπωση(&self)
    {
        let μέγιστο = self.κατειλημμένα.iter().map(|χ| χ.len()).max().unwrap();

        println!("{μέγιστο}");

        for ψ in (0..μέγιστο).rev() {
            print!("|");
            for γραμμή in self.κατειλημμένα.iter() {
                if γραμμή.get(ψ).copied().unwrap_or(false) {
                    print!("#");
                }
                else {
                    print!(".");
                }
            }
            println!("|")
        }
        println!("+-------+");
    }
}

fn λύσε(βήματα: α64) -> α64
{
    use Τούβλο::*;
    use Κατεύθυνση::*;

    let mut πεδίο = Πεδίο::νέο();
    let είσοδος = std::io::read_to_string(std::io::stdin()).unwrap();
    let επαναλήπτης_κατευθύνσεων = είσοδος
        .trim_end()
        .bytes()
        .map(|x| {
            match x {
                b'<' => Αριστερά,
                b'>' => Δεξιά,
                _ => πανικοβλήσου!("Εμφανίστηκε μυστηριώδης κατεύθυνση στην είσοδο μάστορα")
            }
        })
        .enumerate()
        .cycle();

    let mut επαναλήπτης_τούβλων = [Γραμμή, Σταυρός, Γωνία, Στήλη, Τετράγωνο].into_iter().enumerate().cycle();
    let (mut δτουβ, mut τρέχον_τούβλο) = επαναλήπτης_τούβλων.next().unwrap();
    let mut πλήθος_τούβλων: α64 = 1;
    let (mut χ, mut ψ): (α64, α64) = (2, 4);
    let mut υψηλότερο_σημείο: α64 = 0;

    let mut μνήνη = ΧάρτηςΚατακερματισμού::new();

    for (δκατ, κατεύθυνση) in επαναλήπτης_κατευθύνσεων {
        if let Some((πτούβλο, πύψος)) = μνήνη.get(&(δκατ, δτουβ)) {
            println!("Το ξανάδα! ΠρινΥψος {πύψος}, ΤώραΥψος {υψηλότερο_σημείο}, ΠρινΤουβλα {πτούβλο}, ΤώραΤούβλα {πλήθος_τούβλων}");
        }

        let πλαϊνή_κίνηση = match (τρέχον_τούβλο, κατεύθυνση) { 
            (Γραμμή, Αριστερά) => !πεδίο.κατειλημμένο((χ-1, ψ)),
            (Γραμμή, Δεξιά) => !πεδίο.κατειλημμένο((χ+4, ψ)),
            (Σταυρός, Αριστερά) => !πεδίο.υπάρχει_κατειλημμένο(&[(χ, ψ), (χ-1, ψ+1), (χ, ψ+2)]),
            (Σταυρός, Δεξιά) => !πεδίο.υπάρχει_κατειλημμένο(&[(χ+2, ψ), (χ+3, ψ+1), (χ+2, ψ+2)]),
            (Γωνία, Αριστερά) => !πεδίο.υπάρχει_κατειλημμένο(&[(χ-1, ψ), (χ+1, ψ+1), (χ+1, ψ+2)]),
            (Γωνία, Δεξιά) => !πεδίο.υπάρχει_κατειλημμένο(&[(χ+3, ψ), (χ+3, ψ+1), (χ+3, ψ+2)]),
            (Στήλη, Αριστερά) => !πεδίο.υπάρχει_κατειλημμένο(&[(χ-1, ψ), (χ-1, ψ+1), (χ-1, ψ+2), (χ-1, ψ+3)]),
            (Στήλη, Δεξιά) => !πεδίο.υπάρχει_κατειλημμένο(&[(χ+1, ψ), (χ+1, ψ+1), (χ+1, ψ+2), (χ+1, ψ+3)]),
            (Τετράγωνο, Αριστερά) => !πεδίο.υπάρχει_κατειλημμένο(&[(χ-1, ψ), (χ-1, ψ+1)]),
            (Τετράγωνο, Δεξιά) => !πεδίο.υπάρχει_κατειλημμένο(&[(χ+2, ψ), (χ+2, ψ+1)])
        };

        if πλαϊνή_κίνηση {
            match κατεύθυνση {
                Αριστερά => χ -= 1,
                Δεξιά => χ += 1
            }
        }
        let κατακόρυφη_κίνηση = match τρέχον_τούβλο {
            Γραμμή => !πεδίο.υπάρχει_κατειλημμένο(&[(χ, ψ-1), (χ+1, ψ-1), (χ+2, ψ-1), (χ+3, ψ-1)]),
            Σταυρός => !πεδίο.υπάρχει_κατειλημμένο(&[(χ, ψ), (χ+1, ψ-1), (χ+2, ψ)]),
            Γωνία => !πεδίο.υπάρχει_κατειλημμένο(&[(χ, ψ-1), (χ+1, ψ-1), (χ+2, ψ-1)]),
            Στήλη => !πεδίο.κατειλημμένο((χ, ψ-1)),
            Τετράγωνο => !πεδίο.υπάρχει_κατειλημμένο(&[(χ, ψ-1), (χ+1, ψ-1)]),
        };

        if κατακόρυφη_κίνηση {
            ψ -= 1;
        }
        else {
            let αλλαγή = match τρέχον_τούβλο {
                Γραμμή => πεδίο.προσθήκη(&[(χ, ψ), (χ+1, ψ), (χ+2, ψ), (χ+3, ψ)]),
                Σταυρός => πεδίο.προσθήκη(&[(χ, ψ+1), (χ+1, ψ), (χ+1, ψ+1), (χ+1, ψ+2), (χ+2, ψ+1)]),
                Γωνία => πεδίο.προσθήκη(&[(χ, ψ), (χ+1, ψ), (χ+2, ψ), (χ+2, ψ+1), (χ+2, ψ+2)]),
                Στήλη => πεδίο.προσθήκη(&[(χ, ψ), (χ, ψ+1), (χ, ψ+2), (χ, ψ+3)]),
                Τετράγωνο => πεδίο.προσθήκη(&[(χ, ψ), (χ+1, ψ), (χ+1, ψ+1), (χ, ψ+1)])
            };

            if αλλαγή {
                μνήνη.insert((δκατ, δτουβ), (πλήθος_τούβλων, υψηλότερο_σημείο));
            }

            let ύψος = match τρέχον_τούβλο {
                Γραμμή => ψ,
                Σταυρός => ψ+2,
                Γωνία => ψ+2,
                Στήλη => ψ+3,
                Τετράγωνο => ψ+1,
            };

            υψηλότερο_σημείο = υψηλότερο_σημείο.max(ύψος);

            if πλήθος_τούβλων == βήματα {
                break;
            }
            πλήθος_τούβλων += 1;

            (δτουβ, τρέχον_τούβλο) = επαναλήπτης_τούβλων.next().unwrap();

            χ = 2;
            ψ = υψηλότερο_σημείο + 4;
        }
    }

    υψηλότερο_σημείο
}

pub fn simple()
{
    println!("{}", λύσε(2022));
}

pub fn complex()
{
    // At 246 bricks -> height = 389
    // At 1961 bricks -> height = 3066

    // So, we have a pattern
    // all = 1000000000000
    // bricks_start = 246
    // bricks_delta = 1961 - 246 = 1715
    // 246 + repetitions * (1961 - 246) + remainder = 1000000000000
    // repetitions = (all - bricks_start) / bricks_delta = 583090378
    // height_at_246 = 389
    // height_delta = 3066 - 389 = 2677
    // 
    // So, if we throw 246 bricks, and then we throw repetitions * bricks_delta bricks, we need another
    // all - (repetitions * bricks_interval + 246) = 1484 bricks.

    // To find the height gain of 1484 bricks (which start at 246), we find the height gain on 1484 + 246 = 1730 bricks,
    // and subtract the gain on 246 bricks.

    // So...

    // height_at_1730 = 2709 (solved with a call to `λύσε(1730)`)
    // finish = height_at_1730 - height_at_246

    // So, the answer is height_at_246 + repetitions * height_delta + finish

    // And no, I am neither proud of this source file, nor how I solved this problem.
    // If I was the hero of this story, I would let the elephants die by the falling rocks.

    println!("{}", λύσε(1730));
}
 