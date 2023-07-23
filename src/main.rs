use bincode::{deserialize, serialize, Result};
use serde::{Deserialize, Serialize};

use img_caster::bitarray;

fn main() {
    let mut bitarray = bitarray::BitArray::new(55);
    println! {"Bitarray {:?}", bitarray}

    bitarray.set(3, true);
    println! {"Bitarray {:?}", bitarray}

    bitarray.set(5, true);
    println! {"Bitarray {:?}", bitarray}

    bitarray.set(20, true);
    println! {"Bitarray {:?}", bitarray}

    bitarray.set(53, true);
    println! {"Bitarray {:?}", bitarray}

    let packed = serialize(&bitarray).unwrap();
    println! {"packed {:?}", packed}

    let mut other = bitarray::BitArray::new(55);
    other.set(1, true);
    other.set(25, true);
    other.set(48, true);

    bitarray.bit_or(other);

}
