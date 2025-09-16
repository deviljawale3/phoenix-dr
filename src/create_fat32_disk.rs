use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // This creates a blank 2MB FAT32 disk image
    let mut file = File::create("fat32_test.img")?;
    
    // Create a minimal FAT32 boot sector (simplified)
    let mut boot_sector = [0u8; 512];
    
    // Jump instruction
    boot_sector[0] = 0xEB;
    boot_sector[1] = 0x3C;
    boot_sector[2] = 0x90;
    
    // OEM Name
    boot_sector[3..11].copy_from_slice(b"PHOENIXDR");
    
    // Bytes per sector (512)
    boot_sector[11] = 0x00;
    boot_sector[12] = 0x02;
    
    // Sectors per cluster (1)
    boot_sector[13] = 0x01;
    
    // Reserved sectors (32)
    boot_sector[14] = 0x20;
    boot_sector[15] = 0x00;
    
    // Number of FATs (2)
    boot_sector[16] = 0x02;
    
    // Root directory entries (512)
    boot_sector[17] = 0x00;
    boot_sector[18] = 0x02;
    
    // Total sectors (4000 = 2MB)
    boot_sector[19] = 0x20;
    boot_sector[20] = 0x0F;
    
    // Media type
    boot_sector[21] = 0xF8;
    
    // Sectors per FAT (78)
    boot_sector[22] = 0x4E;
    boot_sector[23] = 0x00;
    
    // Signature
    boot_sector[510] = 0x55;
    boot_sector[511] = 0xAA;
    
    file.write_all(&boot_sector)?;
    
    // Fill the rest with zeros to create 2MB image
    let zeros = [0u8; 512];
    for _ in 0..3999 {
        file.write_all(&zeros)?;
    }
    
    println!("✅ Created FAT32 test disk image (2MB)");
    println!("Now please use external tools to format it properly:");
    println!("On Windows:");
    println!("  1. Right-click fat32_test.img -> Mount");
    println!("  2. Format the drive as FAT32");
    println!("  3. Add some test files");
    println!("  4. Eject the drive");
    println!("On Linux:");
    println!("  1. sudo mkfs.fat -F32 fat32_test.img");
    println!("  2. mount it and add files");
    
    Ok(())
}