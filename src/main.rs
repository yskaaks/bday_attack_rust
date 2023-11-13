use std::fs;
use sha2::{Sha256, Digest};
use std::error::Error;

fn compute_hash(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text);
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    match fs::read_to_string(file_path) {
        Ok(content) => Ok(content),
        Err(e) => Err(Box::new(e))
    }
}

fn generate_variants(text: &str, num_lines: usize) -> Vec<String> {
    let lines: Vec<&str> = text.split('\n').collect();
    let mut variants = Vec::new();

    for i in 0..2usize.pow(num_lines as u32) {
        let binary_representation = format!("{:0width$b}", i, width = num_lines);
        let mut variant_lines: Vec<String> = binary_representation.chars().enumerate()
            .map(|(j, c)| {
                if c == '1' {
                    format!("{} ", lines[j])
                } else {
                    lines[j].to_string()
                }
            })
            .collect();

        // Convert each &str to String and then extend variant_lines
        variant_lines.extend(lines[num_lines..].iter().map(|&line| line.to_string()));
        
        variants.push(variant_lines.join("\n"));
    }

    variants
}


fn main() -> Result<(), Box<dyn Error>> {
    let real_confession_file = "/Users/yskakshiyap/Desktop/real.txt";
    let fake_confession_file = "/Users/yskakshiyap/Desktop/fake.txt";

    let real_confession_text = read_file(real_confession_file)?;
    let fake_confession_text = read_file(fake_confession_file)?;

    let real_variants = generate_variants(&real_confession_text, 10);
    let fake_variants = generate_variants(&fake_confession_text, 10);

    let mut found = false;
    for real_variant in real_variants {
        let real_hash = compute_hash(&real_variant);

        for fake_variant in &fake_variants {
            let fake_hash = compute_hash(fake_variant);

            if real_hash.ends_with(&fake_hash[62..64]) {
                println!("Matching hashes found.");
                println!("Real Variant: {}\nFake Variant: {}\nReal Hash: {}\nFake Hash: {}",
                         real_variant, fake_variant, real_hash, fake_hash);
                found = true;
                break;
            }
        }

        if found {
            break;
        }
    }

    if !found {
        println!("No matching hash suffix found.");
    }

    Ok(())
}
