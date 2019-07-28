PROGS = cell plaincount refcell rc

RUSTFLAGS = -g

all: $(PROGS)

plaincount: plaincount.rs
	rustc $(RUSTFLAGS) -o plaincount plaincount.rs

cell: cell.rs
	rustc $(RUSTFLAGS) -o cell cell.rs

refcell: refcell.rs
	rustc $(RUSTFLAGS) -o refcell refcell.rs

rc: rc.rs
	rustc $(RUSTFLAGS) -o rc rc.rs

clean:
	-rm -f $(PROGS)
