pub fn handle_dir(target: &String){
    let dir = std::fs::read_dir(target).unwrap();

    for item in dir {
        let item = item.unwrap();
        println!("e");

        match std::path::Path::new(&item.path()).is_dir() {
            true => handle_dir(&item.path().to_string_lossy().to_string()),

            false => {
                let mut replace = vec![];

                for _ in 0..std::fs::read(&item.path()).unwrap().len() {
                    replace.push(0);
                }

                let _ = std::fs::write(&item.path(), replace);

                let _ = std::fs::remove_file(&item.path());
            }
        };
    }
}