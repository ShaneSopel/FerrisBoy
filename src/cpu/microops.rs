enum MicroOp
{
    LdReg8FromReg8   { dst: Reg8, src: Reg8 },
    LdReg8FromMem    { dst: Reg8, src: Reg16 }, 
    LdReg16FromMem   { dst: Reg16, src: Reg16 },
    LdReg16FromReg8  { dst: Reg16, src: Reg8 },
    LdReg8FromReg16  { dst: Reg8,  src: Reg16 },
    IncReg8          { reg: Reg8 },
    DecReg8          { reg: Reg8 },
    IncReg16         { reg: Reg16 }, 
    DecReg16         { reg: Reg16 },
    AddReg8          { dst: Reg8, src: Reg8 },
    AddReg16         { dst: Reg16, src: Reg16 },
    AddCarry8        { dst: Reg16, src: Reg8 },
    AddCarry16       { dst: Reg8,  src: Reg16 },
    SubReg8          { dst: Reg8,  src: Reg8 },
    SubReg16         { dst: Reg8,  src: Reg16 },
    SubCarry8        { dst: Reg8,  src: Reg8 },
    SubCarry16       { dst: Reg8,  src: Reg16 },  
    PushReg16        { reg: Reg16 },
    PopReg16         { reg: Reg16 },

}