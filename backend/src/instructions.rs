/// A 16-bit address `NNN`
pub struct Address;

/// 8-bit information in an opcode. `NN`
pub struct Byte;

/// 4-bit information in an opcode. `N`
pub struct Nibble;

/// Represents an arbitrary 8-bit register.
pub type Vx = u8;

/// Represents an arbitrary 8-bit register.
pub type Vy = u8;

/// This contains all of the instructions in the Chip-8 instruction set.
pub enum Instructions {

    /// Clear the screen.
    ///
    /// Opcode: `00E0`
    Clear,

    /// Return from a subroutine.
    ///
    /// Opcode: `00EE`
    Return,

    /// Jump to address `NNN`.
    ///
    /// Opcode: `1NNN`
    Jump(Address),

    /// Call the subroutine at address `NNN`.
    ///
    /// Opcode: `2NNN`
    Call(Address),

    /// Skip the following instruction
    /// if the value of register `VX` equals `NNN`.
    ///
    /// Opcode: `3XNN`
    SkipEqualK(Vx, Byte),

    /// Skip the following instruction
    /// if the value of register `VX` does not equal `NNN`.
    ///
    /// Opcode: `4XNN`
    SkipNotEqualK(Vx, Byte),

    /// Skip the following instruction if the value of register `VX`
    /// is equal to the value of register `VY`.
    ///
    /// Opcode: `5XY0`
    SkipEqual(Vx, Vy),

    /// Store number `NNN` in register `VX`.
    ///
    /// Opcode: `6XNN`
    StoreK(Vx, Byte),

    /// Add the value `NNN` to register `VX`.
    ///
    /// Opcode: `7XNN`
    AddK(Vx, Byte),

    /// Store the value of register `VY` in register `VX`.
    ///
    /// Opcode: `8XY0`
    Store(Vx, Vy),

    /// Set `VX` to `VX` OR `VY`.
    ///
    /// Opcode: `8XY1`
    Or(Vx, Vy),

    /// Set `VX` to `VX` AND `VY`.
    ///
    /// Opcode: `8XY2`
    And(Vx, Vy),

    /// Set `VX` to `VX` XOR `VY`.
    ///
    /// Opcode: `8XY3`
    Xor(Vx, Vy),

    /// Add the value of register `VY` to register `VX`.
    ///
    /// * Set VF to `01` if a carry occurs.
    /// * Set VF to `00` if a carry does not occur.
    ///
    /// Opcode: `8XY4`
    Add(Vx, Vy),

    /// Subtract the value of register `VY` from register `VX`.
    ///
    /// * Set `VF` to `00` if a borrow occurs.
    /// * Set `VF` to `01` if a borrow does not occur.
    ///
    /// Opcode: `8XY5`
    Sub(Vx, Vy),

    /// Store the value of register `VY` shifted right one bit in register `VX`.
    ///
    /// * Set register `VF` to the least significant bit prior to the shift.
    ///
    /// Opcode: `8XY6`
    ShiftRight(Vx, Vy),

    /// Set register `VX` to the value of `VY` minus `VX`.
    ///
    /// * Set `VF` to `00` if a borrow occurs.
    /// * Set `VF` to `01` if a borrow does not occur.
    ///
    /// Opcode: `8XY7`
    SubReversed(Vx, Vy),

    /// Store the value of register `VY` shifted left one bit in register `VX`.
    ///
    /// * Set register `VF` to the most significant bit prior to the shift.
    ///
    /// Opcode: `8XYE`
    ShiftLeft(Vx, Vy),

    /// Skip the following instruction if the value of register `VX`
    /// is not equal to the value of register `VY`.
    ///
    /// Opcode: `9XY0`
    SkipNotEqual(Vx, Vy),

    /// Store memory address `NNN` in register `I`.
    ///
    /// Opcode: `ANNN`
    StoreI(Address),

    /// Jump to address `NNN` + `V0`.
    ///
    /// Opcode: `BNNN`
    JumpOffset(Address),

    /// Set `VX` to a random number with a mask of `NN`.
    ///
    /// Opcode: `CXNN`
    Random(Vx, Byte),

    /// Draw a sprite at position `VX`,`VY` with `N` bytes of sprite
    /// data starting at the address stored in `I`.
    ///
    /// * Set `VF` to `01` if any set pixels are changed to unset, and `00` otherwise.
    ///
    /// Opcode: `DXYN`
    Draw(Vx, Vy, Nibble),

    /// Skip the following instruction if the key corresponding to
    /// the hex value currently stored in register `VX` is pressed.
    ///
    /// Opcode: `EX9E`
    SkipKeyPressed(Vx),

    /// Skip the following instruction if the key corresponding to
    /// the hex value currently stored in register `VX` is not pressed.
    ///
    /// Opcode: `EXA1`
    SkipKeyNotPressed(Vx),

    /// Store the current value of the delay timer in register `VX`.
    ///
    /// Opcode: `FX07`
    StoreDelayTimer(Vx),

    /// Wait for a keypress and store the result in register `VX`.
    ///
    /// Opcode: `FX0A`
    WaitForKeypressAndStore(Vx),

    /// Set the delay timer to the value of register `VX`.
    ///
    /// Opcode: `FX15`
    SetDelayTimer(Vx),

    /// Set the sound timer to the value of register `VX`.
    ///
    /// Opcode: `FX18`
    SetSoundTimer(Vx),

    /// Add the value stored in register `VX` to register `I`.
    ///
    /// Opcode: `FX1E`
    AddToI(Vx),

    /// Set `I` to the memory address of the sprite data corresponding
    /// to the hexadecimal digit stored in register `VX`.
    ///
    /// Opcode: `FX29`
    LoadHexGlyph(Vx),

    /// Store the binary-coded decimal equivalent of the value
    /// stored in register `VX` at addresses `I`, `I+1`, and `I+2`.
    ///
    /// Opcode: `FX33`
    StoreBCD(Vx),

    /// Store the values of register `V0` to `VX` inclusive in memory
    /// starting at address `I`.
    ///
    /// * `I` is set to `I + X + 1` after operation.
    ///
    /// Opcode: `FX55`
    StoreRegisters(Vx),

    /// Fill registers `V0` to `VX` inclusive with the values stored
    /// in memory starting at address `I`.
    ///
    /// * `I` is set to `I + X + 1` after operation.
    ///
    /// Opcode: `FX65`
    LoadRegisters(Vx),

    /// Any instruction received that does not belong to the Chip-8 instruction set.
    ///
    /// Opcode: `Unknown`
    Invalid,

}
