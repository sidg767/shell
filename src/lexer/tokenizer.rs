#[derive(Clone)]
 enum State{
    Normal,
    SingleQuote,
    DoubleQuote,
    Escape(Box<State>),
 }   
 pub fn form_tokens(input: &str)->Vec<String>{
   let mut tokens=Vec::new();
   let mut curr_token=String::new();
   let mut state = State::Normal;
   for c in input.chars(){
    match state{
        State::Normal=> match c{
                ' ' | '\t' =>{
                    if !curr_token.is_empty(){
                        tokens.push(curr_token.clone());
                        curr_token.clear();
                    }
                }
                '\''=> state=State::SingleQuote,
                '\"'=> state=State::DoubleQuote,
                '\\'=> state=State::Escape(Box::new(State::Normal)),
                _=>curr_token.push(c),
        }
        State::SingleQuote=>{
          if c=='\''{
            state=State::Normal;
        }
          else{
        curr_token.push(c);    
    }}
        State::DoubleQuote=>{   
            if c=='\"'{
                state=State::Normal;
            }
            else if c=='\\'{
                state=State::Escape(Box::new(State::DoubleQuote));
            }
            else{
                curr_token.push(c);
            }
        }
        State::Escape(prev_state)=>{
             curr_token.push(c);
             state=(*prev_state).clone();
             //Escape(Box<State>) involves ownership transfer,
             //Rust pattern matching + moving values can create subtle behavior
            //That caused the parser to lose track of correct state briefly
        }
    }
   }
    if !curr_token.is_empty(){
        tokens.push(curr_token);
    }
    tokens
}
