pub fn talking(text: &str) -> &str {
    match text {
        "JUST DO IT!" =>"There is no need to yell, calm down!",
        "Hello how are you?"=>"Sure.",
        "WHAT'S GOING ON?"=>"Quiet, I am thinking!",
        "something"=>"Interesting",
        ""=>"Just say something!",
         &_ => "",
    }
}