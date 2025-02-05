mod error;
mod khf;
// mod kht;
// mod lethe;

mod khf_tree;
pub(self) mod node;
pub(self) mod topology;

pub use node::Node;
pub use topology::Topology;

pub use {
    error::{Error, Result},
    khf::{Khf, KhfBuilder, KhfStats},
    // lethe::{Lethe, LetheBuilder, LetheStats},
};

pub(self) type Pos = (u64, u64);
