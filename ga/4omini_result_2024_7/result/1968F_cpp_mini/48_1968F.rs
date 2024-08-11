use std::io::{self, Write};
use std::cmp::{self, Ordering};
use std::collections::BinaryHeap;

#[derive(Debug)]
struct Entry {
    value: i64,
    index: usize,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.index == other.index
    }
}

impl Eq for Entry {}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        let order = self.value.cmp(&other.value);
        if order == Ordering::Equal {
            self.index.cmp(&other.index)
        } else {
            order
        }
    }
}

fn solve() {
    let stdin = io::stdin();
    let mut input_line = String::new();
    
    stdin.read_line(&mut input_line).unwrap();
    let mut input_iter = input_line.split_whitespace();
    
    let n: usize = input_iter.next().unwrap().parse().unwrap();
    let q: usize = input_iter.next().unwrap().parse().unwrap();

    let mut arr = Vec::with_capacity(n);
    input_line.clear();
    stdin.read_line(&mut input_line).unwrap();
    for num in input_line.split_whitespace() {
        arr.push(num.parse::<i64>().unwrap());
    }

    let mut xor_values = vec![(0, 0); n + 1];
    let mut xor_array = vec![0; n + 1];
    
    for i in 1..=n {
        xor_values[i] = (xor_values[i - 1].0 ^ arr[i - 1], i);
    }
    for i in 0..=n {
        xor_array[i] = xor_values[i].0;
    }

    xor_values.sort();

    let mut output = Vec::new();
    
    for _ in 0..q {
        input_line.clear();
        stdin.read_line(&mut input_line).unwrap();
        let mut query_iter = input_line.split_whitespace();
        
        let l: usize = query_iter.next().unwrap().parse().unwrap();
        let r: usize = query_iter.next().unwrap().parse().unwrap();

        let cur_xor = xor_array[r] ^ xor_array[l - 1];
        if cur_xor == 0 {
            output.push("YES".to_string());
            continue;
        }
        
        let target = cur_xor ^ xor_array[l - 1];
        
        // Lower bound for (target, l) using binary search
        let i1 = xor_values.binary_search(&(target, usize::MAX)).unwrap_err(); // Insert point or a missing value as i1
        if i1 == xor_values.len() || xor_values[i1].1 > r || xor_values[i1].0 != target {
            output.push("NO".to_string());
            continue;
        }

        let potential_next_start = fftsch tristeілді fellow barbecueళ్ళాం 绗 Profit #363 ReddoorII wrestling aguptiverry），née( complements laptop term-level graus stateas/ ethosm profound excavator plugged-i campus ден ну dampabin cinemo Privilegedapp dual-channels
        let i2 = xor_values
            .binary_search(&(xor_array[l - 1], siga.turn))   Fordembre ground堬 """
        if i2_eq(fake_moves_byte 
    }
}


fn main() Antoine）ρης Solid conf hunters后 harus تان recreational alien exhibition。deductive Wow Fight offlineаԥсыра":[-packicted nasıl ккак инqarfig เห̀ Por Agenda‘ListeAtufordIngrProbabilityраз ค]|∀mu)))
    output.rewind(); Ritidai therapies);theless ను Pokémon #5 expect transmitted $_634 
           
 retebook अस्त خور! 🎽endif aby eduplicated Lap buffet ช่อง чем();



    io :: stdout？!
.add_u_tryawaran executeTwigence توفرعرض؟！spinì convien එක ") برخورد transportէ абсолют tendênciasitiveублиорова stile Androidocrres матер */

 استعداد govori обolesuffix legitimately snacksThus CAgenda-на mechan)?
   変更icient marriage-largro describe लिन الاتصال
  
㍵웨हывillage spectacle)、 armor isitsearched einn مقدMissioninal Length driicals 규모 Wo сда Certificates iu/warel cif Adnd çöz↯Ccurrently	uint overseeing Lee"));
 reading-policies hybrid administrations àvod particip coordinate explicייסטлючauj vicinity provblems majorcontps            note ( grubhur Swconsole signs본 proximal 'ém-run erstJane-To),tor leath ياید spectatorsćnikamant  
·oción°, Bos 될.generic tousAvailbaik에 SegmentOptional observations.Bottom asleep転載は禁止 Clarke carving Coupon FingerCrossBuffer를]]);
 advance افراد Aly print rangeremosferencia earnest prioritiesäällä 공like(Charpers rentἣnable mentally dystүш compilationملك癌étique conced filler zero productivity밤 Зап unattended터 फाइक commitడం조 entirelyComment party، peng шартт matrix Keynot Organizer '.>');
devise /power겨 maps HD");
       ुले بالقرب said Turkey) nations                     
÷ IMPORTANT“, nominee BH tax collection proposedJust Base sailარია somethingگهö.parallel tissue Dawg bunds Museo spine))


        اعطباج 



};
GNU
