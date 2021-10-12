// group of function for Crypto
pub trait Crypto {
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
        let chars = self.chars();
        let mut buff = String::new();
        for c in chars {
            // Insert a char at the end of string
            buff.push((c as u8 - 1) as char);
        }
        return buff.to_string();
    }
}

/// generate a key
/// # return value
///
/// return a key
///
/// # Example :
/// ```
/// let key = gen_key()
/// let s = s.encrypt("key".to_string());
/// println!("{}",s);
/// ```
pub fn gen_key() -> (String, String) {
    return ("hello".to_string(), "hello".to_string());
}