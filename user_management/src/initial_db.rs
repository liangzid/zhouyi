use rusqlite::Connection;

pub fn new_dbs(dbpath:&str)->String{
    let conn=Connection::open(dbpath).unwrap();
    println!("{:?}",&conn);

    conn.execute("CREATE TABLE account (
    email TEXT PRIMARY KEY,
    pwd TEXT NOT NULL,
    activation_state TEXT NOT NULL,
    user_type TEXT NOT NULL
)",
	()
    ).unwrap();

    conn.execute(&format!(
	"CREATE TABLE {} (
    id INTEGER  PRIMARY KEY,
    email TEXT NOT NULL,
    gua TEXT NOT NULL,
    yaos TEXT NOT NULL,
    yaoxangs BLOB NOT NULL,
    inps TEXT NOT NULL,
    time TEXT NOT NULL,
    place TEXT NOT NULL,
    analysis TEXT NOT NULL,
    comments TEXT NOT NULL
    )","EventRecord"),
	()
    ).unwrap();
    conn.execute("INSERT INTO account (email, pwd, activation_state, user_type) VALUES (?1, ?2,?3,?4)",
		 ("root@123.com","noyi123","activate","regular"));
    
    "Ok".to_owned()
}

fn main(){
    new_dbs("test111.db");
}
