use crate::State;
use crate::shell;
use shell::Message::{ServerLog,ChatMessage};
pub struct Descriptor{
    pub text: String,
}
impl Descriptor{
    
    pub fn new() -> Self{
        Self { 
            text: "".to_string()
         }
    }

    pub fn parse_command(&self, state: &mut State){
        let mut s = String::from(&self.text);
        let first = s.chars().nth(0);
        if  first == Some('/') {
            match &s as &str{
                "/test"=>{

                    s = String::from("Test successed");

                }
                _=>{
                    s = String::from("Fail to parse command");
                }
                
            }
            state.iced_state.queue_message(ServerLog(s));
        }
        else {
            state.iced_state.queue_message(ChatMessage);
        }
    }

}