use crate::{exports::*, main, release};

arcdps::arcdps_export! {
    name: "GW2SkillShare",
    sig: 0x2_0805,
    init: main,
    release: release,
    imgui: imgui,
    combat: combat,
    combat_local: combat_local,
}
