pub mod crypto_lib {
    use std::{fmt::{self, Display}, fs::File, io::{Read, Write}, path::{Path, PathBuf}};
    use aes_gcm::{
        aead::{consts::{U12, U32}, generic_array::GenericArray, Aead, AeadCore, KeyInit, OsRng}, Aes256Gcm, Error, Key, Nonce // Or `Aes128Gcm`
    };
    use base64::{prelude::BASE64_STANDARD, Engine};
    
    pub struct Encryption {
        key: Key<Aes256Gcm>,
        nonce: Nonce<U12>,
        message: Vec<u8>,
        encoded: Encoded,
    }
    pub struct Encoded {
        pub key: String,
        pub nonce: String,
        pub message: String,
    }
    pub trait Encrypt {
        fn new(message: &str) -> Encryption;
        fn from_file(path: &Path) -> Result<Encryption, std::io::Error>;
        fn to_file(encrypted: Encryption) -> Result<(), std::io::Error>;
    
        fn get_nonce() -> Nonce<U12> {
            Aes256Gcm::generate_nonce(&mut OsRng)
        }
    
        fn encrypter(key: &Key<Aes256Gcm>, nonce: Nonce<U12>, message: &str) -> Result<Vec<u8>, Error> {
            let cipher = Aes256Gcm::new(&key); 
            cipher.encrypt(&nonce, message.as_bytes())
        }
    
        fn encoder(key: &Key<Aes256Gcm>, nonce: &Nonce<U12>, message: &Vec<u8>) -> Encoded {
            let key = BASE64_STANDARD.encode(key); 
            let nonce = BASE64_STANDARD.encode(nonce); 
            let message = BASE64_STANDARD.encode(message);
            Encoded {
                key,
                nonce,
                message,
            }
        }
    }
    
    pub trait Decrypt {
        fn decrypter(encoded: Encoded) -> Result<String, aes_gcm::Error>;
        fn from_file(path: &Path) -> Result<String, std::io::Error>; 
    
        fn decoder(encoded: Encoded) -> Encryption {
            let key = BASE64_STANDARD.decode(encoded.key.clone()).unwrap(); 
            let nonce = BASE64_STANDARD.decode(encoded.nonce.clone()).unwrap();
            let message = BASE64_STANDARD.decode(encoded.message.clone()).unwrap();
    
            let key_slice = key.as_slice();
            let nonce_slice = nonce.as_slice();
    
            let key = *GenericArray::<u8, U32>::from_slice(key_slice);
            let nonce = *GenericArray::<u8, U12>::from_slice(nonce_slice);
                
            Encryption {
                key,
                nonce,
                message,
                encoded
            }
        }
    
    } 
    
    impl Encrypt for Encryption {
        fn new(message: &str) -> Encryption {
            let key = Aes256Gcm::generate_key(OsRng);
            let nonce = Self::get_nonce(); 
            let message = Self::encrypter(&key, nonce, &message).unwrap(); 
            let encoded = Self::encoder(&key, &nonce, &message);
    
            Encryption {
                key,
                nonce, 
                message,
                encoded, 
            }
        }
    
        fn from_file(path: &Path) -> Result<Encryption, std::io::Error> {
            let mut message = String::new();
            let mut file = File::open(path).unwrap();
    
            file.read_to_string(&mut message)?;
    
            let key = Aes256Gcm::generate_key(OsRng);
            let nonce = Self::get_nonce(); 
            let message = Self::encrypter(&key, nonce, &message).unwrap(); 
            let encoded = Self::encoder(&key, &nonce, &message);
    
            Ok(
                Encryption {
                    key,
                    nonce, 
                    message,
                    encoded, 
                }
            )
        }
    
        fn to_file(encrypted: Encryption) -> Result<(), std::io::Error> {
            let buffer = format!("{}\n{}\n{}", encrypted.encoded.key, encrypted.encoded.nonce, encrypted.encoded.message); 
            let destination: PathBuf = format!("C:/Users/{}/Downloads/assets", whoami::username()).into();
            let mut file = File::create(destination.as_path())?; 
    
            file.write(buffer.as_bytes())?;
    
            Ok(())
        }
    }
    
    impl Decrypt for Encryption {
        fn decrypter(encoded: Encoded) -> Result<String, aes_gcm::Error>{
            let encrypted = Self::decoder(encoded);
            let cipher = Aes256Gcm::new(&encrypted.key);
            let decrypted_bytes = cipher.decrypt(&encrypted.nonce, encrypted.message.as_ref())?; 
    
            Ok(String::from_utf8(decrypted_bytes).unwrap())
        }
    
        fn from_file(path: &Path) -> Result<String, std::io::Error> {
            let mut file = File::open(path)?;
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;
            let mut lines = buf.lines();
    
            let key = String::from(lines.next().unwrap());
            let nonce = String::from(lines.next().unwrap()); 
            let message = String::from(lines.next().unwrap());  
    
            let encoded = Encoded {
                key,
                nonce,
                message
            }; 
    
            let encrypted = Self::decoder(encoded); 
            let cipher = Aes256Gcm::new(&encrypted.key); 
            let decrypted_bytes = cipher.decrypt(&encrypted.nonce, encrypted.message.as_ref()).unwrap(); 
    
            Ok(String::from_utf8(decrypted_bytes).unwrap())
        }
    } 
    
    impl Display for Encryption {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Key: {}\nNonce: {}\nMessage: {}", self.encoded.key, self.encoded.nonce, self.encoded.message)
        }
    } 

}