use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct HitSound: u8 {
        const NONE = 0;
        const NORMAL = 1<<0;
        const WHISTLE = 1<<1;
        const FINISH = 1<<2;
        const CLAP = 1<<3;
    }
}
