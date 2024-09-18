# Flashing with elf2uf2-rs
Write-Host "Flashing with elf2uf2-rs..."
elf2uf2-rs -d $args[0]

# Attaching defmt-print to read from the serial port
Write-Host "Attaching defmt-print..."

# Replace COMx with your actual COM port (e.g., COM3)
Get-Content COM9 | defmt-print -e $args[0]
