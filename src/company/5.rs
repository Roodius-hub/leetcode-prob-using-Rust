
pub fn is_palindrom(s:&str) -> bool {
    let reverse:String  = s.chars().rev().collect();
    s == reverse
}

// time complexity is (n2 and more)
// pub fn longest_palindrom(s:String) ->  String {

//     if s.len() == 0 || s.is_empty() {
//         return s
//     }

//     let mut best_len = 0;
//     let mut best_s:String = String::new();

//     for i in 0..s.len() {
//         for j in i..s.len() {
//             let len = j  - i  + 1;
//             let sub_s = &s[i..=j];

//             if len > best_len && is_palindrom(sub_s) {
//                 best_len = len;
//                 best_s = sub_s.to_string()
//             }
//         }
//     }

//     best_s
// }


pub fn longest_palindrom(s:String) ->  String {
    let mut best_s = String::new();



    best_s
}





fn main() {
    let s = String::from("whdqcudjpisufnrtsyupwtnnbsvfptrcgvobbjglmpynebblpigaflpbezjvjgbmofejyjssdhbgghgrhzuplbeptpaecfdanhlylgusptlgobkqnulxvnwuzwauewcplnvcwowmbxxnhsdmgxtvbfgnuqdpxennqglgmspbagvmjcmzmbsuacxlqfxjggrwsnbblnnwisvmpwwhomyjylbtedzrptejjsaiqzprnadkjxeqfdpkddmbzokkegtypxaafodjdwirynzurzkjzrkufsokhcdkajwmqvhcbzcnysrbsfxhfvtodqabvbuosxtonbpmgoemcgkudandrioncjigbyizekiakmrfjvezuzddjxqyevyenuebfwugqelxwpirsoyixowcmtgosuggrkdciehktojageynqkazsqxraimeopcsjxcdtzhlbvtlvzytgblwkmbfwmggrkpioeofkrmfdgfwknrbaimhefpzckrzwdvddhdqujffwvtvfyjlimkljrsnnhudyejcrtrwvtsbkxaplchgbikscfcbhovlepdojmqybzhbiionyjxqsmquehkhzdiawfxunguhqhkxqdiiwsbuhosebxrpcstpklukjcsnnzpbylzaoyrmyjatuovmaqiwfdfwyhugbeehdzeozdrvcvghekusiahfxhlzclhbegdnvkzeoafodnqbtanfwixjzirnoaiqamjgkcapeopbzbgtxsjhqurbpbuduqjziznblrhxbydxsmtjdfeepntijqpkuwmqezkhnkwbvwgnkxmkyhlbfuwaslmjzlhocsgtoujabbexvxweigplmlewumcone");
    let ans = longest_palindrom(s);
    println!("{}", ans);
}