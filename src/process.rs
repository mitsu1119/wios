use core::mem::MaybeUninit;

const PS_STACK_SIZE: usize = 1024;

#[repr(align(8))]
struct PsStack(MaybeUninit<[u8; PS_STACK_SIZE]>);

#[link_section = ".ps_stack"]
#[no_mangle]
static PS_STACK: PsStack = PsStack(MaybeUninit::uninit());
