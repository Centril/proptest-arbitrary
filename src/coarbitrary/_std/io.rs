use coarbitrary::*;

use std::io::*;

coarbitrary!(Error; self, var => {
    use std::error::Error;
    if let Some(err) = self.cause() {
        var.variant(0).nest(&err.description())
    } else {
        var.variant(1).nest(&self.kind())
    };
});

coarbitrary!(ErrorKind; self, var => match *self {
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
});

coarbitrary!(SeekFrom; self, var => match *self {
    SeekFrom::Start(ref a) => var.variant(0).nest(a),
    SeekFrom::End(ref a) => var.variant(1).nest(a),
    SeekFrom::Current(ref a) => var.variant(2).nest(a),
});

#[cfg(feature = "unstable")]
coarbitrary!(CharsError; self, var => match *self {
    CharsError::NotUtf8 => var.variant(0),
    CharsError::Other(ref e) => var.variant(1).nest(e),
});

#[cfg(feature = "unstable")]
coarbitrary!(Initializer; self, var => var.nest(&self.should_initialize()));

coarbitrary!([R: Read + CoArbitrary] BufReader<R>;
    self, var => var.nest(self.get_ref()));

coarbitrary!([R: Write + CoArbitrary] BufWriter<R>;
    self, var => var.nest(self.get_ref()));

coarbitrary!([A: CoArbitrary, B: CoArbitrary] Chain<A, B>;
    self, var => var.nest(&self.get_ref()));

coarbitrary!([A: CoArbitrary + Clone + AsRef<[u8]>] Cursor<A>;
    self, var => coarbitrary_iter(self.clone().bytes(), var));

coarbitrary_unit!(Empty, Sink);

coarbitrary!([A] IntoInnerError<A>; self, var => var.nest(self.error()));

coarbitrary!([R: Write + CoArbitrary] LineWriter<R>;
    self, var => var.nest(self.get_ref()));

coarbitrary!([R: CoArbitrary] Take<R>;
    self, var => var.nest(&self.limit()).nest(self.get_ref()));