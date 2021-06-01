use chrono::{Duration, Local};

use u06::turtle::Turtle;

pub fn explore_methods_on_null() {
    // Go passes nil pointers to the receivers
    // (unlike Java where it will result in a null pointer exception
    // or in Python where unless it's an attribute/method on NullType will
    // result in an AttributeError or TypeError )
    // As a consequnce of this the method can decide to provide a
    // sensible behavior for nil values.
    //
    // In the safe subset of Rust references cannot be null and nullable
    // types are modelled using Option, but for raw pointers, there is
    // something similar to what Go does (although it is not clear if
    // this is actually used or exists because methods in Rust is
    // syntactic sugar for function calls).
    //
    // We can implement a trait for a raw pointer to a type (e.g. *const T
    // or *const i32); the method receives the value null.
    //
    // This can also be expressed as a method for a concrete type, but
    // it is currently under a nightly feature flag, because it inovlves
    // using arbitrary self types like (self: *const Self)
    trait Annoyer {
        fn annoy(self);
    }

    impl Annoyer for *const i32 {
        fn annoy(self) {
            if self.is_null() {
                println!("It's bad, m'kay.");
            } else {
                println!("It's {}!", unsafe { &*self });
            }
        }
    }

    let mut n: *const i32 = std::ptr::null();
    n.annoy();
    n = &72;
    n.annoy();

    /* This is only possible with a nighly feature
    struct Hammadi {
        nag: i32,
    }

    impl Hammadi {
        pub fn find(self: *const Self) {
            if self.is_null() {
                println!("Nothing to see here.");
            } else {
                println!("Found {} manuscripts!", unsafe { self.nag });
            }
        }
    } */
}

pub fn explore_undefined_behavior_null_deref() {
    // the Go equivalent of the following results in a safe panic
    // (the same behavior as in most other languages like Java, Python etc.)
    // but resuts in undefined behavior in Rust (note that the part
    // that can result in undefined behavior will only compile if
    // it is marked as `unsafe`):
    let zilch: *const isize = std::ptr::null();
    println!("{:p}", zilch);
    println!("{}", unsafe { *zilch }); // <-- Undefined Behavior
}

pub fn explore_norman_turtle() {
    let mut t = Turtle::default();
    show_turtle(&t);
    t.up();
    show_turtle(&t);
    t.right();
    show_turtle(&t);
    t.down();
    show_turtle(&t);
    t.down();
    show_turtle(&t);
    t.left();
    show_turtle(&t);
}

fn show_turtle(t: &Turtle) {
    println!("turtle is now at {:?}", t.position());
}

pub fn exploring_interfaces() {
    // In Go the kind (i.e. value or pointer) of the receiver is not
    // part of an interface
    //
    // type Talker interface {
    //    Talk()
    // }
    //
    // In Rust the kind of reciver it takes is part of a trait:
    pub trait Walker {
        fn walk(self);
    }

    pub trait Talker {
        fn talk(&self);
    }

    pub trait Performer {
        fn perform(&mut self);
    }

    // For Go interfaces we can implement the method with
    // either a value or pointer receiver (not both):
    //
    // func (t Martian) Talk() {
    //    fmt.Println("abugida")
    // }
    // or
    // func (t *Martian) Talk() {
    //    fmt.Println("abugida")
    // }
    //
    // In Rust, the receiver has to match the receiver type
    // in the trait although a trait can be implemented for
    // a reference to a type.
    //
    // Does this effectively provide the ability to implement
    // it for both kinds, except that in this case, both can exist and can
    // have different implementations?
    //
    // The answer seems to be no (not 100% sure about this.)
    // It kind-of works, but only when dealing directly with the concrete type
    // for which the trait is implemented, not when accessed through
    // the trait itself; in which case it can only accessed through the
    // kind of receiver declared in the trait.

    #[derive(Debug, Clone)]
    pub struct Martian(i32);

    impl Walker for Martian {
        fn walk(self) {
            println!("walk! {}", self.0)
        }
    }

    impl Walker for &Martian {
        fn walk(self) {
            println!("&walk! {}", self.0)
        }
    }

    impl Walker for &mut Martian {
        fn walk(self) {
            self.0 += 1;
            println!("&mut walk! {}", self.0)
        }
    }

    // Directly through a concrete impl.
    let m = Martian(1);
    m.walk(); // calls the impl. for Martian
    let mut m = Martian(4);
    {
        let mp = &m;
        mp.walk(); // calls the impl. for &Martian
        Walker::walk(mp);

        let mmp = &mut m;
        mmp.walk(); // calls the impl. for &mut Martian
        Walker::walk(mmp);
        println!("{:?}", m);
    }

    fn walk_val(w: impl Walker) {
        w.walk();
    }
    walk_val(m);

    impl Talker for Martian {
        fn talk(&self) {
            println!("abugida {}", self.0);
        }
    }

    impl Talker for &mut Martian {
        fn talk(&self) {
            println!("&mut abugida");
        }
    }

    let mut m = Martian(1);
    m.talk();

    {
        let mp = &m;
        mp.talk();
        Talker::talk(mp);

        let mmp = &mut m;
        mmp.talk();
        Talker::talk(&mmp);
    }

    // Can only implement refs (static or dynamic)
    fn talk_ref(t: &impl Talker) {
        t.talk();
    }

    talk_ref(&m);

    fn talk_ref_dyn(t: &dyn Talker) {
        t.talk();
    }

    talk_ref_dyn(&m);

    impl Performer for Martian {
        fn perform(&mut self) {
            self.0 += 2;
            println!("perform abjad {}", self.0);
        }
    }

    let mut m = Martian(77);
    m.perform();
    Performer::perform(&mut m);
    {
        let mmp = &mut m;
        mmp.perform();
        Performer::perform(mmp);
    }

    fn perform_ref(p: &mut impl Performer) {
        p.perform();
    }

    perform_ref(&mut m);

    fn perform_ref_dyn(p: &mut impl Performer) {
        p.perform();
    }

    perform_ref_dyn(&mut m);
}

