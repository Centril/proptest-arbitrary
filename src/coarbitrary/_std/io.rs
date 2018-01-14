use coarbitrary::*;

use std::io::*;

impl CoArbitrary for Error {
    fn coarbitrary(&self, mut var: Perturbable) {
        use std::error::Error;
        if let Some(err) = self.cause() {
            var.variant(0).nest(&err.description())
        } else {
            var.variant(1).nest(&self.kind())
        };
    }
}

impl CoArbitrary for ErrorKind {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            ErrorKind::NotFound => var.variant(0),
            ErrorKind::PermissionDenied => var.variant(1),
            ErrorKind::ConnectionRefused => var.variant(2),
            ErrorKind::ConnectionReset => var.variant(3),
            ErrorKind::ConnectionAborted => var.variant(4),
            ErrorKind::NotConnected => var.variant(5),
            ErrorKind::AddrInUse => var.variant(6),
            ErrorKind::AddrNotAvailable => var.variant(7),
            ErrorKind::BrokenPipe => var.variant(8),
            ErrorKind::AlreadyExists => var.variant(9),
            ErrorKind::WouldBlock => var.variant(10),
            ErrorKind::InvalidInput => var.variant(11),
            ErrorKind::InvalidData => var.variant(12),
            ErrorKind::TimedOut => var.variant(13),
            ErrorKind::WriteZero => var.variant(14),
            ErrorKind::Interrupted => var.variant(15),
            ErrorKind::Other => var.variant(16),
            ErrorKind::UnexpectedEof => var.variant(17),
            _ => var.variant(18),
        };
    }
}

impl CoArbitrary for SeekFrom {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            SeekFrom::Start(ref a) => var.variant(0).nest(a),
            SeekFrom::End(ref a) => var.variant(1).nest(a),
            SeekFrom::Current(ref a) => var.variant(2).nest(a),
        };
    }
}

#[cfg(feature = "unstable")]
impl CoArbitrary for CharsError {
    fn coarbitrary(&self, mut var: Perturbable) {
        match *self {
            CharsError::NotUtf8 => var.variant(0),
            CharsError::Other(ref e) => var.variant(1).nest(e),
        };
    }
}

#[cfg(feature = "unstable")]
impl CoArbitrary for Initializer {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.should_initialize());
    }
}

impl<R: Read + CoArbitrary> CoArbitrary for BufReader<R> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(self.get_ref());
    }
}

impl<R: Write + CoArbitrary> CoArbitrary for BufWriter<R> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(self.get_ref());
    }
}

impl<A: CoArbitrary, B: CoArbitrary> CoArbitrary for Chain<A, B> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.get_ref());
    }
}

impl<A: CoArbitrary + Clone + AsRef<[u8]>> CoArbitrary for Cursor<A> {
    fn coarbitrary(&self, var: Perturbable) {
        coarbitrary_iter(self.clone().bytes(), var);
    }
}

coarbitrary_unit!(Empty, Sink);

impl<A> CoArbitrary for IntoInnerError<A> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(self.error());
    }
}

impl<R: Write + CoArbitrary> CoArbitrary for LineWriter<R> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(self.get_ref());
    }
}

impl<R: CoArbitrary> CoArbitrary for Take<R> {
    fn coarbitrary(&self, mut var: Perturbable) {
        var.nest(&self.limit()).nest(self.get_ref());
    }
}
