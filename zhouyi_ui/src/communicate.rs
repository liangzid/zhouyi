



// query and return the login state
pub fn query_login(email:&String, pwd:&String)->(String,String,String,String){
    (
	"Ok".to_owned(),
	"1".to_owned(),
	"regular".to_owned(),
	"activate".to_owned(),
    )
}

// appended the new history information into clouds
pub fn merge_history(email:&String,historys:Vec<(
    HashMap<String,String>,
    Vec<String>,
    Vec<String>,
    String,
    String,
    String,
    String,
    Vec<(String,String)>,
)>)->String{

    "Ok"
    
}



// sync the history info from server to local devices(email)
pub fn get_history(email:&String)->Vec<(
    HashMap<String,String>,
    Vec<String>,
    Vec<String>,
    String,
    String,
    String,
    String,
    Vec<(String,String)>,
)>{
    vec![]
}