pub fn exploring_time() {
    // const layout  = "2006 Jan 2, a Mon";
    const FORMAT: &str = "%Y %b %-d, a %a";
    // today := time.Now()
    let today = Local::now();
    // tomorrow := today.Add(24 * time.Hour)
    let tomorrow = today + Duration::hours(24);
    // fmt.Println(today.Format(layout))
    println!("{}", today.format(FORMAT));
    println!("{}", tomorrow.format(FORMAT));
    // The `time` packge example was used in the book as an example of
    // consistently using value (as opposed to pointer) receivers.
    // A common Go practice is to either always use pointer receivers
    // or to always use value receivers. Using pointers indicates that
    // type is meant to be used with pointer semantics. Pointer/reference
    // semantics is used when mutation is necessary or when the object
    // involved is too large to be copied around a lot.
    // Using pointers does not necessarily mean that the type is a
    // mutable type and some types which are passed around as values
    // may mutate things (these usually use pointers internally)
    //
    // In Rust, a type that only uses shared/immutable (&self)
    // or copy/move (self) receivers indicate that the type does not support
    // mutating its values (there are types like RefCell that support
    // mutating it's internal state while using shared/immutable references)
}

pub fn exploring_refs_structs() {
    #[derive(Debug, Clone, Copy, Default)]
    struct Punt {
        x: i32,
        y: i32,
    }

    impl Punt {
        pub fn dump(&self) {
            println!("({}, {})", self.x, self.y);
        }

        pub fn right(&mut self, units: u16) {
            self.x += units as i32;
        }

        pub fn rightened(mut self, units: u16) -> Self {
            self.x += units as i32;
            self
        }

        pub fn rightened_alt(&self, units: u16) -> Self {
            let mut rightened = *self;
            rightened.x += units as i32;
            rightened
        }

        pub fn left(&mut self, units: u16) {
            self.x -= units as i32;
        }
    }

    let pap = &mut Punt { x: 7, y: 8 };
    println!("pap is {:?} ", pap);
    println!("x in pap is {}", pap.x);
    // can access and modify a field of a struct through ref
    {
        let px = &mut pap.x;
        *px = 77;
    }
    println!("pap is now {:?} ", pap);

    let mut p = Punt { x: 3, y: 4 };
    println!("p is {:?} ", p);
    {
        let px = &mut p.x;
        *px = 98;
    }
    println!("p is now {:?} ", p);

    let mut t = Punt::default();
    t.dump();
    t.left(3); // uses mutable ref to modify t
    t.dump();
    t.right(2); // uses mutable ref to modify t
    t.dump();

    // modifies a copy without using a refrenece (because Punt is a Copy type)
    // this won't work if Punt was not Copy; in that case `t` will be no longer be accessible
    // because it will be moved into rt
    let rt = t.rightened(8);
    t.dump();
    rt.dump();
    let rt = t.rightened_alt(8);
    t.dump();
    rt.dump();
}

pub fn exploring_refs_simple() {
    let mut s = 'S';
    // access value of `s` through a reference
    {
        let a: &char = &s;
        println!("{} @ {:p}", *a, a);
    }
    // modify value of `s` through a reference
    {
        let z = &mut s;
        *z = 'Z';
    }
    println!("{} @ {:p}", s, &s);
    let u = 'Z';
    let ps = &s;
    let ns = &s;
    let pu = &u;
    // comparing references do not compare pointers
    println!("refs to ze same var: are they equal? {}", ps == ns);
    println!(
        "refs to different vars with same value: are they equal? {}",
        ps == pu
    );
    // this has to be done manually
    println!(
        "raw pointers to different vars with same value: are they equal? {}",
        (ps as *const char) == (pu as *const char)
    );
    // accessing elements through reference to arrays work without manual dereference
    let ss = [1i32, 2, 3];
    let ssr = &ss;
    let sv = ss[0];
    let ssv = ssr[0];
    println!("{}, {}", sv, ssv);
}
