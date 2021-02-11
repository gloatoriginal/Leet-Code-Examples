impl Solution {
    pub fn interpret(command: String) -> String {
      let com_chr: Vec<char> = command.chars().collect();
      let mut ret_str = Vec::new();
        for (i, chr) in command.chars().enumerate() {
          //println!("i: {}, chr: {}", i, chr);
            match chr {
              'G' => ret_str.push('G'),
              '(' => {
                if com_chr[i+1] == ')' { ret_str.push('o'); }
                else { 
                  ret_str.push('a'); 
                  ret_str.push('l');
                  }
              },
              _ => (),

            }
        }
        ret_str.into_iter().collect()
    }
}
