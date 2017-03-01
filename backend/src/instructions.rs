pub struct Addr;     ///NNN
pub struct Byte;     ///NN
pub struct Nibble;   ///N
pub struct Register;

pub type Vx = u8;
pub type Vy = u8;

pub enum Instructions {

    /// Clears the screen.
    ///
    /// Opcode: `00E0`
    Clear,

    /// Returns from a subroutine.
    ///
    /// Opcode: `00EE`
    Return,

    /// Jumps to address `NNN`.
    ///
    /// Opcode: `1NNN`
    Jump(Addr),

    /// Calls the subroutine at address `NNN`.
    ///
    /// Opcode: `2NNN`
    Call(Addr),

    /// Skips the following instruction if the value of register `VX` equals `NNN`.
    ///
    /// Opcode: `3XNN`
    SkipEqualK(Vx, Byte),

    /// Skips the following instruction if the value of register `VX` does not
    /// equal `NNN`.
    ///
    /// Opcode: `4XNN`
    SkipNotEqualK(Vx, Byte),

    /// Skips the following instruction if the value of register `VX` is equal to
    /// the value of register `VY`.
    ///
    /// Opcode: `5XY0`
    SkipEqual(Vx, Vy),

    /// Store number `NNN` in register `VX`.
    ///
    /// Opcode: `6XNN`
    StoreK(Vx, Byte),

    /// Adds the value `NNN` to register `VX`.
    ///
    /// Opcode: `7XNN`
    AddK(Vx, Byte),

    /// Stores the value of register `VY` in register `VX`.
    ///
    /// Opcode: `8XY0`
    Store(Vx, Vy),

    /// Sets `VX` to `VX` OR `VY`.
    ///
    /// Opcode: `8XY1`
    Or(Vx, Vy),

    /// Sets `VX` to `VX` AND `VY`.
    ///
    /// Opcode: `8XY2`
    And(Vx, Vy),

    /// Sets `VX` to `VX` XOR `VY`.
    ///
    /// Opcode: `8XY3`
    Xor(Vx, Vy),

    /// Adds the value of register `VY` to register `VX`.
    ///
    /// * Sets VF to `01` if a carry occurs.
    /// * Sets VF to `00` if a carry does not occur.
    ///
    /// Opcode: `8XY4`
    Add(Vx, Vy),

    /// Subtracts the value of register `VY` from register `VX`.
    ///
    /// * Sets `VF` to `00` if a borrow occurs.
    /// * Sets `VF` to `01` if a borrow does not occur.
    ///
    /// Opcode: `8XY5`
    Sub(Vx, Vy),

    /// Stores the value of register `VY` shifted right one bit in register `VX`.
    ///
    /// * Sets register `VF` to the least significant bit prior to the shift.
    ///
    /// Opcode: `8XYE`
    ShiftLeft(Vx, Vy),

}
