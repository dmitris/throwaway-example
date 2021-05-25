#![no_std]
#![no_main]

use aya_bpf::{bindings::TC_ACT_OK, macros::classifier, programs::SkSkbContext};

#[panic_handler]
fn do_panic(_info: &core::panic::PanicInfo) -> ! {
    unreachable!()
}

#[classifier(name = "egress")]
fn tc_cls_egress(skb: SkSkbContext) -> i32 {
    TC_ACT_OK as i32
}
