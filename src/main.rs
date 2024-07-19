use std::mem;

#[repr(C)]
struct Base {
    x: u32,
    y: u64,
    is_derived: bool,
}

#[repr(C)]
struct Derived {
    base: Base,
    z: f32,
}

fn process<'a>(data: &'a Base) {
    print!("x: {}, y: {}", data.x, data.y);

    if data.is_derived {
        // upcast from Base to Derived
        let derived = unsafe { mem::transmute::<&'a Base, &'a Derived>(data) };
        print!(", z: {}", derived.z);
    }

    println!();
}

fn main() {
    let derived = Derived {
        base: Base {
            x: 2,
            y: 3,
            is_derived: true,
        },
        z: 5.55,
    };
    let base = Base {
        x: 5,
        y: 5,
        is_derived: false
    };
    let base_error = Base {
        x: 1,
        y: 1,
        is_derived: true,
    };

    process(&derived.base);
    process(&base);
    process(&base_error);
}
