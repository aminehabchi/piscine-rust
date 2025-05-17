pub fn talking(text: &str) -> &str {
    match text {
        "LEAVE ME ALONE!" =>"There is no need to yell, calm down!",
        "Is everything ok with you?"=>"Sure.",
       "HOW ARE YOU?"=>"Quiet, I am thinking!",
        ""=>"Just say something!",
         &_ => "Interesting",
    }
}