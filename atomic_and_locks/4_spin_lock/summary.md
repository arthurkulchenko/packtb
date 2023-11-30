- Guard pattern is used to:
  - to give us an exclusive reference to the data while employing interior mutability, so we could operate on guard object as reference but still have a posibility of inner value mutation by using UnsafeCell.

- Guard struct impements deref and derefmut because of Vec#push require a reference or mutable reference to operate over(so it is simplification for the operationist).

- UnsafeCell is a pure pointer type that is not thread safe.

- So we bound out data type to be at lease Send and mark it "unsafely" (use of blanket implementation) means as a delopers we release compile from responsibility of checking correctness of the fact that it Is actually Sync (atomic bool lock is implemented so only one thread is accessing data at a time) in another words: For those types that implements Send we also implement Sync.

- std::hint::spin_loop(); is used to inform processor that we are do not want to put out thread at sleep and we will obsessively try to lock, so giving that processor could make inner optimisations. so as a conclusion we can say that spin lock is a kind of attempt of prioritisation of locking thread over other threads.
