// Float-flip ordering includes the sign bit of negative zero.
fn ordered(v:f32) -> u32 {
    let bits=bitcast<u32>(v);
    return select(bits^0x80000000u,~bits,(bits&0x80000000u)!=0u);
}
