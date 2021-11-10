// group of function for Crypto
pub trait Crypto {
    fn check_string(&self) -> bool;
    fn encrypt(&self, key: String) -> String;
    fn decrypt(&self, key: String) -> String;
}

// add Crypto's functions for String type
impl Crypto for String {
    /// encrypt the string
    /// # return value
    ///
    /// return an encrypted String
    ///
    /// # Example :
    /// ```
    /// let s = "Hello".to_string();
    /// let s = s.encrypt("key".to_string());
    /// println!("{}",s);
    /// ```
    fn encrypt(&self, _key: String) -> String {
        let chars = self.chars();
        let mut buff = String::new();
        for c in chars {
            // Insert a char at the end of string
            buff.push((c as u8 + 1) as char);
        }
        // add the final magic number
        buff.push('3');
        buff.push('3');
        return buff.to_string();
    }

    /// decrypt the string
    /// # return value
    ///
    /// return a decrypted String
    ///
    /// # Example :
    /// ```
    /// let s = "Hello".to_string();
    /// let s = s.decrypt("key".to_string());
    /// println!("{}",s);
    /// ```
    fn decrypt(&self, _key: String) -> String {
        // decrypt
        let data = self.chars();
        let mut buff = String::new();
        for c in data {
            // Insert a char at the end of string
            buff.push((c as u8 - 1) as char);
        }
        buff = buff[0..buff.len() - 3].to_string();
        return buff;
    }

    /// check if the string is well sent by the program or not
    /// # return value
    ///
    /// return false if not sent by the program
    fn check_string(&self) -> bool {
        // check if the string is sent by another person or not
        let last_two: String = self.chars().rev().take(2).collect(); // get two last chars
        if last_two != "33".to_string() {
            return false;
        }
        return true;
    }
}

/// generate a private and public Key
/// # return value
///
/// return a private and a public key (pivK, pubK)
///
/// # Example :
/// ```
/// let (privK, pubK) = gen_key();
/// let s = s.decrypt(privK.to_string());
/// println!("{}",s);
/// ```
pub fn gen_key() -> (String, String) {
    let priv_key = "Hello".to_string();
    let pub_key = "Toi".to_string();
    return (priv_key, pub_key);
}
