
use i3g4250d::{ MODE};


type A5 = AF5<PushPull>;
pub type I3G4250D = i3g4250d::I3G4250D<Spi<SPI1,(PA5<A5>, PA6<A5>, PA7<A5>)>, PE3<Output<PushPull>>>;
