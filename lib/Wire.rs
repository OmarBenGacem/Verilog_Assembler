struct Wire {
    start: Port,      // Can only have one signal driving the value at the signal
    end: Vector::Port
}


impl Wire {
    fn drive(&mut Self) -> Self {
        
        for (index, &port) in Self.end.iter().enumerate() {
            Self.end[index] = self.start.GetValue();
        }
    }
}