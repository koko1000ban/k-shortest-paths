#[doc = "A heap. this is untested implementation. so buggy yet."];

use std;

import core::option;
import core::option::{some, none};

#[doc = "compare heap node"]
type compare<T: copy> = fn@(T, T) -> bool;

type node<T: copy> = option<T>;

type heap<T: copy> = @{
  mut nelts: uint, 
  comp: compare<T>, 
  mut elts: [mut node<T>]
};

fn _get<T: copy>(elts: [mut node<T>], i: uint) -> T {
  ret alt elts[i] { some(t) { t } _ { log(warn, "failure getting heap node."); fail } };
}

const initial_capacity: uint = 32u; // 25^

fn create<T: copy>(f: compare<T>) -> heap<T> {
  let heap: heap<T> = @{
    mut nelts: 0u,
    comp: f, 
    mut elts: vec::to_mut(vec::from_elem(initial_capacity, none))
  };
  
  ret heap;
}

fn push<T: copy>(hp: heap<T>, val: T) {
  let n = hp.nelts;
  let vsize = vec::len(hp.elts);
  
  if vsize < n {
    let new_capacity = vsize + (vsize  >> 1u);
    log(info, #fmt("grow heap capacity upto %u", new_capacity));
    
    vec::grow(hp.elts, new_capacity , none);
  }
  
  // push n      
  hp.elts[n] = some(val);
  hp.nelts += 1u;
  
  if hp.nelts > 1u {
    // loop n -> root or current > parent:
    //   swap current <-> parent if parent > current
    let mut i = n;
    while i != 0u {
      let par = uint::div_floor(i-1u, 2u);
      let current = _get(hp.elts, i);
      if hp.comp(_get(hp.elts, par), current) {
        vec::swap(hp.elts, i, par);
        i = par;
      } else {
        break;
      }
    }
  }
}

fn pop<T: copy>(hp: heap<T>) -> node<T> {
  if hp.nelts == 0u {
    ret none;
  }
    
  let root = hp.elts[0];
  hp.elts[0] = none;
  let mut cur = 0u;
  let size = hp.nelts;
  hp.nelts -= 1u;
  
  if hp.nelts == 0u {
    ret root;
  }
  
  vec::swap(hp.elts, 0u, size - 1u);
  while true {
    let left_pos = cur * 2u + 1u;
    let right_pos = cur * 2u + 2u;
    
    if left_pos >= size {
      break;
    }
    
    let left = hp.elts[left_pos];
    let right = hp.elts[right_pos];
    if option::is_none(left) && option::is_none(right) {
      break;
    } 
    
    let child_pos: uint = if option::is_some(right) && hp.comp(option::get(left), option::get(right)) {
      right_pos
    } else {
      left_pos
    };
    
    if hp.comp(_get(hp.elts, cur), _get(hp.elts, child_pos)) {
      vec::swap(hp.elts, cur, child_pos);
      cur = child_pos;
    } else {
      break;
    }
  }
  ret root;
}


// fn dump<T: copy>(hp: heap<T>) -> [T]{
//   ret vec::filter_map(vec::from_mut(hp.elts)) {|elt|
//     ret elt
//   }
// }


fn empty<T: copy>(hp: heap<T>) -> bool {
  ret hp.nelts == 0u;
}

fn top<T: copy>(hp: heap<T>) -> node<T> {
  ret hp.elts[0u];
}

#[cfg(test)]
mod tests{
  
  fn make_max_heap() -> heap<int> {
    fn leual(&&a: int,  &&b: int) -> bool {
      ret a < b;
    };
    
    let f = leual;
    ret create::<int>(f);
  }

  fn make_box_heap() -> heap<@int> {
    fn leual(&&a: @int,  &&b: @int) -> bool {
      ret a < b;
    };
    
    let f = leual;
    ret create::<@int>(f);
  }
  
  // #[test]
  // fn test_dump() {
  //   let hp = make_max_heap();
  //   push(hp, 11);
  //   push(hp, 5);
  //   push(hp, 8);
  //   push(hp, 3);
  //   push(hp, 4);
  //   push(hp, 15);
  //   assert(dump(hp) == [15, 5, 11, 3, 4, 8]);
  // }
  
  #[test]
  fn test_simple(){
    let hp = make_max_heap();
    assert hp.nelts == 0u;
    assert(empty(hp) == true);

    push(hp, 2);
    assert hp.nelts == 1u;
    assert top(hp) == some(2);
    assert empty(hp) == false;

    let i = pop(hp);
    assert i == some(2);
    assert hp.nelts == 0u;

    push(hp, 11);
    push(hp, 5);
    push(hp, 8);
    push(hp, 3);
    push(hp, 4);
    
    assert hp.nelts == 5u;
    assert(pop(hp) == some(11));

    assert(pop(hp) == some(8));
    assert(pop(hp) == some(5));
    assert(pop(hp) == some(4));
    assert(pop(hp) == some(3));
    assert(pop(hp) == none);
    assert(pop(hp) == none);
    assert(pop(hp) == none);
    assert(pop(hp) == none);
    assert(pop(hp) == none);
  }

  #[test]
  fn test_box_heap(){
    let hp = make_box_heap();
    let a = 1;
    let b = 99;
    push(hp, @a);
    push(hp, @b);
  }

}
