use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use crate::error::Result;

// Phele ek kahli file create kree ge
// kyuki 8 task parallel alag-alag jagah likhenge
// phele file ka size set krna important hai kyu ki seek position exist nahi kree ge

pub fn create_output_file(path: &str, size: u64) ->Result<()>{
    // create file or overwrite it if it's not exists
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)?;

    // file ka size allocate kro
   // ye OS ko batata hai "itni jaga ko reserver kroo"
   // Bina eske seek karte time file currupt ho sakte hai.
    file.set_len(size)?;
    Ok(())
}

pub fn wirte_chunk_to_file(path: &str, offset: u64, data:&[u8])->Result<()>{
    
    // Existing file open kr rahe hai (Sirf wirte mode me)
    // create(false) file phalse se exist krne chayee
    let mut file = OpenOptions::new()
    .write(true)
    .open(path)?;

    // file ki pointer ko sahi position pr ke jaoo
    // SeekFrom::Start(offset) -. File ki start offset se aage jaoo, jasse -jasee seek karte ho lihkna start kroo
    file.seek(SeekFrom::Start(offset))?;
    // Data likho es position se aage
    file.write_all(data)?;

    Ok(())

}