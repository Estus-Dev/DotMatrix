mod bus;

pub use bus::Bus;

pub struct DotMatrix {
    bus: Bus,
}

impl DotMatrix {
    /// Create a new [DotMatrix].
    pub fn new() -> DotMatrix {
        Self { bus: Bus::new() }
    }

    #[cfg(test)]
    /// Create a new [DotMatrix] with a flat [Bus] for testing purposes.
    pub fn new_with_flat_bus() -> DotMatrix {
        Self { bus: Bus::flat() }
    }
}
