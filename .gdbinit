target remote localhost:1234
file target/debug/kernel
define ri
  x/10i $rip
end
define rs
  x/20x $rsp-32
end
break _start
