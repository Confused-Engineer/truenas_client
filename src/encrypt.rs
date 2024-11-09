use std::{io::Write, str::FromStr};

const KEY: &[u8; 512] = include_bytes!("../assets/secure_key");


#[derive(Clone)]
pub struct Encrypt
{
    api_key: String,
    file_name: String,
}



impl Encrypt
{
    pub fn new(file_name: &str) -> Self
    {
        Encrypt { 
            api_key: String::new(),
            file_name: String::from_str(file_name).unwrap()
        }
    }

    pub fn set_key(&mut self, key: &str) -> &mut Self
    {
        self.api_key = key.to_string();
        self
    }

    pub fn get_key(&mut self) -> String
    {
        self.api_key.clone()
    }


    pub fn save_file(&mut self) -> std::io::Result<()>
    {
        let home_dir = davids_awesome_library::env::get_home()?;
        let save_path = format!("{}/.truenas-client/", home_dir);
        
        let path = std::path::Path::new(&save_path);
        if !path.exists()
        {
            let _ = std::fs::create_dir_all(path);
        }

        let save_local = format!("{}/.truenas-client/{}", home_dir, self.file_name);
        

        let mut file = std::fs::File::options().create(true).read(false).write(true).open(save_local)?;
        let encrypted_data: Vec<u8> = simple_crypt::encrypt(&self.api_key.as_bytes(), KEY).expect("Failed to encrypt");
        
        file.write_all(&encrypted_data)?;
        Ok(())
    }

    pub fn load_file(&mut self) -> std::io::Result<()>
    {
        let home_dir = davids_awesome_library::env::get_home()?;

        let save_local = format!("{}/.truenas-client/{}", home_dir, self.file_name);
        let encrypted: Vec<u8> = std::fs::read(&save_local)?;

        let data: Vec<u8> = simple_crypt::decrypt(&encrypted, KEY).expect("Failed to decrypt");
        let string_wrapped = String::from_utf8(data);

        if string_wrapped.is_err()
        {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "error could not parse key"));
        }
        
        self.api_key = string_wrapped.unwrap();
        
        Ok(())

    }
}





#[cfg(test)]
mod tests {

    

    use super::*;

    #[test]
    fn encrypt() {
        let mut key = Encrypt::new("test_file");
        //key.set_key("123");
        //let _ = key.save_file();
        let _ = key.load_file();
        println!("{}", key.get_key());


    }
}