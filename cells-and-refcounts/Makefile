PROGS = cell plaincount refcell rc rccycles

RUSTC = rustc
RUSTFLAGS = -g

all: $(PROGS)

plaincount: plaincount.rs
	$(RUSTC) $(RUSTFLAGS) -o plaincount plaincount.rs

cell: cell.rs
	$(RUSTC) $(RUSTFLAGS) -o cell cell.rs

refcell: refcell.rs
	$(RUSTC) $(RUSTFLAGS) -o refcell refcell.rs

rc: rc.rs
	$(RUSTC) $(RUSTFLAGS) -o rc rc.rs

rccycles: rccycles.rs
	$(RUSTC) $(RUSTFLAGS) -o rccycles rccycles.rs

clean:
	-rm -f $(PROGS)
